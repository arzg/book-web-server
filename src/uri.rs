/// A URI.
///
/// ```
/// use book_web_server::Uri;
///
/// assert_eq!(
///     Uri::new("/"),
///     Ok((
///         "",
///         Uri {
///             components: Vec::new(),
///         }
///     ))
/// );
///
/// assert_eq!(
///     Uri::root(),
///     Uri {
///         components: Vec::new()
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct Uri<'a> {
    pub components: Vec<&'a str>,
}

impl<'a> Uri<'a> {
    pub fn new(s: &'a str) -> nom::IResult<&'a str, Self> {
        use nom::{
            bytes::complete::tag,
            combinator::{map, opt},
            multi::many0,
        };

        let (s, components) = many0(component)(s)?;

        // The trailing slash is only required if no components were found (i.e. if no components
        // are found, then ‘/’ is the bare minimum).
        let (s, _) = if components.is_empty() {
            map(tag("/"), |s| Some(s))(s)?
        } else {
            opt(tag("/"))(s)?
        };

        Ok((s, Self { components }))
    }

    pub fn root() -> Self {
        Self {
            components: Vec::new(),
        }
    }
}

fn component(s: &str) -> nom::IResult<&str, &str> {
    use nom::bytes::complete::{tag, take_while1};

    let (s, _) = tag("/")(s)?;
    take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '.' || c == '_' || c == '~')(
        s,
    )
}
