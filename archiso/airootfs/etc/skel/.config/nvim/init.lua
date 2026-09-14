-- StickLab OS minimal neovim defaults — extend with your own plugins.
-- CHEAT SHEET: <leader> is Space. <leader>e = file explorer. :Explore, :w, :q, u (undo).
vim.opt.number = true
vim.opt.relativenumber = true
vim.opt.expandtab = true
vim.opt.shiftwidth = 2
vim.opt.tabstop = 2
vim.opt.termguicolors = true
vim.opt.ignorecase = true
vim.opt.smartcase = true
vim.opt.clipboard = "unnamedplus"
vim.g.mapleader = " "
vim.keymap.set("n", "<leader>e", ":Explore<CR>", { silent = true })
-- Make it yours (uncomment as you grow):
-- vim.keymap.set("n", "<leader>w", ":w<CR>", { silent = true })       -- quick save
-- vim.keymap.set("n", "<leader>q", ":q<CR>", { silent = true })       -- quick quit
-- Plugins: install lazy.nvim (https://lazy.folke.io/installation), then
-- put plugin specs in ~/.config/nvim/lua/plugins/ and require them here.
