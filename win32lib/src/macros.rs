/// Creates a macro that can make a C-style enum with defined values
#[macro_export]
macro_rules! ENUM {
    {enum $name:ident { $($variant:ident = $value:expr,)+ }} => {
        pub type $name = u32;
        $(pub const $variant: $name = $value;)+
    };
}

#[macro_export]
macro_rules! opaque {
    ($name:ident, $inner:ident) => {
        #[repr(C)] pub struct $inner { private: [u8; 0] }
        pub type $name = *mut $inner;
    }
}

#[macro_export]
macro_rules! STRUCT {
    ($(#[$attrs:meta])* struct $name:ident {
        $($field:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] #[derive(Copy)] $(#[$attrs])*
        pub struct $name {
            $(pub $field: $ftype,)+
        }

        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }

        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { std::mem::zeroed() } }
        }
    );
}

// #[macro_export]
// macro_rules! UNION {
//     ($(#[$attrs:meta])* union $name:ident {
//         [$stype:ty; $ssize:expr],
//         $($variant:ident $variant_mut:ident: $ftype:ty,)+
//     }) => (
//         #[repr(C)] $(#[$attrs])*
//         pub struct $name([$stype; $ssize]);

//         impl Copy for $name{}
//         impl Clone for $name {
//             #[inline]
//             fn clone(&self) -> $name { *self }
//         }

//         impl $name {$(
//             #[inline]
//             pub fn $variant(&self) -> $ftype {
//                 &*(self as *const _ as *const $ftype)
//             }

//             #[inline]
//             pub fn $variant_mut(&mut self) -> &mut $ftype {
//                 &mut *(self as *mut _ as *mut $ftype)
//             }
//         )+}
//     );
// }

#[macro_export]
macro_rules! UNION {
    ($(#[$attrs:meta])* union $name:ident {
        $($field:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] $(#[$attrs])*
        pub union $name {
            $(pub $field: $ftype,)+
        }

        impl Copy for $name{}
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
    );
}
