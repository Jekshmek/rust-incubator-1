use std::borrow::Cow;
use std::net::{AddrParseError, IpAddr, SocketAddr};

fn main() {
    println!("Refactor me!");

    let mut err = Error::new("NO_USER");
    err.status(404).message("User not found");
}

#[derive(Debug)]
pub struct Error<'code, 'msg> {
    code: Cow<'code, str>,
    status: u16,
    message: Cow<'msg, str>,
}

impl<'code, 'msg> Default for Error<'code, 'msg> {
    #[inline]
    fn default() -> Self {
        Self {
            code: "UNKNOWN".into(),
            status: 500,
            message: "Unknown error has happened.".into(),
        }
    }
}

impl<'code, 'msg> Error<'code, 'msg> {
    pub fn new(code: impl Into<Cow<'code, str>>) -> Self {
        Error {
            code: code.into(),
            ..Error::default()
        }
    }

    pub fn status(&mut self, s: u16) -> &mut Self {
        self.status = s;
        self
    }

    pub fn message(&mut self, m: impl Into<Cow<'msg, str>>) -> &mut Self {
        self.message = m.into();
        self
    }
}

#[derive(Debug, Default)]
pub struct Server(Option<SocketAddr>);

impl Server {
    pub fn bind(&mut self, ip: impl Into<IpAddr>, port: u16) {
        self.0 = Some(SocketAddr::new(ip.into(), port))
    }

    pub fn bind_str<'a>(
        &mut self,
        ip: impl Into<&'a str>,
        port: u16,
    ) -> Result<(), AddrParseError> {
        let ip = ip.into().parse()?;
        self.0 = Some(SocketAddr::new(ip, port));
        Ok(())
    }
}

#[cfg(test)]
mod server_spec {
    use super::*;

    mod bind {
        use super::*;

        #[test]
        fn sets_provided_address_to_server() {
            let mut server = Server::default();

            server.bind([127, 0, 0, 1], 8080);
            assert_eq!(format!("{}", server.0.unwrap()), "127.0.0.1:8080");

            server.bind_str("::1", 9911).unwrap();
            assert_eq!(format!("{}", server.0.unwrap()), "[::1]:9911");
        }
    }
}
