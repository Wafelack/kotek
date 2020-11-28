if !exists("g:markdown_map")
	let g:markdown_map = ";"
endif

function! PromptLink()
	let alt = input('Enter alt for link : ')
	let link = input('Enter link : ')
	execute "normal! i[" . alt . "](" . link . ")"
endfunction


" Titles
execute "nnoremap" . g:markdown_map . "h1 VypVr="
execute "nnoremap" . g:markdown_map . "h2 VypVr-"
execute "nnoremap" . g:markdown_map . "h3 0i### <esc>$"

" Links and pictures
execute "nnoremap" . g:markdown_map . "ln :call PromptLink()<cr>"
execute "nnoremap" . g:markdown_map . "pc i! <esc>:call PromptLink()<cr>"

" Code and pre blocks
execute "nnoremap" . g:markdown_map . "code i``<esc>"
execute "nnoremap" . g:markdown_map . "pre i```<cr><cr>```<esc>k"
