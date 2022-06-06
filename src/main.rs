use std::io::prelude::*;
use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::*;
use std::path::*;
use url_escape::*;

fn main() {
    let port = 7878;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    println!("Server running on Port {}", port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8((&buffer[..]).to_vec()).unwrap();

    let request_parsed = request.split(" ").collect::<Vec<&str>>().to_vec();

    let path = request_parsed[1];

    let dir = "./src";

    let mut file_path = decode(&format!("{}{}", &dir, match path {
        "/" => "",
        _ => &path[..path.find("?").unwrap_or(path.len())]
    })).to_string();

    let contents;

    println!("GET {}", &file_path[&file_path.rfind(&dir).unwrap()+&dir.len()..]);

    let exists = Path::new(&file_path).exists();
    let is_file = Path::new(&file_path).is_file();
    if exists && !is_file && Path::new(&format!("{}/index.html", &file_path)).exists() {
         file_path = format!("{}/index.html", &file_path);
         contents = read_to_string(&file_path).unwrap();
    }
    else if is_file {
        contents = read_to_string(&file_path).unwrap();
    }
    else if exists {
        let mut files = String::new();
        let mut dirs = String::new();
        for file in read_dir(&file_path).unwrap() {
            let path = file.unwrap().path().display().to_string().replace("\\", "/");
            if !Path::new(&path).is_file(){
                dirs = format!("{}<a href='{}'><div class='dirEntry'>
              <svg version='1' xmlns='http://www.w3.org/2000/svg' viewBox='0 0 48 48' enable-background='new 0 0 48 48'>
                  <path fill='#E1F5FE' d='M40,12H22l-4-4H8c-2.2,0-4,1.8-4,4v8h40v-4C44,13.8,42.2,12,40,12z'/>
                  <path fill='#90CAF9' d='M40,12H8c-2.2,0-4,1.8-4,4v20c0,2.2,1.8,4,4,4h32c2.2,0,4-1.8,4-4V16C44,13.8,42.2,12,40,12z'/>
              </svg>
              <span>{}</span>
              </div></a>",  &dirs, &path[..][path.rfind(dir).unwrap()+dir.len()..], &path[..][path.rfind('/').unwrap()+1..]);
            }
            else {
                files = format!("{}<a href='{}'><div class='dirEntry'>
              <svg xmlns='http://www.w3.org/2000/svg'  viewBox='0 0 48 48'>
                <path fill='#90CAF9' d='M40 45L8 45 8 3 30 3 40 13z'/>
                <path fill='#E1F5FE' d='M38.5 14L29 14 29 4.5z'/>
                </svg>
              <span>{}</span>
              </div></a>",  &files, &path[..][path.rfind(dir).unwrap()+dir.len()..], &path[..][path.rfind('/').unwrap()+1..]);
            }
        }
        contents = format!("<html>
            <head>
                <style>
                    @import url('https://fonts.googleapis.com/css2?family=Ubuntu:ital,wght@0,300;0,400;0,500;1,300;1,400;1,500;1,700&display=swap');
                    body {{
                        display: flex;
                        flex-direction: column;
                        background-color: #251D3A;
                        font-family: Ubuntu, sans-serif;
                        margin: 0;
                    }}

                    a {{
                        text-decoration: none;
                    }}

                    .dirEntry {{
                        aspect-ratio: 1 / 1;
                        display: flex;
                        flex-direction: column;
                        height: 100px;
                        width: 100px;
                        align-items: center;
                        background-color: #2A2550;
                        padding: 10px;
                        border-radius: 10px;
                        color: white;
                        margin: 5px;
                        cursor: pointer;
                        transition: all 0.5s ease-in-out;
                        box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
                    }}

                    .dirEntry:hover {{
                        background-color: #EC994B;
                    }}

                    .path {{
                        font-size: 20px;
                        color: white;
                        margin: 5px;
                    }}

                    .entries {{
                        display: flex;
                        flex-direction: row;
                        width: 100%;
                        overflow-y: hidden;
                        flex-wrap: wrap;
                        align-items: center;
                        justify-content: center;
                    }}

                    .topBar {{
                        display: flex;
                        flex-direction: row;
                        align-items: center;
                        padding: 5px;
                        background-color: #2A2550;
                        box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
                        margin-bottom: 10px;
                    }}

                    .back, .forward {{
                        background-color: #3A3845;
                        margin: 0;
                        display: grid;
                        place-items: center;
                        width: 30px;
                        height: 30px;
                        border: none;
                        cursor: pointer;
                        transition: all 0.5s ease-in-out;
                    }}

                    .back:hover, .forward:hover {{
                        background-color: #EC994B;
                    }}

                    .back {{
                        border-radius: 10px 0 0 10px;
                    }}

                    .forward {{
                        border-radius: 0 10px 10px 0;
                    }}
                </style>
            </head>

            <body>
                <div class='topBar'>
                <button onclick='history.back()' class='back'>
                <svg style='transform: rotate(180deg)' xmlns='http://www.w3.org/2000/svg' height='22px' viewBox='0 0 22 22'><defs><clipPath><path fill='#00f' fill-opacity='.514' d='m-7 1024.36h34v34h-34z'/></clipPath><clipPath><path fill='#ffffff' fill-opacity='.472' d='m-6 1028.36h32v32h-32z'/></clipPath></defs><path d='m345.44 248.29l-194.29 194.28c-12.359 12.365-32.397 12.365-44.75 0-12.354-12.354-12.354-32.391 0-44.744l171.91-171.91-171.91-171.9c-12.354-12.359-12.354-32.394 0-44.748 12.354-12.359 32.391-12.359 44.75 0l194.29 194.28c6.177 6.18 9.262 14.271 9.262 22.366 0 8.099-3.091 16.196-9.267 22.373' transform='matrix(.03541-.00013.00013.03541 2.98 3.02)' fill='#ffffff'/></svg>
                </button>
                <button onclick='history.forward()' class='forward'>
                <svg xmlns='http://www.w3.org/2000/svg' height='22px' viewBox='0 0 22 22'><defs><clipPath><path fill='#00f' fill-opacity='.514' d='m-7 1024.36h34v34h-34z'/></clipPath><clipPath><path fill='#ffffff' fill-opacity='.472' d='m-6 1028.36h32v32h-32z'/></clipPath></defs><path d='m345.44 248.29l-194.29 194.28c-12.359 12.365-32.397 12.365-44.75 0-12.354-12.354-12.354-32.391 0-44.744l171.91-171.91-171.91-171.9c-12.354-12.359-12.354-32.394 0-44.748 12.354-12.359 32.391-12.359 44.75 0l194.29 194.28c6.177 6.18 9.262 14.271 9.262 22.366 0 8.099-3.091 16.196-9.267 22.373' transform='matrix(.03541-.00013.00013.03541 2.98 3.02)' fill='#ffffff'/></svg>
                </button>
                <span class='path'>
                    {}
                </span>
                </div>
                <div class='entries'>
                {}
                {}
                </div>
            </body>
        </html>", match file_path.len() {
            5 => "/",
            _ => &file_path[5..]
        }, &dirs, &files).to_owned();
    }
    else {
        contents = read_to_string(format!("{}/404.html", &dir)).unwrap();
    }

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
