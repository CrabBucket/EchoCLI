//We could just use libc here but why would I put myself through that struggle when someone has created so many nice bindings for me.
//Maybe convert to libc later for style points
extern crate libc;
extern crate ux;


use std::net::{Ipv4Addr,Ipv6Addr};
use ux::{u1,u4,u13,u17};

static UNIQUE_ID: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(0u16);

#[derive(Copy,Clone)]
struct IPv4Header{
    version: u4, //The version IPv4 or IPv6 we are using IPv4 uses 4.
    header_len: u4, //The header length is 5 since we have no options.
    service_type: u8, //Service type is essentially an upperlevel hint for routing on how to handle the packet.  
                      //We set to 16u8 which should enable the minimize delay flag
    total_len: u16, //Should be the total length of both the ip header and icmp header and the data I believe in 32 bit words
    identification: u16, // Used to track packets.
    nullbit: u1, // Always 0 A reserved bit in the ipv4 header
    dont_frag: u1, // I think I should always set this to 1 I don't believe I should ever have to frag
    more_frag: u1, // Since I'm not fragmenting this should be 0
    frag_offset: u13, // 0 again because we are not fragmenting.
    ttl: u8, // Standard lets say 32 time-to-live but can be set
    protocol: u8, //Should always be 1 as we are alway sending icmp packets and one corresponds to ICMP.
    header_checksum: u16, //Have to calculate this
    ip_source: Ipv4Addr, //u32 source of the ip that sent the packet
    ip_dest: Ipv4Addr, //u32 destionation ip that sent the packet
    

}

