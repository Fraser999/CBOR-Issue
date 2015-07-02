extern crate cbor;
extern crate rustc_serialize;

use cbor::CborTagEncode;
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};

#[derive(Debug)]
// #[derive(Debug, RustcDecodable, RustcEncodable)]
struct Foo32 { data: u32 }

impl Encodable for Foo32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        CborTagEncode::new(11111111, &self.data).encode(encoder)
    }
}

impl Decodable for Foo32 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Foo32, D::Error> {
        try!(decoder.read_u64());
        Ok(Foo32 { data: try!(Decodable::decode(decoder)) })
    }
}

#[derive(Debug)]
// #[derive(Debug, RustcDecodable, RustcEncodable)]
struct Foo64 { data: u64 }

impl Encodable for Foo64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        CborTagEncode::new(22222222, &self.data).encode(encoder)
    }
}

impl Decodable for Foo64 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Foo64, D::Error> {
        try!(decoder.read_u64());
        Ok(Foo64 { data: try!(Decodable::decode(decoder)) })
    }
}

fn main() {
    let foo32 = Foo32{ data: 4294967295 };
    let foo64 = Foo64{ data: 4294967295 + 1 };

    {
        let mut encoder = cbor::Encoder::from_memory();
        encoder.encode(&[&foo32]).unwrap();
        let encoded = encoder.as_bytes();

        println!("Encode foo32 to {} bytes: {:?}", encoded.len(), encoded);

        let mut decoder = cbor::Decoder::from_bytes(encoded);
        let result32: Option<Result<Foo32, cbor::CborError>> = decoder.decode().next();

        println!("Decode foo32 result: {:?}", result32);
    }

    {
        let mut encoder = cbor::Encoder::from_memory();
        encoder.encode(&[&foo64]).unwrap();
        let encoded = encoder.as_bytes();

        println!("Encode foo64 to {} bytes: {:?}", encoded.len(), encoded);

        let mut decoder = cbor::Decoder::from_bytes(encoded);
        let result64: Option<Result<Foo64, cbor::CborError>> = decoder.decode().next();

        println!("Decode foo64 result: {:?}", result64);
    }
}
