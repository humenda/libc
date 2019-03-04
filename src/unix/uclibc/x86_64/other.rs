// Thestyle checker discourages the use of #[cfg], so this has to go into a
// separate module
pub type pthread_t = ::c_ulong;

pub const PTHREAD_STACK_MIN: usize = 16384;

s! {
    pub struct pthread_attr_t {
        __detachstate: ::c_int,
        __schedpolicy: ::c_int,
        __schedparam: __sched_param,
        __inheritsched: ::c_int,
        __scope: ::c_int,
        __guardsize: ::size_t,
        __stackaddr_set: ::c_int,
        __stackaddr: *mut ::c_void, // better don't use it
        __stacksize: ::size_t,
    }
}
