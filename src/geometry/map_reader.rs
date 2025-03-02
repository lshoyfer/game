use std::str::FromStr;

use crate::prelude::*;
use custom_usize_option::CustomUsizeOption;

#[derive(Clone, Copy)]

// For now we only support Rect
enum ParseType {
    Rect,
}

struct MapFileParseError(usize, Option<String>, Option<Box<dyn Error>>);

impl MapFileParseError {
    fn dyn_boxed<T: ToString>(line: usize, msg: T) -> Box<dyn Error> {
        Box::new(MapFileParseError(line, Some(msg.to_string()), None))
    }

    fn dyn_boxed_from_err<T: Error + 'static>(line: usize, parse_err: T) -> Box<dyn Error> {
        Box::new(MapFileParseError(line, None, Some(Box::new(parse_err))))
    }
} 

// Using Display formatting for Debug's impl as well to avoid String's extra "" pair from its Debug impl 
impl Debug for MapFileParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(msg) = self.1.as_ref() {
            write!(f, "[Line #{}] Poorly formatted map file: {}", self.0, msg)?;
        }
        if let Some(parse_err) = self.2.as_ref() {
            write!(f, "[Line #{}] Poorly formatted map file: {}", self.0, parse_err)?;
        }
        Ok(())
    }
}

impl Display for MapFileParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(msg) = self.1.as_ref() {
            write!(f, "[Line #{}] Poorly formatted map file: {}", self.0, msg)?;
        }
        if let Some(parse_err) = self.2.as_ref() {
            write!(f, "[Line #{}] Poorly formatted map file: {}", self.0, parse_err)?;
        }
        Ok(())
    }
}

impl Error for MapFileParseError {}

