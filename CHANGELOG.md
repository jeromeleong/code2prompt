# 變更日誌

## 日期: 2024-08-18

### 變更摘要
- 更新主程式和令牌計數模組以支援 o200k 編碼。
- 更新專案配置和文檔。

### 詳細變更

#### 新增
- 引入新的 o200k 編碼選項。
- 新增 `write-github-changelog-daily` 和 `write-github-changelog-biweekly` 模板，以每天和每兩周總結一次的方式來總結所有提交。
- 新增 `git_log_date` 變量，等同於 `git log -p --since="YYYY-MM-DD" --until="YYYY-MM-DD"`，相關日期會通過交互的方式要求使用者填寫。

#### 修改
- 重構 `token.rs` 以支援多種編碼。
- 更新 `README.md` 以反映最新的命令行配置選項。
- 更新 `git.rs` 以支持新的 Git log 和 diff 功能。
- 優化模板文件以支持新的 Handlebars 變量。
- 修改 `write-git-commit` 模板，根據 [opencommit](https://github.com/di-sukharev/opencommit/) 項目的 Prompt 來進行修改。

#### 移除
- 移除 `-d / --diff` 配置，程式會自動根據 Handlebars 模板文件查詢是否需要相關內容。
- 移除 `--git-diff-branch` 配置，程式會自動根據 Handlebars 模板文件查詢是否需要相關內容，並用交互的方式獲取所需資料。
- 移除 `--git-log-branch` 配置，對我來說有點用不上，在 `Git.rs` 直接移除了相關 Function。

#### 修復
- 修正了 `main.rs` 中模板數據的重複定義。

### 貢獻者
- Jerome <jeromeleong1998@gmail.com>

## 日期: 2024-08-17

### 變更摘要
- 更新了 Git commit 訊息模板，增加了 GitMoji 表情符號指南。
- 更新了依賴版本並優化了代碼格式。
- 重構專案名稱為 `c2p` 並優化了 Git 相關功能。

### 詳細變更

#### 新增
- 在 Git commit 訊息模板中添加了 GitMoji 表情符號指南。
- 新增 `.DS_Store` 到 `.gitignore` 文件中。

#### 修改
- 改進了 Git commit 訊息模板的結構和格式。
- 更新了 `git2`、`termtree` 和 `predicates` 的依賴版本。
- 優化了 `git.rs` 和 `main.rs` 中的日誌輸出格式。
- 修正了 `main.rs` 中模板數據的重複定義。
- 將專案名稱從 `code2prompt` 更改為 `c2p`。
- 優化了 `get_git_diff` 函數，增加了對 Git diff 長度的日誌記錄。
- 更新了 CLI 參數處理，新增 `--hbs` 參數以支援自定義 Handlebars 模板文件。
- 修正了 README 文件中的錯誤並更新安裝和使用說明。

#### 移除
- 移除了 `code2prompt` 專案名稱的相關引用。

#### 修復
- 修正了 `main.rs` 中模板數據的重複定義。

### 貢獻者
- Jerome <jeromeleong1998@gmail.com>