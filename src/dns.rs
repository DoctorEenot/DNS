#[derive(Debug)]
pub struct Header{
    ID:u16,// request ID
    QR:bool,// query(false) or response(true)
    OPCODE:u8,
    AA:bool,
    TC:bool,
    RD:bool,
    RA:bool,
    RCODE:u8,
    QDCOUNT:u16,// amount of entries in question
    ANCOUNT:u16,// amount of entries in answer
    NSCOUNT:u16,
    ARCOUNT:u16
}

impl Header{
    pub fn new(ID:u16,
                QR:bool,
                OPCODE:u8,
                AA:bool,
                TC:bool,
                RD:bool,
                RA:bool,
                RCODE:u8,
                QDCOUNT:u16,
                ANCOUNT:u16,
                NSCOUNT:u16,
                ARCOUNT:u16) -> Header{
        
        let to_return:Header = Header{
            ID:ID,
            QR:QR,
            OPCODE:OPCODE,
            AA:AA,
            TC:TC,
            RD:RD,
            RA:RA,
            RCODE:RCODE,
            QDCOUNT:QDCOUNT,
            ANCOUNT:ANCOUNT,
            NSCOUNT:NSCOUNT,
            ARCOUNT:ARCOUNT
        };

        return to_return;
        
    }
    pub fn to_bytes(&self) -> Vec<u8>{
        let mut to_return:Vec<u8> = Vec::with_capacity(12);

        for &byte in self.ID.to_be_bytes().iter(){
            to_return.push(byte);
        }

        let mut to_add:u8 = 0;
        if self.QR{
            to_add |= 0b10000000;
        }
        to_add |= (self.OPCODE&0b00001111)<<3;
        if self.AA{
            to_add |= 0b00000100;
        }
        if self.TC{
            to_add |= 0b00000010;
        }
        if self.RD{
            to_add |= 0b00000001;
        }
        to_return.push(to_add);

        to_add = 0;
        if self.RA{
            to_add |= 0b10000000;
        }
        to_add |= self.RCODE&0b00001111;
        to_return.push(to_add);

        for &byte in self.QDCOUNT.to_be_bytes().iter(){
            to_return.push(byte);
        }
        for &byte in self.ANCOUNT.to_be_bytes().iter(){
            to_return.push(byte);
        }
        for &byte in self.NSCOUNT.to_be_bytes().iter(){
            to_return.push(byte);
        }
        for &byte in self.ARCOUNT.to_be_bytes().iter(){
            to_return.push(byte);
        }
        
        return to_return;
    }    
}

pub fn name_to_bytes(name:&String) -> Vec<u8>{
    let mut to_return:Vec<u8> = Vec::with_capacity(name.len()+2);

    let mut split = name.split(".");
    for s in split{
        to_return.push(s.len() as u8);
        to_return.extend(s.as_bytes());
    }
    to_return.push(0);

    return to_return;
}

#[derive(Debug)]
pub struct Question{
    QNAME:String,
    QTYPE:u16,
    QCLASS:u16
}

impl Question{
    pub fn new(QNAME:String,
                QTYPE:u16,
                QCLASS:u16) -> Question{
        return Question{QNAME:QNAME,QTYPE:QTYPE,QCLASS:QCLASS};
    }
    pub fn to_bytes(&self) -> Vec<u8>{
        let qname_bytes:Vec<u8> = name_to_bytes(&self.QNAME);
        
        let mut to_return:Vec<u8> = Vec::with_capacity(qname_bytes.len()+32);

        for &byte in qname_bytes.iter(){
            to_return.push(byte);
        }

        for &byte in self.QTYPE.to_be_bytes().iter(){
            to_return.push(byte);
        }

        for &byte in self.QCLASS.to_be_bytes().iter(){
            to_return.push(byte);
        }

        return to_return;
    }
}

#[derive(Debug)]
pub struct Answer{
    NAME:String,
    TYPE:u16,
    CLASS:u16,
    TTL:u32,
    RDATA:Vec<u8>
}

impl Answer{
    pub fn new(NAME:String,
                TYPE:u16,
                CLASS:u16,
                TTL:u32,
                RDATA:Vec<u8>) -> Answer{

        return Answer{NAME:NAME,
                    TYPE:TYPE,
                    CLASS:CLASS,
                    TTL:TTL,
                    RDATA:RDATA};
    }
    pub fn to_bytes(&self) -> Vec<u8>{
        let name_bytes:Vec<u8> = name_to_bytes(&self.NAME);

        let mut to_return:Vec<u8> = Vec::with_capacity(name_bytes.len()
                                                        +80
                                                        +self.RDATA.len());
        
        for &byte in name_bytes.iter(){
            to_return.push(byte);
        }

        for &byte in self.TYPE.to_be_bytes().iter(){
            to_return.push(byte);
        }

        for &byte in self.CLASS.to_be_bytes().iter(){
            to_return.push(byte);
        }

        for &byte in self.TTL.to_be_bytes().iter(){
            to_return.push(byte);
        }

        for &byte in (self.RDATA.len() as u16).to_be_bytes().iter(){
            to_return.push(byte);
        }

        for &byte in self.RDATA.iter(){
            to_return.push(byte);
        }
        return to_return;
        
    }
    
}

#[derive(Debug)]
pub struct Packet{
    header:Header,
    questions:Option<Vec<Question>>,
    answers:Option<Vec<Answer>>
}

