# wasm-hello


### webpack配置记录

1. 创建项目目录
    ```shell
        mkdir wasm-hello
        cd wasm-hello
    ```
    
2. npm安装依赖
    ```shell
        npm install --save-dev webpack webpack-cli
        npm install --save-dev webpack-dev-server
        npm install --save-dev copy-webpack-plugin
    ```
    
3. 添加webpack.config.js文件
    ```shell
        touch webpack.config.js
        touch res/index.html
        touch res/index.js
    ```
    ```js
        const CopyPlugin = require('copy-webpack-plugin');
        const path = require('path');

        module.exports = {
          entry: './res/index.js',
          output: {
            path: path.resolve(__dirname, "dist"),
            filename: "index.js",
          },
          mode: "development",
          plugins: [
            new CopyPlugin({
              patterns: ["res/index.html"],
            }),
          ],
          experiments: {
            asyncWebAssembly: true,
          },
        };
    ```
    
4. 在package.json中添加脚本命令
    ```json
        "scripts": {
            "build": "webpack --config webpack.config.js",
            "start": "webpack-dev-server"
        },
    ```
    
5. 在package.json中添加本地依赖
    ```json
        "dependencies": {
            "wasm-hello": "file:./pkg"
        },
    ```
    
6. 测试本地服务
    ```shell
        npm run start
    ```
    
