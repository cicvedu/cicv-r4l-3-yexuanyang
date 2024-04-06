// SPDX-License-Identifier: GPL-2.0

//! Rust completion device
use core::ops::Deref;
use core::cell::UnsafeCell;

use kernel::prelude::*;
use kernel::{chrdev, file::{self, File}, bindings};
use kernel::io_buffer::{IoBufferReader, IoBufferWriter};
use kernel::sync::Mutex;
use kernel::task::Task;

module! {
    type: CompletionDev,
    name: "completion",
    author: "yyx",
    description: "r4l_experiment completion device",
    license: "GPL",
}

struct Completion(pub(crate) UnsafeCell<bindings::completion>);

static COMPLETION: Mutex<Option<Completion>> = unsafe {
    Mutex::new(None)
};

struct CompletionRust {}

unsafe impl Send for Completion {}

struct CompletionDev {
    _dev: Pin<Box<chrdev::Registration<1>>>,
}

#[vtable]
impl file::Operations for CompletionRust {
    type Data = ();

    fn open(_context: &(), _file: &File) -> Result<()> {
        pr_info!("function open is invoked");
        Ok(())
    }

    fn read(
        _data: (),
        _file: &File,
        _writer: &mut impl IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("function read is invoked");
        pr_info!("process {} is going to sleep", Task::current().pid());
        let completion:*mut bindings::completion;
        {
            let lock = COMPLETION.lock();
            completion = lock.deref().as_ref().unwrap().0.get();
        }
        unsafe { bindings::wait_for_completion(completion) };
        pr_info!("awoken {}", Task::current().pid());
        
        Ok(0)
    }

    fn write(
        _data: (),
        _file: &File,
        reader: &mut impl IoBufferReader,
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("function write is invoked");
        pr_info!("process {} awakening the readers", Task::current().pid());
        let completion:*mut bindings::completion;
        {
            let lock = COMPLETION.lock();
            completion = lock.deref().as_ref().unwrap().0.get()
        }
        unsafe { bindings::complete(completion) };
        Ok(reader.len())
    }
}

impl kernel::Module for CompletionDev {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("init completion device");
        let mut chrdev_reg = chrdev::Registration::new_pinned(name, 0, module)?;
        
        let mut completion = COMPLETION.lock();
        *completion = Some(Completion(UnsafeCell::new(bindings::completion::default())));
        let a = completion.deref().as_ref().unwrap().0.get();
        unsafe{ bindings::init_completion(a) };

        chrdev_reg.as_mut().register::<CompletionRust>()?;

        Ok(CompletionDev { _dev: chrdev_reg })
    }
}

impl Drop for CompletionDev {
    fn drop(&mut self) {
        pr_info!("drop completion device");
    }
}

