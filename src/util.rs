use zeerust::ops::{Location16, Location8, Op, Op::*, Reg16, Reg8};

pub enum Location {
    Loc8(Location8),
    Loc16(Location16),
}

/// Some 8-bit Location
fn sl8(loc: Location8) -> Option<Location> {
    Some(Location::Loc8(loc))
}

/// Some 16-bin Location
fn sl16(loc: Location16) -> Option<Location> {
    Some(Location::Loc16(loc))
}

const ACC: Option<Location> = Some(Location::Loc8(Location8::Reg(Reg8::A)));
const REG_B: Option<Location> = Some(Location::Loc8(Location8::Reg(Reg8::B)));
const HL: Option<Location> = Some(Location::Loc8(Location8::RegIndirect(Reg16::HL)));

/// Return the locations written to and read from by the given operation
pub fn op_dst_src(op: Op) -> (Option<Location>, Option<Location>) {
    match op {
        ADC(dst, src) => (sl8(dst), sl8(src)),
        ADD8(dst, src) => (sl8(dst), sl8(src)),
        INC(loc) => (sl8(loc.clone()), sl8(loc)),

        SBC(dst, src) => (sl8(dst), sl8(src)),
        SUB8(dst, src) => (sl8(dst), sl8(src)),
        DEC(loc) => (sl8(loc.clone()), sl8(loc)),

        AND(loc) => (ACC, sl8(loc)),
        OR(loc) => (ACC, sl8(loc)),
        XOR(loc) => (ACC, sl8(loc)),
        CP(loc) => (None, sl8(loc)),

        CPL => (ACC, ACC),
        NEG => (ACC, ACC),
        CCF => (None, None), // TODO: Show off flags too
        SCF => (None, None),

        NOP => (None, None),
        HALT => (None, None),

        DAA => (None, None),

        RLCA | RLA | RRCA | RRA => (ACC, ACC),

        RLC(loc) | RL(loc) | RRC(loc) | RR(loc) => (sl8(loc.clone()), sl8(loc)),

        SLA(loc) | SRL(loc) | SRA(loc) => (sl8(loc.clone()), sl8(loc)),

        // TODO: src and dst are both used here
        RLD | RRD => (ACC, HL),

        BIT(_, loc) => (None, sl8(loc)), // TODO: flags
        SET(_, loc) | RES(_, loc) => (sl8(loc), None),

        IN(_addr, loc) => (sl8(loc), None),
        OUT(_addr, loc) => (None, sl8(loc)),

        JP(_, loc) => (None, sl16(loc)),
        JR(_, _) => (None, None), // TODO: Immediate

        DJNZ(_) => (REG_B, REG_B),
        CALL(_, _) | RET(_) => (None, None), // TODO: PC, SP

        POP(loc) => (sl16(loc), None),
        PUSH(loc) => (None, sl16(loc)),

        LD8(dst, src) => (sl8(dst), sl8(src)),
        LD16(dst, src) => (sl16(dst), sl16(src)),
    }
}
