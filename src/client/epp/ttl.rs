use crate::client::ttl::{TTLInfo, TTLSet, TTL, Record};
use crate::proto;

impl From<&proto::ttl::EPPTTLInfoData> for TTLInfo {
    fn from(from: &proto::ttl::EPPTTLInfoData) -> Self {
        TTLInfo {
            ttl: from.ttl.iter().map(|t| TTL {
                ttl: t.ttl,
                record: match t.for_rr {
                    proto::ttl::EPPTTLRR::NS => Record::NS,
                    proto::ttl::EPPTTLRR::DS => Record::DS,
                    proto::ttl::EPPTTLRR::DNAME => Record::DNAME,
                    proto::ttl::EPPTTLRR::A => Record::A,
                    proto::ttl::EPPTTLRR::AAAA => Record::AAAA,
                    proto::ttl::EPPTTLRR::Custom => Record::Custom(t.custom_rr.clone()),
                },
                min: t.min,
                max: t.max,
                default: t.default,
            }).collect()
        }
    }
}

impl From<&TTLSet> for proto::ttl::EPPTTLSet {
   fn from(from: &TTLSet) -> Self {
       proto::ttl::EPPTTLSet {
           ttl: from.ttl.iter().map(|t| proto::ttl::EPPTTLCommand {
               ttl: t.ttl,
               for_rr: match &t.record {
                   Record::NS => proto::ttl::EPPTTLRR::NS,
                   Record::DS => proto::ttl::EPPTTLRR::DS,
                   Record::DNAME => proto::ttl::EPPTTLRR::DNAME,
                   Record::A => proto::ttl::EPPTTLRR::A,
                   Record::AAAA => proto::ttl::EPPTTLRR::AAAA,
                   Record::Custom(_) => proto::ttl::EPPTTLRR::Custom,
               },
               custom_rr: match &t.record {
                   Record::Custom(c) => Some(c.clone()),
                   _ => None
               }
           }).collect(),
       }
   } 
}