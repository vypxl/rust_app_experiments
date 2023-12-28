# Experiment 1 - htmx + hyperscript, axum, rust, surrealdb

Basically had to learn all the parts of the stack (except rust) from zero, so this took a bunch of reading work.

htmx is very cool, hyperscript as well. I like the idea of building an api that is just for the app, providing exactly what is needed. Modeling api endpoints like functions that return ui components is fun.
Overall this feels like actually building the app instead of building my own framework on a framework just to be able to do basic things.

Axum and Rust, although with some starting difficulties, seems like a solid way of defining apis. The type system to handle errors and extract query parameters, request body and more is very cool.

Surrealdb obviously is very cool, although the rust sdk docs are not the greatest. For this app tho I only used very basic features.
