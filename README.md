# 📦 LOCALSTORAGE-RS 📦

## Overview

A Rust crate that mimics smart contract storage in system code.

It provides macros for storing and retrieving various data types in a local file system, with configurable storage paths and file extensions.

## How to use

1. Create an .env file using the .env.example file as a template.
2. Set the environment variables STORAGE_PATH and FILE_EXTENSION in .env
3. use the dotenv crate and call dotenv::dotenv().ok(); in the function you are using
