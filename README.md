<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
<!-- [![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url] -->
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]


<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/xerxesar/remind">
    <img src="images/remind-logo.png" alt="Logo" width="128">
  </a>

<h3 align="center">Remind</h3><p>Sticky notes app</p>
  <p align="center">
<!--     <br /> -->
<!--     <a href="https://github.com/xerxesar/remind"><strong>Explore the docs »</strong></a> -->
<!--     <br /> -->
    <br />
<!--     <a href="https://github.com/xerxesar/remind">View Demo</a>
    · -->
    <a href="https://github.com/xerxesar/remind/issues">Report Bug / Request Feature</a>
  </p>
</div>



<!-- ABOUT THE PROJECT -->
## About remind

Remind is a sticky notes application which shows notes by pressing a shortcut key.
by now remind can show markdown files as sticky notes on the screen, but it can become much more.


### Built With
[![image][Rust-logo-url]][Rust-url]


<!-- GETTING STARTED -->
## Getting Started

Currently remind has no standard packaging. use bash scripts in `./scripts` dir to build, install, and setup remind as systemd service.

Also don't forget to set your desired hotkey to run `remind-hc` (located at `/usr/local/bin/remind-hc` by default).
> `remind-hc` executable sends a message to the remind service to toggle showing notes.
### Prerequisites

To build remind from source code, you need to install rust-lang. 
"Remind has been tested on bunsenlabs linux (a debian derived distribution, using X11 and openbox) only."

### Installation

1. Run `./scripts/build` to build the app by source (running `cargo install` may be needed before)
2. Run `./scripts/install` to copy binaries to `/usr/local/bin/` (needs to be run by sudo).
3. Run `./scripts/systemd-install` to create and start systemd service for remind(the service runs in user mode and needs no sudo command).
4. Set your desired hotkey to run `remind-hc` (located at `/usr/local/bin/remind-hc` by default).
>`remind-hc` executable sends a message to the remind service to toggle showing notes.



<!-- USAGE EXAMPLES -->
## Usage

Press your `HOTKEY` (I set `meta+n` for my self) to toggle on screen notes.
Drag any note window to change its position.
Hold `Alt` and drag any note window to resize it.


<!-- ROADMAP -->
## Roadmap

- Notes editor and manager
- Cross-platform support
  - Linux
  - Window
  - macOS
- Save notes to cloud
- Mobile support
  - Android
  - iOS

<!-- See the [open issues](https://github.com/xerxesar/remind/issues) for a full list of proposed features (and known issues). -->


<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request


<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.


<!-- CONTACT -->
## Contact

Khashayar Aryanmehr - khashayar.aryanmehr@gmail.com

[![LinkedIn][linkedin-shield]][linkedin-url]


Project Link: [https://github.com/xerxesar/remind](https://github.com/xerxesar/remind)





<!-- ACKNOWLEDGMENTS -->
<!-- ## Acknowledgments

* []()
* []()
* []()
-->




<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/xerxesar/remind.svg?style=for-the-badge
[contributors-url]: https://github.com/xerxesar/remind/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/xerxesar/remind.svg?style=for-the-badge
[forks-url]: https://github.com/xerxesar/remind/network/members
[stars-shield]: https://img.shields.io/github/stars/xerxesar/remind.svg?style=for-the-badge
[stars-url]: https://github.com/xerxesar/remind/stargazers
[issues-shield]: https://img.shields.io/github/issues/xerxesar/remind.svg?style=for-the-badge
[issues-url]: https://github.com/xerxesar/remind/issues
[license-shield]: https://img.shields.io/github/license/xerxesar/remind.svg?style=for-the-badge
[license-url]: https://github.com/xerxesar/remind/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/xerxesar
[product-screenshot]: images/screenshot.png
[Rust-url]: https://rust-lang.org/
[Rust-logo-url]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
