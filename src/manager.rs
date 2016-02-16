use utils::res_to_result;
use libc::c_void;
use std::ffi::CString;
use notification::{Notification, ExternNotification};
use options::Options;
use ffi::manager as extern_manager;

// Node stuff, will be moved to a separate mod
// Mapping comes from https://github.com/OpenZWave/open-zwave-control-panel/blob/master/zwavelib.cpp
c_like_enum! {
    NodeBasic {
        Controller = 1,
        StaticController = 2,
        Slave = 3,
        RoutingSlave = 4
    }
}

c_like_enum! {
    CommandClass {
        NoOperation = 0,
        Basic = 0x20,
        ControllerReplication = 0x21,
        ApplicationStatus = 0x22,
        ZipServices = 0x23,
        ZipServer = 0x24,
        SwitchBinary = 0x25,
        SwitchMultilevel = 0x26,
        SwitchAll = 0x27,
        SwitchToggleBinary = 0x28,
        SwitchToggleMultilevel = 0x29,
        ChimneyFan = 0x2A,
        SceneActivation = 0x2B,
        SceneActuatorConf = 0x2C,
        SceneControllerConf = 0x2D,
        ZipClient = 0x2E,
        ZipAdvServices = 0x2F,
        SensorBinary = 0x30,
        SensorMultilevel = 0x31,
        Meter = 0x32,
        Color = 0x33,
        ZipAdvClient = 0x34,
        MeterPulse = 0x35,
        ThermostatHeating = 0x38,
        ThermostatMode = 0x40,
        ThermostatOperatingState = 0x42,
        ThermostatSetpoint = 0x43,
        ThermostatFanMode = 0x44,
        ThermostatFanState = 0x45,
        ClimateControlSchedule = 0x46,
        ThermostatSetback = 0x47,
        DoorLockLogging = 0x4C,
        ScheduleEntryLock = 0x4E,
        BasicWindowCovering = 0x50,
        MtpWindowCovering = 0x51,
        Crc16Encap = 0x56,
        DeviceResetLocally = 0x5A,
        CentralScene = 0x5B,
        ZWavePlusInfo = 0x5E,
        MultiInstance = 0x60,
        DoorLock = 0x62,
        UserCode = 0x63,
        Configuration = 0x70,
        Alarm = 0x71,
        ManufacturerSpecific = 0x72,
        Powerlevel = 0x73,
        Protection = 0x75,
        Lock = 0x76,
        NodeNaming = 0x77,
        FirmwareUpdateMd = 0x7A,
        GroupingNane = 0x7B,
        RemoteAssociationActivate = 0x7C,
        RemoteAssociation = 0x7D,
        Battery = 0x80,
        Clock = 0x81,
        Hail = 0x82,
        WakeUp = 0x84,
        Association = 0x85,
        Version = 0x86,
        Indicator = 0x87,
        Proprietary = 0x88,
        Language = 0x89,
        Time = 0x8A,
        TimeParameters = 0x8B,
        GeographicLocation = 0x8C,
        Composite = 0x8D,
        MultiInstanceAssociation = 0x8E,
        MultiCmd = 0x8F,
        EnergyProduction = 0x90,
        ManufacturerProprietary = 0x91,
        ScreenMd = 0x92,
        ScreenAttributes = 0x93,
        SimpleAvControl = 0x94,
        AvContentDirectoryMd = 0x95,
        AvRendererStatus = 0x96,
        AvContentSearchMd = 0x97,
        Security = 0x98,
        AvTaggingMd = 0x99,
        IpConfiguration = 0x9A,
        AssociationCommandConfiguration = 0x9B,
        SensorAlarm = 0x9C,
        SilenceAlarm = 0x9D,
        SensorConfiguration = 0x9E,
        Mark = 0x9F,
        NonInteroperable = 0xF0
    }
}

pub struct Manager {
    ptr: *mut extern_manager::Manager,
    options: Options
}

pub struct Watcher {
    cb: Box<FnMut(Notification) -> ()>
}

impl Watcher {
    pub fn new<F: 'static>(callback: F) -> Watcher
    where F: FnMut(Notification) -> () {
        Watcher {
            cb: Box::new(callback)
        }
    }
}

extern "C" fn watcher_cb(notification: *const ExternNotification, watcher: *mut c_void) {
    let watcher: &mut Watcher = unsafe { &mut *(watcher as *mut Watcher) };
    (watcher.cb)(Notification::new(notification)); // TODO use thread synchronization
}

impl Manager {
    pub fn create(mut options: Options) -> Result<Manager, ()> {
        try!(options.lock());
        let external_manager = unsafe { extern_manager::manager_create() };
        if external_manager.is_null() {
            Err(())
        } else { 
            Ok(Manager { ptr: external_manager, options: options })
        }
    }

    pub fn get() -> Option<Manager> {
        let external_manager = unsafe { extern_manager::manager_get() };
        if external_manager.is_null() {
            None
        } else {
            Some(Manager { ptr: external_manager, options: Options::get().unwrap() })
        }
    }

    pub fn add_watcher(&mut self, watcher: &mut Watcher) -> Result<(), ()> {
        let watcher_ptr: *mut c_void = watcher as *mut _ as *mut c_void;
        res_to_result(unsafe {
            extern_manager::manager_add_watcher(self.ptr, watcher_cb, watcher_ptr)
        })
    }

    pub fn remove_watcher(&mut self, watcher: &mut Watcher) -> Result<(), ()> {
        let watcher_ptr: *mut c_void = watcher as *mut _ as *mut c_void;
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
}

impl Drop for Manager {
    fn drop(&mut self) {
        unsafe { extern_manager::manager_destroy() }
    }
}

