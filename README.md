# 墨稿 · AI 小说工坊

网页版：<https://stella16872.github.io/novel-studio/>（开启 GitHub Pages 以后）

界面全部在 `src/index.html` 里，浏览器打开就能用。写的内容存在浏览器本地（localStorage），不会跨设备同步。

## 网页版和 Mac App 的区别

- **网页版**：浏览器直接连 AI 服务商。有的服务商不允许网页直接调用（「跨域」限制），那就会连不上。
- **Mac App**：用 [Tauri](https://v2.tauri.app/) 把同一个界面打包成桌面程序（`src-tauri/` 里），
  AI 请求从 App 自己发出去，没有跨域问题。打包步骤见 [SETUP_FOR_CLAUDE_CODE.md](SETUP_FOR_CLAUDE_CODE.md)。
- 「模型接入」里的「Anthropic Claude（内置）」只在 Claude 里预览时能用，网页版和 App 里都要选一个服务商、填自己的 Key。

## 文件

| 路径 | 内容 |
|---|---|
| `src/index.html` | 墨稿的全部界面和逻辑 |
| `index.html` | 仓库首页，自动跳到 `src/` |
| `src-tauri/` | 打包成桌面 App 的部分（Rust），包括转发 AI 请求的 `http_proxy` |
| `package.json` | Tauri 命令行工具 |
