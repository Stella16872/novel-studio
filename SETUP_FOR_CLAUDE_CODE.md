# 给 Claude Code 的执行说明

这是"墨稿 AI 小说工坊"的 Tauri 桌面版项目骨架，前端代码已经写好并适配过（会自动识别是否在 Tauri
壳里运行），Rust 后端也写好了一个 HTTP 转发命令用来绕开浏览器跨域限制。你需要做的是把开发环境
装起来、把项目跑起来、最后打出一个 Mac 上能双击打开的 .app / .dmg。

目标用户不熟悉命令行，所以每一步执行完都最好用一句话告诉他发生了什么、要不要等。

## 环境：macOS

## 第一步：检查 Rust 是否已安装

```bash
rustc --version
cargo --version
```

- 如果两个都输出了版本号 → 跳到第二步。
- 如果提示 `command not found` → 执行下面命令安装（这一步会下载安装，需要联网，可能要几分钟）：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

装完之后重新执行 `rustc --version` 确认能看到版本号。

## 第二步：检查 Xcode 命令行工具

```bash
xcode-select -p
```

- 输出一个路径（比如 `/Library/Developer/CommandLineTools`）→ 已装好，跳到第三步。
- 报错 → 执行：

```bash
xcode-select --install
```

这会弹出一个图形界面的安装向导，需要用户自己点几下"安装"，这一步没法完全自动化，装完再继续。

## 第三步：检查 Node.js（用来跑 `npm run tauri` 这些命令）

```bash
node --version
npm --version
```

如果没有，参考网上 Node.js 官网（nodejs.org）下载安装包装一下，或者如果这台机器之前装过就直接用。

## 第四步：安装项目依赖

在这个项目根目录（也就是这个文件所在的目录）执行：

```bash
npm install
```

这一步会把 `@tauri-apps/cli` 装到项目里。

## 第五步：生成应用图标

项目里已经放了一张 1024x1024 的图标源图 `src-tauri/icons/icon-source.png`，用 Tauri 自带的工具
一键生成 Mac/Windows/Linux 各种尺寸和格式：

```bash
npm run tauri icon src-tauri/icons/icon-source.png
```

跑完之后 `src-tauri/icons/` 目录下应该会多出 `icon.icns`、`32x32.png`、`128x128.png` 等文件。

## 第六步：先用开发模式跑起来看看

```bash
npm run tauri dev
```

第一次执行会编译 Rust 依赖，可能要几分钟（甚至十几分钟，取决于电脑性能），耐心等，中途看到很多
`Compiling xxx` 的滚动日志是正常的。编译完会自动弹出一个应用窗口，界面应该跟之前的网页版一模一样。

测试重点：
1. 「模型接入」页选一个服务商（比如 DeepSeek）、填 API Key、点"测试连接" —— 这一步应该不会再出现
   跨域报错了，因为请求现在是从 Rust 后端发出去的。
2. 关掉再重新打开这个 app，检查之前写的内容还在不在（本地存储是否正常）。

如果第 1 点还是报错，把报错内容截图，需要检查 `src-tauri/src/lib.rs` 里的 `http_proxy` 命令有没有
正确注册、以及前端 `smartFetch` 有没有正确识别到 `window.__TAURI__`。

## 第七步：确认没问题后，打正式安装包

```bash
npm run tauri build
```

这一步比 dev 模式更久，会做优化编译。跑完之后，能双击打开的 .app 在这里：

```
src-tauri/target/release/bundle/macos/墨稿.app
```

如果想要 .dmg 安装包（方便分享给别人拖到"应用程序"文件夹）：

```
src-tauri/target/release/bundle/dmg/墨稿_0.1.0_aarch64.dmg
```
（文件名里的版本号、架构后缀可能因电脑是 Intel 还是 Apple Silicon 而略有不同）

把最终这个 .app 或 .dmg 的路径告诉用户，或者直接帮他把 .app 拖到"应用程序"文件夹里。

## 已知注意事项（不用再问，直接照做就行）

- 「模型接入」里的"Anthropic Claude（内置）"选项在桌面版里**用不了**——那个选项依赖 Claude.ai
  网页版专属的后台代理，脱离那个环境肯定报错。这是预期行为，不是 bug，不用去"修"它。用户需要选
  一个真实服务商（DeepSeek/OpenAI/Gemini等）并填自己的 Key。
- 数据存储走的是浏览器标准的 localStorage（在 Tauri 的 WebView 里能正常工作），存在用户本地，不
  会跨设备同步，这也是预期行为。
- 如果 `cargo build` 报缺少某个系统库，通常搜报错信息里的库名 + "macOS" 能找到对应的 `brew install`
  命令，Tauri 官方文档 https://v2.tauri.app/start/prerequisites/ 也有 macOS 部分可以对照。
