// For fast decoding and cache friendly, RISC-V instruction is decoded
// into 64 bit unsigned integer in the following format:
//
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |           | rs2 | rs1 | flg | op2 | rd  | op  | R-type
// +-----+-----+-----+-----+-----+-----+-----+-----+
// |     | rs3 | rs2 | rs1 | flg | op2 | rd  | op  | R4-type
// +-----------+-----------------------------------+
// | rs4 | rs3 | rs2 | rs1 | flg | op2 | rd  | op  | R5-type
// +-----------+-----------------------------------+
// |    immediate    | rs1 | flg | op2 | rd  | op  | I-type
// +-----------------------------------------------+
// |    immediate    | rs1 | flg | op2 | rs2 | op  | S-type/B-type
// +-----------------+-----------------------------+
// |       immediate       | flg | op2 | rd  | op  | U-type/J-type
// +-----+-----+-----+-----+-----+-----+-----+-----+
//
// +flg+ here means a combination of flags, Its format is as follows:
//
// +---+---+---+---+---+---+---+---+
// | 7 | 6 | 5 | 4 | length >> 1   |
// +---+---+---+---+---+---+---+---+
//
// This way each op and register index are in full byte, accessing them
// will be much faster than the original compact form. Hence we will have
// a fast path where the interpreter loop reads instruction directly in this
// format, and a slow path where a full featured decoder decodes RISC-V
// instruction into the internal form here(much like how traces/micro-ops work.)
//
// About +op+ and +op2+:
// When the op value is 0x10-0xff, it expresses a first-level instruction under fast
// path, at this time the value of op2 is ignored.
// When the op value is 0x00-0x0f, op and op2 are combined to express a
// second-level instruction under slow path.
pub type Instruction = u64;

pub type InstructionOpcode = u16;

