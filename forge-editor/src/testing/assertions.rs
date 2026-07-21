//! Assertion macros and helpers

/// Asserts that two values are equal
#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right, "assertion failed: `(left == right)`")
    };
    ($left:expr, $right:expr, $fmt:expr) => {
        assert_eq!($left, $right, $fmt, format_args!($fmt, $($arg),*))
    };
    ($left:expr, $right:expr, $fmt:expr, $($arg:tt)*) => {
        if !macro_rules_private_eq!($left, $right) {
            panic!(
                "assertion failed: `(left == right)`: {}
  left: `{:?}`, right: `{:?}`",
                $fmt, $left, $right
            );
        }
    };
}

/// Asserts that a value is not equal
#[macro_export]
macro_rules! assert_ne {
    ($left:expr, $right:expr) => {
        assert_ne!($left, $right, "assertion failed: `(left != right)`")
    };
    ($left:expr, $right:expr, $fmt:expr) => {
        assert_ne!($left, $right, $fmt, format_args!($fmt, $($arg),*))
    };
    ($left:expr, $right:expr, $fmt:expr, $($arg:tt)*) => {
        if macro_rules_private_eq!($left, $right) {
            panic!(
                "assertion failed: `(left != right)`: {}
  left: `{:?}`, right: `{:?}`",
                $fmt, $left, $right
            );
        }
    };
}

/// Asserts that a condition is true
#[macro_export]
macro_rules! assert {
    ($cond:expr) => {
        assert!($cond, "assertion failed: {}", stringify!($cond))
    };
    ($cond:expr, $fmt:expr) => {
        assert!($cond, $fmt, format_args!($fmt, $($arg),*))
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !$cond {
            panic!(
                "assertion failed: {}: {}",
                stringify!($cond),
                $fmt
            );
        }
    };
}

/// Asserts that a value is true
#[macro_export]
macro_rules! assert_true {
    ($expr:expr) => {
        assert!($expr, "assertion failed: `true`")
    };
    ($expr:expr, $fmt:expr) => {
        assert!($expr, $fmt, format_args!($fmt, $($arg),*))
    };
}

/// Asserts that a value is false
#[macro_export]
macro_rules! assert_false {
    ($expr:expr) => {
        assert!(!$expr, "assertion failed: `false`")
    };
    ($expr:expr, $fmt:expr) => {
        assert!(!$expr, $fmt, format_args!($fmt, $($arg),*))
    };
}

/// Asserts that an option is Some
#[macro_export]
macro_rules! assert_some {
    ($expr:expr) => {
        assert_some!($expr, "assertion failed: `Some(_)`")
    };
    ($expr:expr, $fmt:expr) => {
        assert_some!($expr, $fmt, format_args!($fmt, $($arg),*))
    };
}

/// Asserts that an option is None
#[macro_export]
macro_rules! assert_none {
    ($expr:expr) => {
        assert_none!($expr, "assertion failed: `None`")
    };
    ($expr:expr, $fmt:expr) => {
        assert_none!($expr, $fmt, format_args!($fmt, $($arg),*))
    };
}

/// Asserts that a result is Ok
#[macro_export]
macro_rules! assert_ok {
    ($expr:expr) => {
        assert_ok!($expr, "assertion failed: `Ok(_)`")
    };
    ($expr:expr, $fmt:expr) => {
        assert_ok!($expr, $fmt, format_args!($fmt, $($arg),*))
    };
}

/// Asserts that a result is Err
#[macro_export]
macro_rules! assert_err {
    ($expr:expr) => {
        assert_err!($expr, "assertion failed: `Err(_)`")
    };
    ($expr:expr, $fmt:expr) => {
        assert_err!($expr, $fmt, format_args!($fmt, $($arg),*))
    };
}

/// Asserts that an iterator is empty
#[macro_export]
macro_rules! assert_empty {
    ($expr:expr) => {
        assert_empty!($expr, "assertion failed: `len() == 0`")
    };
    ($expr:expr, $fmt:expr) => {
        assert_empty!($expr, $fmt, format_args!($fmt, $($arg),*))
    };
}

/// Asserts that an iterator is not empty
#[macro_export]
macro_rules! assert_not_empty {
    ($expr:expr) => {
        assert_not_empty!($expr, "assertion failed: `len() > 0`")
    };
    ($expr:expr, $fmt:expr) => {
        assert_not_empty!($expr, $fmt, format_args!($fmt, $($arg),*))
    };
}

