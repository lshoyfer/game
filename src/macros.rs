/// /* FIXME */
// There's no way to control the log level of microquad's environment
// and I can't be fucked to switch to a real logger environment for now
// so I prepend the respective log level to the string format so
// that I can grep it out in my shell with shitty scripts as an interim
// cruft filter solution. There are also some highly terrible things
// going on with the miniquad v.s. macroquad log types. Hence, this
// function exists as that bridge.
pub fn level_to_str(microquad_logcrate_level: macroquad::miniquad::log::Level) -> &'static str {
    use macroquad::miniquad::log::Level;
    match microquad_logcrate_level {
        Level::Error => "ERROR",
        Level::Warn => "WARN",
        Level::Info => "INFO",
        Level::Debug => "DEBUG",
        Level::Trace => "TRACE"
    }
}

#[macro_export]
/// A mix of [`dbg!`] and [`macroquad::miniquad::log!`].
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
/// See comments inline of the implementation for more details.
/// 
/// /* REVIEW */
/// There are too many comments because 
/// 1. I was thinking out loud and 
/// 2. I probably will actually implement the procedural macro
/// the descriptions will be useful for me then and thus 
/// 3. May as well have this in the stream at some point, 
/// and I will trim it eventually regardless of whether the 
/// procedural macro exists.
macro_rules! dlog {
    // Level-only "empty" invocation, e.g.:
    //      dlog!(Level::Trace)
    ($level:expr $(,)?) => {
        macroquad::miniquad::log!(
            $level,
            "[{}] [{}:{}:{}]",
            self::level_to_str($level), // FIXME
            file!(),
            line!(),
            column!()
        );
    };

    // Format + arg type of invocation, e.g.:
    //      dlog!(Level::Debug, "Foo {:?} {:?}", arg1, arg2)
    // OR in degenerative cases, level + string literal, e.g.:
    //      dlog!(Level::Debug, "Literal")
    // So one may except in these cases a `[...] "Literal" = Literal` type output
    // as in the next branch but it will output `[...] Literal`.
    // This is honestly desirable because it lets me use dlog for
    // inserts such as "HERE" + "HERE2" although not necessary
    // since log! exists but may as well since this works and gives
    // the dbg-type formatting -- who knows if that's "desirable."
    // Also, non-string literals don't work, e.g.:
    //      dlog!(Level::Trace, 1);
    // This has consequences on the final arm's behavior (more info there).
    // One may also expect to use this macro as in the final arm's recursive
    // nature, but if they pass in a string literal without any args
    // (i.e. done without the concious intention of str formatting)
    // as the first arg after level, it will match this arm and break. I
    // may eventually add a procedural part to this macro to address this. 
    // This also has consequences on the final arm's behavior (more info there).
    ($level:expr, $fmt:literal $(, $($args:tt)*)?) => {
        {  
            macroquad::miniquad::log!(
                $level,
                "[{}] [{}:{}:{}] {}",
                self::level_to_str($level), // FIXME
                file!(),
                line!(),
                column!(),
                format_args!($fmt $(, $($args)*)?)
            );
            format_args!($fmt $(, $($args)*)?) 
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
                macroquad::miniquad::log!(
                    $level,
                    "[{}] [{}:{}:{}] {} = {:#?}",
                    self::level_to_str($level), // FIXME
                    file!(),
                    line!(),
                    column!(),
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

    /* (Useful info alternative impl) Level + n-exprs, e.g.:
            dlog!(Level::Warn, String::new(), vec![], variable)
        This impl plays worse with non-arg literals but is 
        more expressive with allowing a final nested "arg-type"
        invocation as the last expr group, e.g. becomes legal:
            dlog!(Level::Debug, dpi_scale, "Foo {:?} {:?}", lb_offsets, lb_scales);
        I may eventually add a procedural part to this macro to address
        this & merge the best of both impls, as well as allow any literal to
        be nested, regardless of location, e.g. this doesn't work on either:
            dlog!(Level::Info, dpi_scale, zoom, 1, lb_offsets, lb_scales);
        Note: putting the non-arg string literal last works on either, e.g:
            dlog!(Level::Error, dpi_scale, zoom, lb_offsets, lb_scales, "foo");
        The reason I don't use this impl is because it outputs nested tuples instead
        of flattened ones, and also the need for a final arg-string isn't really
        pressing or important.
    */
    /* ($level:expr, $val:expr, $($rest:expr),+ $(,)?) => {
        (
            dlog!($level, $val),
            dlog!($level, $($rest),*)
        )
    }; */
}