# 尺寸中间值计算 · 桌面版

把 `尺寸中间值计算.html` 套进 Tauri 外壳，编译成 Windows 的 `.exe`。
**页面代码一行没改**，只是外面加了个原生窗口。

## 产物

编译完成后在 `src-tauri/target/release/` 下：

| 文件 | 说明 |
|---|---|
| `尺寸中间值计算.exe` | 绿色单文件，约 5-8 MB，拷走就能双击运行，不用安装 |
| `bundle/msi/*.msi` | 正式安装包（开始菜单 + 卸载项） |
| `bundle/nsis/*-setup.exe` | 另一种安装包，中文界面，装到当前用户 |

## 用 GitHub 编译（不需要本机装 Rust）

本机只需要 `git`，Rust 和 MSVC 交给 GitHub Actions 的 Windows 机器。

```powershell
# 1. 在这个目录初始化仓库
git init
git add .
git commit -m "尺寸中间值计算 桌面版"

# 2. 到 GitHub 建一个空仓库（不要勾 README / gitignore），然后：
git remote add origin https://github.com/你的用户名/size-mid-calc.git
git branch -M main
git push -u origin main
```

推上去之后：

1. 打开仓库 → **Actions** 标签页
2. 左侧点 `build-windows`
3. 首次会自动开始（push 触发的）；之后可以点右边的 **Run workflow** 手动触发
4. 等 10-25 分钟（第一次最慢，Rust 要从头编）
5. 构建完成后，页面底部 **Artifacts** 区域下载：
   - `size-mid-calc-exe` → 解压就是那个绿色 exe
   - `size-mid-calc-installer` → 安装包

**Actions 免费额度**：公开仓库不限量；私有仓库每月 2000 分钟，一次大约 10-25 分钟，够用。

## 本机编译（可选，需要 Rust）

```powershell
# 需要 Rust + MSVC Build Tools，约 2 GB 下载
npm install
npm run tauri build
```

国内建议先配镜像再装 Rust：

```powershell
$env:RUSTUP_DIST_SERVER="https://rsproxy.cn"
$env:RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
```

## 改页面后重新出包

```powershell
copy ..\尺寸中间值计算.html src\index.html
git add -A
git commit -m "更新页面"
git push
```

推上去自动重新编译。

## 说明

- **数据存储位置**：Tauri 的 `localStorage` 存在 `%APPDATA%\com.workshop.size-mid-calc`，
  和浏览器里的不是同一份。第一次打开是空的，属正常。
- **依赖 WebView2**：Win10 1803 以后系统自带；Win7 跑不了。
- **版本号**：改 `src-tauri/tauri.conf.json` 里的 `version` 和 `Cargo.toml` 里的 `version`，
  两处要一致。
