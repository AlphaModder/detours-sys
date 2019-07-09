use super::*;
use winapi::shared::minwindef::FALSE;

#[test]
fn link_success() {
    unsafe {
        assert_eq!(DetourIsHelperProcess(), FALSE)
    }
}
