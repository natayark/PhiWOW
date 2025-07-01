// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use slint_build::CompilerConfiguration;

fn main() {
    let config = CompilerConfiguration::new()
        .with_style("fluent".to_string());
        
    slint_build::compile_with_config("ui/appwindow.slint", config).unwrap(); 

}
