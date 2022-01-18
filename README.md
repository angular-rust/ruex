<div align="center">

[![](https://dudochkin-victor.github.io/assets/ruex/logo-wide.svg)](#top)
# Ruex

[![API Docs][docrs-badge]][docrs-url]
[![Crates.io][crates-badge]][crates-url]
[![Code coverage][codecov-badge]][codecov-url]
[![Tests][tests-badge]][tests-url]
[![MPL-2.0 licensed][license-badge]][license-url]
[![Gitter chat][gitter-badge]][gitter-url]
[![loc][loc-badge]][loc-url]
</div>

[docrs-badge]: https://img.shields.io/docsrs/ruex?style=flat-square
[docrs-url]: https://docs.rs/ruex/
[crates-badge]: https://img.shields.io/crates/v/ruex.svg?style=flat-square
[crates-url]: https://crates.io/crates/ruex
[license-badge]: https://img.shields.io/badge/license-MPL--2.0-blue.svg?style=flat-square
[license-url]: https://github.com/angular-rust/ruex/blob/master/LICENSE
[gitter-badge]: https://img.shields.io/gitter/room/angular_rust/community.svg?style=flat-square
[gitter-url]: https://gitter.im/angular_rust/community
[tests-badge]: https://img.shields.io/github/workflow/status/angular-rust/ruex/Tests?label=tests&logo=github&style=flat-square
[tests-url]: https://github.com/angular-rust/ruex/actions/workflows/tests.yml
[codecov-badge]: https://img.shields.io/codecov/c/github/angular-rust/ruex?logo=codecov&style=flat-square&token=L7KV27OLY0
[codecov-url]: https://codecov.io/gh/angular-rust/ruex
[loc-badge]: https://img.shields.io/tokei/lines/github/angular-rust/ruex?style=flat-square
[loc-url]: https://github.com/angular-rust/ruex

Ruex is a Centralized State Management And Design Patterns for Rust.

## Pattern List

Template name                                                     |    Type    | Links 
:-----------------------------------------------------------------|:----------:|:------------
[Builder](src/foundation/patterns/builder.rs)                     | Creational | [Refactoring.Guru](https://refactoring.guru/design-patterns/builder) |
[Command](src/foundation/patterns/command/simple_command.rs)      | Behavioral | [Refactoring.Guru](https://refactoring.guru/design-patterns/command) |
[Facade](src/foundation/patterns/facade.rs)                       | Structural | [Refactoring.Guru](https://refactoring.guru/design-patterns/facade) |
[Mediator](src/foundation/patterns/mediator/mediator.rs)          | Behavioral | [Refactoring.Guru](https://refactoring.guru/design-patterns/mediator) |
[Proxy](src/foundation/patterns/proxy/proxy.rs)                   | Structural | [Refactoring.Guru](https://refactoring.guru/design-patterns/proxy) |
[Observer](src/foundation/patterns/observer/observer.rs)          | Behavioral | [Refactoring.Guru](https://refactoring.guru/design-patterns/observer) |
[Singleton](src/prelude/singleton.rs)                             | Creational | [Refactoring.Guru](https://refactoring.guru/design-patterns/singleton) |

## Quick Start

Install Ruex:

	cargo add ruex

## Learn More

* [Manual, Docs, etc](https://angular-rust.github.io/)
* [Samples](https://github.com/angular-rust/ux-samples)
* [Apps using Angular Rust](https://github.com/angular-rust/ruex/wiki/Apps-in-the-Wild)
* [Articles Featuring Angular Rust](https://github.com/angular-rust/ruex/wiki/Articles)
* [The Catalog of Design Patterns](https://refactoring.guru/design-patterns/catalog)
* [Design patterns card](http://www.mcdonaldland.info/files/designpatterns/designpatternscard.pdf)

## Community

 [![](https://img.shields.io/badge/Facebook-1877F2?style=for-the-badge&logo=facebook&logoColor=white)](https://www.facebook.com/groups/angular.rust) 
 [![](https://img.shields.io/badge/Stack_Overflow-FE7A16?style=for-the-badge&logo=stack-overflow&logoColor=white)](https://stackoverflow.com/questions/tagged/angular-rust) 
 [![](https://img.shields.io/badge/YouTube-FF0000?style=for-the-badge&logo=youtube&logoColor=white)](https://www.youtube.com/channel/UCBJTkSl_JWShuolUy4JksTQ) 
 [![](https://img.shields.io/badge/Medium-12100E?style=for-the-badge&logo=medium&logoColor=white)](https://medium.com/@angular.rust) 
 [![](https://img.shields.io/gitter/room/angular_rust/angular_rust?style=for-the-badge)](https://gitter.im/angular_rust/community)


## Contributing

We believe the wider community can create better code. The first tool for improving the community is to tell the developers about the project by giving it a star. More stars - more members.

 ![Star a repo](https://dudochkin-victor.github.io/assets/star-me-wide.svg)
 
Angular Rust is a community effort and we welcome all kinds of contributions, big or small, from developers of all backgrounds. We want the Angular Rust community to be a fun and friendly place, so please review our [Code of Conduct](CODE_OF_CONDUCT.md) to learn what behavior will not be tolerated.

### New to Angular Rust?

Start learning about the framework by helping us improve our [documentation](https://angular-rust.github.io/). Pull requests which improve test coverage are also very welcome.

### Looking for inspiration?

Check out the community curated list of awesome things related to Angular Rust / WebAssembly at [awesome-angular-rust](https://github.com/angular-rust/awesome-angular-rust).

### Confused about something?

Feel free to drop into our [Gitter chatroom](https://gitter.im/angular_rust/community) or open a [new "Question" issue](https://github.com/angular-rust/ruex/issues/new/choose) to get help from contributors. Often questions lead to improvements to the ergonomics of the framework, better documentation, and even new features!

### Ready to dive into the code?

After reviewing the [Contributing Code Guidelines](CONTRIBUTING.md), check out the ["Good First Issues"](https://github.com/angular-rust/ruex/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) (they are eager for attention!). Once you find one that interests you, feel free to assign yourself to an issue and don't hesitate to reach out for guidance, the issues vary in complexity.

### Let's help each other!

Come help us on the [issues that matter that the most](https://github.com/angular-rust/ruex/labels/%3Adollar%3A%20Funded%20on%20Issuehunt) and receive a small cash reward for your troubles. We use [Issuehunt](https://issuehunt.io/r/angular-rust/ruex/) to fund issues from our Open Collective funds. If you really care about an issue, you can choose to add funds yourself! 

### Found a bug?

Please [report all bugs!](https://github.com/angular-rust/ruex/issues/new/choose) We are happy to help support developers fix the bugs they find if they are interested and have the time.

## Todo

Template name           |    Type    | Links 
:-----------------------|:----------:|:------
Abstract Factory        | Creational | [Refactoring.Guru](https://refactoring.guru/design-patterns/abstract-factory) |
Adapter                 | Structural | [Refactoring.Guru](https://refactoring.guru/design-patterns/adapter) |
Bridge                  | Structural | [Refactoring.Guru](https://refactoring.guru/design-patterns/bridge) |
Chain of Responsibility | Behavioral | [Refactoring.Guru](https://refactoring.guru/design-patterns/chain-of-responsibility) |
Composite               | Structural | [Refactoring.Guru](https://refactoring.guru/design-patterns/composite) |
Decorator               | Structural | [Refactoring.Guru](https://refactoring.guru/design-patterns/decorator) |
Factory Method          | Creational | [Refactoring.Guru](https://refactoring.guru/design-patterns/factory-method) |
Flyweight               | Structural | [Refactoring.Guru](https://refactoring.guru/design-patterns/flyweight) |
Interpreter             | Behavioral | [Wiki](https://en.wikipedia.org/wiki/Interpreter_pattern) |
Iterator                | Behavioral | [Refactoring.Guru](https://refactoring.guru/design-patterns/iterator) |
Memento                 | Behavioral | [Refactoring.Guru](https://refactoring.guru/design-patterns/memento) |
Prototype               | Creational | [Refactoring.Guru](https://refactoring.guru/design-patterns/prototype) |
State                   | Behavioral | [Refactoring.Guru](https://refactoring.guru/design-patterns/state) |
Strategy                | Behavioral | [Refactoring.Guru](https://refactoring.guru/design-patterns/strategy) |
Template Method         | Behavioral | [Refactoring.Guru](https://refactoring.guru/design-patterns/template-method) |
Visitor                 | Behavioral | [Refactoring.Guru](https://refactoring.guru/design-patterns/visitor) |