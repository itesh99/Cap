use std::collections::HashMap;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

use super::{Bounds, CursorShape, Window};

use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, BOOL, FALSE, HWND, LPARAM, RECT, TRUE};
use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_CLOAKED};
use windows::Win32::Graphics::Gdi::{
    EnumDisplayDevicesW, EnumDisplayMonitors, GetMonitorInfoW, DISPLAY_DEVICEW, HDC, HMONITOR,
    MONITORINFO, MONITORINFOEXW,
};
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_FORMAT, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetCursorInfo, GetWindowRect, GetWindowTextLengthW, GetWindowTextW,
    GetWindowThreadProcessId, IsWindowVisible, LoadCursorW, SetForegroundWindow, CURSORINFO,
    IDC_APPSTARTING, IDC_ARROW, IDC_CROSS, IDC_HAND, IDC_HELP, IDC_IBEAM, IDC_NO, IDC_PERSON,
    IDC_PIN, IDC_SIZEALL, IDC_SIZENESW, IDC_SIZENS, IDC_SIZENWSE, IDC_SIZEWE, IDC_UPARROW,
    IDC_WAIT,
};

#[inline]
pub fn bring_window_to_focus(window_id: u32) {
    let _ = unsafe { SetForegroundWindow(HWND(window_id as isize)) };
}

pub fn get_cursor_shape(cursors: &DefaultCursors) -> CursorShape {
    let mut cursor_info = CURSORINFO {
        cbSize: std::mem::size_of::<CURSORINFO>() as u32,
        ..Default::default()
    };
    match unsafe { GetCursorInfo(&mut cursor_info) } {
        Ok(_) => match cursor_info.hCursor.0 {
            ptr if ptr == cursors.arrow => CursorShape::Arrow,
            ptr if ptr == cursors.ibeam => CursorShape::IBeam,
            ptr if ptr == cursors.wait => CursorShape::Wait,
            ptr if ptr == cursors.cross => CursorShape::Crosshair,
            ptr if ptr == cursors.up_arrow => CursorShape::ResizeUp,
            ptr if ptr == cursors.size_we => CursorShape::ResizeLeftRight,
            ptr if ptr == cursors.size_ns => CursorShape::ResizeUpDown,
            ptr if ptr == cursors.size_nwse => CursorShape::ResizeUpLeftAndDownRight,
            ptr if ptr == cursors.size_nesw => CursorShape::ResizeUpRightAndDownLeft,
            ptr if ptr == cursors.size_all => CursorShape::ResizeAll,
            ptr if ptr == cursors.hand => CursorShape::OpenHand,
            ptr if ptr == cursors.no => CursorShape::NotAllowed,
            ptr if ptr == cursors.appstarting => CursorShape::Appstarting,
            ptr if ptr == cursors.help => CursorShape::Help,
            ptr if ptr == cursors.pin || ptr == cursors.person => CursorShape::OpenHand,
            // Usually 0, meaning the cursor is hidden. On Windows 8+, a value of 2 means the cursor is supressed
            // as the user is using touch input instead.
            _ => CursorShape::Hidden,
        },
        Err(_) => CursorShape::Unknown,
    }
}

/// Keeps handles to default cursor.
/// Read more: [MS Doc - About Cursors](https://learn.microsoft.com/en-us/windows/win32/menurc/about-cursors)
pub struct DefaultCursors {
    arrow: isize,
    ibeam: isize,
    wait: isize,
    cross: isize,
    up_arrow: isize,
    size_nwse: isize,
    size_nesw: isize,
    size_we: isize,
    size_ns: isize,
    size_all: isize,
    no: isize,
    hand: isize,
    appstarting: isize,
    help: isize,
    pin: isize,
    person: isize,
}

impl Default for DefaultCursors {
    fn default() -> Self {
        #[inline]
        fn load_cursor(lpcursorname: PCWSTR) -> isize {
            unsafe { LoadCursorW(None, lpcursorname) }
                .expect("Failed to load default system cursors")
                .0
        }

        DefaultCursors {
            arrow: load_cursor(IDC_ARROW),
            ibeam: load_cursor(IDC_IBEAM),
            cross: load_cursor(IDC_CROSS),
            hand: load_cursor(IDC_HAND),
            help: load_cursor(IDC_HELP),
            no: load_cursor(IDC_NO),
            size_all: load_cursor(IDC_SIZEALL),
            size_ns: load_cursor(IDC_SIZENS),
            size_we: load_cursor(IDC_SIZEWE),
            size_nwse: load_cursor(IDC_SIZENWSE),
            size_nesw: load_cursor(IDC_SIZENESW),
            up_arrow: load_cursor(IDC_UPARROW),
            wait: load_cursor(IDC_WAIT),
            appstarting: load_cursor(IDC_APPSTARTING),
            pin: load_cursor(IDC_PIN),
            person: load_cursor(IDC_PERSON),
        }
    }
}

unsafe fn pid_to_exe_path(pid: u32) -> Result<PathBuf, windows::core::Error> {
    let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid)?;
    if handle.is_invalid() || handle.0 == 0 {
        tracing::error!("Invalid PID {}", pid);
    }
    let mut lpexename = [0u16; 1024];
    let mut lpdwsize = lpexename.len() as u32;

    let query = QueryFullProcessImageNameW(
        handle,
        PROCESS_NAME_FORMAT::default(),
        windows::core::PWSTR(lpexename.as_mut_ptr()),
        &mut lpdwsize,
    );
    CloseHandle(handle).ok();
    query?;

    let os_str = &OsString::from_wide(&lpexename[..lpdwsize as usize]);
    Ok(PathBuf::from(os_str))
}

