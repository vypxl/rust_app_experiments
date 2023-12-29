# Experiment 2 - (FAILED) dioxus

To run, you need to `cargo install dioxus-cli`

My experience with dioxus was.. not good.
From the outside it looks amazing. Great component model, easy server functions and all. However, using it felt like there was a new huge problem every little step of the way.

First, figuring out the different platforms was very weird, as they are not well documented. Especially the differences between web, ssr, liveview and fullstack are unclear. Then, trying to integrate surrealdb was cumbersome, I had to go into the sourcecode to figure out how to launch the dioxus app manually so I could get into the tokio runtime to setup the db adapter.

Then, the UI designing sounds good on paper. Live reload and all. Well, maybe I didn't find out how, but for me it did not work. Most little edits (also just to rsx) causes recompiles, and even when not, reloading the page takes ages (20+ seconds!!), because of a very slow download of one huge wasm file. Before this is loaded, no interaction. Turning cache off was not a solution, then loads of inconsistencies started to happen.

I do believe that Dioxus can be great in the future. The ideas and principles all look amazing. The current state of it sadly is not yet where it needs to be for me to consider using it.
