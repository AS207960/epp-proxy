#[derive(Debug)]
pub struct TrnData {
    pub name: String
}

impl From<super::proto::traficom::EPPObjTrnData> for TrnData {
    fn from(from: super::proto::traficom::EPPObjTrnData) -> Self {
        TrnData {
            name: from.name,
        }
    }
}