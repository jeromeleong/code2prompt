# c2p - 項目代碼提示生成工具

## 簡介
c2p 是根據[code2prompt](https://github.com/mufeedvh/code2prompt)修改的自用工具。
c2p 簡化了 code2prompt 的操作方式，同時也修復了一些在使用時發現的小問題。

## 主要的改動（2024-08-18）
### 命令行的改動
- 移除 `c2p <path>` 命令，因為增加另的子命令，所以這個命令需要轉為子命令

### 命令行子命令的改動
- 增加 `c2p clone <url>` 子命令，此功能會將網上的項目`clone`到臨時文件夾並進行提示生成處理。
- 增加 `c2p path <path>` 子命令，等於原來的`c2p <path>`

### 命令行配置的改動
- 增加 `-l / --lang` 配置，設定AI 回答的語言，如`--lang zh-hant`是使用繁體中文回答
- 增加 `--hbs <.hbs path>` 配置，用於使用自定義 Handlebars 模板文件
- 修改 `-t / --template <template name>` 配置，專注於使用預定義的 Handlebars 模板文件
    - 直接使用`-t`而不填寫`<template name>`，會顯示所有預定義的 Handlebars 模板文件，並提供交互的方式來選擇使用相關 Handlebars 模板。
    - 使用`-t <template name>`時，不需要填寫`.hbs`，如`-t write-git-commit` 即可
- 移除 `-d / --diff` 配置，程式會自動根據 Handlebars 模板文件查詢是否需要相關內容
- 移除 `--git-diff-branch` 配置，程式會自動根據 Handlebars 模板文件查詢是否需要相關內容，並用交互的方式獲取所需資料
- 移除 `--git-log-branch` 配置，對我來說有點用不上，在git.rs 直接移除了相關Function
- 移除 `--token` 配置，因為已經是標配，只能為 True

### 預定義的 Handlebars 模板文件的改動
- 增加 `write-github-changelog-daily`，以每天總結一次的方式來總結所有提交
- 增加 `write-github-changelog-biweekly`，以每兩周總結一次的方式來總結所有提交
- 修改 `write-git-commit`，根據[opencommit](https://github.com/di-sukharev/opencommit/) 項目的 Prompt 來進行修改

### Handlebars 變量的改動
- 增加 `git_log_date`，等同於`git log -p --since="YYYY-MM-DD" --until="YYYY-MM-DD"`，相關日期會通過交互的方式要求使用者填寫
- 移除 `git_log_branch`，對我來說有點用不上

## 功能
- 從代碼庫生成 LLM 提示
- 支持多種模板，包括 Git 提交、GitHub 拉取請求、文檔生成等
- 提供過濾選項，包括包含和排除模式
- 支持自定義 Handlebars 模板
- 計算生成的提示的令牌數量
- 支持將生成的提示複製到剪貼板或寫入文件

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
生成默認提示：
```sh
c2p path /path/to/codebase
```

查看並選擇預定義 Handlebars 模板：

```sh
c2p path /path/to/codebase -t
```

使用指定的預定義 Handlebars 模板：

```sh
c2p path /path/to/codebase -t template
```

使用自定義 Handlebars 模板文件：

```sh
c2p path /path/to/codebase --hbs path/to/template.hbs
```

包含特定文件模式：
```sh
c2p path /path/to/your/codebase --include "*.rs,*.py"
```

排除特定文件模式：
```sh
c2p path /path/to/your/codebase --exclude "*.log,*.txt"
```

從 GitHub 進行臨時克隆，然後生成默認提示：
```sh
c2p clone https://github.com/user/repo.git 
```

從 GitHub 進行臨時克隆，然後查看並選擇預定義 Handlebars 模板：
```sh
c2p clone https://github.com/user/repo.git -t
```

## 配置選項
所有子命令都適用下面的配置
- `--include`: 包含模式（多個模式用逗號分隔）
- `--exclude`: 排除模式（多個模式用逗號分隔）
- `--include-priority`: 在包含和排除模式衝突時，優先包含
- `--exclude-from-tree`: 根據排除模式從源樹中排除文件/文件夾
- `--encoding`: 使用的令牌化器（默認為 cl100k）
- `--output`: 輸出文件路徑
- `--line-number`: 在源代碼中添加行號
- `--no-codeblock`: 禁用將代碼包裝在 Markdown 代碼塊中
- `--relative-paths`: 使用相對路徑而不是絕對路徑
- `--no-clipboard`: 禁用複製到剪貼板
- `--template`: 使用預定義模板
- `--hbs`: 自定義 Handlebars 模板路徑
- `--json`: 以 JSON 格式打印輸出
- `--lang`: 回復使用的語言

## 貢獻指南
歡迎貢獻！請 fork 倉庫，創建分支，進行更改，並提交拉取請求。請確保通過所有測試並遵循代碼風格指南。

## 測試指南
運行測試：
```sh
cargo test
```

## Fork 的變更日誌
詳情請參閱 [CHANGELOG](CHANGELOG.md) 文件。

## 許可證
本項目使用 MIT 許可證。詳情請參閱 [LICENSE](LICENSE) 文件。

## 致謝
感謝所有貢獻者和開源社區的支持。特別感謝 Mufeed VH 和 Olivier D'Ancona 的貢獻。