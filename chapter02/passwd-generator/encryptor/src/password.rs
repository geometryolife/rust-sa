use anyhow::{bail, Error, Result};
use base64::encode;
use hash::merhash::mersenne_hash;

/// 密码子（长度 100），可随意交换次序和增减字符，以实现个性化定制
const CRYPTO: &str = "!pqHr$*+ST1Vst_uv:?wWS%X&Y-/Z01_2.34<ABl\
9ECo|x#yDE^F{GHEI[]JK>LM#NOBWPQ:RaKU@}cde56R7=8f/9gIhi,jkzmn";

/// 哈希密码函数，旨在利用哈希值的高次幂来获取密码子中的字符
///
/// # 示例
/// ```
/// use encryptor::password::generate_password;
///
/// let seed = "jdwnp";
/// let length = 16;
/// let passwd = generate_password(seed, length);
///
/// match passwd {
///     Ok(val) => println!("{:#?}", val),
///     Err(err) => println!("{:#?}", err)
/// }
/// ```
pub fn generate_password(seed: &str, length: usize) -> Result<String, Error> {
    // 判断需要的密码长度，不能太短
    if length < 6 {
        bail!("length must >= 6"); // 返回错误
    }

    // 计算 mer_hash
    let p = match length {
        6..=10 => 1,
        11..=15 => 2,
        16..=20 => 3,
        _ => 3,
    };
    let mut mer_hash = mersenne_hash(seed).pow(p);

    // 由 mer_hash 求 passwd
    let mut passwd = String::new();
    let crypto_len = CRYPTO.len();

    while mer_hash > 9 {
        let loc = mer_hash % crypto_len;
        let nthc = CRYPTO.chars()
            .nth(loc)
            .expect("Error while getting char!");
        passwd.push(nthc);
        mer_hash /= crypto_len;
    }

    // 将 seed 中的字符和 passwd 拼接起来
    let interval = passwd.clone();
    for c in seed.chars() {
        passwd.push(c);
        passwd += &interval;
    }

    // 将 passwd 编码为 base64
    passwd = encode(passwd);
    passwd = passwd.replace("+", "*").replace("/", "*");

    // 长度不够，interval 来凑
    let interval = passwd.clone();
    while passwd.len() < length {
        passwd += &interval;
    }

    // 返回前 length 个字符作为密码
    Ok(format!("{}: {}", seed, &passwd[..length]))
}
