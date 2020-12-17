use objc_foundation::{INSObject, INSString, NSString};
use objc_id::Id;

pub trait INSURL: INSObject {
    fn from_str(s: &str) -> Id<Self> {
        unsafe {
            let s = NSString::from_str(s);
            let ptr = msg_send![class!(NSURL), URLWithString: s];
            Id::from_retained_ptr(ptr)
        }
    }

    fn absolute_string(&self) -> Id<NSString> {
        unsafe {
            let s = msg_send![self, absoluteString];
            Id::from_retained_ptr(s)
        }
    }
}

object_struct!(NSURL);

impl INSURL for NSURL {}
