use crate::{Rect, Screen};
use windows::Win32::Foundation::{LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{
    DEVMODEW, ENUM_CURRENT_SETTINGS, EnumDisplayMonitors, EnumDisplaySettingsW, GetMonitorInfoW,
    HDC, HMONITOR, MONITORINFO, MONITORINFOEXW,
};
use windows::Win32::UI::WindowsAndMessaging::MONITORINFOF_PRIMARY;
use windows::core::{BOOL, PCWSTR};

impl From<RECT> for Rect {
    fn from(value: RECT) -> Self {
        let x = value.left;
        let y = value.top;
        let width = value.right - value.left;
        let height = value.bottom - value.top;
        Self {
            x: x as f64,
            y: y as f64,
            width: width as f64,
            height: height as f64,
        }
    }
}

unsafe extern "system" fn monitor_enum_proc(
    hmonitor: HMONITOR,
    _hdc: HDC,
    _rect: *mut windows::Win32::Foundation::RECT,
    lparam: LPARAM,
) -> BOOL {
    let ptr = lparam.0 as *mut Vec<Screen>;
    let screens = unsafe { &mut *ptr };

    let monitor_info = unsafe {
        let mut monitor_info = MONITORINFOEXW::default();
        monitor_info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
        let ptr = &mut monitor_info as *mut MONITORINFOEXW as *mut MONITORINFO;
        if !GetMonitorInfoW(hmonitor, ptr).as_bool() {
            log::error!("GetMonitorInfoW");
            return BOOL::from(false);
        }
        monitor_info
    };

    let dev_mode = unsafe {
        let mut dev_mode = DEVMODEW {
            dmSize: std::mem::size_of::<DEVMODEW>() as u16,
            ..Default::default()
        };
        let sz_device_ptr = monitor_info.szDevice.as_ptr();
        if !EnumDisplaySettingsW(PCWSTR(sz_device_ptr), ENUM_CURRENT_SETTINGS, &mut dev_mode)
            .as_bool()
        {
            log::error!("EnumDisplaySettingsW");
            return BOOL::from(false);
        }
        dev_mode
    };

    let frame = Rect::from(monitor_info.monitorInfo.rcMonitor);
    let visible_frame = Rect::from(monitor_info.monitorInfo.rcWork);
    let physical_width = dev_mode.dmPelsWidth;
    let physical_height = dev_mode.dmPelsHeight;
    let primary = (monitor_info.monitorInfo.dwFlags & MONITORINFOF_PRIMARY) == MONITORINFOF_PRIMARY;

    let screen = Screen {
        frame,
        visible_frame,
        resolution: (physical_width, physical_height).into(),
        primary,
    };

    screens.push(screen);

    // Continue to enumerate
    BOOL::from(true)
}

impl Screen {
    pub fn screens() -> Vec<Self> {
        let screens: Box<Vec<Screen>> = Box::new(Vec::new());
        let ptr = Box::into_raw(screens) as isize;

        unsafe {
            let _ = EnumDisplayMonitors(None, None, Some(monitor_enum_proc), LPARAM(ptr));

            *Box::from_raw(ptr as *mut Vec<Screen>)
        }
    }
}
