## WIP - Minimal Markdown Editor
- End goal is to create an extremely minimal Markdown editor
- Main features:
    - Write in markdown
    - Preview markdown
    - Basic editor functionality (copy, paste, select all, etc.)
    - Open single file, save/save as


- I'm still learning Rust & GPUI - so there are definitely better ways of implementing this. Open to feedback / contributions!

### Piece Table
- Implemented a piece table as a programming exercise.
- Read more about piece tables here:
    - https://www.averylaird.com/programming/the%20text%20editor/2017/09/30/the-piece-table.html
    - https://www.catch22.net/tuts/neatpad/piece-chains/
    - https://dev.to/_darrenburns/the-piece-table---the-unsung-hero-of-your-text-editor-al8
    - https://code.visualstudio.com/blogs/2018/03/23/text-buffer-reimplementation#_piece-tree

- ⚠️ This implementation is NOT production ready.
- In future, probably makes sense to use a Rope, or other data structure & use a library which is battle tested.

### Other resources
- Inspired by these other projects:
- https://github.com/duanebester/gpui-websocket
- https://github.com/MatthiasGrandl/Loungy