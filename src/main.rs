use std::io::prelude::*;
use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::*;
use std::path::*;
use url_escape::*;

mod pages;
mod lib;

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
    let admin_ips = read_to_string("./.admins").unwrap_or("".to_owned());
    let admin_ips = admin_ips.split("\r\n").collect::<Vec<_>>();
    let connection = stream.local_addr().unwrap();
    let ip = connection.ip().to_string();
    let port = connection.port();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Connection from {}:{}", ip, port);

    let request = String::from_utf8((&buffer[..]).to_vec()).unwrap();

    let req: Vec<&str> = request.lines().collect();

    match req[0].split(" ").collect::<Vec<_>>()[0] {
        "GET" => get(stream, req, admin_ips.contains(&&ip[..])),
        "HEAD" => head(stream, req),
        "OPTIONS" => options(stream, req),
        "PUT" => if (&admin_ips.contains(&&ip[..])).to_owned() || admin_ips[0] == "" { put(stream, req)},
        "POST" => if (&admin_ips.contains(&&ip[..])).to_owned() || admin_ips[0] == "" { post(stream, req)},
        "DELETE" => if (&admin_ips.contains(&&ip[..])).to_owned() || admin_ips[0] == "" { delete(stream, req)},
        _ => println!("Invalid Request received: {}", request)
    }
}

