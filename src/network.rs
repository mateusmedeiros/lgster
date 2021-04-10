use std::io;
use std::io::{Error as IOError, ErrorKind, Read, Write};
use std::net::{TcpStream, ToSocketAddrs, UdpSocket};
use std::time::Duration;

pub fn send_udp_message(target: &str, message: &[u8]) -> io::Result<usize> {
    for port in 1025..=65535 {
        let maybe_socket =
            UdpSocket::bind(("0.0.0.0", port)).and_then(|s| s.set_broadcast(true).map(|_| s));
        match maybe_socket {
            Ok(s) => return s.send_to(message, (target, 9)),
            Err(e) if e.kind() == ErrorKind::AddrInUse => continue,
            Err(e) => return Err(e),
        };
    }

    Err(IOError::new(
        ErrorKind::AddrNotAvailable,
        "No address available (tried 0.0.0.0 from ports 1025 to 65535)",
    ))
}

pub fn send_and_receive_tcp_message(
    target: impl ToSocketAddrs,
    message: &[u8],
) -> io::Result<Vec<u8>> {
    let target_socket_address = target.to_socket_addrs()?.next().ok_or(IOError::new(
        ErrorKind::InvalidInput,
        "message target address is empty",
    ))?;

    let mut tcp_stream =
        TcpStream::connect_timeout(&target_socket_address, Duration::from_secs(15))?;
    tcp_stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    tcp_stream.set_write_timeout(Some(Duration::from_secs(3)))?;
    tcp_stream.write_all(message)?;
    tcp_stream.flush()?;

    let mut response = vec![0; 128];
    tcp_stream.read(&mut response[..16])?;
    let mut bytes_read = 16;

    // after it succesfully read the iv, it means the connection is ok
    // so now we set a short timeout because it was the only way I found to identify
    // without saving that info per command what is the end of the message
    // I tried to set the socket to non blocking and just read until it couldn't
    // but it wasn't reliable
    tcp_stream.set_read_timeout(Some(Duration::from_millis(100)))?;

    loop {
        // NOTE: Unix-like systems will raise WouldBlock while Windows
        //   will raise TimedOut when our read_timeout is passed
        //   both in our context will mean we got our response
        //   see: https://git.io/JOfSZ
        match tcp_stream.read(&mut response[bytes_read..bytes_read + 16]) {
            Ok(16) => bytes_read += 16,

            Ok(_) => break,
            Err(e) if e.kind() == ErrorKind::WouldBlock => break,
            Err(e) if e.kind() == ErrorKind::TimedOut => break,

            Err(e) => return Err(e),
        }
    }

    response.resize(bytes_read, 0);
    Ok(response)
}
