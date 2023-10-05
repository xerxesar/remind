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
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/xerxesar/remind">
    <img src="images/remind-logo.png" alt="Logo" width="240">
  </a>

<h3 align="center">Remind</h3><p>Helps you organize</p>
  <p align="center">
<!--     <br /> -->
<!--     <a href="https://github.com/xerxesar/remind"><strong>Explore the docs »</strong></a> -->
<!--     <br /> -->
    <br />
<!--     <a href="https://github.com/xerxesar/remind">View Demo</a>
    · -->
    <a href="https://github.com/xerxesar/remind/issues">Report Bug</a>
    ·
    <a href="https://github.com/xerxesar/remind/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With Rust</a></li>
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



<!-- ABOUT THE PROJECT -->
## About The Project

Remind is a sticky notes application (currently on linux) showing sticky notes by pressing a shortcut key (instead of switching to desktop).




### Built With

[![image][Rust-logo-url]][Rust-url]





<!-- GETTING STARTED -->
## Getting Started

Currently remind has no standard packaging. use bash scripts in `./scripts` dir to build, install, and setup remind as systemd service.

Also don't forget to set your desired hotkey on remind-hc (located at `/usr/local/bin/remind-hc` by default).

### Prerequisites

Remind has been tested on Bunsenlabs (Debian derived using X11 and openbox) only.
It will be tested and released for windows and macos also.

### Installation

1. Run `./scripts/build` to build the app by source.
2. Run `./scripts/install` to copy binaries to `/usr/local/bin/` (needs to be run by sudo).
3. Run `./scripts/systemd-install` to create and start systemd service for remind.





<!-- USAGE EXAMPLES -->
## Usage

Press your `HOTKEY` (I set `meta+n` for my self) to toggle on screen notes.
Drag any note window to change its position.
Hold `Alt` and drag any note window to resize it.

_For more examples, please refer to the [Documentation](https://example.com)_





<!-- ROADMAP -->
## Roadmap

- [ ] Feature 1
- [ ] Feature 2
- [ ] Feature 3
    - [ ] Nested Feature

See the [open issues](https://github.com/xerxesar/remind/issues) for a full list of proposed features (and known issues).





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

Your Name - [@twitter_handle](https://twitter.com/twitter_handle) - khashayar.aryanmehr@gmail.com

Project Link: [https://github.com/xerxesar/remind](https://github.com/xerxesar/remind)





<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* []()
* []()
* []()





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
[Next.js]: https://img.shields.io/badge/next.js-000000?style=for-the-badge&logo=nextdotjs&logoColor=white
[Next-url]: https://nextjs.org/
[React.js]: https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB
[React-url]: https://reactjs.org/
[Rust-url]: https://rust-lang.org/
[Rust-logo-url]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Vue.js]: https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vuedotjs&logoColor=4FC08D
[Vue-url]: https://vuejs.org/
[Angular.io]: https://img.shields.io/badge/Angular-DD0031?style=for-the-badge&logo=angular&logoColor=white
[Angular-url]: https://angular.io/
[Svelte.dev]: https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00
[Svelte-url]: https://svelte.dev/
[Laravel.com]: https://img.shields.io/badge/Laravel-FF2D20?style=for-the-badge&logo=laravel&logoColor=white
[Laravel-url]: https://laravel.com
[Bootstrap.com]: https://img.shields.io/badge/Bootstrap-563D7C?style=for-the-badge&logo=bootstrap&logoColor=white
[Bootstrap-url]: https://getbootstrap.com
[JQuery.com]: https://img.shields.io/badge/jQuery-0769AD?style=for-the-badge&logo=jquery&logoColor=white
[JQuery-url]: https://jquery.com 
