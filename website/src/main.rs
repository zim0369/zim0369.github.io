use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    stream: String,
    index: usize,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            stream: String::from(" zim welcomes you"),
            index: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.index += 1;

                self.stream = format!(
                    "{}{}{}",
                    &self.stream[..self.index],
                    &self.stream[self.index..self.index + 1].to_uppercase(),
                    &self.stream[self.index + 1..]
                );
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ format!(  "ping {}", &self.index ) }</button>
                <p>{&self.stream }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
