
# 这个函数真实的功能是触发一个异常
.global __test
.align 4
__test:
	# ebreak
	li t6,0x80000000
	ld x0,0(t6)

# 用途是写通用寄存器，用于检查是否可以被正确保存
.global __test_write_general_registers
.align 4
__test_write_general_registers:
	# li      sp, 16
	# li      gp, 24
	# li      tp, 32
	li      s0, 40
	li      s1, 48
	li      a1, 64
	li      a2, 72
	li      a3, 80
	li      a4, 88
	li      a5, 96
	li      a6, 104
	li      a7, 112
	li      s2, 120
	li      s3, 128
	li      s4, 136
	li      s5, 144
	li      s6, 152
	li      s7, 160
	li      s8, 168
	li      s9, 176
	li      s10, 184
	li      s11, 192