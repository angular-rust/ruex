#![doc(html_logo_url = "https://dudochkin-victor.github.io/assets/ruex/logo.svg")]

#![warn(missing_docs)]

//! Design pattern framework on top of PureMVC. 
//! 
//! The PureMVC framework has a very narrow goal. That is to help you
//! separate your application’s coding interests into three discrete tiers:
//! [Model][2], [View][3] and [Controller][1].
//! 
//! This separation of interests, and the tightness and direction of the
//! couplings used to make them work together is of paramount
//! importance in the building of scalable and maintainable applications.
//! 
//! In this implementation of the classic MVC Design meta-pattern, these
//! three tiers of the application are governed by three Singletons (a class
//! where only one instance may be created) called simply [Model][2], [View][3]
//! and [Controller][1]. Together, they are referred to as the ‘Core actors’.
//! 
//! A fourth Singleton, the [Facade][4] simplifies development by providing a
//! single interface for communication with the Core actors.
//! 
//! [Read more..][foundation]
//! 
//! ![PureMVC Diagram](https://raw.githubusercontent.com/wiki/ohyo-io/wampire/images/pure-mvc.svg)
//! 
//! [1]: crate::prelude::Controller
//! [2]: crate::prelude::Model
//! [3]: crate::prelude::View
//! [4]: crate::prelude::Facade
//! 
pub mod foundation;

pub mod prelude;
