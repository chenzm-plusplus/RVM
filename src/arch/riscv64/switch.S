# global_asm!(
#     "
#     .equ REGBYTES, 8
# .macro STORE reg, offset
#     sd  \\reg, \\offset*REGBYTES(sp)
# .endm
# .macro LOAD reg, offset
#     ld  \\reg, \\offset*REGBYTES(sp)
# .endm
# //todo: rust汇编传递常量
# //这段可以直接抄，但是不知道怎么用汇编传递常量orz
# .global __riscv64_entry:
# __riscv64_entry:
# 	//a0是函数参数，我觉得调用参数的时候或许应该传入地址
# 	/* Save Host GPRs (except A0 and T0-T6) */
# 	STORE	ra, (RVMSTATE_HOST_RA)(a0)
# 	STORE	sp, (RVMSTATE_HOST_SP)(a0)
# 	STORE	gp, (RVMSTATE_HOST_GP)(a0)
# 	STORE	tp, (RVMSTATE_HOST_TP)(a0)
# 	STORE	s0, (RVMSTATE_HOST_S0)(a0)
# 	STORE	s1, (RVMSTATE_HOST_S1)(a0)
# 	STORE	a1, (RVMSTATE_HOST_A1)(a0)
# 	STORE	a2, (RVMSTATE_HOST_A2)(a0)
# 	STORE	a3, (RVMSTATE_HOST_A3)(a0)
# 	STORE	a4, (RVMSTATE_HOST_A4)(a0)
# 	STORE	a5, (RVMSTATE_HOST_A5)(a0)
# 	STORE	a6, (RVMSTATE_HOST_A6)(a0)
# 	STORE	a7, (RVMSTATE_HOST_A7)(a0)
# 	STORE	s2, (RVMSTATE_HOST_S2)(a0)
# 	STORE	s3, (RVMSTATE_HOST_S3)(a0)
# 	STORE	s4, (RVMSTATE_HOST_S4)(a0)
# 	STORE	s5, (RVMSTATE_HOST_S5)(a0)
# 	STORE	s6, (RVMSTATE_HOST_S6)(a0)
# 	STORE	s7, (RVMSTATE_HOST_S7)(a0)
# 	STORE	s8, (RVMSTATE_HOST_S8)(a0)
# 	STORE	s9, (RVMSTATE_HOST_S9)(a0)
# 	STORE	s10, (RVMSTATE_HOST_S10)(a0)
# 	STORE	s11, (RVMSTATE_HOST_S11)(a0)

# 	/* Save Host and Restore Guest SSTATUS */
# 	LOAD	t0, (RVMSTATE_GUEST_SSTATUS)(a0)
# 	csrrw	t0, CSR_SSTATUS, t0
# 	STORE	t0, (RVMSTATE_HOST_SSTATUS)(a0)

# 	/* Save Host and Restore Guest HSTATUS */
# 	LOAD	t1, (RVMSTATE_GUEST_HSTATUS)(a0)
# 	csrrw	t1, CSR_HSTATUS, t1
# 	STORE	t1, (RVMSTATE_HOST_HSTATUS)(a0)

# 	/* Save Host and Restore Guest SCOUNTEREN */
# 	LOAD	t2, (RVMSTATE_GUEST_SCOUNTEREN)(a0)
# 	csrrw	t2, CSR_SCOUNTEREN, t2
# 	STORE	t2, (RVMSTATE_HOST_SCOUNTEREN)(a0)

# 	/* Save Host SSCRATCH and change it to struct kvm_vcpu_arch pointer */
# 	csrrw	t3, CSR_SSCRATCH, a0
# 	STORE	t3, (RVMSTATE_HOST_SSCRATCH)(a0)

# 	/* Save Host STVEC and change it to return path */
# 	la	t4, __kvm_switch_return
# 	csrrw	t4, CSR_STVEC, t4
# 	STORE	t4, (RVMSTATE_HOST_STVEC)(a0)

# 	/* Restore Guest SEPC */
# 	LOAD	t0, (RVMSTATE_GUEST_SEPC)(a0)
# 	csrw	CSR_SEPC, t0

