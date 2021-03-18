use crate::Endpoint;

pub type PluginRegistrar = unsafe extern "C" fn(crate::allocator::HostAllocatorPtr) -> Plugin;
pub static PLUGIN_REGISTRAR_SYMBOL: &[u8] = b"_plugin_registrar";

#[repr(C)]
pub struct Plugin {
    pub name: &'static str,
    pub version: &'static str,
    pub plugin_build_info: PluginBuildInfo,
    pub endpoint_builder: fn() -> Box<dyn Endpoint>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginBuildInfo {
    pub api_version: &'static str,
    pub target: &'static str,
    pub host: &'static str,
    pub rustc: &'static str,
}

impl PluginBuildInfo {
    pub const fn get() -> Self {
        Self {
            api_version: env!("API_VERSION"),
            target: env!("TARGET"),
            host: env!("HOST"),
            rustc: env!("RUSTC_VERSION"),
        }
    }
}

#[macro_export]
macro_rules! declare_plugin {
    ($endpoint_ctor : expr) => {
        fn _endpoint_builder() -> Box<dyn $crate::Endpoint> {
            Box::new($endpoint_ctor)
        }

        #[no_mangle]
        pub unsafe extern "C" fn _plugin_registrar(
            host_alloctor: $crate::allocator::HostAllocatorPtr,
        ) -> $crate::registrar::Plugin {
            $crate::allocator::host_alloctor::set_allocator(host_alloctor);
            $crate::registrar::Plugin {
                name: env!("CARGO_PKG_NAME"),
                version: env!("CARGO_PKG_VERSION"),
                plugin_build_info: $crate::registrar::PluginBuildInfo::get(),
                endpoint_builder: _endpoint_builder,
            }
        }
    };
}
