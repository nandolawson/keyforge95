# keyforge95

![downloads](https://img.shields.io/crates/d/keyforge95?style=flat&logo=rust&logoColor=ffffff&label=downloads&color=ffc933) ![version](https://img.shields.io/crates/v/keyforge95?style=flat?&logo=rust&logoColor=fffffflabel=version&color=ffc933) ![build status](https://img.shields.io/github/actions/workflow/status/nandolawson/keyforge95/test.yml?style=flat&logo=githubactions&logoColor=ffffff&label=build)

This library is a cross-platform generator and validator for Windows 95 keys written in Rust. As it turned out, however, it supposedly works for all Microsoft products that require a 10-digit product key in the following format: _`XXX-XXXXXXX`_

It is only sporadically developed as it is solely a learning project for me and does not have particularly significant practical use.

⚠️ **Note**: keyforge95 and I are in no way associated with Microsoft or their products. Only [publicly available information](https://en.wikipedia.org/wiki/Product_key#Examples) from the internet has been used to create this project. It neither bypasses effective copy measures nor constitutes a "crack." The purpose of this repository, from my perspective, is solely to gain experience in programming and enhance my skills and knowledge.

## Compatible Software

- `Access ADI 95`
- `Hell Bender`
- `Office 7.0b`
- `Office Professional 95`
- `Plus! 95`
- `Plus! 98`
- `Return to Arcade`
- `Windows 95`
- `Windows CE Toolkit for Visual Basic 5`
- `Windows CE Toolkit for Visual C++ 5`
- `Windows NT 4.0 Server`
- `Windows NT 4.0 Workstation`
- `Visual Basic Standard 4.0`
- `Visual SourceSafe 4.0`

## Usage

This library only has two public functions: _`generate_product_key()`_ and _`validate_product_key()`_

### Generate

After adding keyforge95 to your project, just use _`generate_product_key("retail" / "oem")`_ to generate a valid product key as a String.

```rs
use keyforge95::generate_product_key;
let product_key: String = generate_product_key("oem");
println("Generated product key: {}", product_key);
```

### Validate

To check the validity of a key, add keyforge95 to your project and use _`validate_product_key("key")`_. This function returns a bool. It is important that the right formatting (_`XXX-XXXXXXX`_) is used for the product key. Otherwise, the validation will fail.

```rs
use keyforge95::validate_product_key;
let product_key: &str = "000-0000000"
match validate_product_key(product_key) {
    true => println!("Valid key: {}", product_key),
    false => println!("Invalid key: {}", product_key)
}
```

## Contributing

Anyone who wants to contribute is more than welcome to do so. I would be delighted to learn from the contributions of other users. If you find a bug or have a feature in mind that you think would be useful, please feel free to create a pull request on GitHub.
If you decide to fork this project, please make sure to adhere to the [license](https://github.com/nandolawson/keyforge95/blob/master/LICENSE). Your involvement and feedback are highly appreciated!
