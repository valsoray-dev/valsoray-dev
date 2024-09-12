# valsoray-dev

The source code of my website, which uses [HTMX](https://github.com/bigskysoftware/htmx) to create a so-called internet joke.

All this works thanks to [actix-web](https://github.com/actix/actix-web) and the [Rust programming language](https://github.com/rust-lang/rust).

Because of this, not a single line of JavaScript is used in this project. (TypeScript is good, btw)

## Installation

1. Clone this repository:

   ```bash
   git clone https://github.com/valsoray-dev/valsoray-dev.git
   cd valsoray-dev
   ```

2. Build and run the project by using `cargo`:

   ```bash
   cargo run
   ```

3. Or by using Docker:

   - Build the Docker image:

     ```bash
     docker build -t valsoray-dev .
     ```

   - Run the container:

     ```bash
     docker run -d --rm --name valsoray-dev -p 8080:8080 valsoray-dev
     ```

## License

[MIT License](LICENSE).
