🦀 Rust Learning Roadmap
Welcome — this repository is a structured, hands-on guide to the Rust beginner topics I covered. 
It’s written to be both beginner-friendly and professionally organized for GitHub.


📂 Repository layout (high level)
The repo contains 14 lesson folders, named 01 through 14.
Each lesson folder contains:
a main.rs (code example),
and a .txt tutorial file with explanations and notes.
are lessons are single-file examples.(main.rs)


🔎 Important: How to start each lesson
Do not start by opening the .txt file directly.
Open the main.rs file first.
At the top of every main.rs you will find a comment that tells you which .txt file to open first and how to follow the lesson.
Read the referenced .txt tutorial next.
The .txt contains detailed explanations and examples for that lesson.
At the end of each .txt there is a roadmap entry that points to the next lesson (which .txt/folder to read next).
Continue following the main.rs → .txt → next pointer pattern through the lessons in order.
This order is intentional: the main.rs comments provide the exact entry point and reading sequence for that lesson.


🧭 Recommended workflow
Start at folder 01.
Open 01/main.rs and follow its top comment to the first .txt.
Read the .txt, run the code, then follow the roadmap pointer to the next lesson.
Repeat until 14.


⚙️ Running the examples
Single-file lessons

If the lesson is a single main.rs file:
rustc main.rs
./main

Install Rust (if needed)
Use rustup to install Rust and the build tools:
curl https://sh.rustup.rs -sSf | sh


🧩 Who this is for
This tutorial is designed for learners who already know at least one programming language and want to learn Rust’s fundamentals quickly and practically.
Prior programming experience will help you get the most from the exercises.


💡 Need help?
If you get stuck:
Open an issue in this repository describing the problem,
or Ask for help using ChatGPT (or another tutor). Provide the lesson number and error/output — that will speed up troubleshooting.


✅ Note
Every lesson is intentionally documented: main.rs contains entry instructions and each .txt ends with the next step in the roadmap.
The lessons build on each other; follow them in numeric order for best results.
