#[macro_export]
/// A mix of [`dbg!`] and [`log::log!`].
/// 
/// Like [`dbg!`], returns the value of the given expression.
/// If multiple expressions, comes back as a tuple.
/// 
/// Format-type inputs return the formatted literal,
/// with literals returning themselves as a result.
/// This is the case because fuck it.
/// 
/// *Note, for all of the examples, the Level variant 
/// has no influence on whether they work or not and
/// just walk through the level scale*.
/// ## Some working examples
/// ```
/// dlog!(Level::Trace)
/// dlog!(Level::Debug, "Literal")
/// dlog!(Level::Info, vec![0, 1, 2, 3])
/// dlog!(Level::Warn, String::new(), vec![], variable)
/// dlog!(Level::Error, dpi_scale, zoom, "test", lb_offsets, lb_scales); 
/// ```
/// ## Some non-working examples
/// ```
/// dlog!(Level::Trace, 1);
/// dlog!(Level::Debug, dpi_scale, "Foo {:?} {:?}", lb_offsets, lb_scales);
/// dlog!(Level::Info, dpi_scale, zoom, 1, lb_offsets, lb_scales);
/// ```
macro_rules! dlog {
    // Level-only "empty" invocation, e.g.:
    //      dlog!(Level::Trace)
    ($level:expr $(,)?) => {
        log::log!(
            $level,
            "",
        );
    };

    // Format + arg type of invocation, e.g.:
    //      dlog!(Level::Debug, "Foo {:?} {:?}", arg1, arg2)
    // OR in degenerative cases, level + string literal, e.g.:
    //      dlog!(Level::Debug, "Literal")
    ($level:expr, $fmt:literal $(, $($args:tt)*)?) => {
        {  
            log::log!(
                $level,
                "{}",
                format_args!($fmt $(, $($args)*)?)
            )
        }
    };

    // Level + 1 non-literal expr, e.g.:
    //      dlog!(Level::Info, vec![0, 1, 2, 3])
    ($level:expr, $val:expr $(,)?) => {
        /* Passed along from std's dbg! macro comments:
            Use of `match` here is intentional because it affects the lifetimes
            of temporaries - https://stackoverflow.com/a/48732525/1063961 */
        match $val {
            tmp => {
                log::log!(
                    $level,
                    "{} = {:#?}",
                    stringify!($val),
                    &tmp
                );
                // Return the value
                tmp
            }
        }
    };

    // Level + n-exprs that also allow non-arg string literals, e.g.:
    //      dlog!(Level::Error, dpi_scale, zoom, "test", lb_offsets, lb_scales); 
    ($level:expr, $($val:expr),+ $(,)?) => {
        ($(dlog!($level, $val)),+,)
    };
}