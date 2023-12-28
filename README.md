# SVN-Stash

SVN-Stash is a command-line tool designed to manage SVN stashes effectively. It provides a set of commands to create, list, view, apply, and drop stashes within your SVN repository, enhancing the workflow for developers who frequently manage a large number of changes.

## Features

- **Stash Creation**: Save your local modifications to a new stash.
- **Stash Listing**: Display a list of all the available stashes.
- **Stash Viewing**: View the contents of a specific stash.
- **Stash Dropping**: Remove a specific stash or the most recent one.
- **Stash Popping**: Apply the changes from a stash and then remove it.

## Installation

To install SVN-Stash, ensure you have Rust installed on your system. You can then clone this repository and build the tool using Cargo:

```sh
git clone https://github.com/yourusername/svn-stash.git
cd svn-stash
cargo build --release
```

The executable will be available in `target/release`.

## Usage

SVN-Stash is straightforward to use, with a simple command-line interface:

- **To stash changes:**
  ```sh
  svn-stash stash
  svn-stash stash --name "Feature XYZ"
  ```
- **To list stashes:**
  ```sh
  svn-stash list
  ```
- **To drop a stash:**
  ```sh
  svn-stash drop
  svn-stash drop --id 123
  ```
- **To pop a stash:**
  ```sh
  svn-stash pop
  svn-stash pop --id 123
  ```
- **To view a stash:**
  ```sh
  svn-stash view
  svn-stash view --id 123
  ```

## Contributing

Contributions to SVN-Stash are welcome! Please feel free to submit pull requests, create issues for bugs and feature requests, and provide feedback to improve the tool.

## License

SVN-Stash is distributed under the MIT License. See `LICENSE` for more information.
