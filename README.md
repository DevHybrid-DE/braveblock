<p align="center">
    <a href="https://github.com/intsights/Braveblock">
        <img src="https://raw.githubusercontent.com/intsights/Braveblock/master/images/logo.png" alt="Logo">
    </a>
    <h3 align="center">
        A fast and easy adblockplus parser and matcher based on adblock-rust package
    </h3>
</p>


![license](https://img.shields.io/badge/MIT-License-blue)
![Python](https://img.shields.io/badge/Python-3.8%20%7C%203.9%20%7C%203.10%20%7C%203.11%20%7C%203.12%20%7C%203.13-blue)

![OS](https://img.shields.io/badge/OS-Mac%20%7C%20Linux%20%7C%20Windows-blue)
![Build](https://github.com/intsights/Braveblock/workflows/Build/badge.svg)
[![PyPi](https://img.shields.io/pypi/v/Braveblock.svg)](https://pypi.org/project/Braveblock/)

## Table of Contents

- [Table of Contents](#table-of-contents)
- [About The Project](#about-the-project)
  - [Fork](#fork)
  - [Built With](#built-with)
  - [Installation](#installation)
- [Usage](#usage)
- [License](#license)
- [Contact](#contact)


## About The Project

This library is a Python binding to the [adblock-rust](https://github.com/brave/adblock-rust) library that was written by Brave's browser team. 
The binding uses [pyo3](https://github.com/PyO3/pyo3) to interact with the rust package.

## Fork

This repository is a fork of the original https://github.com/intsights/Braveblock.  

It includes updated dependencies to support Python 3.12 and 3.13, 
as well as compatibility with the latest versions of `adblock-rust` and `pyo3` (built against `pyo3` version 0.25.1 as of April 2025).

### Built With

* [pyo3](https://github.com/PyO3/pyo3)
* [adblock-rust](https://github.com/brave/adblock-rust)


### Installation

I have not published this fork to PyPI. Instead, I maintain a local build for use in my own projects.

To use this library, you can clone this repository and run the provided `buildsetup.sh` script to set up a complete Rust/Python build environment:

```sh
git clone https://github.com/YOUR_USERNAME/braveblock
cd braveblock
chmod +x buildsetup.sh
./buildsetup.shi didn#t 
```


After the build is complete, the compiled wheel can be found in ./target/wheels/.
You can install it into your project environment using:

```
uv pip install ./target/wheels/braveblock-*.whl
```

This approach ensures compatibility with Python 3.12 and 3.13 and uses the latest adblock-rust and pyo3 libraries.


## Usage

The public Python interface remains unchanged. Only internal Rust method calls have been updated to match the current API of `adblock-rust`.

```python
import braveblock

# Initialize the engine loaded with a rules list
# One can download easylist and load its lines into the engine
adblocker = braveblock.Adblocker(
    rules=[
        "-advertisement-icon.",
        "-advertisement/script.",
    ]
)

# This function checks whether the specified URL should be blocked
adblocker.check_network_urls(
    url="http://example.com/-advertisement-icon.",
    source_url="http://example.com/helloworld",
    request_type="image",
)


## License

Distributed under the MIT License. See `LICENSE` for more information.


## Contact

Gal Ben David - gal@intsights.com

Project Link: [https://github.com/intsights/Braveblock](https://github.com/intsights/Braveblock)
