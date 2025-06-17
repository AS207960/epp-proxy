use super::super::client;
use super::epp_proto;
use std::convert::TryFrom;

impl TryFrom<epp_proto::ttl::TtlSet> for client::ttl::TTLSet {
    type Error = tonic::Status;

    fn try_from(from: epp_proto::ttl::TtlSet) -> Result<Self, Self::Error> {
        Ok(client::ttl::TTLSet {
            ttl: from
                .ttl
                .into_iter()
                .map(|t| Ok(client::ttl::TTLCommand {
                    ttl: t.ttl,
                    record: match t.record {
                        Some(epp_proto::ttl::ttl_command::Record::WkRecord(r)) => {
                            match epp_proto::ttl::Record::try_from(r) {
                                Ok(epp_proto::ttl::Record::Ns) => client::ttl::Record::NS,
                                Ok(epp_proto::ttl::Record::Ds) => client::ttl::Record::DS,
                                Ok(epp_proto::ttl::Record::Dname) => client::ttl::Record::DNAME,
                                Ok(epp_proto::ttl::Record::A) => client::ttl::Record::A,
                                Ok(epp_proto::ttl::Record::Aaaa) => client::ttl::Record::AAAA,
                                Err(_) => {
                                    return Err(tonic::Status::invalid_argument(
                                        "Invalid record type",
                                    ))
                                }
                            }
                        }
                        Some(epp_proto::ttl::ttl_command::Record::CustomRecord(r)) => {
                            client::ttl::Record::Custom(r)
                        },
                        None => {
                            return Err(tonic::Status::invalid_argument(
                                "Record type must be specified",
                            ))
                        }
                    },
                }))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl From<client::ttl::TTLInfo> for epp_proto::ttl::TtlInfo {
    fn from(from: client::ttl::TTLInfo) -> Self {
        epp_proto::ttl::TtlInfo {
            ttl: from
                .ttl
                .into_iter()
                .map(|t| epp_proto::ttl::Ttl {
                    ttl: t.ttl,
                    min: t.min,
                    max: t.max,
                    default: t.default,
                    record: Some(match t.record {
                        client::ttl::Record::NS => {
                            epp_proto::ttl::ttl::Record::WkRecord(epp_proto::ttl::Record::Ns.into())
                        }
                        client::ttl::Record::DS => {
                            epp_proto::ttl::ttl::Record::WkRecord(epp_proto::ttl::Record::Ds.into())
                        }
                        client::ttl::Record::DNAME => epp_proto::ttl::ttl::Record::WkRecord(
                            epp_proto::ttl::Record::Dname.into(),
                        ),
                        client::ttl::Record::A => {
                            epp_proto::ttl::ttl::Record::WkRecord(epp_proto::ttl::Record::A.into())
                        }
                        client::ttl::Record::AAAA => epp_proto::ttl::ttl::Record::WkRecord(
                            epp_proto::ttl::Record::Aaaa.into(),
                        ),
                        client::ttl::Record::Custom(r) => {
                            epp_proto::ttl::ttl::Record::CustomRecord(r)
                        }
                    }),
                })
                .collect(),
        }
    }
}
