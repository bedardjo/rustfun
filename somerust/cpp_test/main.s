	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 11, 0	sdk_version 11, 1
	.globl	__Z3fibi                ## -- Begin function _Z3fibi
	.p2align	4, 0x90
__Z3fibi:                               ## @_Z3fibi
	.cfi_startproc
## %bb.0:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r14
	pushq	%rbx
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	movl	$1, %r14d
	cmpl	$2, %edi
	jl	LBB0_3
## %bb.1:
	movl	%edi, %ebx
	incl	%ebx
	movl	$1, %r14d
	.p2align	4, 0x90
LBB0_2:                                 ## =>This Inner Loop Header: Depth=1
	leal	-3(%rbx), %edi
	callq	__Z3fibi
	addl	%eax, %r14d
	decl	%ebx
	cmpl	$2, %ebx
	jg	LBB0_2
LBB0_3:
	movl	%r14d, %eax
	popq	%rbx
	popq	%r14
	popq	%rbp
	retq
	.cfi_endproc
                                        ## -- End function
	.globl	_main                   ## -- Begin function main
	.p2align	4, 0x90
_main:                                  ## @main
	.cfi_startproc
## %bb.0:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movl	$20, %edi
	popq	%rbp
	jmp	__Z3fibi                ## TAILCALL
	.cfi_endproc
                                        ## -- End function
.subsections_via_symbols
