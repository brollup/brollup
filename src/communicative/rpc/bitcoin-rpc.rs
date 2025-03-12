use crate::{prevout::Prevout, txn::outpoint::Outpoint};
use bitcoincore_rpc::{
    bitcoin::hashes::Hash,
    json::{ScanTxOutRequest, ScanTxOutResult},
    Auth, Client, RpcApi,
};

pub fn scan_prevouts() {
    let rpc_url = "http://159.203.92.87:38332";
    let rpc_user = "brollup";
    let rpc_password = "brollup";

    let rpc = Client::new(
        rpc_url,
        Auth::UserPass(rpc_user.to_string(), rpc_password.to_string()),
    )
    .unwrap();

    let scan_request = vec![ScanTxOutRequest::Single(
        "addr(tb1qtwr5ft6tc6cy2nayzu2pm9huzuqcecgh2vauat)".to_string(),
    )];

    let result: ScanTxOutResult = match rpc.scan_tx_out_set_blocking(&scan_request) {
        Ok(result) => result,
        Err(err) => return println!("rpc err: {}", err),
    };

    let mut prevouts = Vec::<Prevout>::new();

    for utxo in result.unspents.iter() {
        let outpoint = {
            let prev: [u8; 32] = utxo.txid.to_byte_array();
            let vout = utxo.vout;
            Outpoint::new(prev, vout)
        };

        let prevout = Prevout::new(
            outpoint,
            utxo.amount.to_sat(),
            utxo.script_pub_key.to_bytes(),
            Some(utxo.height),
        );

        prevouts.push(prevout);
    }

    print!("prevouts len: {}", prevouts.len());
}
