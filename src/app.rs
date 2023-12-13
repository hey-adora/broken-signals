use leptos::*;
use leptos_meta::*;

fn resize(imgs: &[(usize, RwSignal<usize>)]) {
    for img in imgs {
        img.1.update(|i| *i += 1);
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(RwSignal::new(vec![(0usize, RwSignal::new(0usize))]));
    let global_imgs = use_context::<RwSignal<Vec<(usize, RwSignal<usize>)>>>().unwrap();
    let received_imgs = RwSignal::new(0usize);

    create_effect(move |_| {
        received_imgs.with(move |received_imgs| {
            let mut new_imgs: Vec<(usize, RwSignal<usize>)> = Vec::new();
            let len = global_imgs.with(|i| i.len());
            for i in len..len + received_imgs {
                new_imgs.push((i, RwSignal::new(len)))
            }
            global_imgs.update(move |imgs| {
                imgs.extend(new_imgs);
                resize(imgs);
            });
        });
    });

    let on_add = move |_| {
        received_imgs.set(25);
    };
    let on_inc = move |_| {
        global_imgs.with(|imgs| {
            resize(imgs);
        })
    };

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <button on:click=on_add>"ADD"</button>
        <button on:click=on_inc>"INCREMENT"</button>
        <ul>
            <For each=global_imgs key=|img|img.0 let:img>
                <li>{
                    let a = img.1;
                    view!{
                        <span>{move || a.get()}</span>
                    }
                }</li>
            </For>
        </ul>
    }
}
