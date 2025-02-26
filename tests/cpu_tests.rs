use gb_emulator::cpu::CPU;

#[test]
fn test_cpu_initialization() {
    let cpu = CPU::new();
    assert_eq!(cpu.pc, 0);
    assert_eq!(cpu.sp, 0);
    assert_eq!(cpu.a, 0);
}
