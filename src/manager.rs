use ffi::utils::res_to_result;
use libc::c_void;
use std::ffi::CString;
use notification::{Notification, ExternNotification};
use options::Options;
use ffi::manager as extern_manager;
use value_classes::value_id::ValueID;

pub struct Manager {
    ptr: *mut extern_manager::Manager,
    options: Options,
    watchers: Vec<Option<Box<WatcherWrapper>>>
}

unsafe impl Send for Manager {}
unsafe impl Sync for Manager {}

pub trait NotificationWatcher: Sync {
    fn on_notification(&self, Notification);
}

struct WatcherWrapper {
    watcher: Box<NotificationWatcher>
}

// watcher is actually a Box<WatcherWrapper>
extern "C" fn watcher_cb(notification: *const ExternNotification, watcher: *mut c_void) {
    let watcher_wrapper: &mut WatcherWrapper = unsafe { &mut *(watcher as *mut WatcherWrapper) };
    watcher_wrapper.watcher.on_notification(Notification::new(notification));
}

impl Manager {
    pub fn create(mut options: Options) -> Result<Manager, ()> {
        try!(options.lock());
        let external_manager = unsafe { extern_manager::manager_create() };
        if external_manager.is_null() {
            Err(())
        } else {
            Ok(Manager {
                ptr: external_manager,
                options: options,
                watchers: Vec::with_capacity(1)
            })
        }
    }

    /* disable for now, we don't need this anywhere yet
    pub fn get() -> Option<Manager> {
        let external_manager = unsafe { extern_manager::manager_get() };
        if external_manager.is_null() {
            None
        } else {
            Some(Manager { ptr: external_manager, options: Options::get().unwrap() })
        }
    }
    */

    pub fn add_watcher<T: 'static + NotificationWatcher>(&mut self, watcher: T) -> Result<usize, ()> {
        let mut watcher_wrapper = Box::new(WatcherWrapper { watcher: Box::new(watcher) });

        let watcher_ptr: *mut c_void = &mut *watcher_wrapper as *mut _ as *mut c_void;
        let api_res = unsafe {
            extern_manager::manager_add_watcher(self.ptr, watcher_cb, watcher_ptr)
        };

        if api_res {
            let position = self.watchers.len();
            self.watchers.push(Some(watcher_wrapper));
            Ok(position)
        } else {
            Err(())
        }
    }

    pub fn remove_watcher(&mut self, position: usize) -> Result<(), ()> {
        let wrapper = self.watchers[position].take();

        if let Some(mut wrapper) = wrapper {
            let result = self.remove_watcher_impl(&mut wrapper);
            if result.is_err() {
                // put the watcher back to the vec
                self.watchers[position] = Some(wrapper);
            }
            result
        } else {
            Err(())
        }
    }

    fn remove_watcher_impl(&self, wrapper: &mut WatcherWrapper) -> Result<(), ()> {
        let watcher_ptr: *mut c_void = wrapper as *mut _ as *mut c_void;
        res_to_result(unsafe {
            extern_manager::manager_remove_watcher(self.ptr, watcher_cb, watcher_ptr)
        })
    }

    pub fn add_driver(&mut self, device: &str) -> Result<(), ()> {
        let device = CString::new(device).unwrap();
        res_to_result(unsafe {
            extern_manager::manager_add_driver(self.ptr, device.as_ptr(), &extern_manager::ControllerInterface::ControllerInterface_Serial)
        })
    }

    pub fn add_usb_driver(&mut self) -> Result<(), ()> {
        let device = CString::new("HID Controller").unwrap();
        res_to_result(unsafe {
            extern_manager::manager_add_driver(self.ptr, device.as_ptr(), &extern_manager::ControllerInterface::ControllerInterface_Hid)
        })
    }

    pub fn remove_driver(&mut self, device: &str) -> Result<(), ()> {
        let device = CString::new(device).unwrap();
        res_to_result(unsafe {
            extern_manager::manager_remove_driver(self.ptr, device.as_ptr())
        })
    }

    pub fn get_poll_interval(&mut self) -> i32 {
        unsafe {
            extern_manager::manager_get_poll_interval(self.ptr)
        }
    }

    pub fn set_poll_interval(&mut self, interval_ms: i32, is_between_each_poll: bool) {
        unsafe {
            extern_manager::manager_set_poll_interval(self.ptr, interval_ms, is_between_each_poll)
        }
    }

    pub fn enable_poll_with_intensity(&mut self, vid: &ValueID, intensity: u8) -> bool {
        unsafe {
            extern_manager::manager_enable_poll_with_intensity(self.ptr, &vid.as_ozw_vid(), intensity)
        }
    }

    pub fn enable_poll(&mut self, vid: &ValueID) -> bool {
        unsafe {
            extern_manager::manager_enable_poll(self.ptr, &vid.as_ozw_vid())
        }
    }

    pub fn disable_poll(&mut self, vid: &ValueID) -> bool {
        unsafe {
            extern_manager::manager_disable_poll(self.ptr, &vid.as_ozw_vid())
        }
    }

    pub fn is_polled(&mut self, vid: &ValueID) -> bool {
        unsafe {
            extern_manager::manager_is_polled(self.ptr, &vid.as_ozw_vid())
        }
    }

    pub fn set_poll_intensity(&mut self, vid: &ValueID, intensity: u8) {
        unsafe {
            extern_manager::manager_set_poll_intensity(self.ptr, &vid.as_ozw_vid(), intensity)
        }
    }

    pub fn get_poll_intensity(&mut self, vid: &ValueID) -> u8 {
        unsafe {
            extern_manager::manager_get_poll_intensity(self.ptr, &vid.as_ozw_vid())
        }
    }
}

impl Drop for Manager {
    fn drop(&mut self) {
        let watchers: Vec<_> = self.watchers.drain(..).collect();
        for watcher in watchers {
            if let Some(mut watcher) = watcher {
                self.remove_watcher_impl(&mut watcher).unwrap();
            }
        }

        unsafe { extern_manager::manager_destroy() }
    }
}

