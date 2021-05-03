.equ REGBYTES, 8
.macro STORE reg, offset
    sd  \reg, \offset*REGBYTES(a0)
.endm
.macro LOAD reg, offset
    ld  \reg, \offset*REGBYTES(a0)
.endm
//todo: rust汇编传递常量
//这段可以直接抄，但是不知道怎么用汇编传递常量orz
    .section .text
    .global __riscv64_entry
	.align 4
__riscv64_entry:
    // 把 Host的寄存器保存在用户栈上
    // 准备进入guest os
    /* Save Host GPRs (except A0 and T0-T6) */
	//a0是函数参数
	//a0是函数参数，我觉得调用参数的时候或许应该传入地址(??)
	/* Save Host GPRs (except A0 and T0-T6) */
	sd  ra, 8(a0)
    sd  sp, 16(a0)
    sd  gp, 24(a0)
    sd  tp, 32(a0)
	sd  s0, 40(a0)
    sd  s1, 48(a0)
    sd  a1, 64(a0)
    sd  a2, 72(a0)
    sd  a3, 80(a0)
    sd  a4, 88(a0)
    sd  a5, 96(a0)

    sd  a6, 104(a0)
    sd  a7, 112(a0)
    sd  s2, 120(a0)
    sd  s3, 128(a0)
    sd  s4, 136(a0)
    sd  s5, 144(a0)
    sd  s6, 152(a0)
    sd  s7, 160(a0)
    sd  s8, 168(a0)
    sd  s9, 176(a0)
    sd  s10, 184(a0)
    sd  s11, 192(a0)

    /* Save Host and Restore Guest SSTATUS */
    ld  t0, 0x200(a0)	#guest sstatus
    csrrw	t0, 0x100, t0
    sd  t0, 0xd0(a0)	#host sstatus

    /* Save Host and Restore Guest HSTATUS */
    # ld  t1, 296(a0)
    # csrrw	t1, 0x600, t1
    # sd  t1, 216(a0)

    /* Save Host and Restore Guest SCOUNTEREN */
    # ld  t2, 304(a0)
    # csrrw	t2, 0x106, t2
    # sd  t2, 240(a0)

    /* Save Host SSCRATCH and change it to struct kvm_vcpu_arch pointer */
	csrrw	t3, 0x140, a0
    sd  t3, 224(a0)

    /* Save Host STVEC and change it to return path */
	la	t5, __riscv64_exit	#就是说
	# csrrw	t4, 0x105, t5
	csrr	t4, 0x105
	csrw	0x105, t5
    sd  t4, 0xe8(a0)	#host stvec 232

	#ATTENTION


	/* Restore Guest SEPC */
	ld	t0, 504(a0)
	csrw	0x141, t0

	/* Restore Guest GPRs (except A0) */
    # ld  ra, 256(a0)
    # ld  sp, 264(a0)
    # ld  gp, 272(a0)
    # ld  tp, 280(a0)
    # ld  t0, 288(a0)
    # ld  t1, 296(a0)
    # ld  t2, 304(a0)
    # ld  s0, 312(a0)
    # ld  s1, 320(a0)
    # ld  a1, 336(a0)
    # ld  a2, 344(a0)
    # ld  a3, 352(a0)
    # ld  a4, 360(a0)
    # ld  a5, 368(a0)
    # ld  a6, 376(a0)
    # ld  a7, 384(a0)
    # ld  s2, 392(a0)
    # ld  s3, 400(a0)
    # ld  s4, 408(a0)
    # ld  s5, 416(a0)
    # ld  s6, 424(a0)
    # ld  s7, 432(a0)
    # ld  s8, 440(a0)
    # ld  s9, 448(a0)
    # ld  s10, 456(a0)
    # ld  s11, 464(a0)
    # ld  t3, 472(a0)
    # ld  t4, 480(a0)
    # ld  t5, 488(a0)
    # ld  t6, 496(a0)

	/* Restore Guest A0 */
	ld  a0, 328(a0)

	/* Resume Guest */
	sret

	.global __riscv64_exit
	.align 4