impl Packet{
    pub fn new(header:Header,
            questions:Option<Vec<Question>>,
            answers:Option<Vec<Answer>>) -> Packet{
        
        return Packet{header:header,
            questions:questions,
            answers:answers};
        
    }
    pub fn to_bytes(&self) -> Vec<u8>{
        let header_as_bytes:Vec<u8> = self.header.to_bytes();

        let mut to_return:Vec<u8> = Vec::with_capacity(header_as_bytes.len());

        for &byte in header_as_bytes.iter(){
            to_return.push(byte);
        }

        if !self.questions.is_none(){
            for question in self.questions.as_ref().unwrap(){
                to_return.append(&mut (question.to_bytes()));
            }
        }

        if !self.answers.is_none(){
            for answer in self.answers.as_ref().unwrap(){
                to_return.append(&mut (answer.to_bytes()));
            }
        }
        return to_return;
    }
    pub fn parse(data:&[u8]) -> Packet{
        // parsing header
        let ID:u16 = ((data[0] as u16)<<8)|(data[1] as u16);

        let first_byte:u8 = data[2];

        let QR:bool = (first_byte&0b10000000) != 0;

        let OPCODE:u8 = (first_byte&0b01111000)>>3;
        let AA:bool = (first_byte&0b00000100) != 0;
        let TC:bool = (first_byte&0b00000010) != 0;
        let RD:bool = (first_byte&0b00000001) != 0;

        let second_byte:u8 = data[3];

        let RA:bool = (second_byte&0b10000000) != 0;
        let RCODE:u8 = second_byte&0b00001111;

        let QDCOUNT:u16 = u16::from_be_bytes([data[4],data[5]]);
        let ANCOUNT = u16::from_be_bytes([data[6],data[7]]);
        let NSCOUNT = u16::from_be_bytes([data[8],data[9]]);
        let ARCOUNT = u16::from_be_bytes([data[10],data[11]]);

        let header:Header = Header::new(ID,QR,OPCODE,AA,TC,RD,RA,RCODE,
                                        QDCOUNT,ANCOUNT,NSCOUNT,ARCOUNT);

        // PARSING BODY
        let mut index:usize = 12;

        // parsing questions
        let mut questions:Option<Vec<Question>>;
        if QDCOUNT == 0{
            questions = None;
        }else{
            questions = Some(Vec::with_capacity(QDCOUNT as usize));
            for i in 0..QDCOUNT{
                // parsing QNAME
                let mut raw_string:Vec<u8> = Vec::new();
                for n in index+1..index+1+data[index] as usize{
                    raw_string.push(data[n]);
                }
                index += 1+data[index] as usize;
                while data[index] != 0{
                    raw_string.push(b'.');
                    for n in index+1..index+1+data[index] as usize{
                        raw_string.push(data[n]);
                    }
                    index += 1+data[index] as usize;
                }
                index += 1;
                let QNAME:String = String::from_utf8(raw_string.clone()).unwrap();
                
                // parsing QTYPE
                let QTYPE:u16 = u16::from_be_bytes([data[index],data[index+1]]);
                index += 2;

                // parsing QCLASS
                let QCLASS:u16 = u16::from_be_bytes([data[index],data[index+1]]);
                index += 2;

                let question:Question = Question::new(QNAME,QTYPE,QCLASS);

                questions.as_mut().unwrap().push(question);
            }
        }

        // parsing answers
        let combined_answers_amount:usize = ANCOUNT as usize
                                        +NSCOUNT as usize
                                        +ARCOUNT as usize;

        let mut answers:Option<Vec<Answer>>;
        if combined_answers_amount == 0{
            answers = None;
        }else{
            answers = Some(Vec::with_capacity(combined_answers_amount));
            for i in 0..combined_answers_amount{
                // parsing Name
                let mut raw_string:Vec<u8> = Vec::new();
                let mut buf_index = index;
                if data[index]&0b11000000 != 0{
                    // if pointer
                    buf_index = index;
                    index = u16::from_be_bytes([data[index]&0b00111111,
                                                 data[index+1]]) as usize;
                    if data[index] == 0{
                        index += 1;
                    }
                } 
                for n in index+1..index+1+data[index] as usize{
                    raw_string.push(data[n]);
                }
                index += 1+data[index] as usize;
                while data[index] != 0{
                    raw_string.push(b'.');
                    for n in index+1..index+1+data[index] as usize{
                        raw_string.push(data[n]);
                    }
                    index += 1+data[index] as usize;
                }
                if buf_index > index{
                    index = buf_index;
                    index += 2;
                }
                else{
                    index += 1;
                }
                
                let NAME:String = String::from_utf8(raw_string.clone()).unwrap();

                //parsing TYPE
                let TYPE:u16 = u16::from_be_bytes([data[index],data[index+1]]);
                index += 2;

                //parsing CLASS
                let CLASS:u16 = u16::from_be_bytes([data[index],data[index+1]]);
                index += 2;

                //parsing TTL
                let TTL:u32 = u32::from_be_bytes([data[index],
                                                data[index+1],
                                                data[index+2],
                                                data[index+3]]);
                
                index += 4;

                // parsing RDLENGTH
                let RDLENGTH:u16 = u16::from_be_bytes([data[index],data[index+1]]);
                index += 2;

                //parsing RDATA
                let mut RDATA:Vec<u8> = Vec::with_capacity(RDLENGTH as usize);
                for n in index..index+RDLENGTH as usize{
                    RDATA.push(data[n]);
                }

                index += RDLENGTH as usize;

                let answer:Answer = Answer::new(NAME,
                                                TYPE,
                                                CLASS,
                                                TTL,
                                                RDATA);
                answers.as_mut().unwrap().push(answer);
            }
        }

        let packet:Packet = Packet::new(header, 
                                        questions, 
                                        answers);
        
        return packet;
    }
}


