use std::{net::TcpStream, io::{Read, Write}};

pub struct PackReader{
    pub stream:TcpStream,
}
impl PackReader {
    pub fn new(stream:TcpStream)->Self{
        PackReader { stream }
    }
    pub fn read_pack(&mut self)->Pack{
        // 读取头
        let need_len= self.read_head();
        let mut data = Vec::with_capacity(need_len);
        data.resize(need_len, 0);
        let mut len = 0;
        while len <need_len {
            let n = self.stream.read(&mut data[len..]).unwrap();
            len = len + n;
        }
        Pack { length: len as u32, data}
    }
    pub fn write_pack(&mut self,pack:Pack){
        let buf:Vec<u8>=pack.into();
        println!("@@@ write buf:{:?}",buf);
        let mut len = 0;
        while len != buf.len() {
            let n = self.stream.write(& buf[len..]) .unwrap();
            len = len + n;
            println!("@@@ write length:{:?}/total:{}",n,len);
        }
    }
    fn read_head(&mut self)->usize{
        let mut buf = [0;4];
        let mut n=0;
        eprintln!("@@@{:?}",buf[n..].len());
        while n<4 {
            let n_1=self.stream.read(&mut buf[n..])
                .expect("读取头错误");
            n = n+n_1;
            eprintln!("@@@ n:{},n_1:{}",n,n_1);
        }
        let mut len=0 as u32;
        for i in 0..=3  {
            len = (len<<8) +buf[i] as u32;
        }
        println!("@@@ len:{} buff:{:?}",len,buf);
        len as usize
    }
}

pub enum PackStatus{
    Full(u32), // return number of left in buffer
    NeedMore(u32), // return number of need
}

#[derive(Debug)]
pub struct Pack{
    pub length:u32,
    pub data:Vec<u8>,
}
impl From<&[u8]> for Pack {
    fn from(buf: &[u8]) -> Self {
        let mut len=0 as u32;
        for i in 0..=3  {
            len = (len<<8) +buf[i] as u32;
        }
        Pack { length: len, data: buf[4..].to_vec() }
    }
}
impl From<Pack> for Vec<u8> {
    fn from(mut p: Pack) -> Self {
        let mut v=Vec::with_capacity((p.length+4) as usize);
        let mut len = p.length;
        for _ in 0..4 {
            let x=(len&0xFF_00_00_00)>>24;
            v.push(x as u8 );
            len = len<<8;
        }
        v.append(&mut p.data);
        v
    }
}
impl Pack {
    pub fn fill(&mut self,buf:&[u8])->PackStatus{
        let total = self.data.len() + buf.len();
        let need = self.length as usize;
        let have = self.data.len();
        if total >= need{
            self.data.append(buf[.. need-have].to_vec().as_mut());
            PackStatus::Full((total-need) as u32)
        }else {
            self.data.append(buf.to_vec().as_mut());
            PackStatus::NeedMore((need-total) as u32)
        }
    }
}

#[derive(Debug)]
pub enum Cmd{
    Nop,
    Quit,
    Msg(String),
}

impl From<Pack> for Cmd{
    fn from(pack: Pack) -> Self {
        match pack.data[0] {
            0 =>Cmd::Nop,
            1 => Cmd::Quit,
            2 => Cmd::Msg(String::from_utf8(pack.data[1..].to_vec()).unwrap()),
            _ => Cmd::Nop,
        }
    }
}
impl From<Cmd> for Pack {
    fn from(cmd: Cmd) -> Self {
        match cmd{
            Cmd::Nop => Pack { length: 1, data: vec![0] },
            Cmd::Quit => Pack { length: 1, data: vec![1] },
            Cmd::Msg(info) => {
                let mut data = vec![2];
                data.append(info.as_bytes().to_vec().as_mut());
                Pack { length: 1+info.len() as u32, data }
            },
        }
    }
}
impl From<Cmd> for Vec<u8> {
    fn from(c: Cmd) -> Self {
        Vec::from(Pack::from(c))
    }
}

#[cfg(test)]
mod test_pack{
    use super::Pack;

    #[test]
    fn pack_from_u8(){
        let v=[0,0,0,8,1,1,1,1];
        let p:Pack=v.as_slice().into();
        println!("{:?}",p);
        let tv:Vec<u8> = p.into();
        println!("{:?}",tv);
    }
    #[test]
    fn pack_fill(){
        let v=[0,0,0,42,1,1];
        let mut p:Pack=v.as_slice().into();
        println!("need:{} have:{}",p.length,p.data.len());
        for i in 0..100 {
            println!("{},buf len :{} ",i,v.len());
            let x=p.fill(&v);
            match x {
                crate::package::PackStatus::Full(left) => {
                    print!("\t {} left ",left);
                    println!("need:{} have:{}",p.length,p.data.len());
                    break;
                },
                crate::package::PackStatus::NeedMore(need) => {
                    print!("\tneed {} more ",need);
                    println!("need:{} have:{}",p.length,p.data.len());
                },
            }
        }
    }

}
