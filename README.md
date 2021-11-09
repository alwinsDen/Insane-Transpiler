# Insane-Compiler

A Rust-based .ic (custom format) to .js code convertor for easier web development

## Installation

Use the Rust's cargo to install Insane-Compiler.

```bash
sudo apt-install rustc
git clone https://github.com/alwinsDen/insane-compiler.git
cd insane-compiler
cargo build
cargo run create sample_project
```

## Usage

```rust
//sample_project/src/index.ic

//define the root element

#root

h1 >> "This is the first tag" || "headertag"
click () "headertag" => alert => "You are at the header."  <-() LOGGER
click () "headertag" => alert => "This is the second alert" <-() LOGGER
h2    >> "This is the first tag" || "header"
h3    >> "This is the second tag" || "buttonColor"
p     >> "This compiler is pretty amazing"
//style >> custom "buttonColor" = color : red -> fontSize : 2rem -> background : yellow -> color : red
//style >> custom "header" = color : red -> background : green -> textDecoration : underline -> cursor :  pointer
mouseenter () "header" => alert => "This will pop up." <-() LOGGER
mouseleave () "header" => alert => "This mouse is leaving." <-() LOGGER
//this line will be ignored
button >> "This is button text" || "clickedButton"
click () "clickedButton" => alert => "Yeah. the button now works" <-() LOGGER
style >> default
style >> custom "headertag" = color : red -> backgroundColor : black -> cursor : pointer
style >> custom "clickedButton" = padding : .5rem -> backgroundColor : skyblue -> color : red -> fontSize : 1rem -> cursor : pointer
```

## Building
```bash
cargo run compile prod sample_project 
```

## Output
The compiler will process the data into a build.js file.
```javascript
//compiled with Insane-Compiler by github@alwinsDen;
let root = document.getElementById('root');
let writeElement2 = document.createElement('h1');
writeElement2.id = "headertag";
writeElement2.appendChild(document.createTextNode("This is the first tag"));
root.appendChild(writeElement2);
document.getElementById("headertag").addEventListener('click', () => window.alert("You are at the header."));
document.getElementById("headertag").removeEventListener('click', () => window.alert("You are at the header."));
document.getElementById("headertag").addEventListener('click', () => console.log("headertag" + ' was clicked'));;
document.getElementById("headertag").addEventListener('click', () => window.alert("This is the second alert"));
document.getElementById("headertag").removeEventListener('click', () => window.alert("This is the second alert"));
document.getElementById("headertag").addEventListener('click', () => console.log("headertag" + ' was clicked'));;
let writeElement5 = document.createElement('h2');
writeElement5.id = "header";
writeElement5.appendChild(document.createTextNode("This is the first tag"));
root.appendChild(writeElement5);
let writeElement6 = document.createElement('h3');
writeElement6.id = "buttonColor";
writeElement6.appendChild(document.createTextNode("This is the second tag"));
root.appendChild(writeElement6);
let writeElement7 = document.createElement('p');
writeElement7.appendChild(document.createTextNode("This compiler is pretty amazing"));
root.appendChild(writeElement7);
document.getElementById("header").addEventListener('mouseenter', () => window.alert("This will pop up."));
document.getElementById("header").removeEventListener('mouseenter', () => window.alert("This will pop up."));
document.getElementById("header").addEventListener('mouseenter', () => console.log("header" + ' was clicked'));;
document.getElementById("header").addEventListener('mouseleave', () => window.alert("This mouse is leaving."));
document.getElementById("header").removeEventListener('mouseleave', () => window.alert("This mouse is leaving."));
document.getElementById("header").addEventListener('mouseleave', () => console.log("header" + ' was clicked'));;
let inputElement10 = document.createElement('button');
inputElement10.id = "clickedButton";
inputElement10.appendChild(document.createTextNode("This is button text"));
root.appendChild(inputElement10);
document.getElementById("clickedButton").addEventListener('click', () => window.alert("Yeah. the button now works"));
document.getElementById("clickedButton").removeEventListener('click', () => window.alert("Yeah. the button now works"));
document.getElementById("clickedButton").addEventListener('click', () => console.log("clickedButton" + ' was clicked'));;
let allElements = document.querySelectorAll('*');
for (let i = 0; i < allElements.length; i++) {
    allElements[i].style.margin = '0';
    allElements[i].style.padding = '0';
    allElements[i].style.boxSizing = 'border-box'
};
document.getElementById("headertag").style.color = 'red';
document.getElementById("headertag").style.backgroundColor = 'black';
document.getElementById("headertag").style.cursor = 'pointer';
document.getElementById("clickedButton").style.padding = '.5rem';
document.getElementById("clickedButton").style.backgroundColor = 'skyblue';
document.getElementById("clickedButton").style.color = 'red';
document.getElementById("clickedButton").style.fontSize = '1rem';
document.getElementById("clickedButton").style.cursor = 'pointer';
```
##TAGS
<li>root -> root element of the html file</li>

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
