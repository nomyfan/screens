use crate::Screen;
use cocoa::appkit::NSScreen;
use cocoa::base::nil;
use cocoa::foundation::NSArray;

impl Screen {
    pub fn screens() -> Vec<Self> {
        unsafe {
            // NSArray of NSScreen
            let screens = NSScreen::screens(nil);
            // NSScreen
            let main_screen = NSScreen::mainScreen(nil);
            let count = screens.count();

            let mut result = Vec::with_capacity(count as usize);

            for i in 0..count {
                let screen = screens.objectAtIndex(i);
                let frame = screen.frame();
                let visible_frame = screen.visibleFrame();
                let scale_factor = screen.backingScaleFactor();
                let is_main = std::ptr::eq(main_screen, screen);

                result.push(Screen {
                    width: frame.size.width,
                    height: frame.size.height,
                    visible_width: visible_frame.size.width,
                    visible_height: visible_frame.size.height,
                    scale_factor,
                    is_main,
                });
            }

            return result;
        }
    }
}
