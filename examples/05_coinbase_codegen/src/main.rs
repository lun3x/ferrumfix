#[allow(dead_code)]
#[rustfmt::skip]
mod gdax;

use fefix::definitions::fix50sp2;
use fefix::prelude::FieldAccess;
use fefix::tagvalue::Encoder;
use fefix::tagvalue::{Config, Decoder};
use fefix::{prelude, Dictionary, GetConfig};

const QUICKFIX_SPEC: &str = include_str!("FIX50-prod-sand.xml");

const FIX_MESSAGE_SECURITY_LIST_REQUEST: &[u8] = b"8=FIXT.1.1|9=19|35=x|320=XXX|559=4|10=000|";

fn main() {
    let mut config = Config::default();
    config.set_separator(b'|');
    config.set_verify_checksum(false);
    let mut decoder = fix_decoder();
    *decoder.config_mut() = config;
    let msg = decoder
        .decode(FIX_MESSAGE_SECURITY_LIST_REQUEST)
        .expect("Invalid FIX message");
    assert_eq!(msg.fv(gdax::BEGIN_STRING), Ok("FIXT.1.1"));
    assert_eq!(
        msg.fv(gdax::MSG_TYPE),
        Ok(gdax::MsgType::SecurityListRequest)
    );
    assert_eq!(msg.fv(fix50sp2::SECURITY_LIST_REQUEST_TYPE), Ok(4));

    let mut encoder = Encoder::new(config);
    let mut buffer = Vec::new();
    let mut msg = encoder.start_message(b"FIXT.1.1", &mut buffer, b"Logon");
    msg.set(fix50sp2::MSG_SEQ_NUM, 215);
    msg.set(fix50sp2::SENDER_COMP_ID, "CLIENT12");
    msg.set(fix50sp2::TARGET_COMP_ID, "Coinbase");
    msg.set(fix50sp2::ENCRYPT_METHOD, fix50sp2::EncryptMethod::None);
    msg.set(fix50sp2::HEART_BT_INT, 10);
    let msg = msg.wrap();
    println!("{}", std::str::from_utf8(msg).unwrap());
}

fn fix_decoder() -> Decoder<Config> {
    let fix_dictionary = Dictionary::from_quickfix_spec(QUICKFIX_SPEC).unwrap();
    Decoder::<Config>::new(fix_dictionary)
}
