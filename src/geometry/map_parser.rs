use crate::prelude::*;
use std::num::ParseFloatError;

#[derive(Clone, Copy)]
enum ParseType {
    Line,
    Rect,
}

struct MapFileParseError(usize, Option<String>, Option<ParseFloatError>);

impl MapFileParseError {
    fn dyn_boxed<T: ToString>(line: usize, msg: T) -> Box<dyn Error> {
        Box::new(MapFileParseError(line, Some(msg.to_string()), None))
    }

    fn dyn_boxed_from_parse_err(line: usize, parse_err: ParseFloatError) -> Box<dyn Error> {
        Box::new(MapFileParseError(line, None, Some(parse_err)))
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
pub async fn parse_map_file() -> GResult<GeometryMap> {
    let file = {
        let mut file = load_file("assets/maps/test_map").await?;
        file.push(b'\n');
        file
    };
    let mut geometry = vec![];

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
        if i >= 4 && byte != b'\n' && !in_comment {
            return Err(MapFileParseError::dyn_boxed(line, "Too many input fields/commas/closing-parenthesis."));
        }
        if in_comment && byte != b'\n' {
            continue;
        }

        // Non-trivial / Parse Logic Guards
        match byte {
            b')' | b',' | b'\n' => {
                if curr_parse.is_some() && i < 4 {
                    // There's no real need to interpret any of these as a default value
                    // or anything, we can just reject it for simplicity in format.
                    if n.is_empty() {
                        return Err(MapFileParseError::dyn_boxed(line, "Received { '(,) (,)\" | '(, , ,)\' | '() ()' | '(    )' | '()' | ')' | ')\\n' (without 4 populated inputs) | ',' | ',\\n' (without 4 populated inputs) } etc-type of input. Ensure numbers are present in input."));
                    }
                    // SAFETY: if Some(Line) or Some(Rect), all non-utf8 has been error'd by now.
                    // For similar reasons, parse() technically only returns Ok() as n is populated only
                    // with ascii digits or a period, as those are the only bytes the guard match statement
                    // lets through.
                    c[i] = unsafe { std::str::from_utf8_unchecked(&n).parse::<f32>().map_err(|e| MapFileParseError::dyn_boxed_from_parse_err(line, e))? };
                    i += 1;
                    n.clear();
                } 

                // Attempts to finish line-parse if any, pushing finished geometry to collection vec
                if byte == b'\n' {


                    in_comment = false;
                    line += 1;

                    if let Some(parse_type) = curr_parse {
                        // if requesting a consuming (geometry generating) new line without all inputs filled
                        if i < 4 {
                            return Err(MapFileParseError::dyn_boxed(line, "Missing final inputs, received { 'n, n, n, [empty]\\n' | 'n, n, [empty]\\n' | 'n, [empty]\\n' | '[empty]\\n' }-type of input. Ensure numbers are present in every input."));
                        }
                        match parse_type {
                            ParseType::Line => geometry.push(Geometry::new_line(c[0], c[1], c[2], c[3])),
                            ParseType::Rect => geometry.push(Geometry::new_rect(c[0], c[1], c[2], c[3])),
                        }
                        c = Default::default();
                        i = 0;
                        curr_parse = None;
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
                b'L' => ParseType::Line,
                b'R' => ParseType::Rect,
                _ => return Err(MapFileParseError::dyn_boxed(line, format_args!(
                        "Unrecognized character for first character in line that determines geometry type: '{}'. Valid values: {{ 'L' for a Line; 'R' for a Rect }}.",
                        byte as char
                    )
                )),
            }.into();
        }
    }

    Ok(GeometryMap { inner: geometry })
}
