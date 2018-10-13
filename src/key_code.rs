use cocoa::appkit::{
    NSEvent,
    NSEventModifierFlags,
    NSEventSubtype::NSScreenChangedEventType,
    NSEventType::NSSystemDefined,
};
use cocoa::base::{id, nil};
use cocoa::foundation::NSPoint;
use core_graphics::event::{
    CGEvent,
    // CGEventPost,
    CGEventRef,
    CGEventTapLocation,
    CGKeyCode,
    KeyCode,
};
use foreign_types::{ForeignType, ForeignTypeRef};

// impl KeyCode {
//     pub const RETURN: CGKeyCode = 0x24;
//     pub const TAB: CGKeyCode = 0x30;
//     pub const SPACE: CGKeyCode = 0x31;
//     pub const DELETE: CGKeyCode = 0x33;
//     pub const ESCAPE: CGKeyCode = 0x35;
//     pub const COMMAND: CGKeyCode = 0x37;
//     pub const SHIFT: CGKeyCode = 0x38;
//     pub const CAPS_LOCK: CGKeyCode = 0x39;
//     pub const OPTION: CGKeyCode = 0x3A;
//     pub const CONTROL: CGKeyCode = 0x3B;
//     pub const RIGHT_COMMAND: CGKeyCode = 0x36;
//     pub const RIGHT_SHIFT: CGKeyCode = 0x3C;
//     pub const RIGHT_OPTION: CGKeyCode = 0x3D;
//     pub const RIGHT_CONTROL: CGKeyCode = 0x3E;
//     pub const FUNCTION: CGKeyCode = 0x3F;
//     pub const VOLUME_UP: CGKeyCode = 0x48;
//     pub const VOLUME_DOWN: CGKeyCode = 0x49;
//     pub const MUTE: CGKeyCode = 0x4A;
// }

//   kVK_F13                       = 0x69,
//   kVK_F14                       = 0x6B,
//   kVK_F15                       = 0x71,
//   kVK_F16                       = 0x6A,
//   kVK_F17                       = 0x40,
//   kVK_F18                       = 0x4F,
//   kVK_F19                       = 0x50,
//   kVK_F20                       = 0x5A,
//
// enum {
//   kVK_Return                    = 0x24,
//   kVK_Tab                       = 0x30,
//   kVK_Space                     = 0x31,
//   kVK_Delete                    = 0x33,
//   kVK_Escape                    = 0x35,
//   kVK_Command                   = 0x37,
//   kVK_Shift                     = 0x38,
//   kVK_CapsLock                  = 0x39,
//   kVK_Option                    = 0x3A,
//   kVK_Control                   = 0x3B,
//   kVK_RightCommand              = 0x36,
//   kVK_RightShift                = 0x3C,
//   kVK_RightOption               = 0x3D,
//   kVK_RightControl              = 0x3E,
//   kVK_Function                  = 0x3F,
//   kVK_F17                       = 0x40,
//   kVK_VolumeUp                  = 0x48,
//   kVK_VolumeDown                = 0x49,
//   kVK_Mute                      = 0x4A,
//   kVK_F18                       = 0x4F,
//   kVK_F19                       = 0x50,
//   kVK_F20                       = 0x5A,
//   kVK_F5                        = 0x60,
//   kVK_F6                        = 0x61,
//   kVK_F7                        = 0x62,
//   kVK_F3                        = 0x63,
//   kVK_F8                        = 0x64,
//   kVK_F9                        = 0x65,
//   kVK_F11                       = 0x67,
//   kVK_F13                       = 0x69,
//   kVK_F16                       = 0x6A,
//   kVK_F14                       = 0x6B,
//   kVK_F10                       = 0x6D,
//   kVK_F12                       = 0x6F,
//   kVK_F15                       = 0x71,
//   kVK_Help                      = 0x72,
//   kVK_Home                      = 0x73,
//   kVK_PageUp                    = 0x74,
//   kVK_ForwardDelete             = 0x75,
//   kVK_F4                        = 0x76,
//   kVK_End                       = 0x77,
//   kVK_F2                        = 0x78,
//   kVK_PageDown                  = 0x79,
//   kVK_F1                        = 0x7A,
//   kVK_LeftArrow                 = 0x7B,
//   kVK_RightArrow                = 0x7C,
//   kVK_DownArrow                 = 0x7D,
//   kVK_UpArrow                   = 0x7E
// };



    // unsafe fn otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2_(
    //     _: Self,
    //     eventType: NSEventType,
    //     location: NSPoint,
    //     modifierFlags: NSEventModifierFlags,
    //     timestamp: NSTimeInterval,
    //     windowNumber: NSInteger,
    //     context: id /* (NSGraphicsContext *) */,
    //     subtype: NSEventSubtype,
    //     data1: NSInteger,
    //     data2: NSInteger) -> id /* (NSEvent *) */