# 	/* Restore Guest GPRs (except A0) */
# 	LOAD	ra, (RVMSTATE_GUEST_RA)(a0)
# 	LOAD	sp, (RVMSTATE_GUEST_SP)(a0)
# 	LOAD	gp, (RVMSTATE_GUEST_GP)(a0)
# 	LOAD	tp, (RVMSTATE_GUEST_TP)(a0)
# 	LOAD	t0, (RVMSTATE_GUEST_T0)(a0)
# 	LOAD	t1, (RVMSTATE_GUEST_T1)(a0)
# 	LOAD	t2, (RVMSTATE_GUEST_T2)(a0)
# 	LOAD	s0, (RVMSTATE_GUEST_S0)(a0)
# 	LOAD	s1, (RVMSTATE_GUEST_S1)(a0)
# 	LOAD	a1, (RVMSTATE_GUEST_A1)(a0)
# 	LOAD	a2, (RVMSTATE_GUEST_A2)(a0)
# 	LOAD	a3, (RVMSTATE_GUEST_A3)(a0)
# 	LOAD	a4, (RVMSTATE_GUEST_A4)(a0)
# 	LOAD	a5, (RVMSTATE_GUEST_A5)(a0)
# 	LOAD	a6, (RVMSTATE_GUEST_A6)(a0)
# 	LOAD	a7, (RVMSTATE_GUEST_A7)(a0)
# 	LOAD	s2, (RVMSTATE_GUEST_S2)(a0)
# 	LOAD	s3, (RVMSTATE_GUEST_S3)(a0)
# 	LOAD	s4, (RVMSTATE_GUEST_S4)(a0)
# 	LOAD	s5, (RVMSTATE_GUEST_S5)(a0)
# 	LOAD	s6, (RVMSTATE_GUEST_S6)(a0)
# 	LOAD	s7, (RVMSTATE_GUEST_S7)(a0)
# 	LOAD	s8, (RVMSTATE_GUEST_S8)(a0)
# 	LOAD	s9, (RVMSTATE_GUEST_S9)(a0)
# 	LOAD	s10, (RVMSTATE_GUEST_S10)(a0)
# 	LOAD	s11, (RVMSTATE_GUEST_S11)(a0)
# 	LOAD	t3, (RVMSTATE_GUEST_T3)(a0)
# 	LOAD	t4, (RVMSTATE_GUEST_T4)(a0)
# 	LOAD	t5, (RVMSTATE_GUEST_T5)(a0)
# 	LOAD	t6, (RVMSTATE_GUEST_T6)(a0)

# 	/* Restore Guest A0 */
# 	LOAD	a0, (RVMSTATE_GUEST_A0)(a0)

# 	/* Resume Guest */
# 	sret
#     // 把 Host的寄存器保存在用户栈上
#     // 准备进入guest os
#     /* Save Host GPRs (except A0 and T0-T6) */
# 	//a0是函数参数

# .global __riscv64_exit
# __riscv64_exit:
#     /* Swap Guest A0 with SSCRATCH */
# 	csrrw	a0, CSR_SSCRATCH, a0

# 	/* Save Guest GPRs (except A0) */
# 	STORE	ra, (RVMSTORE_GUEST_RA)(a0)
# 	STORE	sp, (RVMSTORE_GUEST_SP)(a0)
# 	STORE	gp, (RVMSTORE_GUEST_GP)(a0)
# 	STORE	tp, (RVMSTORE_GUEST_TP)(a0)
# 	STORE	t0, (RVMSTORE_GUEST_T0)(a0)
# 	STORE	t1, (RVMSTORE_GUEST_T1)(a0)
# 	STORE	t2, (RVMSTORE_GUEST_T2)(a0)
# 	STORE	s0, (RVMSTORE_GUEST_S0)(a0)
# 	STORE	s1, (RVMSTORE_GUEST_S1)(a0)
# 	STORE	a1, (RVMSTORE_GUEST_A1)(a0)
# 	STORE	a2, (RVMSTORE_GUEST_A2)(a0)
# 	STORE	a3, (RVMSTORE_GUEST_A3)(a0)
# 	STORE	a4, (RVMSTORE_GUEST_A4)(a0)
# 	STORE	a5, (RVMSTORE_GUEST_A5)(a0)
# 	STORE	a6, (RVMSTORE_GUEST_A6)(a0)
# 	STORE	a7, (RVMSTORE_GUEST_A7)(a0)
# 	STORE	s2, (RVMSTORE_GUEST_S2)(a0)
# 	STORE	s3, (RVMSTORE_GUEST_S3)(a0)
# 	STORE	s4, (RVMSTORE_GUEST_S4)(a0)
# 	STORE	s5, (RVMSTORE_GUEST_S5)(a0)
# 	STORE	s6, (RVMSTORE_GUEST_S6)(a0)
# 	STORE	s7, (RVMSTORE_GUEST_S7)(a0)
# 	STORE	s8, (RVMSTORE_GUEST_S8)(a0)
# 	STORE	s9, (RVMSTORE_GUEST_S9)(a0)
# 	STORE	s10, (RVMSTORE_GUEST_S10)(a0)
# 	STORE	s11, (RVMSTORE_GUEST_S11)(a0)
# 	STORE	t3, (RVMSTORE_GUEST_T3)(a0)
# 	STORE	t4, (RVMSTORE_GUEST_T4)(a0)
# 	STORE	t5, (RVMSTORE_GUEST_T5)(a0)
# 	STORE	t6, (RVMSTORE_GUEST_T6)(a0)

