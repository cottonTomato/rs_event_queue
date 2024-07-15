use std::{
    io::{self, Read, Write},
    net,
};

mod kqueue;
mod poll;

fn get_req(path: &str) -> Vec<u8> {
    format!(
        "GET {path} HTTP/1.1\r\n\
        Host: localhost\r\n\
        Connection: close\r\n\
        \r\n"
    )
    .into()
}

fn handle_events(events: &[kqueue::kevent_t], streams: &mut [net::TcpStream]) -> io::Result<usize> {
    let mut handled_events = 0;
    for event in events {
        let index = event.udata as usize;
        let mut data = vec![0u8; 4096];

        loop {
            match streams[index].read(&mut data) {
                Ok(0) => {
                    handled_events += 1;
                    break;
                }
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);

                    println!("RECIEVED: {:?}", event);
                    println!("{txt}\n-----\n");
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }
    }

    Ok(handled_events)
}

fn main() -> io::Result<()> {
    let mut poll = poll::Poll::new()?;
    let n_events = 5;

    let mut streams = Vec::<net::TcpStream>::new();
    let addr = "localhost:8080";

    for i in 0..n_events {
        let delay = (n_events - i) * 1000;
        let url_path = format!("/{delay}/request-{i}");
        let request = get_req(&url_path);

        let mut stream = net::TcpStream::connect(addr)?;
        stream.set_nonblocking(true)?;
        stream.write_all(&request)?;

        poll.registry()
            .register(&stream, i, poll::Intreasts::FdRead)?;

        streams.push(stream);
    }

    let mut handled_event = 0;

    while handled_event < n_events {
        let mut events = Vec::<kqueue::kevent_t>::with_capacity(10);
        poll.poll(&mut events, None)?;

        if events.is_empty() {
            println!("TIMEOUT OR SPURIOUS FAIL");
            continue;
        }

        handled_event += handle_events(&events, &mut streams)?;
    }

    println!("Finished!");
    Ok(())
}
