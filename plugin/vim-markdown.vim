if !exists("g:markdown_map")
	let g:markdown_map = ";"
endif

" Titles
execute "nnoremap" . g:markdown_map . "h1 VypVr="
execute "nnoremap" . g:markdown_map . "h2 VypVr-"
execute "nnoremap" . g:markdown_map . "h3 0i### <esc>$"

" Links and pictures
execute "nnoremap" . g:markdown_map . "ln i[]()<esc>hh"
execute "nnoremap" . g:markdown_map . "pc i![]()<esc>hh"

" Code and pre blocks
execute "nnoremap" . g:markdown_map . "code i``<esc>"
execute "nnoremap" . g:markdown_map . "pre i```<cr><cr>```<esc>k"