__riscv64_exit:
    // Store the guest registers not covered by the VMCS. At this point,
    // 
    /* Swap Guest A0 with b  */
	csrrw   a0, 0x140, a0

	# /* Save Guest GPRs (except A0) */
	# sd      ra, 256(a0)
	# sd      sp, 264(a0)
	# sd      gp, 272(a0)
	# sd      tp, 280(a0)
	# sd      t0, 288(a0)
	# sd      t1, 296(a0)
	# sd      t2, 304(a0)
	# sd      s0, 312(a0)
	# sd      s1, 320(a0)
	# sd      a1, 336(a0)
	# sd      a2, 344(a0)
	# sd      a3, 352(a0)
	# sd      a4, 360(a0)
	# sd      a5, 368(a0)
	# sd      a6, 376(a0)
	# sd      a7, 384(a0)
	# sd      s2, 392(a0)
	# sd      s3, 400(a0)
	# sd      s4, 408(a0)
	# sd      s5, 416(a0)
	# sd      s6, 424(a0)
	# sd      s7, 432(a0)
	# sd      s8, 440(a0)
	# sd      s9, 448(a0)
	# sd      s10, 456(a0)
	# sd      s11, 464(a0)
	# sd      t3, 472(a0)
	# sd      t4, 480(a0)
	# sd      t5, 488(a0)
	# sd      t6, 496(a0)

	# # /* Save Guest SEPC */
	# # csrr	t0, CSR_SEPC
	# # STORE	t0, (RVMSTORE_GUEST_SEPC)(a0)
	# csrr    t0, 0x141
	# sd      t0, 504(a0)

	# # /* Restore Host STVEC */
	# # LOAD	t1, (RVMSTORE_HOST_STVEC)(a0)
	# # csrw	CSR_STVEC, t1
	# ld      t1, 232(a0)
	# csrw    0x105, t1

	# # /* Save Guest A0 and Restore Host SSCRATCH */
	# # LOAD	t2, (RVMSTORE_HOST_SSCRATCH)(a0)
	# # csrrw	t2, CSR_SSCRATCH, t2
	# # STORE	t2, (RVMSTORE_GUEST_A0)(a0)
	# ld      t2, 224(a0)
	# csrrw   t2, 0x140, t2
	# sd      t2, 328(a0)

	# # /* Save Guest and Restore Host SCOUNTEREN */
	# # LOAD	t3, (RVMSTORE_HOST_SCOUNTEREN)(a0)
	# # csrrw	t3, CSR_SCOUNTEREN, t3
	# # STORE	t3, (RVMSTORE_GUEST_SCOUNTEREN)(a0)
	# ld      t3, 240(a0)
	# csrrw   t3, 0x106, t3
	# sd      t3, 528(a0)

	# # /* Save Guest and Restore Host HSTATUS */
	# # LOAD	t4, (RVMSTORE_HOST_HSTATUS)(a0)
	# # csrrw	t4, CSR_HSTATUS, t4
	# # STORE	t4, (RVMSTORE_GUEST_HSTATUS)(a0)
	# ld      t4, 216(a0)
	# csrrw   t4, 0x600, t4
	# sd      t4, 520(a0)

	# /* Save Guest and Restore Host SSTATUS */
	ld      t5, 208(a0)	#host sstatus
	csrrw   t5, 0x100, t5
	sd      t5, 512(a0)	#guest sstatus

	# # /* Restore Host GPRs (except A0 and T0-T6) */
	ld      ra, 8(a0)

	ld      sp, 16(a0)
	ld      gp, 24(a0)
	ld      tp, 32(a0)
	ld      s0, 40(a0)
	ld      s1, 48(a0)
	ld      a1, 64(a0)
	ld      a2, 72(a0)
	ld      a3, 80(a0)
	ld      a4, 88(a0)
	ld      a5, 96(a0)
	ld      a6, 104(a0)
	ld      a7, 112(a0)
	ld      s2, 120(a0)
	ld      s3, 128(a0)
	ld      s4, 136(a0)
	ld      s5, 144(a0)
	ld      s6, 152(a0)
	ld      s7, 160(a0)
	ld      s8, 168(a0)
	ld      s9, 176(a0)
	ld      s10, 184(a0)
	ld      s11, 192(a0)

	/* Return to C code */
	ret	
	#jalr	x0, 0(x1)
	#x0 是zero， x1是ra（return address）

.global __test
__test:
	# ebreak
	li t6,0x80000000
	ld x0,0(t6)