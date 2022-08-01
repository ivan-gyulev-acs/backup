:set relativenumber
:set mouse=a
:set autoindent
:set noexpandtab
:set tabstop=4
:set shiftwidth=4
:set guifont=Source\ Code\ Pro:16
:set clipboard+=unnamedplus
:set nowrap

call plug#begin()
Plug 'https://github.com/preservim/tagbar'
Plug 'https://github.com/junegunn/fzf.vim'
Plug 'https://github.com/vim-airline/vim-airline'
Plug 'https://github.com/preservim/nerdtree'
call plug#end()

nnoremap <M-f> :NERDTreeFocus<CR>
nnoremap <M-n> :NERDTree<CR>
nnoremap <M-t> :NERDTreeToggle<CR>
