// use serde::{Deserialize, Serialize};
// use std::collections::HashSet;
// use yew::worker::*;

// #[derive(Serialize, Deserialize, Debug)]
// pub enum Request {
//     EventBusMsg(String),
// }

// pub struct EventBus {
//     link: AgentLink<EventBus>,
//     subscribers: HashSet<HandlerId>,
// }

// impl Agent for EventBus {
//     type Reach = Context<Self>;
//     type Message = ();
//     type Input = Request;
//     type Output = String;

//     fn create(link: AgentLink<Self>) -> Self {
//         Self {
//             link,
//             subscribers: HashSet::new(),
//         }
//     }

//     fn update(&mut self, _msg: Self::Message) {}

//     fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
//         match msg {
//             Request::EventBusMsg(s) => {
//                 for sub in self.subscribers.iter() {
//                     self.link.respond(*sub, s.clone());
//                 }
//             }
//         }
//     }

//     fn connected(&mut self, id: HandlerId) {
//         self.subscribers.insert(id);
//     }

//     fn disconnected(&mut self, id: HandlerId) {
//         self.subscribers.remove(&id);
//     }
// }


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