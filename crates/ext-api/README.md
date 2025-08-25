# Extension API

## Struct API

```rust

use raykit_api::{Extension, Command, CommandOutput register_extension};

pub struct TestExtension;

impl Extension for TestExtension {

    fn new() -> Self {
        Self {}
    }

    fn run_commands(command: Command) -> Result<CommandOutput, String> {
        match command.name.as_str() {
            "search.files" => {
                // 搜索文件
            }
            "search.app" => {å
                // 搜索应用
            }
            command => {
                // 其它
            }
        }
    }
}

register_extension!(TestExtension);

```

## Macro API

```rust

use raykit_api::{command, CommandOutput register_commands};

#[command(name = "search.files")]
fn search_files() -> String {
    // 搜索文件
}

#[command(name = "search.app")]
fn search_app() -> String {
    // 搜索应用
}

register_commands!([search_files, search_app]);

```

## VSCode API

```rust 

use raykit_api::{Extension, commands, CommandOutput register_extension};

pub struct TestExtension;

impl Extension for TestExtension {

    fn new() -> Self {
        Self {}
    }

    fn activate(ctx: Context) {
        ctx.subscriptions.push(commands::register_command("search.files", || {
            // 搜索文件
        }))

        ctx.subscriptions.push(commands::register_command("search.app", || {
            // 搜索文件
        }))
    }

    fn destroy() {

    } 
}

register_extension!(TestExtension);

```

## Build API

```rust

use raykit_api::Builder;

Builder::default()
    .command("search.files", || {
        // 搜索文件
    })
    .command("search.app", || {
        // 搜索应用
    })
    .build();

```