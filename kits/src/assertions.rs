#[macro_export]
macro_rules! kits_assert {
    ($cond:expr) => {
        assert!($cond);
    };
}

//marks code paths that should never execute
#[macro_export]
macro_rules! assert_not_reached {
    ()=> {
        panic!("ASSERT_NOT_REACHED");
    };
}


//placeholder
pub fn not_implemented() -> ! {
    panic!("NOT_IMPLEMENTED");
}