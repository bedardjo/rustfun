.globl _fib
_fib:
	cmpl	$2, %edi
  jl fib_end

  movl %edi, %r14d
  pushq %r14

  decl %r14d
  pushq %r14
  movl %r14d, %edi
  callq _fib
  popq %r14
  movl %r14d, %edi
  decl %edi

  movl %eax, %r14d
  pushq	%r14

  callq _fib
  movl %eax, %r14d
  pushq	%r14

  popq %r14
  popq %r15
  addl %r14d, %r15d
	movl %r15d, %eax

  popq %r14
  movl %r14d, %edi

	retq

fib_end:
	movl $1, %eax
	retq

