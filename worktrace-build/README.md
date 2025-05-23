# WorkTrace Build 劳动溯源·构建工具

Build script utilities behind the [WorkTrace](https://crates.io/crates/worktrace) crate. This crate is intended to be used in your project's build script (`build.rs` and `build-dependencies`) rather than direct `dependencies`. It contains utilities for building including:

1. Generate license notation from template file.
2. Encapsulations for generating Proto Buffer code.
3. Update changelog of current version into a file from raw changelog (usually used for release notes).

[劳动溯源](https://crates.io/crates/worktrace)项目的源码构建工具。此库仅用于构建脚本使用(在`build.rs`中使用，并通过`build-dependencies`引用)，并非用于直接依赖(`dependencies`)。

1. 从模板文件中生成许可证声明注释。
2. 生成 Proto Buffer 代码的相关封装。
3. 从变更日志中生成当前版本的变更(常用于发行版说明)。

## License 开源协议

WorkTrace is licensed under [Mulan PSL v2](http://license.coscl.org.cn/MulanPSL2). You can find the license text in the [LICENSE](../LICENSE) file, and all contributors are listed in [CONTRIBUTORS.yaml](../CONTRIBUTORS.yaml).

本库以木兰宽松协议开源。您可以在 [LICENSE](../LICENSE) 文件中找到协议文本，所有贡献者名单在 [CONTRIBUTORS.yaml](../CONTRIBUTORS.yaml) 中。
