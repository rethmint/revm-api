use tokio::runtime::Runtime;
use std::sync::Once;

static mut RUNTIME: Option<Runtime> = None;
static INIT: Once = Once::new();

pub fn get_runtime() -> &'static Runtime {
    unsafe {
        INIT.call_once(|| {
            RUNTIME = Some(Runtime::new().unwrap());
        });
        RUNTIME.as_ref().unwrap()
    }
}