// IMC
pub const OP_UNLOADED: InstructionOpcode = 0x10;
pub const OP_ADD: InstructionOpcode = 0x11;
pub const OP_ADDI: InstructionOpcode = 0x12;
pub const OP_ADDIW: InstructionOpcode = 0x13;
pub const OP_ADDW: InstructionOpcode = 0x14;
pub const OP_AND: InstructionOpcode = 0x15;
pub const OP_ANDI: InstructionOpcode = 0x16;
pub const OP_AUIPC: InstructionOpcode = 0x17;
pub const OP_BEQ: InstructionOpcode = 0x18;
pub const OP_BGE: InstructionOpcode = 0x19;
pub const OP_BGEU: InstructionOpcode = 0x1a;
pub const OP_BLT: InstructionOpcode = 0x1b;
pub const OP_BLTU: InstructionOpcode = 0x1c;
pub const OP_BNE: InstructionOpcode = 0x1d;
pub const OP_DIV: InstructionOpcode = 0x1e;
pub const OP_DIVU: InstructionOpcode = 0x1f;
pub const OP_DIVUW: InstructionOpcode = 0x20;
pub const OP_DIVW: InstructionOpcode = 0x21;
pub const OP_EBREAK: InstructionOpcode = 0x22;
pub const OP_ECALL: InstructionOpcode = 0x23;
pub const OP_FENCE: InstructionOpcode = 0x24;
pub const OP_FENCEI: InstructionOpcode = 0x25;
pub const OP_JAL: InstructionOpcode = 0x26;
pub const OP_JALR_VERSION0: InstructionOpcode = 0x27;
pub const OP_JALR_VERSION1: InstructionOpcode = 0x28;
pub const OP_LB_VERSION0: InstructionOpcode = 0x29;
pub const OP_LB_VERSION1: InstructionOpcode = 0x2a;
pub const OP_LBU_VERSION0: InstructionOpcode = 0x2b;
pub const OP_LBU_VERSION1: InstructionOpcode = 0x2c;
pub const OP_LD_VERSION0: InstructionOpcode = 0x2d;
pub const OP_LD_VERSION1: InstructionOpcode = 0x2e;
pub const OP_LH_VERSION0: InstructionOpcode = 0x2f;
pub const OP_LH_VERSION1: InstructionOpcode = 0x30;
pub const OP_LHU_VERSION0: InstructionOpcode = 0x31;
pub const OP_LHU_VERSION1: InstructionOpcode = 0x32;
pub const OP_LUI: InstructionOpcode = 0x33;
pub const OP_LW_VERSION0: InstructionOpcode = 0x34;
pub const OP_LW_VERSION1: InstructionOpcode = 0x35;
pub const OP_LWU_VERSION0: InstructionOpcode = 0x36;
pub const OP_LWU_VERSION1: InstructionOpcode = 0x37;
pub const OP_MUL: InstructionOpcode = 0x38;
pub const OP_MULH: InstructionOpcode = 0x39;
pub const OP_MULHSU: InstructionOpcode = 0x3a;
pub const OP_MULHU: InstructionOpcode = 0x3b;
pub const OP_MULW: InstructionOpcode = 0x3c;
pub const OP_OR: InstructionOpcode = 0x3d;
pub const OP_ORI: InstructionOpcode = 0x3e;
pub const OP_REM: InstructionOpcode = 0x3f;
pub const OP_REMU: InstructionOpcode = 0x40;
pub const OP_REMUW: InstructionOpcode = 0x41;
pub const OP_REMW: InstructionOpcode = 0x42;
pub const OP_SB: InstructionOpcode = 0x43;
pub const OP_SD: InstructionOpcode = 0x44;
pub const OP_SH: InstructionOpcode = 0x45;
pub const OP_SLL: InstructionOpcode = 0x46;
pub const OP_SLLI: InstructionOpcode = 0x47;
pub const OP_SLLIW: InstructionOpcode = 0x48;
pub const OP_SLLW: InstructionOpcode = 0x49;
pub const OP_SLT: InstructionOpcode = 0x4a;
pub const OP_SLTI: InstructionOpcode = 0x4b;
pub const OP_SLTIU: InstructionOpcode = 0x4c;
pub const OP_SLTU: InstructionOpcode = 0x4d;
pub const OP_SRA: InstructionOpcode = 0x4e;
pub const OP_SRAI: InstructionOpcode = 0x4f;
pub const OP_SRAIW: InstructionOpcode = 0x50;
pub const OP_SRAW: InstructionOpcode = 0x51;
pub const OP_SRL: InstructionOpcode = 0x52;
pub const OP_SRLI: InstructionOpcode = 0x53;
pub const OP_SRLIW: InstructionOpcode = 0x54;
pub const OP_SRLW: InstructionOpcode = 0x55;
pub const OP_SUB: InstructionOpcode = 0x56;
pub const OP_SUBW: InstructionOpcode = 0x57;
pub const OP_SW: InstructionOpcode = 0x58;
pub const OP_XOR: InstructionOpcode = 0x59;
pub const OP_XORI: InstructionOpcode = 0x5a;
// A
pub const OP_LR_W: InstructionOpcode = 0x5b;
pub const OP_SC_W: InstructionOpcode = 0x5c;
pub const OP_AMOSWAP_W: InstructionOpcode = 0x5d;
pub const OP_AMOADD_W: InstructionOpcode = 0x5e;
pub const OP_AMOXOR_W: InstructionOpcode = 0x5f;
pub const OP_AMOAND_W: InstructionOpcode = 0x60;
pub const OP_AMOOR_W: InstructionOpcode = 0x61;
pub const OP_AMOMIN_W: InstructionOpcode = 0x62;
pub const OP_AMOMAX_W: InstructionOpcode = 0x63;
pub const OP_AMOMINU_W: InstructionOpcode = 0x64;
pub const OP_AMOMAXU_W: InstructionOpcode = 0x65;
pub const OP_LR_D: InstructionOpcode = 0x66;
pub const OP_SC_D: InstructionOpcode = 0x67;
pub const OP_AMOSWAP_D: InstructionOpcode = 0x68;
pub const OP_AMOADD_D: InstructionOpcode = 0x69;
pub const OP_AMOXOR_D: InstructionOpcode = 0x6a;
pub const OP_AMOAND_D: InstructionOpcode = 0x6b;
pub const OP_AMOOR_D: InstructionOpcode = 0x6c;
pub const OP_AMOMIN_D: InstructionOpcode = 0x6d;
pub const OP_AMOMAX_D: InstructionOpcode = 0x6e;
pub const OP_AMOMINU_D: InstructionOpcode = 0x6f;
pub const OP_AMOMAXU_D: InstructionOpcode = 0x70;
// B
pub const OP_ADDUW: InstructionOpcode = 0x71;
pub const OP_ANDN: InstructionOpcode = 0x72;
pub const OP_BCLR: InstructionOpcode = 0x73;
pub const OP_BCLRI: InstructionOpcode = 0x74;
pub const OP_BEXT: InstructionOpcode = 0x75;
pub const OP_BEXTI: InstructionOpcode = 0x76;
pub const OP_BINV: InstructionOpcode = 0x77;
pub const OP_BINVI: InstructionOpcode = 0x78;
pub const OP_BSET: InstructionOpcode = 0x79;
pub const OP_BSETI: InstructionOpcode = 0x7a;
pub const OP_CLMUL: InstructionOpcode = 0x7b;
pub const OP_CLMULH: InstructionOpcode = 0x7c;
pub const OP_CLMULR: InstructionOpcode = 0x7d;
pub const OP_CLZ: InstructionOpcode = 0x7e;
pub const OP_CLZW: InstructionOpcode = 0x7f;
pub const OP_CPOP: InstructionOpcode = 0x80;
pub const OP_CPOPW: InstructionOpcode = 0x81;
pub const OP_CTZ: InstructionOpcode = 0x82;
pub const OP_CTZW: InstructionOpcode = 0x83;
pub const OP_MAX: InstructionOpcode = 0x84;
pub const OP_MAXU: InstructionOpcode = 0x85;
pub const OP_MIN: InstructionOpcode = 0x86;
pub const OP_MINU: InstructionOpcode = 0x87;
pub const OP_ORCB: InstructionOpcode = 0x88;
pub const OP_ORN: InstructionOpcode = 0x89;
pub const OP_REV8: InstructionOpcode = 0x8a;
pub const OP_ROL: InstructionOpcode = 0x8b;
pub const OP_ROLW: InstructionOpcode = 0x8c;
pub const OP_ROR: InstructionOpcode = 0x8d;
pub const OP_RORI: InstructionOpcode = 0x8e;
pub const OP_RORIW: InstructionOpcode = 0x8f;
pub const OP_RORW: InstructionOpcode = 0x90;
pub const OP_SEXTB: InstructionOpcode = 0x91;
pub const OP_SEXTH: InstructionOpcode = 0x92;
pub const OP_SH1ADD: InstructionOpcode = 0x93;
pub const OP_SH1ADDUW: InstructionOpcode = 0x94;
pub const OP_SH2ADD: InstructionOpcode = 0x95;
pub const OP_SH2ADDUW: InstructionOpcode = 0x96;
pub const OP_SH3ADD: InstructionOpcode = 0x97;
pub const OP_SH3ADDUW: InstructionOpcode = 0x98;
pub const OP_SLLIUW: InstructionOpcode = 0x99;
pub const OP_XNOR: InstructionOpcode = 0x9a;
pub const OP_ZEXTH: InstructionOpcode = 0x9b;
// Mop
pub const OP_WIDE_MUL: InstructionOpcode = 0x9c;
pub const OP_WIDE_MULU: InstructionOpcode = 0x9d;
pub const OP_WIDE_MULSU: InstructionOpcode = 0x9e;
pub const OP_WIDE_DIV: InstructionOpcode = 0x9f;
pub const OP_WIDE_DIVU: InstructionOpcode = 0xa0;
pub const OP_FAR_JUMP_REL: InstructionOpcode = 0xa1;
pub const OP_FAR_JUMP_ABS: InstructionOpcode = 0xa2;
pub const OP_ADC: InstructionOpcode = 0xa3;
pub const OP_SBB: InstructionOpcode = 0xa4;
pub const OP_ADCS: InstructionOpcode = 0xa5;
pub const OP_SBBS: InstructionOpcode = 0xa6;
pub const OP_ADD3A: InstructionOpcode = 0xa7;
pub const OP_ADD3B: InstructionOpcode = 0xa8;
pub const OP_ADD3C: InstructionOpcode = 0xa9;
pub const OP_CUSTOM_LOAD_UIMM: InstructionOpcode = 0xaa;
pub const OP_CUSTOM_LOAD_IMM: InstructionOpcode = 0xab;
pub const OP_CUSTOM_TRACE_END: InstructionOpcode = 0xac;

