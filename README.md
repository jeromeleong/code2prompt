# c2p

## 簡介
`c2p` 是一個命令行工具（CLI），用於從任何大小的代碼庫快速生成 LLM 提示。它支持源代碼樹、提示模板化和令牌計數。

## 功能
- 從代碼庫快速生成 LLM 提示。
- 自定義提示生成使用 Handlebars 模板。
- 尊重 `.gitignore`。
- 使用 glob 模式過濾和排除文件。
- 顯示生成提示的令牌計數。
- 可選地包含 Git diff 輸出（已暫存的文件）。
- 自動將生成的提示複製到剪貼板。
- 將生成的提示保存到輸出文件。
- 按名稱或路徑排除文件和文件夾。
- 為源代碼塊添加行號。

## 安裝說明
### 最新發布版本
從 [Releases](https://github.com/jeromeleong/c2p/releases) 下載適用於您操作系統的最新二進制文件，或使用 `cargo` 安裝：

```sh
cargo install --git https://github.com/jeromeleong/c2p
```

### 從源代碼構建
您需要安裝以下工具：
- [Git](https://git-scm.org/downloads)
- [Rust](https://rust-lang.org/tools/install)
- Cargo（安裝 Rust 時自動安裝）

```sh
git clone https://github.com/jeromeleong/c2p.git
cd c2p/
cargo build --release
```

## 使用示例
生成代碼庫的提示：

```sh
c2p path/to/codebase
```

查看預設模板：

```sh
c2p path/to/codebase -t
```

使用預設模板：

```sh
c2p path/to/codebase -t template
```

使用自定義 Handlebars 模板文件：

```sh
c2p path/to/codebase --hbs path/to/template.hbs
```

使用 glob 模式過濾文件：

```sh
c2p path/to/codebase --include="*.rs,*.toml"
```

排除文件使用 glob 模式：

```sh
c2p path/to/codebase --exclude="*.txt,*.md"
```

顯示生成提示的令牌計數：

```sh
c2p path/to/codebase --tokens
```

指定令牌計數的令牌化器：

```sh
c2p path/to/codebase --tokens --encoding=p50k
```

保存生成的提示到輸出文件：

```sh
c2p path/to/codebase --output=output.txt
```

指定回應語言：

```sh
c2p path/to/codebase --lang zh-hant
```

## 配置選項
- `--include`: 包含的文件模式。
- `--exclude`: 排除的文件模式。
- `--include-priority`: 在包含和排除模式衝突時優先包含。
- `--tokens`: 顯示令牌計數。
- `--encoding`: 指定令牌化器的編碼。
- `--output`: 輸出文件路徑。
- `--diff`: 包含 Git diff 輸出。
- `--git-diff-branch`: 生成兩個分支之間的 Git diff。
- `--git-log-branch`: 檢索兩個分支之間的 Git log。
- `--line-number`: 為源代碼塊添加行號。
- `--no-codeblock`: 禁用將代碼包裝在 markdown 代碼塊中。
- `--relative-paths`: 使用相對路徑。
- `--no-clipboard`: 禁用複製到剪貼板。
- `--hbs`: 自定義 Handlebars 模板文件路徑。
- `--json`: 以 JSON 格式打印輸出。
- `--lang`: 指定回應語言。

## 貢獻指南
歡迎貢獻！您可以通過以下方式參與：
- 提出功能建議
- 報告錯誤
- 修復問題並提交拉取請求
- 幫助文檔
- 推廣項目

## 測試說明
項目包含單元測試和集成測試。您可以使用以下命令運行測試：

```sh
cargo test
```

## 許可證
本項目採用 MIT 許可證，詳情請參閱 [LICENSE](https://github.com/jeromeleong/c2p/blob/master/LICENSE)。

## 致謝
感謝所有貢獻者和開源社區的支持。特別感謝 Mufeed VH 和 Olivier D'Ancona 的貢獻。