impl IPv4Header{
    pub fn new(ip_source: Ipv4Addr, ip_dest: Ipv4Addr) -> IPv4Header {
        let mut iphead = IPv4Header{
            version: u4::new(4), //Always 4
            header_len: u4::new(5), //I believe the header length is 5 since we have no options.
            service_type: 16, //This should set the minimize delay TOS in the ipheader
            total_len: 7, // 5 32 bit words for the ip header, 2 32 bit words for the icmp header
            identification: UNIQUE_ID.load(std::sync::atomic::Ordering::Relaxed), // Variable
            nullbit: u1::new(0), // Always 0
            dont_frag: u1::new(1), // I think I should always set this to 1 I don't believe I should ever have to frag
            more_frag: u1::new(0), // Since I'm not fragmenting this should be 0
            frag_offset: u13::new(0), // 0 again because we are not fragmenting.
            ttl: 32, // Standard lets say 32 time-to-live but can be set through a different constructor
            protocol: 1, //Should always be 1 as we are alway sending icmp packets
            header_checksum: 0, //Will calculate after instantiation.
            ip_source: ip_source, //u32 source of the ip that sent the packet
            ip_dest: ip_dest, //u32 destionation ip that sent the packet
        };
        UNIQUE_ID.store(iphead.identification+1,std::sync::atomic::Ordering::Relaxed);
        iphead.header_checksum = iphead.checksum();
        iphead
    }
    pub fn checksum_valid(self) -> bool {
        let words = self.to_word_array();
        let mut result = u17::new(0);
        for index in 0..words.len(){
            
            result = result.wrapping_add(u17::from(words[index]));
            let mut carry = u32::from(result).checked_shr(16).unwrap();
            while carry != 0 {
                result = result.wrapping_add(u17::from(carry as u16))-u17::from(u16::max_value())+u17::from(1u16);
                carry = u32::from(result).checked_shr(16).unwrap();
            }
            //println!("{}",u32::from(result.wrapping_add(u17::from(self.header_checksum))));
        };
        
        //What we are asserting here is basically that we got all one's for our 16bit result which is 0 in 1's complement.  This how they check the check_sum according to the standard.
        u32::from(result) == (u16::max_value() as u32)
    }
    pub fn to_word_array(self) -> [u16;10] {
        let mut words: [u16; 10] = [0; 10];
        words[0] = ((u8::from(self.version).checked_shl(4).unwrap() + u8::from(self.header_len)) as u16).checked_shl(8).unwrap() + (self.service_type as u16);  
        words[1] = self.total_len;
        words[2] = self.identification;
        //We just fit the first 3 flags into the topmost 3 bits and then the 13 bit field fills in the bottom.
        words[3] = u16::from(bool::from(self.nullbit)).checked_shl(15).unwrap()   
        + u16::from(bool::from(self.dont_frag)).checked_shl(14).unwrap() 
        + u16::from(bool::from(self.more_frag)).checked_shl(13).unwrap() 
        + u16::from(self.frag_offset);
        words[4] = (self.ttl as u16).checked_shl(8).unwrap()  + self.protocol as u16;
        let sourceoctets = self.ip_source.octets();
        words[5] = self.header_checksum;
        words[6] = (sourceoctets[0] as u16).checked_shl(8).unwrap() + (sourceoctets[1] as u16);
        words[7] = (sourceoctets[2] as u16).checked_shl(8).unwrap() + (sourceoctets[3] as u16);
        let destoctets = self.ip_dest.octets();
        words[8] = (destoctets[0] as u16).checked_shl(8).unwrap() + (destoctets[1] as u16);
        words[9] = (destoctets[2] as u16).checked_shl(8).unwrap() + (destoctets[3] as u16);
        words
    }
    pub fn cs_word_array(self) -> [u16; 9] {
        let mut words: [u16; 9] = [0; 9];
        words[0] = ((u8::from(self.version).checked_shl(4).unwrap() + u8::from(self.header_len)) as u16).checked_shl(8).unwrap() + (self.service_type as u16);  
        words[1] = self.total_len;
        words[2] = self.identification;
        //We just fit the first 3 flags into the topmost 3 bits and then the 13 bit field fills in the bottom.
        words[3] = u16::from(bool::from(self.nullbit)).checked_shl(15).unwrap()   
        + u16::from(bool::from(self.dont_frag)).checked_shl(14).unwrap() 
        + u16::from(bool::from(self.more_frag)).checked_shl(13).unwrap() 
        + u16::from(self.frag_offset);
        words[4] = (self.ttl as u16).checked_shl(8).unwrap()  + self.protocol as u16;
        let sourceoctets = self.ip_source.octets();
        words[5] = (sourceoctets[0] as u16).checked_shl(8).unwrap() + (sourceoctets[1] as u16);
        words[6] = (sourceoctets[2] as u16).checked_shl(8).unwrap() + (sourceoctets[3] as u16);
        let destoctets = self.ip_dest.octets();
        words[7] = (destoctets[0] as u16).checked_shl(8).unwrap() + (destoctets[1] as u16);
        words[8] = (destoctets[2] as u16).checked_shl(8).unwrap() + (destoctets[3] as u16);
        words
    } 
    //We need to create
    pub fn checksum(self) -> u16 {
        // We concat the two 4 bit fields into an 8 bit field, then concat the two 8 bit fields.
        let words = self.cs_word_array();
        let mut result = u17::new(0);
        for index in 0..words.len(){
            
            result = result.wrapping_add(u17::from(words[index]));
            let mut carry = u32::from(result).checked_shr(16).unwrap();
            while carry != 0 {
                //println!("result = {}",result);
                result = result.wrapping_add(u17::from(carry as u16))-u17::from(u16::max_value())+u17::from(1u16);
                carry = u32::from(result).checked_shr(16).unwrap();
            }
            
        };
        !u32::from(result) as u16
    } 
    pub fn to_byte_buffer(self) -> Vec<u8> {
        let mut buf = vec![0u8;20];
        let mut index = 0;
        for word in self.to_word_array().iter() {
            let bytes = word.to_be_bytes();
            *buf.get_mut(index).unwrap() = bytes[0];
            *buf.get_mut(index).unwrap() = bytes[1];
            index+=2;
        }
        buf
    }
    
}

struct ICMPHeader {
    ipheader: IPv4Header,
    icmptype: u8, 
    code: u8,
    checksum: u16,
    data: u32,
}

pub fn test() {
    let mut iphead = IPv4Header::new(Ipv4Addr::new(127,0,0,1),Ipv4Addr::new(127,0,0,1));
    let mut iphead2 = IPv4Header::new(Ipv4Addr::new(127,0,0,1),Ipv4Addr::new(127,0,0,1));
    println!("Checksum:{:x}, {:x}",iphead.checksum(),iphead2.checksum());
    println!("Valid? {}",iphead.checksum_valid());
    println!("To Byte Buffer: {:?}", iphead.to_byte_buffer());
}





