if !exists('s:cryptoJobId')
  let s:cryptoJobId = 0
endif

let s:scriptdir = resolve(expand('<sfile>:p:h') . '/..')
let s:bin       = s:scriptdir . '/target/release/vim-crypto'

" Constants for RPC messages.
let s:Crypto    = 'Crypto'
let s:CryptoTop = 'CryptoTop'

" Entry point. Initialize RPC.
" If it succeeds, then attach commands to the `rpcnotify` invocations.
function! s:connect()
  let id = s:initRpc()
  if 0 == id
    echoerr "Crypto: cannot start rpc process"
  elseif -1 == id
    echoerr "Crypto: rpc process is not executable"
  else
    let s:cryptoJobId = id
    call s:configureCommands()
  endif
endfunction

" Initialize RPC and return Job ID
function! s:initRpc()
  if s:cryptoJobId == 0
    let jobid = jobstart([s:bin], { 'rpc': v:true })
    return jobid
  else
    return s:cryptoJobId
  endif
endfunction

" Commands -> RPC
function! s:configureCommands()
  command! -nargs=1 Crypto    :call s:crypto(<f-args>)
  command! -nargs=0 CryptoTop :call s:rpc(s:CryptoTop)
endfunction

function! s:crypto(symbol)
  call rpcnotify(s:cryptoJobId, s:Crypto, a:symbol)
endfunction

" RPC Msg -> Remote Process
function! s:rpc(rpcMsg)
    call rpcnotify(s:cryptoJobId, a:rpcMsg)
endfunction

call s:connect()