// https://stackoverflow.com/questions/11045814/emulate-media-key-press-on-mac/11048135#11048135
// https://stackoverflow.com/questions/10459085/cocoa-simulate-macbook-upper-keys-multimedia-keys/50574159#50574159
unsafe fn press_play() {
    let NX_KEYTYPE_PLAY = 16;
    let code = NX_KEYTYPE_PLAY;

    // let key_down = NSEvent::otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2_(
    //     nil,
    //     NSSystemDefined,
    //     NSPoint::new(0.0, 0.0),
    //     // NSEventModifierFlags::NSDeviceIndependentModifierFlagsMask, // 0xa00 0xb00
    //     // 0xa00,
    //     // NSEventModifierFlags::empty(),
    //     NSEventModifierFlags::from_bits(0xa00).unwrap(),
    //     0.0,
    //     0,
    //     nil,
    //     NSScreenChangedEventType,
    //     (code << 16 as i32) | (0xa << 8 as i32),
    //     -1
    // );
    // let event = key_down.CGEvent() as *mut CGEvent;
    // let event = &*event;
    // event.post(CGEventTapLocation::HID);
    //
    // let key_up = NSEvent::otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2_(
    //     nil,
    //     NSSystemDefined,
    //     NSPoint::new(0.0, 0.0),
    //     // NSEventModifierFlags::NSDeviceIndependentModifierFlagsMask,
    //     // NSEventModifierFlags::empty(),
    //     NSEventModifierFlags::from_bits(0xb00).unwrap(),
    //     0.0,
    //     0,
    //     nil,
    //     NSScreenChangedEventType,
    //     (code << 16 as i32) | (0xb << 8 as i32),
    //     -1
    // );
    // let event = key_up.CGEvent() as *mut CGEvent;
    // let event = &*event;
    // event.post(CGEventTapLocation::HID);

    let event: id = msg_send![class!(NSEvent), otherEventWithType:NSSystemDefined
                                         location:NSPoint::new(0.0, 0.0)
                                    modifierFlags:0xa00
                                        timestamp:0.0
                                     windowNumber:0
                                          context:nil
                                          subtype:NSScreenChangedEventType
                                            data1:(code << 16 as i32) | (0xa << 8 as i32)
                                            data2:-1];

    // let cg_event: *mut CGEvent = msg_send![event, CGEvent];
    // let n_cg_event = &*cg_event;
    // println!("{}", cg_event.as_ptr());
    // n_cg_event.post(CGEventTapLocation::HID);
    // println!("Failed");

    let cg_event_ref: CGEventRef = msg_send![event, CGEvent];
    // CGEventPost(CGEventTapLocation::HID, cg_event_ref);
    // let cg_event = CGEvent::from_ptr(cg_event_ref);
    println!("What");
    // let cg_event: CGEvent = cg_event_ref.to_owned();  // TODO: doesn't work
        // let handle: *mut $ctype = $clone($crate::ForeignTypeRef::as_ptr(self));
        // $crate::ForeignType::from_ptr(handle)
    let fucking_handle = CGEventRef::as_ptr(&cg_event_ref);
    let cg_event = CGEvent::from_ptr(fucking_handle);
    println!("Fuck");
    cg_event.post(CGEventTapLocation::HID);  // fucking segfaults
    println!("Failed");

    // let event: id = msg_send![class!(NSEvent), otherEventWithType:NSSystemDefined
    //                                      location:NSPoint::new(0.0, 0.0)
    //                                 modifierFlags:0xb00
    //                                     timestamp:0.0
    //                                  windowNumber:0
    //                                       context:nil
    //                                       subtype:NSScreenChangedEventType
    //                                         data1:(code << 16 as i32) | (0xb << 8 as i32)
    //                                         data2:-1];
    //
    // // let cg_event: *mut CGEvent = msg_send![event, CGEvent];
    // // let cg_event = &*cg_event;
    // // cg_event.post(CGEventTapLocation::HID);
    // let cg_event_ref: CGEventRef = msg_send![event, CGEvent];
    // let fucking_handle = CGEventRef::as_ptr(&cg_event_ref);
    // let cg_event = CGEvent::from_ptr(fucking_handle);
    // cg_event.post(CGEventTapLocation::HID);
}


#[cfg(test)]
mod tests {
use super::*;

#[test]
fn send_media_key_event() {
    unsafe {
        press_play();
    }
}
}
