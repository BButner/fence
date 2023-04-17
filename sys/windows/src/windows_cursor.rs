use std::{io::Error, mem, sync::Arc};

use backend::{display::Display, grpc::State};
use tokio::sync::Mutex;
use windows::Win32::{
    Foundation::{BOOL, LPARAM, LRESULT, RECT, TRUE, WPARAM},
    Graphics::Gdi::{EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFOEXW},
    UI::WindowsAndMessaging::{
        CallNextHookEx, GetMessageA, SetCursorPos, SetWindowsHookExW, HHOOK, MOUSEHOOKSTRUCT,
        WH_MOUSE_LL,
    },
};

pub struct WindowsCursorHandler {}

static mut STATE: Option<Arc<Mutex<State>>> = None;
static mut MOUSE_HOOK: Option<HHOOK> = None;

impl WindowsCursorHandler {
    pub async fn new() -> Self {
        let result = backend::init_fence(Self::enumerate_monitors).await;

        unsafe { STATE = result };

        Self {}
    }

    pub fn start_mouse_hook(&self) {
        std::thread::spawn(|| unsafe {
            let result = SetWindowsHookExW(WH_MOUSE_LL, Some(Self::mouse_hook_callback), None, 0);

            match result {
                Ok(hook) => {
                    MOUSE_HOOK = Some(hook);

                    println!("Hook installed: {:?}", hook);

                    loop {
                        let mut msg = std::mem::zeroed();
                        GetMessageA(&mut msg, None, 0, 0);
                    }

                    // println!("Hook uninstalled");

                    // UnhookWindowsHookEx(hook);
                }
                Err(e) => println!("Error: {:?}", e),
            }
        });
    }

    fn enumerate_monitors() -> Vec<Display> {
        let mut monitors = Vec::<MONITORINFOEXW>::new();
        let userdata = &mut monitors as *mut _;

        let result = unsafe {
            EnumDisplayMonitors(
                HDC::default(),
                None,
                Some(Self::enumerate_monitors_callback),
                LPARAM(userdata as isize),
            )
        };

        if result != TRUE {
            panic!("Could not enumerate monitors: {}", Error::last_os_error());
        }

        monitors
            .iter()
            .map(|monitor| Display {
                width: monitor.monitorInfo.rcMonitor.right - monitor.monitorInfo.rcMonitor.left,
                height: monitor.monitorInfo.rcMonitor.bottom - monitor.monitorInfo.rcMonitor.top,
                top: monitor.monitorInfo.rcMonitor.top,
                left: monitor.monitorInfo.rcMonitor.left,
                is_primary: monitor.monitorInfo.dwFlags == 1,
            })
            .collect()
    }

    unsafe extern "system" fn mouse_hook_callback(
        code: i32,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        let mouse_info = (l_param.0 as *const MOUSEHOOKSTRUCT).read();
        let mouse_x = mouse_info.pt.x;
        let mouse_y = mouse_info.pt.y;

        let mut state = STATE.as_ref().unwrap().blocking_lock();

        let result = state.try_update_cursor_location(mouse_x, mouse_y);

        if result.updated {
            return CallNextHookEx(MOUSE_HOOK.unwrap(), code, w_param, l_param);
        } else {
            SetCursorPos(result.location.x, result.location.y);
        }

        LRESULT(1)
    }

    unsafe extern "system" fn enumerate_monitors_callback(
        monitor: HMONITOR,
        _: HDC,
        _: *mut RECT,
        userdata: LPARAM,
    ) -> BOOL {
        let monitors: &mut Vec<MONITORINFOEXW> = mem::transmute(userdata);
        let mut monitor_info: MONITORINFOEXW = mem::zeroed();

        monitor_info.monitorInfo.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;
        let monitor_info_ptr = <*mut _>::cast(&mut monitor_info.monitorInfo);

        let result = GetMonitorInfoW(monitor, monitor_info_ptr);
        if result == TRUE {
            monitors.push(monitor_info);
        }

        TRUE
    }
}