pub const MINIMAL_OPCODE: InstructionOpcode = OP_UNLOADED;
pub const MAXIMUM_OPCODE: InstructionOpcode = OP_CUSTOM_TRACE_END;

pub const INSTRUCTION_OPCODE_NAMES: [&str; (MAXIMUM_OPCODE - MINIMAL_OPCODE + 1) as usize] = [
    "UNLOADED",
    "ADD",
    "ADDI",
    "ADDIW",
    "ADDW",
    "AND",
    "ANDI",
    "AUIPC",
    "BEQ",
    "BGE",
    "BGEU",
    "BLT",
    "BLTU",
    "BNE",
    "DIV",
    "DIVU",
    "DIVUW",
    "DIVW",
    "EBREAK",
    "ECALL",
    "FENCE",
    "FENCEI",
    "JAL",
    "JALR_VERSION0",
    "JALR_VERSION1",
    "LB_VERSION0",
    "LB_VERSION1",
    "LBU_VERSION0",
    "LBU_VERSION1",
    "LD_VERSION0",
    "LD_VERSION1",
    "LH_VERSION0",
    "LH_VERSION1",
    "LHU_VERSION0",
    "LHU_VERSION1",
    "LUI",
    "LW_VERSION0",
    "LW_VERSION1",
    "LWU_VERSION0",
    "LWU_VERSION1",
    "MUL",
    "MULH",
    "MULHSU",
    "MULHU",
    "MULW",
    "OR",
    "ORI",
    "REM",
    "REMU",
    "REMUW",
    "REMW",
    "SB",
    "SD",
    "SH",
    "SLL",
    "SLLI",
    "SLLIW",
    "SLLW",
    "SLT",
    "SLTI",
    "SLTIU",
    "SLTU",
    "SRA",
    "SRAI",
    "SRAIW",
    "SRAW",
    "SRL",
    "SRLI",
    "SRLIW",
    "SRLW",
    "SUB",
    "SUBW",
    "SW",
    "XOR",
    "XORI",
    "LR_W",
    "SC_W",
    "AMOSWAP_W",
    "AMOADD_W",
    "AMOXOR_W",
    "AMOAND_W",
    "AMOOR_W",
    "AMOMIN_W",
    "AMOMAX_W",
    "AMOMINU_W",
    "AMOMAXU_W",
    "LR_D",
    "SC_D",
    "AMOSWAP_D",
    "AMOADD_D",
    "AMOXOR_D",
    "AMOAND_D",
    "AMOOR_D",
    "AMOMIN_D",
    "AMOMAX_D",
    "AMOMINU_D",
    "AMOMAXU_D",
    "ADDUW",
    "ANDN",
    "BCLR",
    "BCLRI",
    "BEXT",
    "BEXTI",
    "BINV",
    "BINVI",
    "BSET",
    "BSETI",
    "CLMUL",
    "CLMULH",
    "CLMULR",
    "CLZ",
    "CLZW",
    "CPOP",
    "CPOPW",
    "CTZ",
    "CTZW",
    "MAX",
    "MAXU",
    "MIN",
    "MINU",
    "ORCB",
    "ORN",
    "REV8",
    "ROL",
    "ROLW",
    "ROR",
    "RORI",
    "RORIW",
    "RORW",
    "SEXTB",
    "SEXTH",
    "SH1ADD",
    "SH1ADDUW",
    "SH2ADD",
    "SH2ADDUW",
    "SH3ADD",
    "SH3ADDUW",
    "SLLIUW",
    "XNOR",
    "ZEXTH",
    "WIDE_MUL",
    "WIDE_MULU",
    "WIDE_MULSU",
    "WIDE_DIV",
    "WIDE_DIVU",
    "FAR_JUMP_REL",
    "FAR_JUMP_ABS",
    "ADC",
    "SBB",
    "ADCS",
    "SBBS",
    "ADD3A",
    "ADD3B",
    "ADD3C",
    "CUSTOM_LOAD_UIMM",
    "CUSTOM_LOAD_IMM",
    "CUSTOM_TRACE_END",
];

pub fn instruction_opcode_name(i: InstructionOpcode) -> &'static str {
    INSTRUCTION_OPCODE_NAMES[(i - MINIMAL_OPCODE) as usize]
}
