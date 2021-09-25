mod dns;
use std::net::UdpSocket;
use rand::Rng;

fn main() {
    let header = dns::Header::new(
        65,
        false,
        0,
        false,
        false,
        true,
        false,
        0,
        1,
        0,
        0,
        0
    );

    let question = dns::Question::new(String::from("www.ieft.org"), 1, 1);

    let mut questions:Vec<dns::Question> = Vec::with_capacity(2);

    questions.push(question);

    let packet = dns::Packet::new(header,Some(questions),None);

    println!("{:?}",packet);

    let packet_dumped = packet.to_bytes();

    //let parsed_packet = dns::Packet::parse(packet_dumped.as_slice());
    //println!("{:?}",parsed_packet);

    let mut rng = rand::thread_rng();

    let mut port:u16 = 0;
    let mut socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    
    socket.send_to(&packet_dumped as &[u8],String::from("8.8.8.8:53"));
    let mut buffer:[u8;1024] = [0;1024];

    let (amt, src) = socket.recv_from(&mut buffer).unwrap();

    let received_packet = dns::Packet::parse(&buffer);
    println!("{:?}",received_packet);

}
