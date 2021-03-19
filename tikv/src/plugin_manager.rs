use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
    sync::{mpsc, Arc, RwLock},
    thread,
    time::Duration,
};

use libloading::{Library, Symbol};
use notify::{DebouncedEvent, RecursiveMode, Watcher};
use plugin_api::{BoxFuture, Endpoint, Store};
use semver::{Version, VersionReq};

pub struct PluginManager {
    plugins: Arc<RwLock<HashMap<String, (Version, Box<dyn Endpoint>, OsString)>>>,
}

impl PluginManager {
    pub fn start(plugin_path: impl Into<PathBuf>) -> PluginManager {
        let plugin_path = plugin_path.into();

        let plugins = Arc::new(RwLock::new(HashMap::new()));
        let plugins_cloned = plugins.clone();

        // Load plugins
        for entry in std::fs::read_dir(&plugin_path).unwrap() {
            let file = entry.unwrap().path();
            if is_library_file(&file) {
                Self::load_plugin(plugins.as_ref(), &file);
            }
        }

        // Watch the plugins path. Load / unload plugin on file create or remove event.
        thread::spawn(move || {
            let (tx, rx) = mpsc::channel();
            let mut watcher = notify::watcher(tx, Duration::from_secs(3)).unwrap();
            watcher
                .watch(plugin_path, RecursiveMode::NonRecursive)
                .unwrap();

            loop {
                match rx.recv() {
                    Ok(DebouncedEvent::Create(file)) => {
                        if is_library_file(&file) {
                            Self::load_plugin(plugins_cloned.as_ref(), &file);
                        }
                    }
                    Ok(DebouncedEvent::Remove(file)) => {
                        if is_library_file(&file) {
                            Self::unload_plugin(plugins_cloned.as_ref(), &file);
                        }
                    }
                    _ => (),
                }
            }
        });

        PluginManager { plugins }
    }

    pub fn handle_request<'a>(
        &'a self,
        name: &str,
        version_req: &VersionReq,
        req: Vec<u8>,
        store: Box<dyn Store>,
    ) -> Result<BoxFuture<Result<Vec<u8>, ()>>, String> {
        let plugins = self.plugins.read().unwrap();
        let (version, endpoint, _) = plugins
            .get(name)
            .ok_or_else(|| format!("Unable to find plugin \"{}\"", name))?;

        if version_req.matches(version) {
            Ok(endpoint.handle_request(req, store))
        } else {
            Err(format!(
                "Plugin \"{}\" is loaded, but the version is \"{}\" which mismatched with \"{}\"",
                name, version, version_req
            ))
        }
    }

    // TODO: Error handling
    fn load_plugin(
        plugins: &RwLock<HashMap<String, (Version, Box<dyn Endpoint>, OsString)>>,
        file: &Path,
    ) {
        unsafe {
            let lib = Library::new(file).unwrap();

            // It's important to check ABI before calling registrar.
            let get_build_info: Symbol<plugin_api::registrar::PluginGetBuildInfo> = lib
                .get(plugin_api::registrar::PLUGIN_GET_BUILD_INFO_SYMBOL)
                .unwrap();
            assert_eq!((get_build_info)(), plugin_api::registrar::BuildInfo::get());

            let registrar: Symbol<plugin_api::registrar::PluginRegistrar> = lib
                .get(plugin_api::registrar::PLUGIN_REGISTRAR_SYMBOL)
                .unwrap();
            let plugin = registrar(plugin_api::allocator::get_allocator());

            // TODO: check if the plugin has been loaded (even with different version)
            plugins.write().unwrap().insert(
                plugin.name.to_string(),
                (
                    Version::parse(plugin.version).unwrap(),
                    (plugin.endpoint_builder)(),
                    file.file_name().unwrap().to_owned(),
                ),
            );
            println!(
                "Host: plugin loaded: {} {} ({})",
                plugin.name,
                plugin.version,
                file.display()
            );

            std::mem::forget(lib);
        }
    }

    // TODO: Error handling
    fn unload_plugin(
        plugins: &RwLock<HashMap<String, (Version, Box<dyn Endpoint>, OsString)>>,
        path: &Path,
    ) {
        let plugin_name_version = plugins
            .read()
            .unwrap()
            .iter()
            .find(|(_, (_, _, file_name))| file_name == path.file_name().unwrap())
            .map(|(name, (version, _, _))| (name.clone(), version.clone()));

        if let Some(name_version) = plugin_name_version {
            plugins.write().unwrap().remove(&name_version.0);
            println!(
                "Host: plugin unloaded: {} {}",
                name_version.0, name_version.1
            );
        }
    }
}

#[cfg(target_os = "linux")]
fn is_library_file(path: &Path) -> bool {
    path.extension() == Some(OsStr::new("so"))
}

#[cfg(target_os = "macos")]
fn is_library_file(path: &Path) -> bool {
    path.extension() == Some(OsStr::new("dylib"))
}
