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

    pub unsafe fn set_allocator(allocator: HostAllocatorPtr) {
        HOST_ALLOCATOR.alloc_fn.store(allocator.alloc_fn as usize, Ordering::SeqCst);
        HOST_ALLOCATOR.dealloc_fn.store(allocator.dealloc_fn as usize, Ordering::SeqCst);
    }

    struct HostAllocator {
        alloc_fn: AtomicUsize,
        dealloc_fn: AtomicUsize
    }

    #[global_allocator]
    static HOST_ALLOCATOR: HostAllocator = HostAllocator {
        alloc_fn: AtomicUsize::new(0),
        dealloc_fn: AtomicUsize::new(0),
    };

    unsafe impl GlobalAlloc for HostAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            (std::mem::transmute::<_, AllocFn>(self.alloc_fn.load(Ordering::Relaxed)))(layout)
        }
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            (std::mem::transmute::<_, DeallocFn>(self.dealloc_fn.load(Ordering::Relaxed)))(ptr, layout)
        }
    }
}
