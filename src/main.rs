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

    if request_parsed[0] == "GET" {
        get(stream, request_parsed);
    }
}

fn get(mut stream: TcpStream, request: Vec<&str>) {
    let path = request[1];

    let dir = "./src";

    let mut file_path = decode(&format!("{}{}", &dir, match path {
        "/" => "",
        _ => &path[..path.find("?").unwrap_or(path.len())]
    })).to_string();

    let contents;

    println!("GET {}", &file_path[&file_path.rfind(&dir).unwrap()+&dir.len()..]);

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
            contents = read_to_string("./src/403.html").unwrap_or("<!DOCTYPE html>
        <html lang='en'>
        <head>
            <meta charset='UTF-8'>
            <meta http-equiv='X-UA-Compatible' content='IE=edge'>
            <meta name='viewport' content='width=device-width, initial-scale=1.0'>
            <title>404</title>
            <style>
            @import url('https://fonts.googleapis.com/css2?family=Ubuntu:ital,wght@0,300;0,400;0,500;1,300;1,400;1,500;1,700&display=swap');
            
            body {
                color: white;
                background-color: #251D3A;
                font-family: Ubuntu, sans-serif;
                position: absolute;
                inset: 0;
                margin: 0;
            }
        
            .content {
                display: grid;
                place-items: center;
            }
        
            .content div {
                display: flex;
                flex-direction: column;
                align-items: center;
            }
        
            h1 {
                font-size: 60px;
            }
        
            span {
                font-size: 24px;
            }
        
            p {
                font-size: 18px;
            }
        
            .license {
                height: 10px;
                color: #ffffff;
                position: absolute;
                right: 5px;
                top: 5px;
            }
        
            .topBar {
                display: flex;
                flex-direction: row;
                align-items: center;
                padding: 5px;
                height: 40px;
                background-color: #2A2550;
                box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
                margin-bottom: 10px;
            }
        
            .back, .forward {
                background-color: #3A3845;
                margin: 0;
                display: grid;
                place-items: center;
                width: 30px;
                height: 30px;
                border: none;
                cursor: pointer;
                transition: all 0.5s ease-in-out;
            }
        
            .back:hover, .forward:hover {
                background-color: #EC994B;
            }
        
            .back {
                border-radius: 10px 0 0 10px;
            }
        
            .forward {
                border-radius: 0 10px 10px 0;
            }
        
            .pageInfo {
                position: fixed;
                width: 30%;
                left: 35%;
                height: 50%;
                bottom: -100%;
                background-color: rgb(58, 56, 69);
                text-align: center;
                color: white;
                border-radius: 50px;
                transition: all 0.5s ease-in-out;
            }
        
            .pageInfo a {
                text-decoration: underline;
                color: white;
            }
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
                <a id='info' class='license'>
                <svg xmlns='http://www.w3.org/2000/svg'  viewBox='0 0 48 48' width='40px' height='40px'><path fill='#2196f3' d='M44,24c0,11.045-8.955,20-20,20S4,35.045,4,24S12.955,4,24,4S44,12.955,44,24z'/><path fill='#fff' d='M22 22h4v11h-4V22zM26.5 16.5c0 1.379-1.121 2.5-2.5 2.5s-2.5-1.121-2.5-2.5S22.621 14 24 14 26.5 15.121 26.5 16.5z'/></svg>
                </a>
                </div>
            <div class='content'>
                <div>
                <h1>403</h1>
                <span>Oops! That's forbidden!</span>
                <p>You don't have permission to the ressource you requested.</p>
                </div>
            </div>
            <div class='pageInfo'>
                <h2>Info</h2>
                Website powered by the <a href='https://github.com/BlackBirdTV/netxplorer'>NetXplorer</a><br>
                Icons by <a href='https://icons8.com'>Icons8</a><br>
                Ubuntu font used under <a href='https://ubuntu.com/legal/font-licence'>this License</a>
            </div>
            <script>
                const pageInfo = document.querySelector('.pageInfo');
                const info = document.querySelector('#info');
        
                let infoShown = false;
        
                info.addEventListener('click', () => {{
                    infoShown = !infoShown;
                    pageInfo.style.bottom = (infoShown ? '25%' : '-100%');
                    console.log((infoShown ? '25%' : '-100%'));
                    console.log(infoShown)
                }})
            </script>
        </body>
        </html>".to_owned());
        }
    }
    else if exists {
        let mut files = String::new();
        let mut dirs = String::new();
        for file in read_dir(&file_path).unwrap() {
            let path = file.unwrap().path().display().to_string().replace("\\", "/");
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
        contents = read_to_string("./explorer.html").unwrap_or("<html>
            <head>
                <style>
                    @import url('https://fonts.googleapis.com/css2?family=Ubuntu:ital,wght@0,300;0,400;0,500;1,300;1,400;1,500;1,700&display=swap');
                    body {
                        display: flex;
                        flex-direction: column;
                        background-color: #251D3A;
                        font-family: Ubuntu, sans-serif;
                        margin: 0;
                    }

                    a {
                        text-decoration: none;
                    }

                    .dirEntry {
                        aspect-ratio: 1 / 1;
                        display: flex;
                        flex-direction: column;
                        height: 80px;
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
                    }

                    .dirEntry:hover {
                        background-color: #EC994B;
                    }
                    
                    .dirEntry span {
                        width: 90px; 
                        height: 30px;
                        font-size: 15px;
                        text-align: center;
                        text-overflow: ellipsis;
                        white-space: nowrap;
                        overflow: hidden; 
                    }

                    .path {
                        font-size: 20px;
                        color: white;
                        margin: 5px;
                    }

                    .entries {
                        display: flex;
                        flex-direction: row;
                        width: 100%;
                        flex-wrap: wrap;
                        justify-content: center;
                    }

                    .license {
                        height: 10px;
                        color: #ffffff;
                        position: absolute;
                        right: 5px;
                        top: 5px;
                    }

                    .topBar {
                        display: flex;
                        flex-direction: row;
                        align-items: center;
                        padding: 5px;
                        height: 40px;
                        background-color: #2A2550;
                        box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
                        margin-bottom: 10px;
                    }

                    .back, .forward {
                        background-color: #3A3845;
                        margin: 0;
                        display: grid;
                        place-items: center;
                        width: 30px;
                        height: 30px;
                        border: none;
                        cursor: pointer;
                        transition: all 0.5s ease-in-out;
                    }

                    .back:hover, .forward:hover {
                        background-color: #EC994B;
                    }

                    .back {
                        border-radius: 10px 0 0 10px;
                    }

                    .forward {
                        border-radius: 0 10px 10px 0;
                    }

                    .pageInfo {
                        position: fixed;
                        width: 30%;
                        left: 35%;
                        height: 50%;
                        bottom: -100%;
                        background-color: rgb(58, 56, 69);
                        text-align: center;
                        color: white;
                        border-radius: 50px;
                        transition: all 0.5s ease-in-out;
                    }

                    .pageInfo a {
                        text-decoration: underline;
                        color: white;
                    }
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
                    {path}
                </span>
                <a id='info' class='license'>
                <svg xmlns='http://www.w3.org/2000/svg'  viewBox='0 0 48 48' width='40px' height='40px'><path fill='#2196f3' d='M44,24c0,11.045-8.955,20-20,20S4,35.045,4,24S12.955,4,24,4S44,12.955,44,24z'/><path fill='#fff' d='M22 22h4v11h-4V22zM26.5 16.5c0 1.379-1.121 2.5-2.5 2.5s-2.5-1.121-2.5-2.5S22.621 14 24 14 26.5 15.121 26.5 16.5z'/></svg>
                </a>
                </div>
                <div class='entries'>
                {content}
                </div>
                <div class='pageInfo'>
                    <h2>Info</h2>
                    Website powered by the <a href='https://github.com/BlackBirdTV/netxplorer'>NetXplorer</a><br>
                    Icons by <a href='https://icons8.com'>Icons8</a><br>
                    Ubuntu font used under <a href='https://ubuntu.com/legal/font-licence'>this License</a>
                </div>
                <script>
                    const pageInfo = document.querySelector('.pageInfo');
                    const info = document.querySelector('#info');

                    let infoShown = false;

                    info.addEventListener('click', () => {{
                        infoShown = !infoShown;
                        pageInfo.style.bottom = (infoShown ? '25%' : '-100%');
                        console.log((infoShown ? '25%' : '-100%'));
                        console.log(infoShown)
                    }})
                </script>
            </body>
        </html>".to_owned()).replace("{path}", match file_path.len() {
            5 => "/",
            _ => &file_path[dir.len()..]
        }).replace("{content}", &format!("{}{{content}}", &dirs)[..]).replace("{content}", &files);
    }
    else {
        code = "404 Not Found";
        contents = read_to_string("./src/404.html").unwrap_or("<!DOCTYPE html>
        <html lang='en'>
        <head>
            <meta charset='UTF-8'>
            <meta http-equiv='X-UA-Compatible' content='IE=edge'>
            <meta name='viewport' content='width=device-width, initial-scale=1.0'>
            <title>404</title>
            <style>
            @import url('https://fonts.googleapis.com/css2?family=Ubuntu:ital,wght@0,300;0,400;0,500;1,300;1,400;1,500;1,700&display=swap');
            
            body {
                color: white;
                background-color: #251D3A;
                font-family: Ubuntu, sans-serif;
                position: absolute;
                inset: 0;
                margin: 0;
            }
        
            .content {
                display: grid;
                place-items: center;
            }
        
            .content div {
                display: flex;
                flex-direction: column;
                align-items: center;
            }
        
            h1 {
                font-size: 60px;
            }
        
            span {
                font-size: 24px;
            }
        
            p {
                font-size: 18px;
            }
        
            .license {
                height: 10px;
                color: #ffffff;
                position: absolute;
                right: 5px;
                top: 5px;
            }
        
            .topBar {
                display: flex;
                flex-direction: row;
                align-items: center;
                padding: 5px;
                height: 40px;
                background-color: #2A2550;
                box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
                margin-bottom: 10px;
            }
        
            .back, .forward {
                background-color: #3A3845;
                margin: 0;
                display: grid;
                place-items: center;
                width: 30px;
                height: 30px;
                border: none;
                cursor: pointer;
                transition: all 0.5s ease-in-out;
            }
        
            .back:hover, .forward:hover {
                background-color: #EC994B;
            }
        
            .back {
                border-radius: 10px 0 0 10px;
            }
        
            .forward {
                border-radius: 0 10px 10px 0;
            }
        
            .pageInfo {
                position: fixed;
                width: 30%;
                left: 35%;
                height: 50%;
                bottom: -100%;
                background-color: rgb(58, 56, 69);
                text-align: center;
                color: white;
                border-radius: 50px;
                transition: all 0.5s ease-in-out;
            }
        
            .pageInfo a {
                text-decoration: underline;
                color: white;
            }
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
                <a id='info' class='license'>
                <svg xmlns='http://www.w3.org/2000/svg'  viewBox='0 0 48 48' width='40px' height='40px'><path fill='#2196f3' d='M44,24c0,11.045-8.955,20-20,20S4,35.045,4,24S12.955,4,24,4S44,12.955,44,24z'/><path fill='#fff' d='M22 22h4v11h-4V22zM26.5 16.5c0 1.379-1.121 2.5-2.5 2.5s-2.5-1.121-2.5-2.5S22.621 14 24 14 26.5 15.121 26.5 16.5z'/></svg>
                </a>
                </div>
            <div class='content'>
                <div>
                <h1>404</h1>
                <span>Hmm... Nothing found...</span>
                <p>The page you requested doesn't seem to exist on our servers. Maybe you made a typo?</p>
                </div>
            </div>
            <div class='pageInfo'>
                <h2>Info</h2>
                Website powered by the <a href='https://github.com/BlackBirdTV/netxplorer'>NetXplorer</a><br>
                Icons by <a href='https://icons8.com'>Icons8</a><br>
                Ubuntu font used under <a href='https://ubuntu.com/legal/font-licence'>this License</a>
            </div>
            <script>
                const pageInfo = document.querySelector('.pageInfo');
                const info = document.querySelector('#info');
        
                let infoShown = false;
        
                info.addEventListener('click', () => {{
                    infoShown = !infoShown;
                    pageInfo.style.bottom = (infoShown ? '25%' : '-100%');
                    console.log((infoShown ? '25%' : '-100%'));
                    console.log(infoShown)
                }})
            </script>
        </body>
        </html>".to_owned());
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
