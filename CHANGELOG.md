# 變更日誌

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