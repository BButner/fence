use std::sync::Arc;

use backend::grpc::State;
use tokio::sync::Mutex;
use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
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
        let result = backend::init_fence().await;

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
}
