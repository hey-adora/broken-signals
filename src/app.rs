use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(Debug, Clone)]
struct Img {
    id: usize,
    data: RwSignal<usize>,
}

impl Img {
    pub fn new(id: usize, data: usize) -> Self {
        Self {
            id,
            data: RwSignal::new(data)
        }
    }
}


#[derive(Debug, Clone, Copy)]
struct GlobalState {
    imgs: RwSignal<Vec<Img>>
}

impl GlobalState {
    pub fn new() -> Self {
        GlobalState {
            imgs: RwSignal::new(Vec::new())
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(GlobalState::new());

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <a href="/" >page one</a>
        <br/>
        <a href="/two" >page two</a>
        <br/>
        <Router>
        <Routes>
                <Route path="" view=PageOne/>
                <Route path="/two" view=PageTwo/>
            </Routes>
    </Router>
    }
}

#[component]
fn PageOne() -> impl IntoView {
    let global_state = use_context::<GlobalState>().unwrap();

    let on_add = move |_| {
        global_state.imgs.update(|imgs| {
            imgs.push(Img::new(imgs.len(),0));
        });
    };

    let on_inc = move |_| {
        global_state.imgs.with(|imgs| {
            for img in imgs {
                img.data.update(|data| *data += 1);
            }
        });
    };

    view! {
        <button on:click=on_add>"ADD"</button>
        <button on:click=on_inc>"INCREMENT"</button>
        <h1>"PAGE ONE"</h1>
        <ul>
            <For each=global_state.imgs key=|img|img.id let:img>
                <li>{
                    view!{
                        <span>{move || img.data.get()}</span>
                    }
                }</li>
            </For>
        </ul>
    }
}

#[component]
fn PageTwo() -> impl IntoView {
    let global_state = use_context::<GlobalState>().unwrap();

    view! {
        <h1>"PAGE TWO"</h1>
        <ul>
            <For each=global_state.imgs key=|img|img.id let:img>
                <li>{
                    view!{
                        <span>{move || img.data.get()}</span>
                    }
                }</li>
            </For>
        </ul>
    }
}