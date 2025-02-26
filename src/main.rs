use dioxus::prelude::*;
use rand::prelude::*;

// Define assets
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Include assets
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: TAILWIND_CSS }

        // Render the main content
        RandomNumberGenerator {}
    }
}

#[component]
pub fn RandomNumberGenerator() -> Element {
    // State for the maximum number and the random number
    let mut rng = rand::thread_rng();
    let mut max_number = use_signal(|| 20); // Default max number
    let max: u32 = (*max_number.read()) + 1;
    let mut random_number = use_signal(|| rng.gen_range(1..max));

    // Function to generate a random number
    let generate_random_number = move |_| {
        let max: u32 = (*max_number.read()) + 1;
        random_number.set(rng.gen_range(1..max));
    };

    rsx! {
        div {
            class: "flex flex-col items-center justify-center min-h-screen bg-gray-900",
            // Small input at the top
            input {
                class: "w-32 p-2 border border-gray-300 rounded-lg text-center mt-4 bg-gray-800 text-white",
                r#type: "number",
                value: "{max_number}",
                oninput: move |e| {
                    if let Ok(value) = e.value().parse::<u32>() {
                        max_number.set(value);
                    }
                },
                placeholder: "Max number",
            }

            // Huge random number
            div {
                class: "text-9xl font-bold text-center text-white my-8",
                "{random_number}"
            }

            // Rectangle with lines and circles
            Rectangle { random_number: *random_number.read() }

            // Large centered button
            button {
                class: "px-8 py-4 bg-blue-600 text-white text-2xl font-semibold rounded-lg hover:bg-blue-700 transition duration-300",
                onclick: generate_random_number,
                "Generate"
            }
        }
    }
}

#[component]
fn Rectangle(random_number: u32) -> Element {
    // Define the lines and their corresponding circles
    // Each line is represented by a tuple: (line_number, line_style, circle_1_style, circle_2_style)
    let lines = [
        // Top-left lines (1, 2, 3)
        (10, "top-[-15px] left-0 w-20 h-4", "top-[-50px] left-[0px]", "top-[-50px] left-[50px]"),
        (1, "top-[-15px] left-35 w-20 h-4", "top-[-50px] left-[140px]", "top-[-50px] left-[190px]"),
        // Right lines (4, 5, 6)
        (2, "top-0 right-0 h-20 w-4", "top-[0px] right-[-35px]", "top-[50px] right-[-35px]"),
        (3, "top-25 right-0 h-20 w-4", "top-[100px] right-[-35px]", "top-[150px] right-[-35px]"),
        (4, "top-50 right-0 h-20 w-4", "top-[200px] right-[-35px]", "top-[250px] right-[-35px]"),
        // Bottom lines (7, 8)
        (5, "bottom-0 left-30 w-20 h-4", "bottom-[-35px] left-[165px]", "bottom-[-35px] left-[120px]"),
        (6, "bottom-0 left-5 w-20 h-4", "bottom-[-35px] left-[60px]", "bottom-[-35px] left-[20px]"),
        // Left lines (9, 10)
        (7, "bottom-0 left-0 h-20 w-4","bottom-[0px] left-[-35px]", "bottom-[50px] left-[-35px]" ),
        (8, "bottom-25 left-0 h-20 w-4",  "bottom-[100px] left-[-35px]","bottom-[150px] left-[-35px]"),
        (9, "bottom-50 left-0 h-20 w-4",  "bottom-[200px] left-[-35px]","bottom-[250px] left-[-35px]"),
    ];

    let bucket = ((random_number + 1) / 2) as u32;
    let is_odd = random_number & 1 != 0;

    rsx! {
        div {
            class: "relative w-55 h-70 mb-10 pt-5 mt-5",
            // Render each line and its corresponding circles
            for (i, line_style, circle_1_style, circle_2_style) in lines.iter() {
                div {
                    class: format!("absolute {} {}", line_style, if bucket == *i { "bg-yellow-500" } else { "bg-red-500" }),
                    id: format!("line-{}", i)
                }
                div {
                    class: format!("absolute w-8 h-8 rounded-full {} {}", circle_1_style, if bucket == *i && is_odd { "bg-yellow-500" } else { "bg-blue-800" }),
                    id: format!("circle-{}", i)
                }
                div {
                    class: format!("absolute w-8 h-8 rounded-full {} {}", circle_2_style, if bucket == *i && !is_odd { "bg-yellow-500" } else { "bg-gray-500" }),
                    id: format!("circle2-{}", i)
                }
            }
        }
    }
}