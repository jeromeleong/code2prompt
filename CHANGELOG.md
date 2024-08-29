# CHANGELOG

## [2024-08-29]

### Summary of Changes
- 新增操作手冊、維護手冊和代碼文檔模板。
- 調整 `main.rs` 中的模板常量描述，使其更加清晰。

### Detailed Changes

#### Added
- 新增 `write-operation-manual` 模板，用於撰寫操作手冊。
- 新增 `write-maintenance-manual` 模板，用於撰寫維護手冊。
- 新增 `write-installation-manual` 模板，用於撰寫安裝手冊。

#### Changed
- 修改 `document-the-code` 模板描述，更正為「生成代碼文檔」。
- 調整 `main.rs` 中的模板常量描述，使其更加清晰。

### Contributors
- Jerome <jeromeleong1998@gmail.com>

## [2024-08-18]

### Summary of Changes
- 增加命令行子命令選項，更新 README.md 中的使用示例和配置選項。
- 簡化 GitHub Actions 工作流程並更新模板。
- 更新主程式和令牌計數模組以支援 o200k 編碼。

### Detailed Changes

#### Added
- 增加 `c2p clone <url>` 子命令，用於將網上的項目 `clone` 到臨時文件夾並進行提示生成處理。
- 增加 `c2p path <path>` 子命令，等於原來的 `c2p <path>`。
- 引入新的 o200k 編碼選項。

#### Changed
- 更新 `README.md` 中的使用示例和配置選項。
- 簡化 GitHub Actions 工作流程配置。
- 重構 `token.rs` 以支援多種編碼。
- 更新 `main.rs` 以反映版本 2.1.1 在 CLI 解析器中。
- 優化 `write-git-commit.hbs` 模板，增加更嚴格的 GitMoji 使用指南。
- 更新 changelog 模板，一致提及 README 和 CHANGELOG 更新，不詳細列出變更。
- 改進 `write-github-pull-request.hbs` 模板，提高清晰度和一致性。

#### Removed
- 移除冗餘的 `dispatch-build.yml` 工作流程。
- 移除 `--token` 配置，因為已經是標配，只能為 True。

#### Fixed
- 修正 `main.rs` 中模板數據的重複定義。

### Contributors
- Jerome <jeromeleong1998@gmail.com>

## [2024-08-17]

### Summary of Changes
- 更新 Git commit 訊息模板，增加 GitMoji 表情符號指南。
- 更新依賴版本並優化代碼格式。
- 重構專案名稱為 `c2p` 並優化 Git 相關功能。

### Detailed Changes

#### Added
- 在 Git commit 訊息模板中添加了 GitMoji 表情符號指南。
- 新增 `.DS_Store` 到 `.gitignore` 文件中。

#### Changed
- 改進了 Git commit 訊息模板的結構和格式。
- 更新了 `git2`、`termtree` 和 `predicates` 的依賴版本。
- 優化了 `git.rs` 和 `main.rs` 中的日誌輸出格式。
- 修正了 `main.rs` 中模板數據的重複定義。
- 將專案名稱從 `code2prompt` 更改為 `c2p`。
- 優化了 `get_git_diff` 函數，增加對 Git diff 長度的日誌記錄。
- 更新了 CLI 參數處理，新增 `--hbs` 參數以支援自定義 Handlebars 模板文件。
- 修正了 README 文件中的錯誤並更新安裝和使用說明。

#### Removed
- 移除了 `code2prompt` 專案名稱的相關引用。

#### Fixed
- 修正了 `main.rs` 中模板數據的重複定義。

### Contributors
- Jerome <jeromeleong1998@gmail.com>