fn get(mut stream: TcpStream, request: Vec<&str>, is_admin: bool) {
    let path = request[0].split(" ").collect::<Vec<_>>()[1];

    let dir = "./src";

    let mut file_path = decode(&format!("{}{}", &dir, match path {
        "/" => "",
        _ => &path[..path.find("?").unwrap_or(path.len())]
    })).to_string();

    let contents;
    let mut urls: Vec<String> = vec![];

    println!("GET {}", &file_path[&file_path.rfind(&dir).unwrap()+&dir.len()..]);

    let mut code = "200 OK";
    let exists = Path::new(&file_path).exists();
    let is_file = Path::new(&file_path).is_file();
    if exists && !is_file && Path::new(&format!("{}/index.html", &file_path)).exists() {
         file_path = format!("{}/index.html", &file_path);
         contents = read_to_string(&file_path).unwrap();
    }
    else if is_file {
        contents = match read_to_string(&file_path) {
            Ok(result) => result,
            _ => {
                code = "403 Forbidden";
                read_to_string("./src/403.html").unwrap_or(pages::P403.to_owned())
            }
        }
    }
    else if exists {
        let mut files = String::new();
        let mut dirs = String::new();
        for file in read_dir(&file_path).unwrap() {
            let path = file.unwrap().path().display().to_string().replace("\\", "/");
            urls.push((&path).to_owned());

            if !Path::new(&path).is_file(){
                dirs = format!("{}{}", &dirs, read_to_string("./src/dir.html").unwrap_or("<a href=\"{url}\"><div class='dirEntry'>
              <svg version='1' xmlns='http://www.w3.org/2000/svg' viewBox='0 0 48 48' enable-background='new 0 0 48 48'>
                  <path fill='#E1F5FE' d='M40,12H22l-4-4H8c-2.2,0-4,1.8-4,4v8h40v-4C44,13.8,42.2,12,40,12z'/>
                  <path fill='#90CAF9' d='M40,12H8c-2.2,0-4,1.8-4,4v20c0,2.2,1.8,4,4,4h32c2.2,0,4-1.8,4-4V16C44,13.8,42.2,12,40,12z'/>
              </svg>
              <span>{name}</span>
              </div></a>".to_owned()))
              .replace("{url}", &encode_path(&path[..][path.rfind(dir).unwrap()+dir.len()..]).to_string())
              .replace("{name}", &path[..][path.rfind('/').unwrap()+1..]);
            }
            else {
                files = format!("{}{}",  &files, read_to_string("./src/file.html").unwrap_or("<a href=\"{url}\"><div class='dirEntry'>
                <svg xmlns='http://www.w3.org/2000/svg'  viewBox='0 0 48 48'>
                    <path fill='#90CAF9' d='M40 45L8 45 8 3 30 3 40 13z'/>
                    <path fill='#E1F5FE' d='M38.5 14L29 14 29 4.5z'/>
                </svg>
                <span>{name}</span>
                </div></a>".to_owned()))
                .replace("{url}", &encode_path(&path[..][path.rfind(dir).unwrap()+dir.len()..]).to_string())
                .replace("{name}", &path[..][path.rfind('/').unwrap()+1..]);
            }
        }
        contents = read_to_string("./explorer.html").unwrap_or(pages::EXPLORER.to_owned())
        .replace("{path}", match file_path.len() {
            5 => "/",
            _ => &file_path[dir.len()..]
        }) 
        .replace("{content}", &format!("{}{{content}}", &dirs)[..])
        .replace("{content}", &files)
        .replace("{isAdmin}", match is_admin {
            true => "true",
            _ => "false"
        })
        .replace("{urls}", &format!("[{}]", lib::urls_to_js(urls)));

    }
    else {
        code = "404 Not Found";
        contents = read_to_string("./src/404.html").unwrap_or(pages::P404.to_owned());
    }

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        code,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn head(mut stream: TcpStream, request: Vec<&str>) {
    let path = request[0].split(" ").collect::<Vec<_>>()[1];

    let dir = "./src";

    let mut file_path = decode(&format!("{}{}", &dir, match path {
        "/" => "",
        _ => &path[..path.find("?").unwrap_or(path.len())]
    })).to_string();

    let contents;

    println!("HEAD {}", &file_path[&file_path.rfind(&dir).unwrap()+&dir.len()..]);
    let mut urls: Vec<String> = vec![];
    let mut code = "200 OK";
    let exists = Path::new(&file_path).exists();
    let is_file = Path::new(&file_path).is_file();
    if exists && !is_file && Path::new(&format!("{}/index.html", &file_path)).exists() {
         file_path = format!("{}/index.html", &file_path);
         contents = read_to_string(&file_path).unwrap();
    }
    else if is_file {
        if !File::open(&file_path).unwrap().metadata().unwrap().permissions().readonly() {
            contents = read_to_string(&file_path).unwrap();
        }
        else {
            code = "403 Forbidden";
            contents = read_to_string("./src/403.html").unwrap_or(pages::P403.to_owned());
        }
    }
    else if exists {
        let mut files = String::new();
        let mut dirs = String::new();
        for file in read_dir(&file_path).unwrap() {
            let path = file.unwrap().path().display().to_string().replace("\\", "/");
            urls.push((&path).to_owned());
            if !Path::new(&path).is_file(){
                dirs = format!("{}{}", &dirs, read_to_string("./src/dir.html").unwrap_or("<div onclick=\"window.location.href='{url}';\" class='dirEntry'>
              <svg version='1' xmlns='http://www.w3.org/2000/svg' viewBox='0 0 48 48' enable-background='new 0 0 48 48'>
                  <path fill='#E1F5FE' d='M40,12H22l-4-4H8c-2.2,0-4,1.8-4,4v8h40v-4C44,13.8,42.2,12,40,12z'/>
                  <path fill='#90CAF9' d='M40,12H8c-2.2,0-4,1.8-4,4v20c0,2.2,1.8,4,4,4h32c2.2,0,4-1.8,4-4V16C44,13.8,42.2,12,40,12z'/>
              </svg>
              <span>{name}</span>
              </div>".to_owned()))
              .replace("{url}", &path[..][path.rfind(dir).unwrap()+dir.len()..])
              .replace("{name}", &path[..][path.rfind('/').unwrap()+1..]);
            }
            else {
                files = format!("{}{}",  &files, read_to_string("./src/file.html").unwrap_or("<div onclick=\"window.location.href='{url}';\" class='dirEntry'>
                <svg xmlns='http://www.w3.org/2000/svg'  viewBox='0 0 48 48'>
                    <path fill='#90CAF9' d='M40 45L8 45 8 3 30 3 40 13z'/>
                    <path fill='#E1F5FE' d='M38.5 14L29 14 29 4.5z'/>
                </svg>
                <span>{name}</span>
                </div>".to_owned()))
                .replace("{url}", &path[..][path.rfind(dir).unwrap()+dir.len()..])
                .replace("{name}", &path[..][path.rfind('/').unwrap()+1..]);
            }
        }
        contents = read_to_string("./explorer.html").unwrap_or(pages::EXPLORER.to_owned())
        .replace("{path}", match file_path.len() {
            5 => "/",
            _ => &file_path[dir.len()..]
        })
        .replace("{content}", &format!("{}{{content}}", &dirs)[..])
        .replace("{content}", &files)
        .replace("{urls}", &format!("[{}]", lib::urls_to_js(urls)));
    }
    else {
        code = "404 Not Found";
        contents = read_to_string("./src/404.html").unwrap_or(pages::P404.to_owned());
    }

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n",
        code,
        contents.len()
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn options(mut stream: TcpStream, request: Vec<&str>) {
    let data = request[request.len()-1];
    let response = format!(
        "HTTP/1.1 200 OK\r\nAllow: OPTIONS, GET, HEAD\r\nContent-Length: {}",
        data.len()
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn put(mut stream: TcpStream, request: Vec<&str>) {
    let data = request[request.len()-1];
    let mut response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}",
        data.len()
    );
    let content = request[request.len()-1];
    let file = request[0].split(" ").collect::<Vec<_>>()[1];
    println!("PUT {} Content: {}", file, content);
    if !Path::new(&format!("./src{}", file)).exists() {
        let _ = write(&format!("./src{}", file), content).expect("Unable to write file");
    }
    else {
        response = format!(
            "HTTP/1.1 403 Forbidden\r\nContent-Length: {}",
            data.len()
        );
    }

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn post(mut stream: TcpStream, request: Vec<&str>) {
    let data = request[request.len()-1];
    let mut response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}",
        data.len()
    );
    let content = request[request.len()-1].replace("\\n", "\n").replace("\0", "");
    let file = request[0].split(" ").collect::<Vec<_>>()[1];
    println!("POST {} Content: {}", file, content);
    match write(&format!("./src{}", file), content) {
        Ok(_) => (),
        _ => response = format!(
            "HTTP/1.1 403 Forbidden\r\nContent-Length: {}",
            data.len()
            )
    }
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn delete(mut stream: TcpStream, request: Vec<&str>) {
    let data = request[request.len()-1];
    let mut response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}",
        data.len()
    );
    let file = request[0].split(" ").collect::<Vec<_>>()[1];
    println!("DELETE {}", file);
    match remove_file(&format!("./src{}", file)) {
        Ok(_) => (),
        _ => response = format!(
            "HTTP/1.1 403 Forbidden\r\nContent-Length: {}",
            data.len()
            )
    }
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