pub fn get_on_screen_windows() -> Vec<Window> {
    let mut windows = Vec::<Window>::new();

    unsafe extern "system" fn enum_window_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        if hwnd.0 == 0 {
            return TRUE;
        }
        let windows = &mut *(lparam.0 as *mut Vec<Window>);

        if !IsWindowVisible(hwnd).as_bool() {
            return TRUE;
        }

        let mut pvattribute_cloaked = 0u32;
        DwmGetWindowAttribute(
            hwnd,
            DWMWA_CLOAKED,
            &mut pvattribute_cloaked as *mut _ as *mut std::ffi::c_void,
            std::mem::size_of::<u32>() as u32,
        )
        .ok();

        if pvattribute_cloaked != 0 {
            return TRUE;
        }

        let mut process_id = 0;
        let _thrad_id = GetWindowThreadProcessId(hwnd, Some(&mut process_id));

        let wnamelen = GetWindowTextLengthW(hwnd);
        if wnamelen == 0 {
            return TRUE;
        }
        let mut wname = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut wname);

        let owner_process_path = match pid_to_exe_path(process_id) {
            Ok(path) => path,
            Err(_) => return TRUE,
        };

        if owner_process_path.starts_with("C:\\Windows\\SystemApps") {
            return TRUE;
        }

        let owner_name = match owner_process_path.file_stem() {
            Some(exe_name) => exe_name.to_string_lossy().into_owned(),
            None => owner_process_path.to_string_lossy().into_owned(),
        };

        // Windows 10 build 1607 or later
        // Credits: TAO src/platform_impl/windows/dpi.rs
        const BASE_DPI: u32 = 96;
        let dpi = match GetDpiForWindow(hwnd) {
            0 => BASE_DPI,
            dpi => dpi,
        } as i32;

        let scale_factor = dpi as f64 / BASE_DPI as f64;
        let mut rect = RECT::default();
        GetWindowRect(hwnd, &mut rect).ok();

        let lpos_x = rect.left as f64 / scale_factor;
        let lpos_y = rect.top as f64 / scale_factor;

        let window = Window {
            window_id: hwnd.0 as u32,
            name: String::from_utf16_lossy(&wname[..len as usize]),
            owner_name,
            process_id,
            bounds: Bounds {
                x: match lpos_x {
                    x if x.is_sign_negative() => 0.0,
                    _ => lpos_x,
                },
                y: match lpos_y {
                    y if y.is_sign_negative() => 0.0,
                    _ => lpos_y,
                },
                width: (rect.right - rect.left) as f64 / scale_factor,
                height: (rect.bottom - rect.top) as f64 / scale_factor,
            },
        };

        windows.push(window);
        TRUE
    }

    let _ = unsafe {
        EnumWindows(
            Some(enum_window_proc),
            LPARAM(core::ptr::addr_of_mut!(windows) as isize),
        )
    };
    windows
}

pub fn monitor_bounds(id: u32) -> Bounds {
    let bounds = Bounds::default();
    let idx = 0u32;
    let lparams = (id, idx, bounds);
    unsafe extern "system" fn enum_monitor_proc(
        hmonitor: HMONITOR,
        _hdc: HDC,
        _lprc_clip: *mut RECT,
        lparam: LPARAM,
    ) -> BOOL {
        let (target_id, idx, bounds) = &mut *(lparam.0 as *mut (u32, u32, Bounds));

        let mut minfo = MONITORINFOEXW::default();
        minfo.monitorInfo.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
        if !GetMonitorInfoW(
            hmonitor,
            &mut minfo as *mut MONITORINFOEXW as *mut MONITORINFO,
        )
        .as_bool()
        {
            return TRUE;
        };

        *idx += 1;
        if idx != target_id {
            return TRUE;
        }

        let mi = minfo.monitorInfo;
        *bounds = Bounds {
            x: mi.rcMonitor.left as f64,
            y: mi.rcMonitor.top as f64,
            width: (mi.rcMonitor.right - mi.rcMonitor.left) as f64,
            height: (mi.rcMonitor.bottom - mi.rcMonitor.top) as f64,
        };

        FALSE
    }

    let _ = unsafe {
        EnumDisplayMonitors(
            None,
            None,
            Some(enum_monitor_proc),
            LPARAM(std::ptr::addr_of!(lparams) as isize),
        );
    };
    bounds
}

pub fn window_names() -> HashMap<u32, String> {
    let mut names = HashMap::new();
    unsafe extern "system" fn monitor_enum_proc(
        hmonitor: HMONITOR,
        _hdc: HDC,
        _lprc_clip: *mut RECT,
        lparam: LPARAM,
    ) -> BOOL {
        let monitors = &mut *(lparam.0 as *mut HashMap<u32, String>);

        let mut minfo = MONITORINFOEXW::default();
        minfo.monitorInfo.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
        if !GetMonitorInfoW(
            hmonitor,
            &mut minfo as *mut MONITORINFOEXW as *mut MONITORINFO,
        )
        .as_bool()
        {
            return TRUE;
        };

        let mut display_device = DISPLAY_DEVICEW::default();
        display_device.cb = std::mem::size_of::<DISPLAY_DEVICEW>() as u32;

        if !EnumDisplayDevicesW(PCWSTR(minfo.szDevice.as_ptr()), 0, &mut display_device, 0)
            .as_bool()
        {
            return TRUE;
        };

        let device_name = OsString::from_wide(&display_device.DeviceName)
            .to_string_lossy()
            .into_owned();
        let num = monitors.len() as u32;
        monitors.insert(num, device_name);
        TRUE
    }

    let _ = unsafe {
        EnumDisplayMonitors(
            None,
            None,
            Some(monitor_enum_proc),
            LPARAM(core::ptr::addr_of_mut!(names) as isize),
        )
    };
    names
}