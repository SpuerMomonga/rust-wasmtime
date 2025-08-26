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