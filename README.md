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

Design pattern framework on top of PureMVC. 

The PureMVC framework has a very narrow goal. That is to help you
separate your application’s coding interests into three discrete tiers:
[Model][2], [View][3] and [Controller][1].

This separation of interests, and the tightness and direction of the
couplings used to make them work together is of paramount
importance in the building of scalable and maintainable applications.

In this implementation of the classic MVC Design meta-pattern, these
three tiers of the application are governed by three Singletons (a class
where only one instance may be created) called simply [Model][2], [View][3]
and [Controller][1]. Together, they are referred to as the ‘Core actors’.

A fourth Singleton, the [Facade][4] simplifies development by providing a
single interface for communication with the Core actors.

## Model & Proxies

The [Model][2] simply caches named references to Proxies. Proxy code
manipulates the data model, communicating with remote services if
need be to persist or retrieve it.

This results in portable Model tier code.

## View & Mediators

The View primarily caches named references to [Mediators][7]. [Mediator][7]
code stewards View Components, adding event listeners, sending
and receiving notifications to and from the rest of the system on
their behalf and directly manipulating their state.

This separates the View definition from the logic that controls it.

## Controller & Commands

The [Controller][1] maintains named mappings to Command classes,
which are stateless, and only created when needed.

[Commands][9] may retrieve and interact with Proxies, send
Notifications, execute other [Commands][9], and are often used to
orchestrate complex or system-wide activities such as application
startup and shutdown. They are the home of your application’s
Business Logic.

## Facade & Core

The [Facade][4], another Singleton, initializes the Core actors ([Model][2],
[View][3] and [Controller][1]), and provides a single place to access all of
their public methods.

By extending the [Facade][4], your application gets all the benefits of
Core actors without having to import and work with them directly.
You will implement a concrete [Facade][4] for your application only once
and it is simply done.

[Proxies][6], [Mediators][7] and [Commands][9] may then use your application’s
concrete [Facade][4] in order to access and communicate with each
other.

## Observers & Notifications

PureMVC applications may run in environments without access to
Event and EventDispatcher classes, so the framework
implements an [Observer][8] notification scheme for communication
between the Core MVC actors and other parts of the system in a
loosely-coupled way.

You need not be concerned about the details of the PureMVC
[Observer][8]/[Notification][5] implementation; it is internal to the
framework. You will use a simple method to send [Notifications][5] from
[Proxies][6], [Mediators][7], [Commands][9] and the Facade itself that doesn’t
even require you to create a [Notification][5] instance.

[1]: https://docs.rs/ruex/latest/ruex/prelude/trait.Controller.html
[2]: https://docs.rs/ruex/latest/ruex/prelude/trait.Model.html
[3]: https://docs.rs/ruex/latest/ruex/prelude/trait.View.html
[4]: https://docs.rs/ruex/latest/ruex/prelude/trait.Facade.html
[5]: https://docs.rs/ruex/latest/ruex/prelude/trait.Notification.html
[6]: https://docs.rs/ruex/latest/ruex/prelude/trait.Proxy.html
[7]: https://docs.rs/ruex/latest/ruex/prelude/trait.Mediator.html
[8]: https://docs.rs/ruex/latest/ruex/prelude/trait.Observer.html
[9]: https://docs.rs/ruex/latest/ruex/prelude/trait.Command.html


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

