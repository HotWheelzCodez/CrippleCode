format ELF64 executable
to_print db "hello", 0, 10
to_print_end:
to_print_len equ to_print_end-to_print
print_0 db "Begin", 0, 10
print_end_0:
print_len_0 equ print_end_0-print_0
print_1 db "End", 0, 10
print_end_1:
print_len_1 equ print_end_1-print_1
entry _start
_start:
mov rax, 1
mov rdi, 1
mov rsi, print_0
mov rdx, print_len_0
syscall
repeat 10
mov rax, 1
mov rdi, 1
mov rsi, to_print
mov rdx, to_print_len
syscall
end repeat
mov rax, 1
mov rdi, 1
mov rsi, print_1
mov rdx, print_len_1
syscall
