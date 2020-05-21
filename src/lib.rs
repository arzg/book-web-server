mod header;
mod method;
mod request;
mod response;
mod status;
mod uri;
mod version;

pub use header::Header;
pub use method::Method;
pub use request::Request;
pub use response::Response;
pub use status::Status;
pub use uri::Uri;
pub use version::Version;

fn space(s: &str) -> nom::IResult<&str, &str> {
    use nom::bytes::complete::take_while1;

    take_while1(|c| c == ' ')(s)
}

fn crlf(s: &str) -> nom::IResult<&str, &str> {
    use nom::bytes::complete::tag;

    tag("\r\n")(s)
}
