// SPDX-License-Identifier: GPL-2.0

//! Rust completion device

use core::cell::UnsafeCell;
// use core::ptr::null;

// use kernel::PointerWrapper;
use kernel::prelude::*;
use kernel::{chrdev, file::{self, File}, bindings};
use kernel::chrdev::Registration;
use kernel::io_buffer::{IoBufferReader, IoBufferWriter};

module! {
    type: CompletionDev,
    name: "completion",
    author: "yyx",
    description: "r4l_experiment completion device",
    license: "GPL",
}

struct Completion(UnsafeCell<bindings::completion>);

// static mut COMPLETION: *const Completion = null();

unsafe impl Sync for Completion {}
unsafe impl Send for Completion {}

impl Completion {
    /// Create a new Completion and init it
    pub(crate) fn new() -> Self {
        let completion = Completion(UnsafeCell::new(bindings::completion::default()));
        unsafe{ bindings::init_completion(completion.0.get()); }
        completion
    }

    /// Wait for a completion
    pub(crate) fn wait_for_completion(&self) {
        pr_info!("wait_for_completion: ptr address is {:?}", self.0.get());
        unsafe{ bindings::wait_for_completion(self.0.get() ) }
    }

    /// Complete a completion
    pub(crate) fn complete(&self) {
        pr_info!("complete: ptr address is {:?}", self.0.get());
        unsafe{ bindings::complete(self.0.get()) }
    }
}

struct CompletionDev {
    _dev: Pin<Box<chrdev::Registration<1>>>,
}

#[vtable]
impl file::Operations for Completion {
    type Data = Box<Completion>;

    fn open(_context: &(), _file: &File) -> Result<Box<Self>> {
        pr_info!("open completion device");
        // pr_info!("COMPLETION is {:?}", COMPLETION);
        // unsafe {
        //     if COMPLETION.is_null() {
        //         pr_info!("null");
        //         let box_completion = Box::try_new(Completion::new())?;
        //         COMPLETION = box_completion.as_ref() as *const _;
        //         pr_info!("after modify COMPLETION is {:?}", COMPLETION);
        //         Ok(box_completion)

        //     } else {
        //         pr_info!("no null");
        //         Ok(Box::from_pointer(COMPLETION as _))
        //     }
        // }
        Ok(
            Box::try_new(Completion::new())?
        )
    }

    fn read(
        data: &Self,
        _file: &File,
        _writer: &mut impl IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("function read is invoked");
        pr_info!("read ptr addr: {:?}", data as *const _);
        data.wait_for_completion();
        Ok(0)
    }

    fn write(
        data: &Self,
        _file: &File,
        reader: &mut impl IoBufferReader,
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("function write is invoked");
        pr_info!("write ptr addr: {:?}", data as *const _);
        data.complete();
        Ok(reader.len())
    }
}



impl kernel::Module for CompletionDev {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("init completion device");
        let mut chrdev_reg = Registration::new_pinned(name, 0, module)?;
        
        chrdev_reg.as_mut().register::<Completion>()?;

        Ok(CompletionDev { _dev: chrdev_reg })
    }
}

impl Drop for CompletionDev {
    fn drop(&mut self) {
        pr_info!("drop completion device");
    }
}

