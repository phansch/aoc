Start a new day by running `. ./new_day N` where N is the number of the challenge.

That script automates a bunch of stuff, because my time is limited:

1. Sets up a new rust binary project in the `dayN` directory
2. Downloads the input for the first part of the challenge into the root of that project
3. Copies over the `main.rs` from the template file
4. `cd`s into the project and opens up the `main.rs` in vim.