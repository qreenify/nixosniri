# Neovim Learning Guide

This neovim config is designed to help you learn and improve. Here's what's included:

## 🎯 Key Learning Features

### Which-Key (Your Best Friend!)
**Press `Space` and wait** - A popup will show you all available commands!
This is the BEST way to discover shortcuts as you learn.

### Surround Text Objects
Learn vim motions while editing:
- `ysiw"` - Surround word with quotes
- `ds"` - Delete surrounding quotes
- `cs"'` - Change double quotes to single quotes
- `yss)` - Surround entire line with parentheses

### Undo Tree (`Space + u`)
Visualize your entire undo history - never lose work again!

### Trouble (`Space + xx`)
Better way to see errors and diagnostics

## 🔥 Essential Shortcuts

### File Operations
- `Space + e` - Toggle file explorer
- `Space + w` - Save file
- `Space + q` - Quit
- `Space + wq` - Save and quit

### Finding Things (Telescope)
- `Space + ff` - Find files
- `Space + fg` - Search in files (grep)
- `Space + fb` - Find open buffers

### Git Integration
- `Space + gs` - Git status
- `Space + gc` - Git commit
- `Space + gp` - Git push
- `Space + gl` - Git pull
- `Space + gb` - Git blame

### Code Navigation (LSP)
- `gd` - Go to definition
- `gr` - Find references
- `K` - Show documentation
- `Space + rn` - Rename symbol
- `Space + ca` - Code actions

### Window Management
- `Ctrl + h/j/k/l` - Move between splits

### Editing Tricks
**Visual Mode:**
- `<` / `>` - Indent left/right (stays in visual mode)
- `J` / `K` - Move selected lines up/down

**Normal Mode:**
- `Ctrl + d` / `Ctrl + u` - Scroll with cursor centered

## 📚 Learning Path

### Week 1: Basic Navigation
1. Practice moving with `h j k l` instead of arrow keys
2. Use `w` (word forward) and `b` (word back)
3. Try `0` (start of line) and `$` (end of line)

### Week 2: Editing
1. Learn the verbs: `d` (delete), `c` (change), `y` (yank/copy)
2. Combine with motions: `dw` (delete word), `ciw` (change in word)
3. Use visual mode: `v` for character, `V` for line, `Ctrl+v` for block

### Week 3: Advanced
1. Practice surround commands
2. Use `.` to repeat last action
3. Learn macros: `qa` to start recording, `q` to stop, `@a` to replay

### Week 4: Customization
1. Explore Telescope thoroughly
2. Customize your keybindings in `~/claude/modules/home.nix`
3. Add LSP servers for your languages

## 🎓 Tips

- **Press Space and wait** - Let which-key teach you!
- **Don't try to learn everything** - Master one thing at a time
- **Use what you know** - It's okay to use mouse/arrow keys while learning
- **Practice daily** - Muscle memory takes time

## 🔧 Common Tasks

### Opening Files
```
n filename.txt          # Open from terminal
:e filename.txt         # Open from within nvim
Space + ff              # Find and open with fuzzy search
```

### Multiple Files
```
:vs filename.txt        # Open in vertical split
:sp filename.txt        # Open in horizontal split
Ctrl + h/j/k/l          # Move between splits
```

### Search and Replace
```
/pattern                # Search forward
?pattern                # Search backward
n / N                   # Next/previous match
:%s/old/new/g           # Replace all in file
:'<,'>s/old/new/g       # Replace in selection
```

## 🚀 Power User Tips

1. **Use relative line numbers** - Jump with `10j` (10 lines down)
2. **Master text objects** - `ci"` (change inside quotes), `da{` (delete around braces)
3. **Learn marks** - `ma` to set mark 'a', `'a` to jump back
4. **Use registers** - `"ayy` to yank to register 'a', `"ap` to paste

## 🆘 Getting Help

- `:help <topic>` - Built-in help (e.g., `:help motions`)
- `Space` (wait) - See available commands
- `K` over a word - Show documentation

Remember: **Everyone started where you are**. Take it slow, and have fun! 🎉
