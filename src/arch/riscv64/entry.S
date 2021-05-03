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
    ld  t0, 288(a0)
    csrrw	t0, 0x100, t0
    sd  t0, 208(a0)

    /* Save Host and Restore Guest HSTATUS */
    ld  t1, 296(a0)
    csrrw	t1, 0x600, t1
    sd  t1, 216(a0)

    /* Save Host and Restore Guest SCOUNTEREN */
    ld  t2, 304(a0)
    csrrw	t2, 0x106, t2
    sd  t2, 240(a0)

    /* Save Host SSCRATCH and change it to struct kvm_vcpu_arch pointer */
	csrrw	t3, 0x140, a0
    sd  t3, 224(a0)

    /* Save Host STVEC and change it to return path */
	la	t4, __riscv64_exit
	csrrw	t4, 0x105, t4
    sd  t4, 232(a0)

	/* Restore Guest SEPC */
	ld	t0, 504(a0)
	csrw	0x141, t0

	/* Restore Guest GPRs (except A0) */
    ld  ra, 256(a0)
    ld  sp, 264(a0)
    ld  gp, 272(a0)
    ld  tp, 280(a0)
    ld  t0, 288(a0)
    ld  t1, 296(a0)
    ld  t2, 304(a0)
    ld  s0, 312(a0)
    ld  s1, 320(a0)
    ld  a1, 336(a0)
    ld  a2, 344(a0)
    ld  a3, 352(a0)
    ld  a4, 360(a0)
    ld  a5, 368(a0)
    ld  a6, 376(a0)
    ld  a7, 384(a0)
    ld  s2, 392(a0)
    ld  s3, 400(a0)
    ld  s4, 408(a0)
    ld  s5, 416(a0)
    ld  s6, 424(a0)
    ld  s7, 432(a0)
    ld  s8, 440(a0)
    ld  s9, 448(a0)
    ld  s10, 456(a0)
    ld  s11, 464(a0)
    ld  t3, 472(a0)
    ld  t4, 480(a0)
    ld  t5, 488(a0)
    ld  t6, 496(a0)

	/* Restore Guest A0 */
	ld  a0, 328(a0)

	/* Resume Guest */
	sret

