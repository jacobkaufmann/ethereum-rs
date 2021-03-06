/// An EVM instruction.
#[derive(Debug)]
enum Instruction {
    // Stop and arithmetic operations.
    Stop,
    Add,
    Mul,
    Sub,
    Div,
    SDiv,
    Mod,
    SMod,
    AddMod,
    MulMod,
    Exp,
    SignExtend,
    // Comparison and bitwise logic operations.
    Lt,
    Gt,
    SLt,
    SGt,
    Eq,
    IsZero,
    And,
    Or,
    Xor,
    Not,
    Byte,
    ShL,
    ShR,
    SAR,
    // Keccak 256.
    Keccak256,
    // Environmental information.
    Address,
    Balance,
    Origin,
    Caller,
    CallValue,
    CallDataLoad,
    CallDataSize,
    CallDataCopy,
    CodeSize,
    CodeCopy,
    GasPrice,
    ExtCodeSize,
    ExtCodeCopy,
    ReturnDataSize,
    ReturnDataCopy,
    ExtCodeHash,
    // Block information.
    BlockHash,
    Coinbase,
    Timestamp,
    Number,
    Difficulty,
    GasLimit,
    ChainId,
    SelfBalance,
    // Stack, memory, storage, and flow.
    Pop,
    MLoad,
    MStore,
    MStore8,
    SLoad,
    SStore,
    Jump,
    JumpI,
    Pc,
    MSize,
    Gas,
    JumpDest,
    // Push operations.
    Push1,
    Push2,
    Push3,
    Push4,
    Push5,
    Push6,
    Push7,
    Push8,
    Push9,
    Push10,
    Push11,
    Push12,
    Push13,
    Push14,
    Push15,
    Push16,
    Push17,
    Push18,
    Push19,
    Push20,
    Push21,
    Push22,
    Push23,
    Push24,
    Push25,
    Push26,
    Push27,
    Push28,
    Push29,
    Push30,
    Push31,
    Push32,
    // Duplication operations.
    Dup1,
    Dup2,
    Dup3,
    Dup4,
    Dup5,
    Dup6,
    Dup7,
    Dup8,
    Dup9,
    Dup10,
    Dup11,
    Dup12,
    Dup13,
    Dup14,
    Dup15,
    Dup16,
    // Exchange operations.
    Swap1,
    Swap2,
    Swap3,
    Swap4,
    Swap5,
    Swap6,
    Swap7,
    Swap8,
    Swap9,
    Swap10,
    Swap11,
    Swap12,
    Swap13,
    Swap14,
    Swap15,
    Swap16,
    // Logging operations.
    Log0,
    Log1,
    Log2,
    Log3,
    Log4,
    // System operations.
    Create,
    Call,
    CallCode,
    Return,
    DelegateCall,
    Create2,
    StaticCall,
    Revert,
    Invalid,
    SelfDestruct,
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        use Instruction::*;
        match value {
            0x00 => Stop,
            0x01 => Add,
            0x02 => Mul,
            0x03 => Sub,
            0x04 => Div,
            0x05 => SDiv,
            0x06 => Mod,
            0x07 => SMod,
            0x08 => AddMod,
            0x09 => MulMod,
            0x0a => Exp,
            0x0b => SignExtend,
            0x10 => Lt,
            0x11 => SLt,
            0x12 => Gt,
            0x13 => SGt,
            0x14 => Eq,
            0x15 => IsZero,
            0x16 => And,
            0x17 => Or,
            0x18 => Xor,
            0x19 => Not,
            0x1a => Byte,
            0x1b => ShL,
            0x1c => ShR,
            0x1d => SAR,
            0x20 => Keccak256,
            0x30 => Address,
            0x31 => Balance,
            0x32 => Origin,
            0x33 => Caller,
            0x34 => CallValue,
            0x35 => CallDataLoad,
            0x36 => CallDataSize,
            0x37 => CallDataCopy,
            0x38 => CodeSize,
            0x39 => CodeCopy,
            0x3a => GasPrice,
            0x3b => ExtCodeSize,
            0x3c => ExtCodeCopy,
            0x3d => ReturnDataSize,
            0x3e => ReturnDataCopy,
            0x3f => ExtCodeHash,
            0x40 => BlockHash,
            0x41 => Coinbase,
            0x42 => Timestamp,
            0x43 => Number,
            0x44 => Difficulty,
            0x45 => GasLimit,
            0x46 => ChainId,
            0x47 => SelfBalance,
            0x50 => Pop,
            0x51 => MLoad,
            0x52 => MStore,
            0x53 => MStore8,
            0x54 => SLoad,
            0x55 => SStore,
            0x56 => Jump,
            0x57 => JumpI,
            0x58 => Pc,
            0x59 => MSize,
            0x5a => Gas,
            0x5b => JumpDest,
            0x60 => Push1,
            0x61 => Push2,
            0x62 => Push3,
            0x63 => Push4,
            0x64 => Push5,
            0x65 => Push6,
            0x66 => Push7,
            0x67 => Push8,
            0x68 => Push9,
            0x69 => Push10,
            0x6a => Push11,
            0x6b => Push12,
            0x6c => Push13,
            0x6d => Push14,
            0x6e => Push15,
            0x6f => Push16,
            0x70 => Push17,
            0x71 => Push18,
            0x72 => Push19,
            0x73 => Push20,
            0x74 => Push21,
            0x75 => Push22,
            0x76 => Push23,
            0x77 => Push24,
            0x78 => Push25,
            0x79 => Push26,
            0x7a => Push27,
            0x7b => Push28,
            0x7c => Push29,
            0x7d => Push30,
            0x7e => Push31,
            0x7f => Push32,
            0x80 => Dup1,
            0x81 => Dup2,
            0x82 => Dup3,
            0x83 => Dup4,
            0x84 => Dup5,
            0x85 => Dup6,
            0x86 => Dup7,
            0x87 => Dup8,
            0x88 => Dup9,
            0x89 => Dup10,
            0x8a => Dup11,
            0x8b => Dup12,
            0x8c => Dup13,
            0x8d => Dup14,
            0x8e => Dup15,
            0x8f => Dup16,
            0x90 => Swap1,
            0x91 => Swap2,
            0x92 => Swap3,
            0x93 => Swap4,
            0x94 => Swap5,
            0x95 => Swap6,
            0x96 => Swap7,
            0x97 => Swap8,
            0x98 => Swap9,
            0x99 => Swap10,
            0x9a => Swap11,
            0x9b => Swap12,
            0x9c => Swap13,
            0x9d => Swap14,
            0x9e => Swap15,
            0x9f => Swap16,
            0xa0 => Log0,
            0xa1 => Log1,
            0xa2 => Log2,
            0xa3 => Log3,
            0xa4 => Log4,
            0xf0 => Create,
            0xf1 => Call,
            0xf2 => CallCode,
            0xf3 => Return,
            0xf4 => DelegateCall,
            0xf5 => Create2,
            0xfa => StaticCall,
            0xfd => Revert,
            0xfe => Invalid,
            0xff => SelfDestruct,
            _ => Invalid,
        }
    }
}

