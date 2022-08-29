fn main() {
    //let mut file = File::create("base64-map.txt");
    let mut file_string = String::new();
    let mut byte = 0;
    let mut s = 65 as u8;
    for _ in 0..64 {
        let str_zero_or_one = format!("{:06b}", byte)
            .chars()
            .map(|zero_or_one| match zero_or_one {
                '0' => "Bit::Zero".to_string(),
                '1' => "Bit::One".to_string(),
                _ => panic!("{} is not 0 1", zero_or_one),
            })
            .fold(String::new(), |acc, cur| format!("{}{},", acc, cur));
        let char: Result<char, _> = s.try_into();
        s += 1;
        if s == 91 {
            s = 97
        }
        if s == 123 {
            s = 0;
        }
        file_string = format!(
            "{}({:?},Base64BitStream::new([{}])),\n",
            file_string,
            char.unwrap(),
            str_zero_or_one,
        );
        byte += 1;
    }
    println!("{}", file_string)
}
