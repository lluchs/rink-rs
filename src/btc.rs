use std::fs::File;
use std::time::Duration;
use ast::{Defs, Def, Expr};
use std::io::Read;
use std::rc::Rc;
use json;

static URL: &'static str = "https://btc-e.com/api/3/ticker/btc_usd-ltc_usd-nmc_usd-nvc_usd-ppc_usd-eth_usd";

pub fn parse(mut f: File) -> Result<Defs, String> {
    let mut buf = String::new();
    try!(f.read_to_string(&mut buf).map_err(|x| format!("{}", x)));
    let parsed = try!(json::parse(&*buf).map_err(|x| format!("{}", x)));
    let mut out = vec![];
    for (k, v) in parsed.entries() {
        if k.ends_with("_usd") {
            let name = k[0..k.len()-"_usd".len()].to_uppercase();
            let avg = v["avg"].as_number();
            if let Some(avg) = avg {
                let (sign, mantissa, exp) = avg.as_parts();
                let integer = format!("{}{}", if sign { "" } else { "-" }, mantissa);
                out.push((name, Rc::new(Def::Unit(
                    Expr::Mul(vec![
                        Expr::Const(integer, None, Some(exp.to_string())),
                        Expr::Unit("USD".to_owned())
                    ])))));
            }
        }
    }
    Ok(Defs {
        defs: out
    })
}

pub fn load() -> Result<Defs, String> {
    ::cached("btc.json", URL, Duration::from_secs(3*60*60)).and_then(parse)
}
