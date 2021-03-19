use std::alloc::Layout;

type AllocFn = unsafe fn(Layout) -> *mut u8;
type DeallocFn = unsafe fn(*mut u8, Layout);

#[repr(C)]
pub struct HostAllocatorPtr {
    alloc_fn: AllocFn,
    dealloc_fn: DeallocFn,
}

#[cfg(feature = "host")]
pub fn get_allocator() -> HostAllocatorPtr {
    HostAllocatorPtr {
        alloc_fn: std::alloc::alloc,
        dealloc_fn: std::alloc::dealloc,
    }
}

#[cfg(not(feature = "host"))]
pub mod host_alloctor {
    use super::*;
    use std::alloc::{GlobalAlloc, Layout};
    use std::sync::atomic::{AtomicUsize, Ordering};

    static ALLOC_FN: AtomicUsize = AtomicUsize::new(0);
    static DEALLOC_FN: AtomicUsize = AtomicUsize::new(0);

    pub unsafe fn set_allocator(allocator: HostAllocatorPtr) {
        ALLOC_FN.store(allocator.alloc_fn as usize, Ordering::SeqCst);
        DEALLOC_FN.store(allocator.dealloc_fn as usize, Ordering::SeqCst);
    }

    #[global_allocator]
    static HOST_ALLOCATOR: HostAllocator = HostAllocator;

    struct HostAllocator;

    unsafe impl GlobalAlloc for HostAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            (std::mem::transmute::<_, AllocFn>(ALLOC_FN.load(Ordering::Relaxed)))(layout)
        }
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            (std::mem::transmute::<_, DeallocFn>(DEALLOC_FN.load(Ordering::Relaxed)))(ptr, layout)
        }
    }
}
