pub struct MustUseChannel<S: DropHelper> {
    _s: S,
    chan: i32,
}

impl<S: DropHelper> MustUseChannel<S> {
    pub fn new(chan: i32) -> MustUseChannel<Undroppable> {
        MustUseChannel {
            _s: Undroppable {},
            chan,
        }
    }

    pub fn send(self, message: i32) -> MustUseChannel<Droppable> {
        let new_a = MustUseChannel {
            _s: Droppable {},
            chan: self.chan,
        };
        let to_drop = std::mem::ManuallyDrop::new(self);
        std::mem::forget(to_drop);
        new_a
    }
}

pub trait DropHelper: Sized {
    fn specialized_drop(_a: &mut MustUseChannel<Self>);
}

// State type options.
pub struct Undroppable {} // expecting to call send on a channel
pub struct Droppable {} // channel has been used and can be dropped now

impl DropHelper for Undroppable {
    fn specialized_drop(_a: &mut MustUseChannel<Self>) {
        extern "C" {
            // This will show a useful error instead of gibberish
            #[link_name = "\n\nERROR: `Undroppable` implicitly dropped without calling `send`\n\n"]
            #[allow(dead_code)]
            fn unlinkable() -> !;
        }
        unsafe { unlinkable() }
    }
}

impl DropHelper for Droppable {
    fn specialized_drop(_a: &mut MustUseChannel<Self>) {
        // Do nothing
    }
}

impl<S: DropHelper> std::ops::Drop for MustUseChannel<S> {
    fn drop(&mut self) {
        S::specialized_drop(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send() {
        let a = MustUseChannel {
            _s: Undroppable {},
            chan: 10,
        };
        let _b = a.send(10);
    }

    // This code will not compile, because we left code in Undroppable state
    //  #[test]
    //fn test_no_send() {
    //    let _a = MustUseChannel {
    //        _s: Undroppable {},
    //        chan: 10,
    //    };
    // }
}
