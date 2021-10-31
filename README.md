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
h1    >> "This is the first tag" || "header"
h1    >> "This is the second tag" || "buttonColor"
p     >> "This compiler is pretty amazing"
style >> custom "buttonColor" = color : red -> fontSize : 2rem -> background : yellow -> color : red
style >> custom "header" = color : red -> background : green -> textDecoration : underline -> cursor :  pointer
mouseenter () "header" => alert => "This will pop up." <-()
mouseleave () "header" => alert => "This mouse is leaving." <-()
```

## Building
```bash
cargo run compile prod forms_creator 
```

## Output
The compiler will process the data into a build.js file.
```javascript
//compiled with Insane-Compiler by github@alwinsDen;
let root = document.getElementById("root");
let writeElement1 = document.createElement("h1");
writeElement1.id = "header";
writeElement1.appendChild(document.createTextNode("This is the first tag"));
root.appendChild(writeElement1);
let writeElement2 = document.createElement("h1");
writeElement2.id = "buttonColor";
writeElement2.appendChild(document.createTextNode("This is the second tag"));
root.appendChild(writeElement2);
let writeElement3 = document.createElement("p");
writeElement3.appendChild(
    document.createTextNode("This compiler is pretty amazing")
);
root.appendChild(writeElement3);
document.getElementById("buttonColor").style.color = "red";
document.getElementById("buttonColor").style.fontSize = "2rem";
document.getElementById("buttonColor").style.background = "yellow";
document.getElementById("buttonColor").style.color = "red";
document.getElementById("header").style.color = "red";
document.getElementById("header").style.background = "green";
document.getElementById("header").style.textDecoration = "underline";
document.getElementById("header").style.cursor = "pointer";
document
    .getElementById("header")
    .addEventListener("mouseenter", () => window.alert("This will pop up."));
document
    .getElementById("header")
    .removeEventListener("mouseenter", () => window.alert("This will pop up."));
document
    .getElementById("header")
    .addEventListener("mouseleave", () => window.alert("This mouse is leaving."));
document
    .getElementById("header")
    .removeEventListener("mouseleave", () =>
        window.alert("This mouse is leaving.")
    );

```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)