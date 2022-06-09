# NetXplorer
NetXplorer is a HTTP-Server written in Rust that allows you to easily browse your servers files in a modern UI Interface.

## Usage
First, download the latest release. After that, create a folder called `src` in your server executables directory. This is how NetXplorer treats requests: If a file gets requested, the server will return the file or 404 if it doesn't exist. If the request contains a directory, the server will look for an `index.html` file and return it. If no index.html is found, the server will return a html page containing the Explorer UI. If you don't want people accessing a directory, you can simply create a index.html file in it.
In order to allow users to create and delete files on your server, create a file called `.admins` in the directory of your server executable and put the IP adresses of the administrated users into it, each IP taking up it's own line.
If everything went right, a blue button with a plus should appear in the bottom right corner of the admin's explorer. Clicking it will pull up the file creation Menu. If you right click on a file while having administrator privileges, the admin context menu will appear, in which you can delete the file.

## Customisation
NetXplorer allows you to easily customize every UI Element.<br>
All the error pages (`404 Not Found`, `403 Forbidden`) can be replaced with a custom error page by creating an according html file (`404.html`, `403.html`) in the `src` directory.<br>
To replace the explorer UI, create a file called `explorer.html` in the `src` directory. In it, you can specify two areas: one where you want the files and directories, symbolized by `{content}` and one, where the path will be shown, symbolized by `{path}`.<br>
Replacing the buttons for directories and files is just as easy: simply create a `dir.html` for the directory button and `file.html` for the file button. Now, instead of making a whole html file, just place one element inside. E.g.:
```
<a href="{url}">
{name}
</a>
```
As you just saw, you have to specify two attributes: name of the file/directory and url of the file/directory.<br>
Thats all! It's just as easy! Convenient, right? Of course, the built-in UI is perfectly usable, but customisation is power.
