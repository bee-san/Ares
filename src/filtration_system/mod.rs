///! Proposal: https://broadleaf-angora-7db.notion.site/Filtration-System-7143b36a42f1466faea3077bfc7e859e
///! Given a filter object, return an array of decoders/crackers which have been filtered
/// 

use crate::decoders::base64_decoder::{Base64Decoder};

/// Currently takes no args as this is just a spike to get all the basic functionality working
pub fn filter_and_get_decoders() {
    let base64 = Base64Decoder::new();
    let decoders = vec![base64];
}