### pub(crate) xxxx 只能在当前 crate 中使用
- lib.rs 中引入的 pub(crate) xxx 则不能在 main 中使用
- lib.rs 是库 crate 入口
- main.rs 是二进制 crate 入口