impl Instruction {
    /// Returns a tuple containing the number of items removed from the stack (delta) and the
    /// number of items added to the stack (alpha) for the instruction.
    fn delta_alpha(&self) -> (u8, u8) {
        use Instruction::*;
        match self {
            Stop => (0, 0),
            Add => (2, 1),
            Mul => (2, 1),
            Sub => (2, 1),
            Div => (2, 1),
            SDiv => (2, 1),
            Mod => (2, 1),
            SMod => (2, 1),
            AddMod => (3, 1),
            MulMod => (3, 1),
            Exp => (2, 1),
            SignExtend => (2, 1),
            Lt => (2, 1),
            Gt => (2, 1),
            SLt => (2, 1),
            SGt => (2, 1),
            Eq => (2, 1),
            IsZero => (1, 1),
            And => (2, 1),
            Or => (2, 1),
            Xor => (2, 1),
            Not => (1, 1),
            Byte => (2, 1),
            ShL => (2, 1),
            ShR => (2, 1),
            SAR => (2, 1),
            Keccak256 => (2, 1),
            Address => (0, 1),
            Balance => (1, 1),
            Origin => (0, 1),
            Caller => (0, 1),
            CallValue => (0, 1),
            CallDataLoad => (1, 1),
            CallDataSize => (0, 1),
            CallDataCopy => (3, 0),
            CodeSize => (0, 1),
            CodeCopy => (3, 0),
            GasPrice => (0, 1),
            ExtCodeSize => (1, 1),
            ExtCodeCopy => (4, 0),
            ReturnDataSize => (0, 1),
            ReturnDataCopy => (3, 0),
            ExtCodeHash => (1, 1),
            BlockHash => (1, 1),
            Coinbase => (0, 1),
            Timestamp => (0, 1),
            Number => (0, 1),
            Difficulty => (0, 1),
            GasLimit => (0, 1),
            ChainId => (0, 1),
            SelfBalance => (0, 1),
            Pop => (1, 0),
            MLoad => (1, 1),
            MStore => (2, 0),
            MStore8 => (2, 0),
            SLoad => (1, 1),
            SStore => (2, 0),
            Jump => (1, 0),
            JumpI => (2, 0),
            Pc => (0, 1),
            MSize => (0, 1),
            Gas => (0, 1),
            JumpDest => (0, 0),
            Push1 | Push2 | Push3 | Push4 | Push5 | Push6 | Push7 | Push8 | Push9 | Push10
            | Push11 | Push12 | Push13 | Push14 | Push15 | Push16 | Push17 | Push18 | Push19
            | Push20 | Push21 | Push22 | Push23 | Push24 | Push25 | Push26 | Push27 | Push28
            | Push29 | Push30 | Push31 | Push32 => (0, 1),
            Dup1 => (1, 2),
            Dup2 => (2, 3),
            Dup3 => (3, 4),
            Dup4 => (4, 5),
            Dup5 => (5, 6),
            Dup6 => (6, 7),
            Dup7 => (7, 8),
            Dup8 => (8, 9),
            Dup9 => (9, 10),
            Dup10 => (10, 11),
            Dup11 => (11, 12),
            Dup12 => (12, 13),
            Dup13 => (13, 14),
            Dup14 => (14, 15),
            Dup15 => (15, 16),
            Dup16 => (16, 17),
            Swap1 => (2, 2),
            Swap2 => (3, 3),
            Swap3 => (4, 4),
            Swap4 => (5, 5),
            Swap5 => (6, 6),
            Swap6 => (7, 7),
            Swap7 => (8, 8),
            Swap8 => (9, 9),
            Swap9 => (10, 10),
            Swap10 => (11, 11),
            Swap11 => (12, 12),
            Swap12 => (13, 13),
            Swap13 => (14, 14),
            Swap14 => (15, 15),
            Swap15 => (16, 16),
            Swap16 => (17, 17),
            Log0 => (2, 0),
            Log1 => (3, 0),
            Log2 => (4, 0),
            Log3 => (5, 0),
            Log4 => (6, 0),
            Create => (3, 1),
            Call => (7, 1),
            CallCode => (7, 1),
            Return => (2, 0),
            DelegateCall => (6, 1),
            Create2 => (4, 1),
            StaticCall => (6, 1),
            Revert => (2, 0),
            // Technically, delta and alpha are undefined for the invalid instruction.
            Invalid => (0, 0),
            SelfDestruct => (1, 0),
        }
    }
}
