pub mod password; // 导出 password 模块

#[cfg(test)]
mod tests {
    use crate::password::generate_password;

    #[test]
    fn generate_password_works() {
        let seed = "jdwnp";
        let length = 16;
        let passwd = generate_password(seed, length);

        match passwd {
            Ok(val) => println!("{:#?}", val),
            Err(err) => println!("{:#?}", err)
        }
    }
}