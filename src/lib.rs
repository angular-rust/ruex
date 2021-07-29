#![allow(unused_imports)]
#![allow(unused_variables)]

#![doc(html_logo_url = "https://dudochkin-victor.github.io/assets/ruex/logo.svg")]

mod core;
mod event_bus;
mod interfaces;
mod patterns;
mod producer;
mod subscriber;

// use producer::Producer;
// use subscriber::Subscriber;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// pub struct Model;

// impl Component for Model {
//     type Message = ();
//     type Properties = ();

//     fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
//         Self
//     }

//     fn change(&mut self, _msg: Self::Properties) -> ShouldRender {
//         false
//     }

//     fn update(&mut self, _props: Self::Message) -> ShouldRender {
//         unimplemented!()
//     }

//     fn view(&self) -> Html {
//         html! {
//             <>
//                 <Producer />
//                 <Subscriber />
//             </>
//         }
//     }
// }

pub mod prelude {
    pub use super::patterns::builder::*;
}
