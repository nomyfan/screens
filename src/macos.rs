use crate::Screen;
use cocoa::base::{id, nil};
use cocoa::foundation::NSArray;
use cocoa::{appkit::NSScreen, foundation::NSString};
use core_graphics::display::{CGDirectDisplayID, CGDisplayMode, kDisplayModeNativeFlag};
use objc::{msg_send, sel, sel_impl};

impl Screen {
    pub fn screens() -> Vec<Self> {
        unsafe {
            // NSArray of NSScreen
            let ns_screens = NSScreen::screens(nil);
            // NSScreen
            let main_screen = NSScreen::mainScreen(nil);
            let count = ns_screens.count();

            let mut screens = Vec::with_capacity(count as usize);

            for i in 0..count {
                let ns_screen = ns_screens.objectAtIndex(i);
                if let Some(resolution) = resolve_native_resolution(ns_screen) {
                    let frame = ns_screen.frame();
                    let visible_frame = ns_screen.visibleFrame();
                    let primary = std::ptr::eq(main_screen, ns_screen);

                    screens.push(Screen {
                        logical_size: (frame.size.width, frame.size.height),
                        visible_size: (visible_frame.size.width, visible_frame.size.height),
                        resolution,
                        primary,
                    });
                }
            }

            return screens;
        }
    }
}

/// https://stackoverflow.com/questions/53595111/how-to-get-the-physical-display-resolution-on-macos
fn resolve_native_resolution<S>(ns_screen: S) -> Option<(u32, u32)>
where
    S: NSScreen,
{
    let device_description = unsafe { ns_screen.deviceDescription() };
    let display_id_key = unsafe { NSString::alloc(nil).init_str("NSScreenNumber") };
    #[allow(unexpected_cfgs)]
    let display_id_nsnumber: id =
        unsafe { msg_send![device_description, objectForKey:display_id_key] };
    #[allow(unexpected_cfgs)]
    let display_id: CGDirectDisplayID = unsafe { msg_send![display_id_nsnumber, unsignedIntValue] };
    let modes = CGDisplayMode::all_display_modes(display_id, std::ptr::null());

    if let Some(modes) = modes {
        let mut width = 0;
        let mut height = 0;
        for mode in modes.iter() {
            let pixel_width = mode.pixel_width();
            if ((mode.io_flags() & kDisplayModeNativeFlag) == 0) || mode.width() != pixel_width {
                continue;
            }

            if pixel_width > width {
                width = pixel_width;
                height = mode.pixel_height();
            }
        }

        return Some((width as u32, height as u32));
    }

    log::error!(
        "Cannot resolve native monitor resolution for display id {}",
        display_id
    );

    None
}