# 	/* Save Guest SEPC */
# 	csrr	t0, CSR_SEPC
# 	STORE	t0, (RVMSTORE_GUEST_SEPC)(a0)

# 	/* Restore Host STVEC */
# 	LOAD	t1, (RVMSTORE_HOST_STVEC)(a0)
# 	csrw	CSR_STVEC, t1

# 	/* Save Guest A0 and Restore Host SSCRATCH */
# 	LOAD	t2, (RVMSTORE_HOST_SSCRATCH)(a0)
# 	csrrw	t2, CSR_SSCRATCH, t2
# 	STORE	t2, (RVMSTORE_GUEST_A0)(a0)

# 	/* Save Guest and Restore Host SCOUNTEREN */
# 	LOAD	t3, (RVMSTORE_HOST_SCOUNTEREN)(a0)
# 	csrrw	t3, CSR_SCOUNTEREN, t3
# 	STORE	t3, (RVMSTORE_GUEST_SCOUNTEREN)(a0)

# 	/* Save Guest and Restore Host HSTATUS */
# 	LOAD	t4, (RVMSTORE_HOST_HSTATUS)(a0)
# 	csrrw	t4, CSR_HSTATUS, t4
# 	STORE	t4, (RVMSTORE_GUEST_HSTATUS)(a0)

# 	/* Save Guest and Restore Host SSTATUS */
# 	LOAD	t5, (RVMSTORE_HOST_SSTATUS)(a0)
# 	csrrw	t5, CSR_SSTATUS, t5
# 	STORE	t5, (RVMSTORE_GUEST_SSTATUS)(a0)

# 	/* Restore Host GPRs (except A0 and T0-T6) */
# 	LOAD	ra, (RVMSTORE_HOST_RA)(a0)
# 	LOAD	sp, (RVMSTORE_HOST_SP)(a0)
# 	LOAD	gp, (RVMSTORE_HOST_GP)(a0)
# 	LOAD	tp, (RVMSTORE_HOST_TP)(a0)
# 	LOAD	s0, (RVMSTORE_HOST_S0)(a0)
# 	LOAD	s1, (RVMSTORE_HOST_S1)(a0)
# 	LOAD	a1, (RVMSTORE_HOST_A1)(a0)
# 	LOAD	a2, (RVMSTORE_HOST_A2)(a0)
# 	LOAD	a3, (RVMSTORE_HOST_A3)(a0)
# 	LOAD	a4, (RVMSTORE_HOST_A4)(a0)
# 	LOAD	a5, (RVMSTORE_HOST_A5)(a0)
# 	LOAD	a6, (RVMSTORE_HOST_A6)(a0)
# 	LOAD	a7, (RVMSTORE_HOST_A7)(a0)
# 	LOAD	s2, (RVMSTORE_HOST_S2)(a0)
# 	LOAD	s3, (RVMSTORE_HOST_S3)(a0)
# 	LOAD	s4, (RVMSTORE_HOST_S4)(a0)
# 	LOAD	s5, (RVMSTORE_HOST_S5)(a0)
# 	LOAD	s6, (RVMSTORE_HOST_S6)(a0)
# 	LOAD	s7, (RVMSTORE_HOST_S7)(a0)
# 	LOAD	s8, (RVMSTORE_HOST_S8)(a0)
# 	LOAD	s9, (RVMSTORE_HOST_S9)(a0)
# 	LOAD	s10, (RVMSTORE_HOST_S10)(a0)
# 	LOAD	s11, (RVMSTORE_HOST_S11)(a0)

# 	/* Return to C code */
# 	ret
#     "
# );