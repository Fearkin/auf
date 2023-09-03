<a name="readme-top"></a>



<!-- PROJECT LOGO -->




<!-- TABLE OF CONTENTS 
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>
-->


<!-- ABOUT THE PROJECT -->
## About The Project
# auf - A Universal dns-information Finder
Cross-platform console application for viewing DNS records for a domain, and more!

### Built With
* [![Rust][Rust]][Rust-url]

<!-- GETTING STARTED -->
### Installation
For now, go to [Releases](https://github.com/Fearkin/auf/releases/)


### Building from source
auf is written in Rust, so you'll need to grab a Rust installation in order to compile it. In general, auf tracks the latest stable release of the Rust compiler.

To build auf for Linux:
```
$ git clone https://github.com/Fearkin/auf
$ cargo build --release
$ ./target/release/auf --version
auf 0.1.0
```
For Windows (you might need to install mingw-w64 first):
```
$ rustup toolchain install stable-x86_64-pc-windows-gnu
$ rustup target add x86_64-pc-windows-gnu
$ git clone https://github.com/Fearkin/auf
$ cargo build --target x86_64-pc-windows-gnu --release
$ ./target/x86_64-pc-windows-gnu/release/auf --version
auf 0.1.0
```
<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

```
auf <DOMAIN> [RESOLVER]

Arguments:
  <DOMAIN>
          Domain name to lookup

  [RESOLVER]
          DNS-resolver to use (default is Quad9 - 9.9.9.9)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
<p align="right">(<a href="#readme-top">back to top</a>)</p>

Example:
```
auf google.com

Querying records...
Querying whois...
Found google.com. 227 IN A 192.178.50.46
Found google.com. 146 IN AAAA 2607:f8b0:4008:805::200e
Found google.com. 300 IN MX 10 smtp.google.com.
Found google.com. 7991 IN NS ns2.google.com.
Found google.com. 7991 IN NS ns4.google.com.
Found google.com. 7991 IN NS ns3.google.com.
Found google.com. 7991 IN NS ns1.google.com.
Found google.com. 3600 IN TXT docusign=1b0a6754-49b1-4db5-8540-d2c12664b289
Found google.com. 3600 IN TXT google-site-verification=TV9-DBe4R80X4v0M4U_bd_J9cpOJM0nikft0jAgjmsQ
Found google.com. 3600 IN TXT atlassian-domain-verification=5YjTmWmjI92ewqkx2oXmBaD60Td9zWon9r6eakvHX6B77zzkFQto8PQ9QsKnbf4I
Found google.com. 3600 IN TXT v=spf1 include:_spf.google.com ~all
Found google.com. 3600 IN TXT webexdomainverification.8YX6G=6e6922db-e3e6-4a36-904e-a805c28087fa
Found google.com. 3600 IN TXT facebook-domain-verification=22rm551cu4k0ab0bxsw536tlds4h95
Found google.com. 3600 IN TXT docusign=05958488-4752-4ef2-95eb-aa7ba8a3bd0e
Found google.com. 3600 IN TXT onetrust-domain-verification=de01ed21f2fa4d8781cbc3ffb89cf4ef
Found google.com. 3600 IN TXT globalsign-smime-dv=CDYX+XFHUw2wml6/Gb8+59BsH31KzUr6c1l2BPvqKX8=
Found google.com. 3600 IN TXT google-site-verification=wD8N7i1JTNTkezJ49swvWW48f8_9xveREV4oB-0Hf5o
Found google.com. 3600 IN TXT MS=E4A68B9AB2BB9670BCE15412F62916164C0B20BB
Found google.com. 3600 IN TXT apple-domain-verification=30afIBcvSuDV2PLX
Found google.com. 43200 IN CAA 0 issue "pki.goog"
Found 46.50.178.192.in-addr.arpa. 2317 IN PTR lcmiaa-aa-in-f14.1e100.net.
Found google.com. 53 IN SOA ns1.google.com. dns-admin.google.com. 561875416 900 900 1800 60

WHOIS Info:
   Domain Name: GOOGLE.COM
   Registry Domain ID: 2138514_DOMAIN_COM-VRSN
   Registrar WHOIS Server: whois.markmonitor.com
   Registrar URL: http://www.markmonitor.com
```
and so on.


<!-- ROADMAP -->
## Roadmap
# WIP

See the [Open Issues](https://github.com/Fearkin/auf/issues) for a full list of proposed features (and known issues).




<!-- CONTRIBUTING -->

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the BSD-2-Clause license. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Fearkin - fearjin1@gmail.com

Project Link: [https://github.com/Fearkin/auf](https://github.com/Fearkin/auf)

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/Fearkin/auf.svg?style=for-the-badge
[contributors-url]: https://github.com/Fearkin/auf/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/Fearkin/auf.svg?style=for-the-badge
[forks-url]: https://github.com/Fearkin/auf/network/members
[stars-shield]: https://img.shields.io/github/stars/Fearkin/auf.svg?style=for-the-badge
[stars-url]: https://github.com/Fearkin/auf/stargazers
[issues-shield]: https://img.shields.io/github/issues/Fearkin/auf.svg?style=for-the-badge
[issues-url]: https://github.com/Fearkin/auf/issues
[product-screenshot]: images/screenshot.png
[Next.js]: https://img.shields.io/badge/next.js-000000?style=for-the-badge&logo=nextdotjs&logoColor=white
[Rust]: https://img.shields.io/badge/rust-8B4513?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
