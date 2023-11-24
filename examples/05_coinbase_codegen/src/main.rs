#[allow(dead_code)]
#[rustfmt::skip]
// mod gdax;

use fefix::definitions::fix50sp2;
use fefix::tagvalue::Encoder;
use fefix::tagvalue::{Config, Decoder};
use fefix::{prelude, Dictionary, FieldMap, GetConfig, RepeatingGroup, SetField};

const QUICKFIX_SPEC: &str = include_str!("FIX50-prod-sand.xml");

const FIX_MESSAGE: &[u8] = b"8=FIXT.1.1\x019=254\x0135=X\x0149=Coinbase\x0156=97735e75e5f69eb664c90b14314380ac\x0134=806\x0150=TEST\x0152=20231124-15:26:48.854322\x01369=3\x01262=REQ_XLM-USD\x01268=1\x01279=0\x01269=1\x01278=33800899-9cb6-488b-b749-b5aa70fb8fae\x0183=14789029420\x0155=XLM-USD\x01270=0.118482\x01271=23497\x0160=20231124-15:26:48.852606\x0110=002\x01";

fn main() {
    let mut config = Config::default();
    // config.separator = b'|';
    config.verify_checksum = false;

    let mut decoder = fix_decoder();
    *decoder.config_mut() = config;
    let msg = decoder.decode(FIX_MESSAGE).expect("Invalid FIX message");

    assert_eq!(msg.get(fix50sp2::BEGIN_STRING), Ok("FIXT.1.1"));
    assert_eq!(
        msg.get(fix50sp2::MSG_TYPE),
        Ok(fix50sp2::MsgType::Marketdataincrementalrefresh)
    );
    let group = msg.group(fix50sp2::NO_MD_ENTRIES).unwrap();
    assert!(group.len() == 1);
    for i in 0..group.len() {
        let msg = group.get(i).unwrap();
        assert_eq!(msg.get(fix50sp2::MD_UPDATE_ACTION), Ok(0));
        println!("{}", msg.get::<&str>(fix50sp2::MD_UPDATE_ACTION).unwrap());
        println!("{}", msg.get::<&str>(fix50sp2::MD_ENTRY_TYPE).unwrap());
    }

    // let mut encoder = Encoder::new();
    // *encoder.config_mut() = config;

    // let mut buffer = Vec::new();
    // let mut msg: fefix::tagvalue::EncoderHandle<'_, Vec<u8>> =
    //     encoder.start_message(b"FIXT.1.1", &mut buffer, b"Logon");
    // msg.set(fix50sp2::MSG_SEQ_NUM, 215);
    // msg.set(fix50sp2::SENDER_COMP_ID, "CLIENT12");
    // msg.set(fix50sp2::TARGET_COMP_ID, "Coinbase");
    // msg.set(fix50sp2::ENCRYPT_METHOD, fix50sp2::EncryptMethod::None);
    // msg.set(fix50sp2::HEART_BT_INT, 10);
    // let (msg, offset) = msg.done();
    // println!("{}", std::str::from_utf8(msg).unwrap());
}

fn fix_decoder() -> Decoder {
    let fix_dictionary = Dictionary::fix50sp2();
    Decoder::new(fix_dictionary)
}

#[cfg(test)]
#[test]
fn run() {
    main();
}