.global __riscv64_exit
__riscv64_exit:
    // Store the guest registers not covered by the VMCS. At this point,
    // 
    /* Swap Guest A0 with SSCRATCH */
	# csrrw	a0, CSR_SSCRATCH, a0

	# /* Save Guest GPRs (except A0) */
	# STORE	ra, (RVMSTORE_GUEST_RA)(a0)
	# STORE	sp, (RVMSTORE_GUEST_SP)(a0)
	# STORE	gp, (RVMSTORE_GUEST_GP)(a0)
	# STORE	tp, (RVMSTORE_GUEST_TP)(a0)
	# STORE	t0, (RVMSTORE_GUEST_T0)(a0)
	# STORE	t1, (RVMSTORE_GUEST_T1)(a0)
	# STORE	t2, (RVMSTORE_GUEST_T2)(a0)
	# STORE	s0, (RVMSTORE_GUEST_S0)(a0)
	# STORE	s1, (RVMSTORE_GUEST_S1)(a0)
	# STORE	a1, (RVMSTORE_GUEST_A1)(a0)
	# STORE	a2, (RVMSTORE_GUEST_A2)(a0)
	# STORE	a3, (RVMSTORE_GUEST_A3)(a0)
	# STORE	a4, (RVMSTORE_GUEST_A4)(a0)
	# STORE	a5, (RVMSTORE_GUEST_A5)(a0)
	# STORE	a6, (RVMSTORE_GUEST_A6)(a0)
	# STORE	a7, (RVMSTORE_GUEST_A7)(a0)
	# STORE	s2, (RVMSTORE_GUEST_S2)(a0)
	# STORE	s3, (RVMSTORE_GUEST_S3)(a0)
	# STORE	s4, (RVMSTORE_GUEST_S4)(a0)
	# STORE	s5, (RVMSTORE_GUEST_S5)(a0)
	# STORE	s6, (RVMSTORE_GUEST_S6)(a0)
	# STORE	s7, (RVMSTORE_GUEST_S7)(a0)
	# STORE	s8, (RVMSTORE_GUEST_S8)(a0)
	# STORE	s9, (RVMSTORE_GUEST_S9)(a0)
	# STORE	s10, (RVMSTORE_GUEST_S10)(a0)
	# STORE	s11, (RVMSTORE_GUEST_S11)(a0)
	# STORE	t3, (RVMSTORE_GUEST_T3)(a0)
	# STORE	t4, (RVMSTORE_GUEST_T4)(a0)
	# STORE	t5, (RVMSTORE_GUEST_T5)(a0)
	# STORE	t6, (RVMSTORE_GUEST_T6)(a0)

	# /* Save Guest SEPC */
	# csrr	t0, CSR_SEPC
	# STORE	t0, (RVMSTORE_GUEST_SEPC)(a0)

	# /* Restore Host STVEC */
	# LOAD	t1, (RVMSTORE_HOST_STVEC)(a0)
	# csrw	CSR_STVEC, t1

	# /* Save Guest A0 and Restore Host SSCRATCH */
	# LOAD	t2, (RVMSTORE_HOST_SSCRATCH)(a0)
	# csrrw	t2, CSR_SSCRATCH, t2
	# STORE	t2, (RVMSTORE_GUEST_A0)(a0)

	# /* Save Guest and Restore Host SCOUNTEREN */
	# LOAD	t3, (RVMSTORE_HOST_SCOUNTEREN)(a0)
	# csrrw	t3, CSR_SCOUNTEREN, t3
	# STORE	t3, (RVMSTORE_GUEST_SCOUNTEREN)(a0)

	# /* Save Guest and Restore Host HSTATUS */
	# LOAD	t4, (RVMSTORE_HOST_HSTATUS)(a0)
	# csrrw	t4, CSR_HSTATUS, t4
	# STORE	t4, (RVMSTORE_GUEST_HSTATUS)(a0)

	# /* Save Guest and Restore Host SSTATUS */
	# LOAD	t5, (RVMSTORE_HOST_SSTATUS)(a0)
	# csrrw	t5, CSR_SSTATUS, t5
	# STORE	t5, (RVMSTORE_GUEST_SSTATUS)(a0)

	# /* Restore Host GPRs (except A0 and T0-T6) */
	# LOAD	ra, (RVMSTORE_HOST_RA)(a0)
	# LOAD	sp, (RVMSTORE_HOST_SP)(a0)
	# LOAD	gp, (RVMSTORE_HOST_GP)(a0)
	# LOAD	tp, (RVMSTORE_HOST_TP)(a0)
	# LOAD	s0, (RVMSTORE_HOST_S0)(a0)
	# LOAD	s1, (RVMSTORE_HOST_S1)(a0)
	# LOAD	a1, (RVMSTORE_HOST_A1)(a0)
	# LOAD	a2, (RVMSTORE_HOST_A2)(a0)
	# LOAD	a3, (RVMSTORE_HOST_A3)(a0)
	# LOAD	a4, (RVMSTORE_HOST_A4)(a0)
	# LOAD	a5, (RVMSTORE_HOST_A5)(a0)
	# LOAD	a6, (RVMSTORE_HOST_A6)(a0)
	# LOAD	a7, (RVMSTORE_HOST_A7)(a0)
	# LOAD	s2, (RVMSTORE_HOST_S2)(a0)
	# LOAD	s3, (RVMSTORE_HOST_S3)(a0)
	# LOAD	s4, (RVMSTORE_HOST_S4)(a0)
	# LOAD	s5, (RVMSTORE_HOST_S5)(a0)
	# LOAD	s6, (RVMSTORE_HOST_S6)(a0)
	# LOAD	s7, (RVMSTORE_HOST_S7)(a0)
	# LOAD	s8, (RVMSTORE_HOST_S8)(a0)
	# LOAD	s9, (RVMSTORE_HOST_S9)(a0)
	# LOAD	s10, (RVMSTORE_HOST_S10)(a0)
	# LOAD	s11, (RVMSTORE_HOST_S11)(a0)

	/* Return to C code */
	ret