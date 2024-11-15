// old:
// 描述渲染内容实际(绝对)位置的结构
// 比如terminal高度是40, 那么就有 0 ~ 39 行
// 如果此时光标的location.y是40, 就不会在窗口中显示
// 如果实现了滚动功能, 那么position.y应当是39
// #[derive(Copy, Clone, Default)]
// pub struct Location {
//     pub x: usize,
//     pub y: usize,
// }

#[derive(Copy, Clone, Default)]
pub struct Location {
    pub grapheme_index: usize,
    pub line_index: usize,
}