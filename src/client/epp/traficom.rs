use super::super::traficom::TrnData;

impl From<super::proto::traficom::EPPObjTrnData> for TrnData {
    fn from(from: super::proto::traficom::EPPObjTrnData) -> Self {
        TrnData { name: from.name }
    }
}
