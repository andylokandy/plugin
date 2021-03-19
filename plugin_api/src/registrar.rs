use crate::Endpoint;

pub type PluginRegistrar = unsafe extern "C" fn(crate::allocator::HostAllocatorPtr) -> Plugin;
pub type PluginGetBuildInfo = extern "C" fn() -> BuildInfo;
pub static PLUGIN_REGISTRAR_SYMBOL: &[u8] = b"_plugin_registrar";
pub static PLUGIN_GET_BUILD_INFO_SYMBOL: &[u8] = b"_plugin_get_build_info";

#[repr(C)]
pub struct Plugin {
    pub name: &'static str,
    pub version: &'static str,
    pub endpoint: Box<dyn Endpoint>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildInfo {
    pub api_version: &'static str,
    pub target: &'static str,
    pub rustc: &'static str,
}

impl BuildInfo {
    pub const fn get() -> Self {
        Self {
            api_version: env!("API_VERSION"),
            target: env!("TARGET"),
            rustc: env!("RUSTC_VERSION"),
        }
    }
}

#[macro_export]
macro_rules! declare_plugin {
    ($plugin_name: expr, $plugin_version: expr, $endpoint_ctor : expr) => {
        #[no_mangle]
        pub unsafe extern "C" fn _plugin_get_build_info() -> $crate::registrar::BuildInfo {
            $crate::registrar::BuildInfo::get()
        }

        #[no_mangle]
        pub unsafe extern "C" fn _plugin_registrar(
            host_alloctor: $crate::allocator::HostAllocatorPtr,
        ) -> $crate::registrar::Plugin {
            $crate::allocator::host_alloctor::set_allocator(host_alloctor);
            $crate::registrar::Plugin {
                name: $plugin_name,
                version: $plugin_version,
                endpoint: Box::new($endpoint_ctor),
            }
        }
    };
}
