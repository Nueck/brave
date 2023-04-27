## Brave

项目的诞生的原因是现在个人搭建服务器学习成本比较高，需要学习nginx等。虽然目前有wordpass等博客管理可以一键搭建，但是也需要购买个人服务器。于是Brave想法诞生了，一个服务器搭建，多人可以共享，但又不像知乎，小红书一样，这里将给予每个用户有着和个人博客的独立性和扩展性，将更多的自定义交给用户。

### 🥶项目开发计划

- [x] 登陆功能
- [x] 邮箱验证码登陆
- [x] 注册功能
- [x] 忘记密码找回
- [x] 权限控制
- [x] 主页渲染(渲染是最初形态，这一部分计划后期是模板化)
- [x] 后台管理Vue项目加载
- [x] 初始化接口（之后要优化）
- [x] 初始化系统设置（比现在更加自定义）
- [x] 博客渲染(渲染是最初形态)
- [x] 用户管理
- [ ] 留言功能
- [ ] 皮肤管理（根据博客渲染的效果来实现）
- [x] 文章编辑（最初阶段）
- [x] 文章管理(最初阶段)
- [ ] 系统整理控制
- [ ] 系统管理中主页设置
- [ ] 皮肤个性化主题设置
- [ ] ...等等

### 🤖 项目开发

##### 克隆项目

```shell
git clone https://github.com/Nueck/brave.git
cd brave
```

##### 配置

修改config.yaml,根据自己的需求更改相应的值

修改.env的内容，本项目使用的数据库是pgsql15，可以根据自己数据库参数配置

```env
#数据库
PG__USER=postgres
PG__HOST=127.0.0.1
PG__PORT=5432
PG__DBNAME=postgres
```

可以进入后台管理项目 admin文件夹下 根据需求更改.env的配置

##### 运行

```
cd admin
pnpm install && pnpm build
cd ../
cargo run 
```

之后浏览器可以访问localhost:2078 （注:由于主页现在开发使用的第三方的模型，所以，访问/没有页面显示，也可以自行添加自己的主页到templates文件夹）
可以使用这个仓库的模板 [template](https://github.com/Nueck/brave-template)

默认访问localhost:2078/are-you到你的后台管理

## License

[MIT © Nueck-2023](https://github.com/Nueck/brave/blob/main/LICENSE)



