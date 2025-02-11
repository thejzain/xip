# xip - Easy to use File Archiver and Extractor

xip is a command-line tool written in Rust designed to be an easy-to-use file archiver and extractor.  Currently, it supports **zip** and **tar.gz** formats.

**Note:** xip is a work in progress.

## Features

  * **Extraction:**

      * ✅  Extracts `.zip` archives.
      * ✅  Extracts `.tar.gz` archives.

  * **Archiving:**

      * ✅  Creates `.tar.gz` archives from directories.
      * 🚧  Archiving of individual files into `.tar.gz` is **currently not working as expected** and needs to be fixed.
      * ❌  Archiving to `.zip` format is **not yet implemented**.

## Usage

```bash
xip <path> [OPTIONS]
```

  * `<path>`:  Specifies the path to the archive file for extraction or the path to the directory for archiving.

### Options

  * `-d <DIR>`, `--dir <DIR>`:  Specify the directory to extract the archive contents to. If not provided, files will be extracted to the current directory. This option is used for **extraction**.

    ```bash
    xip archive.zip -d output_directory
    xip archive.tar.gz --dir /path/to/extraction
    ```

  * `<paths>`:  Optional list of paths to directories that should be included in the tar.gz archive. If this option is used, `<path>` should be the desired output path for the archive file. This option is used for **archiving**.

    ```bash
    xip my_archive.tar.gz path/to/directory1 path/to/directory2
    ```

**Examples:**

**Extraction:**

  * Extract `my_archive.zip` to the current directory:

    ```bash
    xip my_archive.zip
    ```

  * Extract `my_archive.tar.gz` to the `extracted_files` directory:

    ```bash
    xip my_archive.tar.gz -d extracted_files
    ```

**Archiving (Tar.gz - Directory only for now):**

  * Create a `my_archive.tar.gz` archive from `my_directory`:

    ```bash
    xip my_archive.tar.gz my_directory
    ```

  * Create a `backup.tar.gz` archive containing `directory1` and `directory2`:

    ```bash
    xip backup.tar.gz directory1 directory2
    ```

## Installation

To install `xip`, you will need to have Rust and Cargo installed on your system. You can install Rust from [https://www.rust-lang.org/tools/install](https://www.google.com/url?sa=E&source=gmail&q=https://www.google.com/url?sa=E%26source=gmail%26q=https://www.rust-lang.org/tools/install).

Once Rust is installed, you can build and install `xip` using Cargo:

```bash
cargo install --path .
```

Navigate to the root directory of the `xip` project in your terminal and run the command above. This will build the executable and place it in your Cargo bin directory (usually `~/.cargo/bin`). Ensure this directory is in your system's `PATH` environment variable to run `xip` from anywhere in your terminal.

## Work in Progress & Limitations

  * **Tar.gz Archiving for Files:**  Currently, archiving using `tar.gz` only works correctly when archiving directories. Archiving individual files or a mix of files and directories into a `.tar.gz` archive is not yet functioning as expected and will be addressed in future updates.
  * **Zip Archiving:** Archiving to `.zip` format is not implemented in this version.
  * **Error Handling:** Error handling is basic and will be improved for more informative messages.
  * **Format Support:**  Currently, only `.zip` and `.tar.gz` are supported. More archive formats may be added in the future.
``
