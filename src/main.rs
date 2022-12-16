extern crate csv;

fn main() {
    println!("this is for transform csv data to json");
    println!("csv like this will parse like below json");
    println!("line1: title | day");
    println!("line2: hiyoko| 23");
    println!("line3: piyo  | 0");
    println!("->json");
    //{{, }}, \"はそれぞれ{, }, "のエスケープ
    println!("
    {{
        please_rewrite_me: [
            {{
                \"title\": \"hiyoko\",
                \"day\": \"0\"
            }},
            {{
                \"title\": \"piyo\",
                \"day\": \"0\"
            }}
        ]
    }}
    ");

    

    println!("then input name of the target .csv file");
    println!("ex: if test.csv, input \"test\". then test.json will export.");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("err: can't read name. harfsize alphabet is recommended");
    let name = buf.trim().to_string();

    println!("now, serch for file {}.csv", name);
    let path = format!("./{}.csv", name);
    println!("target path: {}", path);
    let file = std::fs::File::open(path).expect("err: can't find file. check if the filename is correct and file exists in the same folder.");

    println!("find file.");
    let mut reader = csv::Reader::from_reader(file);
    
    let headers = reader.headers().unwrap().clone();
    println!("column headers are");
    println!("{:?}", headers);
    println!("counted column header is {}", headers.len());

    println!("contents are");
    let mut inner_json = String::new();
    for result in reader.records() {
        let record = result.expect("err: unable to read line");
        println!("{:?}", record);

        let mut data = String:: new();
        for i in 0..headers.len() as usize {
            //strだとコンパイル時のサイズが未定とか言われるのでto_string
            if i == headers.len() -1 {
                //最後は,をつけない
                data += &format!("\"{}\": \"{}\"\n", headers[i].to_string(), record[i].to_string());
            }else{
                data += &format!("\"{}\": \"{}\",\n", headers[i].to_string(), record[i].to_string());
            }
            
        }
        //これでひとかたまり
        inner_json += &format!("{{
                {}
            }},", data);
    }

    let mut json = String::new();
    json += &format!("{{
        \"{}\": [
            {}
        ]
    }}", name, inner_json);


    //書き出し
    println!("format ok. start make json file. file name is {}.json", name);
    let json_path = format!("./{}.json", name);
    println!("target path is {}", json_path);
    let mut json_file = std::fs::File::create(json_path).expect("unable to create file. makesure this directory is ok ");
    

    let buf: Vec<u8> = json.bytes().collect();
    use std::io::Write;
    json_file.write_all(&buf).expect("failed to write string to json file");


    println!("\n\nfinished! \npress Enter to close this.");
    let mut dummy = String::new();
    std::io::stdin().read_line(&mut dummy).unwrap();



    
}
