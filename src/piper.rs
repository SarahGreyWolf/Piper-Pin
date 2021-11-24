use std::io::prelude::*;
use std::io;

pub trait FromReq {
    fn size(self) -> usize;
    fn read(bytes: &[u8]) -> Self;
}

pub trait AsResp {
    fn size(self) -> usize;
    fn bytes(self) -> Vec<u8>;
}

#[allow(dead_code)]
pub enum ContentType {
    Text=0x00,
    GemText=0x01,
    Ascii=0x02,
    File=0x10,
    Redirect=0x20,
    ErrorNotFound=0x22,
    ErrorInternal=0x23,
    SpecVersion=0x24
}

#[derive(Debug)]
pub struct Request {
    length: u16,
    content: String
}

impl FromReq for Request {
    fn size(self) -> usize {
        self.length as usize + self.content.len()
    }
    fn read(bytes: &[u8]) -> Self {
        let mut iter_bytes = bytes.iter();
        let length = *iter_bytes.next().unwrap() as u16 + *iter_bytes.next().unwrap() as u16;
        let mut content_bytes = iter_bytes.map(|b| *b ).collect::<Vec<u8>>();
        content_bytes.truncate(length as usize);
        Self {
            length,
            content: String::from_utf8(content_bytes).unwrap()
        }
    }
}

pub struct TextResponse(pub String);

impl AsResp for TextResponse {
    fn size(self) -> usize {
        self.0.as_bytes().len() + std::mem::size_of::<u8>() + std::mem::size_of::<u64>()
    }

    fn bytes(self) -> Vec<u8> {
        let mut response: Vec<u8> = vec![ContentType::Text as u8];
        response.append(&mut self.0.clone().as_bytes().len().to_le_bytes().to_vec());
        response.append(&mut self.0.clone().as_bytes().to_vec());
        response
    }
}




