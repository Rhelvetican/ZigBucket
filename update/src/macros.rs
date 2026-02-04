#[macro_export]
macro_rules! s {
    ($s:literal) => {
        ::std::alloc::String::from($s)
    };

    ($i:ident) => {
        ::std::alloc::String::from($s)
    };
}
