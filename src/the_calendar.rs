use std::collections::HashMap;

pub struct Calendar {
    header: String,
    body: HashMap<(u32, u32), String>, // (month, date), item
}

impl Calendar {
    pub fn new(raw_data: String) -> Self {
        let chunk: Vec<&str> = raw_data.split("\r\n\r\n").collect();
        let header: String = chunk[0].to_string();
        let mut body = HashMap::new();

        for i in (1..13) {
            let mut lines = chunk[i].lines();
            let month = {
                let temp:Vec<&str> = lines.next().unwrap().split('\u{3000}').collect();
                temp[1].strip_suffix("月").unwrap().parse::<u32>().unwrap()
            };

            while let Some(line) = lines.next() {
                let (date, item) = {
                    let temp: Vec<&str> = line.splitn(3, |c| c == '\t' || c == '\u{3000}').collect();
                    (temp[0].strip_suffix("日").unwrap().parse::<u32>().unwrap(), temp[2].to_string())
                };
                body.insert((month, date), item);
            }
        }

        Self {
            header: header,
            body: body
        }
    }

    pub fn get(&self, month: u32, date: u32) {
        if let Some(result) = self.body.get(&(month, date)) {
            println!("{result}");
        } else {
            println!("収集なし");
        }
    }
}