/// Asserts that a value is within a range
#[macro_export]
macro_rules! assert_range {
    ($value:expr, min:expr, max:expr) => {
        assert_range!($value, min, max, "assertion failed: range check")
    };
    ($value:expr, min:expr, max:expr, $fmt:expr) => {
        assert_range!($value, min, max, $fmt, format_args!($fmt, $($arg),*))
    };
    ($value:expr, min:expr, max:expr, $fmt:expr, $($arg:tt)*) => {
        if $value < min || $value > max {
            panic!(
                "assertion failed: range check: {}
  value: `{:?}`, min: `{}`, max: `{}`",
                $fmt, $value, min, max
            );
        }
    };
}

/// Asserts that a value is greater than
#[macro_export]
macro_rules! assert_gt {
    ($left:expr, $right:expr) => {
        assert_gt!($left, $right, "assertion failed: `(left > right)`")
    };
    ($left:expr, $right:expr, $fmt:expr) => {
        assert_gt!($left, $right, $fmt, format_args!($fmt, $($arg),*))
    };
    ($left:expr, $right:expr, $fmt:expr, $($arg:tt)*) => {
        if !(left > right) {
            panic!(
                "assertion failed: `(left > right)`: {}
  left: `{:?}`, right: `{:?}`",
                $fmt, left, right
            );
        }
    };
}

/// Asserts that a value is greater than or equal
#[macro_export]
macro_rules! assert_ge {
    ($left:expr, $right:expr) => {
        assert_ge!($left, $right, "assertion failed: `(left >= right)`")
    };
    ($left:expr, $right:expr, $fmt:expr) => {
        assert_ge!($left, $right, $fmt, format_args!($fmt, $($arg),*))
    };
    ($left:expr, $right:expr, $fmt:expr, $($arg:tt)*) => {
        if !(left >= right) {
            panic!(
                "assertion failed: `(left >= right)`: {}
  left: `{:?}`, right: `{:?}`",
                $fmt, left, right
            );
        }
    };
}

/// Asserts that a value is less than
#[macro_export]
macro_rules! assert_lt {
    ($left:expr, $right:expr) => {
        assert_lt!($left, $right, "assertion failed: `(left < right)`")
    };
    ($left:expr, $right:expr, $fmt:expr) => {
        assert_lt!($left, $right, $fmt, format_args!($fmt, $($arg),*))
    };
    ($left:expr, $right:expr, $fmt:expr, $($arg:tt)*) => {
        if !(left < right) {
            panic!(
                "assertion failed: `(left < right)`: {}
  left: `{:?}`, right: `{:?}`",
                $fmt, left, right
            );
        }
    };
}

/// Asserts that a value is less than or equal
#[macro_export]
macro_rules! assert_le {
    ($left:expr, $right:expr) => {
        assert_le!($left, $right, "assertion failed: `(left <= right)`")
    };
    ($left:expr, $right:expr, $fmt:expr) => {
        assert_le!($left, $right, $fmt, format_args!($fmt, $($arg),*))
    };
    ($left:expr, $right:expr, $fmt:expr, $($arg:tt)*) => {
        if !(left <= right) {
            panic!(
                "assertion failed: `(left <= right)`: {}
  left: `{:?}`, right: `{:?}`",
                $fmt, left, right
            );
        }
    };
}

/// Asserts that a panic occurred
#[macro_export]
macro_rules! assert_panic {
    ($expr:expr) => {
        assert_panic!($expr, "assertion failed: panic message")
    };
    ($expr:expr, $fmt:expr) => {
        assert_panic!($expr, $fmt, format_args!($fmt, $($arg),*))
    };
}

/// Asserts that a panic did not occur
#[macro_export]
macro_rules! assert_no_panic {
    ($expr:expr) => {
        assert_no_panic!($expr, "assertion failed: no panic expected")
    };
    ($expr:expr, $fmt:expr) => {
        assert_no_panic!($expr, $fmt, format_args!($fmt, $($arg),*))
    };
}

/// Helper function for equality checks in macros
#[doc(hidden)]
#[inline]
pub fn macro_rules_private_eq<T: PartialEq>(left: T, right: T) -> bool {
    left == right
}