// Does the job for now
pub async fn read_map_file(path: &str) -> GResult<GeometryMap> {
    let file = {
        let mut file = load_file(path).await?;
        file.push(b'\n');
        file
    };
    let mut geometry = vec![];

    let (mut space_index, mut atlas_index, mut inner_index) = (None::<NonZeroUsize>, None::<usize>, None::<usize>);
    let mut c = [0f32; 4];
    let mut i = 0usize;
    let mut n = vec![];
    let mut line = 1usize;
    let mut in_comment = false; 
    let mut curr_parse: Option<ParseType> = None;

    for byte in file {
        dlog!(Level::Trace, "Map Parser Iteration {} c:{:?} n:{:?}", byte as char, &c, &n.iter().map(|&b| b as char).collect::<Vec<_>>());
        // Basic sequentially-important guards
        if byte == b' ' || byte == b'(' {
            continue;
        }
        if byte == b'#' {
            in_comment = true;
            continue;
        }
        if i > 5 && byte != b'\n' && !in_comment {
            return Err(MapFileParseError::dyn_boxed(line, "Too many input fields/commas/closing-parenthesis."));
        }
        if in_comment && byte != b'\n' {
            continue;
        }

        // Non-trivial / Parse Logic Guards
        match byte {
            b')' | b',' | b'\n' => {
                if curr_parse.is_some() && i < 6 {
                    // There's no real need to interpret any of these as a default value
                    // or anything, we can just reject it for simplicity in format.
                    if n.is_empty() {
                        return Err(MapFileParseError::dyn_boxed(line, "Received { '(,) (,)\" | '(, , ,)\' | '() ()' | '(    )' | '()' | ')' | ')\\n' (without all populated inputs) | ',' | ',\\n' (without all poppulated inputs) } etc-type of input. Ensure numbers are present in input."));
                    }


                    // SAFETY (parse fn): if curr_parse.is_some(), all non-utf8 has been error'd by now.
                    // As an aside, for similar reasons parse() """typically""" returns Ok() as n is populated only
                    // with ascii digits or a period, as those are the only bytes the guard match statement
                    // lets through. Only malformed inputs using those characters would Err(), e.g: a sole "."
                    // for both f32 and the integer parses. STD has documentation this parsing behavior if needed.
                    unsafe {
                        if i < 4 { // First 4 inputs (Normal positional/dimensional inputs)
                            c[i] = parse::<_, f32>(line, &n)?;
                        } else if i == 4 {
                            space_index = Some(parse::<_, NonZeroUsize>(line, &n)?);
                        } else if i == 5 {
                            atlas_index = Some(parse::<_, usize>(line, &n)?);
                        } else if i == 6 {
                            inner_index = Some(parse::<_, usize>(line, &n)?);
                        }
                    }
                    i += 1;
                    n.clear();
                } 

                // Attempts to finish line-parse if any, pushing finished geometry to collection Vec
                if byte == b'\n' {
                    in_comment = false;
                    line += 1;

                    if let Some(parse_type) = curr_parse.take() {
                        if i < 4 {
                            return Err(MapFileParseError::dyn_boxed(line, "Missing final positional inputs on this non-texture geometry, received { 'n, n, n, [empty]\\n' | 'n, n, [empty]\\n' | 'n, [empty]\\n' | '[empty]\\n' }-type of input. Ensure numbers are present in every input."));
                        } else if i == 5 {
                            return Err(MapFileParseError::dyn_boxed(line, "Missing atlas_index for texture map argument. Received only space_index. E.g, received { 'n, n, n, n, S, [empty A]\\n' }-type of input. Ensure numbers are present in every input."));
                        } else if i == 6 {
                            return Err(MapFileParseError::dyn_boxed(line, "Missing inner_index for texture map argument. Received only space_index and atlas_index. E.g, received { 'n, n, n, n, S, A, [empty I]\\n' }-type of input. Ensure numbers are present in every input."));
                        }
                        match parse_type {
                            ParseType::Rect => {
                                if space_index.is_some() && inner_index.expect("parser safety bullshit (ii)") == usize::MAX {
                                    return Err(MapFileParseError::dyn_boxed(line, "inner_index cannot have a value equivalent to usize::MAX."));
                                }
                                let maybe_texture_index = space_index.map(|space_index| unsafe {
                                    // SAFETY (unwrap): Parser ensures you cannot have Some(atlas_index) and Some(inner_index) without a Some(space_index) via above (the i == 5/6 check)
                                    let atlas_index = atlas_index.expect("parser safety bullshit (ai)");
                                    let inner_index = CustomUsizeOption::some(inner_index.expect("parser safety bullshit (ii)")); // SAFETY: We checked for non-usize::MAX above (inside this match block)
                                    TextureIndex::new(space_index, atlas_index, inner_index)
                                });
                                geometry.push(Geometry::new_rect(c[0], c[1], c[2], c[3], maybe_texture_index));
                            }
                        }
                        (space_index, atlas_index, inner_index) = (None, None, None);
                        c = Default::default();
                        i = 0;
                    }
                }
                continue;
            },
            _ => ()
        }

        // Determine parse mode OR if in parse mode, ensure only pushing numbers or decimals
        // (the error msg also accounts for the ability to arrive here from any unrecognized character)
        if curr_parse.is_some() {
            match byte {
                b'+' | b'-' | b'.' | b'0'..=b'9' => (),
                _ => return Err(MapFileParseError::dyn_boxed(line, format_args!(
                        "Unrecognized character for geometry parse: '{}'. Valid values: [#()+,-.0-9] as well as arbitrary spaces and newlines.",
                        byte as char
                    )
                )),
            }
            n.push(byte);
        } else {
            curr_parse = match byte {
                b'R' => ParseType::Rect,
                _ => return Err(MapFileParseError::dyn_boxed(line, format_args!(
                        "Unrecognized character for first character in line that determines geometry type: '{}'. Valid values: {{ 'R' for a Rect }}.",
                        byte as char
                    )
                )),
            }.into();
        }
    }

    Ok(GeometryMap { inner: geometry })
}

/// ## Safety
/// Caller must guarantee input bytes contain valid UTF-8.
unsafe fn parse<In, Out>(line: usize, input: In) -> GResult<Out>
where 
    In: AsRef<[u8]>,
    Out: FromStr,
    <Out as FromStr>::Err: Error + 'static
{
    std::str::from_utf8_unchecked(input.as_ref()).parse::<Out>().map_err(|e| MapFileParseError::dyn_boxed_from_err(line, e))
}