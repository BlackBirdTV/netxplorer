# NetXplorer
NetXplorer is a HTTP-Server written in Rust that allows you to easily browse your servers files in a modern UI Interface.

## Usage
First, download the latest release. After that, create a folder called `src` in your server executables directory. In this folder, create a file called `404.html`, otherwise, the server will crash when the user tries to get a file / folder that doesn't exist. This is how NetXplorer treats requests: If a file gets requested, the server will return the file or 404 if it doesn't exist. If the request contains a directory, the server will look for an `index.html` file and return it. If no index.html is found, the server will return a html page containing the Explorer UI.