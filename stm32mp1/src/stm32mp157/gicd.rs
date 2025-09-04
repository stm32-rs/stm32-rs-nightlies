#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctlr: CTLR,
    typer: TYPER,
    iidr: IIDR,
    _reserved3: [u8; 0x74],
    igroupr0: IGROUPR0,
    igroupr1: IGROUPR1,
    igroupr2: IGROUPR2,
    igroupr3: IGROUPR3,
    igroupr4: IGROUPR4,
    igroupr5: IGROUPR5,
    igroupr6: IGROUPR6,
    igroupr7: IGROUPR7,
    igroupr8: IGROUPR8,
    _reserved12: [u8; 0x5c],
    isenabler0: ISENABLER0,
    isenabler1: ISENABLER1,
    isenabler2: ISENABLER2,
    isenabler3: ISENABLER3,
    isenabler4: ISENABLER4,
    isenabler5: ISENABLER5,
    isenabler6: ISENABLER6,
    isenabler7: ISENABLER7,
    isenabler8: ISENABLER8,
    _reserved21: [u8; 0x5c],
    icenabler0: ICENABLER0,
    icenabler1: ICENABLER1,
    icenabler2: ICENABLER2,
    icenabler3: ICENABLER3,
    icenabler4: ICENABLER4,
    icenabler5: ICENABLER5,
    icenabler6: ICENABLER6,
    icenabler7: ICENABLER7,
    icenabler8: ICENABLER8,
    _reserved30: [u8; 0x5c],
    ispendr0: ISPENDR0,
    ispendr1: ISPENDR1,
    ispendr2: ISPENDR2,
    ispendr3: ISPENDR3,
    ispendr4: ISPENDR4,
    ispendr5: ISPENDR5,
    ispendr6: ISPENDR6,
    ispendr7: ISPENDR7,
    ispendr8: ISPENDR8,
    _reserved39: [u8; 0x5c],
    icpendr0: ICPENDR0,
    icpendr1: ICPENDR1,
    icpendr2: ICPENDR2,
    icpendr3: ICPENDR3,
    icpendr4: ICPENDR4,
    icpendr5: ICPENDR5,
    icpendr6: ICPENDR6,
    icpendr7: ICPENDR7,
    icpendr8: ICPENDR8,
    _reserved48: [u8; 0x5c],
    isactiver0: ISACTIVER0,
    isactiver1: ISACTIVER1,
    isactiver2: ISACTIVER2,
    isactiver3: ISACTIVER3,
    isactiver4: ISACTIVER4,
    isactiver5: ISACTIVER5,
    isactiver6: ISACTIVER6,
    isactiver7: ISACTIVER7,
    isactiver8: ISACTIVER8,
    _reserved57: [u8; 0x5c],
    icactiver0: ICACTIVER0,
    icactiver1: ICACTIVER1,
    icactiver2: ICACTIVER2,
    icactiver3: ICACTIVER3,
    icactiver4: ICACTIVER4,
    icactiver5: ICACTIVER5,
    icactiver6: ICACTIVER6,
    icactiver7: ICACTIVER7,
    icactiver8: ICACTIVER8,
    _reserved66: [u8; 0x5c],
    ipriorityr0: IPRIORITYR0,
    ipriorityr1: IPRIORITYR1,
    ipriorityr2: IPRIORITYR2,
    ipriorityr3: IPRIORITYR3,
    ipriorityr4: IPRIORITYR4,
    ipriorityr5: IPRIORITYR5,
    ipriorityr6: IPRIORITYR6,
    ipriorityr7: IPRIORITYR7,
    ipriorityr8: IPRIORITYR8,
    ipriorityr9: IPRIORITYR9,
    ipriorityr10: IPRIORITYR10,
    ipriorityr11: IPRIORITYR11,
    ipriorityr12: IPRIORITYR12,
    ipriorityr13: IPRIORITYR13,
    ipriorityr14: IPRIORITYR14,
    ipriorityr15: IPRIORITYR15,
    ipriorityr16: IPRIORITYR16,
    ipriorityr17: IPRIORITYR17,
    ipriorityr18: IPRIORITYR18,
    ipriorityr19: IPRIORITYR19,
    ipriorityr20: IPRIORITYR20,
    ipriorityr21: IPRIORITYR21,
    ipriorityr22: IPRIORITYR22,
    ipriorityr23: IPRIORITYR23,
    ipriorityr24: IPRIORITYR24,
    ipriorityr25: IPRIORITYR25,
    ipriorityr26: IPRIORITYR26,
    ipriorityr27: IPRIORITYR27,
    ipriorityr28: IPRIORITYR28,
    ipriorityr29: IPRIORITYR29,
    ipriorityr30: IPRIORITYR30,
    ipriorityr31: IPRIORITYR31,
    ipriorityr32: IPRIORITYR32,
    ipriorityr33: IPRIORITYR33,
    ipriorityr34: IPRIORITYR34,
    ipriorityr35: IPRIORITYR35,
    ipriorityr36: IPRIORITYR36,
    ipriorityr37: IPRIORITYR37,
    ipriorityr38: IPRIORITYR38,
    ipriorityr39: IPRIORITYR39,
    ipriorityr40: IPRIORITYR40,
    ipriorityr41: IPRIORITYR41,
    ipriorityr42: IPRIORITYR42,
    ipriorityr43: IPRIORITYR43,
    ipriorityr44: IPRIORITYR44,
    ipriorityr45: IPRIORITYR45,
    ipriorityr46: IPRIORITYR46,
    ipriorityr47: IPRIORITYR47,
    ipriorityr48: IPRIORITYR48,
    ipriorityr49: IPRIORITYR49,
    ipriorityr50: IPRIORITYR50,
    ipriorityr51: IPRIORITYR51,
    ipriorityr52: IPRIORITYR52,
    ipriorityr53: IPRIORITYR53,
    ipriorityr54: IPRIORITYR54,
    ipriorityr55: IPRIORITYR55,
    ipriorityr56: IPRIORITYR56,
    ipriorityr57: IPRIORITYR57,
    ipriorityr58: IPRIORITYR58,
    ipriorityr59: IPRIORITYR59,
    ipriorityr60: IPRIORITYR60,
    ipriorityr61: IPRIORITYR61,
    ipriorityr62: IPRIORITYR62,
    ipriorityr63: IPRIORITYR63,
    ipriorityr64: IPRIORITYR64,
    ipriorityr65: IPRIORITYR65,
    ipriorityr66: IPRIORITYR66,
    ipriorityr67: IPRIORITYR67,
    ipriorityr68: IPRIORITYR68,
    ipriorityr69: IPRIORITYR69,
    ipriorityr70: IPRIORITYR70,
    ipriorityr71: IPRIORITYR71,
    _reserved138: [u8; 0x02e0],
    itargetsr0: ITARGETSR0,
    itargetsr1: ITARGETSR1,
    itargetsr2: ITARGETSR2,
    itargetsr3: ITARGETSR3,
    itargetsr4: ITARGETSR4,
    itargetsr5: ITARGETSR5,
    itargetsr6: ITARGETSR6,
    itargetsr7: ITARGETSR7,
    itargetsr8: ITARGETSR8,
    itargetsr9: ITARGETSR9,
    itargetsr10: ITARGETSR10,
    itargetsr11: ITARGETSR11,
    itargetsr12: ITARGETSR12,
    itargetsr13: ITARGETSR13,
    itargetsr14: ITARGETSR14,
    itargetsr15: ITARGETSR15,
    itargetsr16: ITARGETSR16,
    itargetsr17: ITARGETSR17,
    itargetsr18: ITARGETSR18,
    itargetsr19: ITARGETSR19,
    itargetsr20: ITARGETSR20,
    itargetsr21: ITARGETSR21,
    itargetsr22: ITARGETSR22,
    itargetsr23: ITARGETSR23,
    itargetsr24: ITARGETSR24,
    itargetsr25: ITARGETSR25,
    itargetsr26: ITARGETSR26,
    itargetsr27: ITARGETSR27,
    itargetsr28: ITARGETSR28,
    itargetsr29: ITARGETSR29,
    itargetsr30: ITARGETSR30,
    itargetsr31: ITARGETSR31,
    itargetsr32: ITARGETSR32,
    itargetsr33: ITARGETSR33,
    itargetsr34: ITARGETSR34,
    itargetsr35: ITARGETSR35,
    itargetsr36: ITARGETSR36,
    itargetsr37: ITARGETSR37,
    itargetsr38: ITARGETSR38,
    itargetsr39: ITARGETSR39,
    itargetsr40: ITARGETSR40,
    itargetsr41: ITARGETSR41,
    itargetsr42: ITARGETSR42,
    itargetsr43: ITARGETSR43,
    itargetsr44: ITARGETSR44,
    itargetsr45: ITARGETSR45,
    itargetsr46: ITARGETSR46,
    itargetsr47: ITARGETSR47,
    itargetsr48: ITARGETSR48,
    itargetsr49: ITARGETSR49,
    itargetsr50: ITARGETSR50,
    itargetsr51: ITARGETSR51,
    itargetsr52: ITARGETSR52,
    itargetsr53: ITARGETSR53,
    itargetsr54: ITARGETSR54,
    itargetsr55: ITARGETSR55,
    itargetsr56: ITARGETSR56,
    itargetsr57: ITARGETSR57,
    itargetsr58: ITARGETSR58,
    itargetsr59: ITARGETSR59,
    itargetsr60: ITARGETSR60,
    itargetsr61: ITARGETSR61,
    itargetsr62: ITARGETSR62,
    itargetsr63: ITARGETSR63,
    itargetsr64: ITARGETSR64,
    itargetsr65: ITARGETSR65,
    itargetsr66: ITARGETSR66,
    itargetsr67: ITARGETSR67,
    itargetsr68: ITARGETSR68,
    itargetsr69: ITARGETSR69,
    itargetsr70: ITARGETSR70,
    itargetsr71: ITARGETSR71,
    _reserved210: [u8; 0x02e0],
    icfgr0: ICFGR0,
    icfgr1: ICFGR1,
    icfgr2: ICFGR2,
    icfgr3: ICFGR3,
    icfgr4: ICFGR4,
    icfgr5: ICFGR5,
    icfgr6: ICFGR6,
    icfgr7: ICFGR7,
    icfgr8: ICFGR8,
    icfgr9: ICFGR9,
    icfgr10: ICFGR10,
    icfgr11: ICFGR11,
    icfgr12: ICFGR12,
    icfgr13: ICFGR13,
    icfgr14: ICFGR14,
    icfgr15: ICFGR15,
    icfgr16: ICFGR16,
    icfgr17: ICFGR17,
    _reserved228: [u8; 0xb8],
    ppisr: PPISR,
    _reserved229: [u8; 0x04],
    spisr1: SPISR1,
    spisr2: SPISR2,
    spisr3: SPISR3,
    spisr4: SPISR4,
    spisr5: SPISR5,
    spisr6: SPISR6,
    spisr7: SPISR7,
    _reserved236: [u8; 0x01dc],
    sgir: SGIR,
    _reserved237: [u8; 0x0c],
    cpendsgir0: CPENDSGIR0,
    cpendsgir1: CPENDSGIR1,
    cpendsgir2: CPENDSGIR2,
    cpendsgir3: CPENDSGIR3,
    spendsgir0: SPENDSGIR0,
    spendsgir1: SPENDSGIR1,
    spendsgir2: SPENDSGIR2,
    spendsgir3: SPENDSGIR3,
    _reserved245: [u8; 0xa0],
    pidr4: PIDR4,
    pidr5: PIDR5,
    pidr6: PIDR6,
    pidr7: PIDR7,
    pidr0: PIDR0,
    pidr1: PIDR1,
    pidr2: PIDR2,
    pidr3: PIDR3,
    cidr0: CIDR0,
    cidr1: CIDR1,
    cidr2: CIDR2,
    cidr3: CIDR3,
}
impl RegisterBlock {
    ///0x00 - GICD control register
    #[inline(always)]
    pub const fn ctlr(&self) -> &CTLR {
        &self.ctlr
    }
    ///0x04 - GICD interrupt controller type register
    #[inline(always)]
    pub const fn typer(&self) -> &TYPER {
        &self.typer
    }
    ///0x08 - GICD implementer identification register
    #[inline(always)]
    pub const fn iidr(&self) -> &IIDR {
        &self.iidr
    }
    ///0x80 - For interrupts ID
    #[inline(always)]
    pub const fn igroupr0(&self) -> &IGROUPR0 {
        &self.igroupr0
    }
    ///0x84 - For interrupts ID
    #[inline(always)]
    pub const fn igroupr1(&self) -> &IGROUPR1 {
        &self.igroupr1
    }
    ///0x88 - For interrupts ID
    #[inline(always)]
    pub const fn igroupr2(&self) -> &IGROUPR2 {
        &self.igroupr2
    }
    ///0x8c - For interrupts ID = x*32 to ID = x*32+31
    #[inline(always)]
    pub const fn igroupr3(&self) -> &IGROUPR3 {
        &self.igroupr3
    }
    ///0x90 - For interrupts ID = x*32 to ID = x*32+31
    #[inline(always)]
    pub const fn igroupr4(&self) -> &IGROUPR4 {
        &self.igroupr4
    }
    ///0x94 - For interrupts ID
    #[inline(always)]
    pub const fn igroupr5(&self) -> &IGROUPR5 {
        &self.igroupr5
    }
    ///0x98 - For interrupts ID
    #[inline(always)]
    pub const fn igroupr6(&self) -> &IGROUPR6 {
        &self.igroupr6
    }
    ///0x9c - For interrupts ID
    #[inline(always)]
    pub const fn igroupr7(&self) -> &IGROUPR7 {
        &self.igroupr7
    }
    ///0xa0 - For interrupts ID
    #[inline(always)]
    pub const fn igroupr8(&self) -> &IGROUPR8 {
        &self.igroupr8
    }
    ///0x100 - For interrupts ID = 0 to ID = 31
    #[inline(always)]
    pub const fn isenabler0(&self) -> &ISENABLER0 {
        &self.isenabler0
    }
    ///0x104 - For interrupts ID
    #[inline(always)]
    pub const fn isenabler1(&self) -> &ISENABLER1 {
        &self.isenabler1
    }
    ///0x108 - For interrupts ID
    #[inline(always)]
    pub const fn isenabler2(&self) -> &ISENABLER2 {
        &self.isenabler2
    }
    ///0x10c - For interrupts ID
    #[inline(always)]
    pub const fn isenabler3(&self) -> &ISENABLER3 {
        &self.isenabler3
    }
    ///0x110 - For interrupts ID
    #[inline(always)]
    pub const fn isenabler4(&self) -> &ISENABLER4 {
        &self.isenabler4
    }
    ///0x114 - For interrupts ID
    #[inline(always)]
    pub const fn isenabler5(&self) -> &ISENABLER5 {
        &self.isenabler5
    }
    ///0x118 - For interrupts ID
    #[inline(always)]
    pub const fn isenabler6(&self) -> &ISENABLER6 {
        &self.isenabler6
    }
    ///0x11c - For interrupts ID
    #[inline(always)]
    pub const fn isenabler7(&self) -> &ISENABLER7 {
        &self.isenabler7
    }
    ///0x120 - For interrupts ID
    #[inline(always)]
    pub const fn isenabler8(&self) -> &ISENABLER8 {
        &self.isenabler8
    }
    ///0x180 - For interrupts ID = 0 to ID = 31
    #[inline(always)]
    pub const fn icenabler0(&self) -> &ICENABLER0 {
        &self.icenabler0
    }
    ///0x184 - For interrupts ID
    #[inline(always)]
    pub const fn icenabler1(&self) -> &ICENABLER1 {
        &self.icenabler1
    }
    ///0x188 - For interrupts ID
    #[inline(always)]
    pub const fn icenabler2(&self) -> &ICENABLER2 {
        &self.icenabler2
    }
    ///0x18c - For interrupts ID
    #[inline(always)]
    pub const fn icenabler3(&self) -> &ICENABLER3 {
        &self.icenabler3
    }
    ///0x190 - For interrupts ID
    #[inline(always)]
    pub const fn icenabler4(&self) -> &ICENABLER4 {
        &self.icenabler4
    }
    ///0x194 - For interrupts ID
    #[inline(always)]
    pub const fn icenabler5(&self) -> &ICENABLER5 {
        &self.icenabler5
    }
    ///0x198 - For interrupts ID
    #[inline(always)]
    pub const fn icenabler6(&self) -> &ICENABLER6 {
        &self.icenabler6
    }
    ///0x19c - For interrupts ID
    #[inline(always)]
    pub const fn icenabler7(&self) -> &ICENABLER7 {
        &self.icenabler7
    }
    ///0x1a0 - For interrupts ID
    #[inline(always)]
    pub const fn icenabler8(&self) -> &ICENABLER8 {
        &self.icenabler8
    }
    ///0x200 - For interrupts ID
    #[inline(always)]
    pub const fn ispendr0(&self) -> &ISPENDR0 {
        &self.ispendr0
    }
    ///0x204 - For interrupts ID
    #[inline(always)]
    pub const fn ispendr1(&self) -> &ISPENDR1 {
        &self.ispendr1
    }
    ///0x208 - For interrupts ID
    #[inline(always)]
    pub const fn ispendr2(&self) -> &ISPENDR2 {
        &self.ispendr2
    }
    ///0x20c - For interrupts ID
    #[inline(always)]
    pub const fn ispendr3(&self) -> &ISPENDR3 {
        &self.ispendr3
    }
    ///0x210 - For interrupts ID
    #[inline(always)]
    pub const fn ispendr4(&self) -> &ISPENDR4 {
        &self.ispendr4
    }
    ///0x214 - For interrupts ID
    #[inline(always)]
    pub const fn ispendr5(&self) -> &ISPENDR5 {
        &self.ispendr5
    }
    ///0x218 - For interrupts ID
    #[inline(always)]
    pub const fn ispendr6(&self) -> &ISPENDR6 {
        &self.ispendr6
    }
    ///0x21c - For interrupts ID
    #[inline(always)]
    pub const fn ispendr7(&self) -> &ISPENDR7 {
        &self.ispendr7
    }
    ///0x220 - For interrupts ID
    #[inline(always)]
    pub const fn ispendr8(&self) -> &ISPENDR8 {
        &self.ispendr8
    }
    ///0x280 - For interrupts ID
    #[inline(always)]
    pub const fn icpendr0(&self) -> &ICPENDR0 {
        &self.icpendr0
    }
    ///0x284 - For interrupts ID
    #[inline(always)]
    pub const fn icpendr1(&self) -> &ICPENDR1 {
        &self.icpendr1
    }
    ///0x288 - For interrupts ID
    #[inline(always)]
    pub const fn icpendr2(&self) -> &ICPENDR2 {
        &self.icpendr2
    }
    ///0x28c - For interrupts ID
    #[inline(always)]
    pub const fn icpendr3(&self) -> &ICPENDR3 {
        &self.icpendr3
    }
    ///0x290 - For interrupts ID
    #[inline(always)]
    pub const fn icpendr4(&self) -> &ICPENDR4 {
        &self.icpendr4
    }
    ///0x294 - For interrupts ID
    #[inline(always)]
    pub const fn icpendr5(&self) -> &ICPENDR5 {
        &self.icpendr5
    }
    ///0x298 - For interrupts ID
    #[inline(always)]
    pub const fn icpendr6(&self) -> &ICPENDR6 {
        &self.icpendr6
    }
    ///0x29c - For interrupts ID
    #[inline(always)]
    pub const fn icpendr7(&self) -> &ICPENDR7 {
        &self.icpendr7
    }
    ///0x2a0 - For interrupts ID
    #[inline(always)]
    pub const fn icpendr8(&self) -> &ICPENDR8 {
        &self.icpendr8
    }
    ///0x300 - For interrupts ID
    #[inline(always)]
    pub const fn isactiver0(&self) -> &ISACTIVER0 {
        &self.isactiver0
    }
    ///0x304 - For interrupts ID
    #[inline(always)]
    pub const fn isactiver1(&self) -> &ISACTIVER1 {
        &self.isactiver1
    }
    ///0x308 - For interrupts ID
    #[inline(always)]
    pub const fn isactiver2(&self) -> &ISACTIVER2 {
        &self.isactiver2
    }
    ///0x30c - For interrupts ID
    #[inline(always)]
    pub const fn isactiver3(&self) -> &ISACTIVER3 {
        &self.isactiver3
    }
    ///0x310 - For interrupts ID
    #[inline(always)]
    pub const fn isactiver4(&self) -> &ISACTIVER4 {
        &self.isactiver4
    }
    ///0x314 - For interrupts ID
    #[inline(always)]
    pub const fn isactiver5(&self) -> &ISACTIVER5 {
        &self.isactiver5
    }
    ///0x318 - For interrupts ID
    #[inline(always)]
    pub const fn isactiver6(&self) -> &ISACTIVER6 {
        &self.isactiver6
    }
    ///0x31c - For interrupts ID
    #[inline(always)]
    pub const fn isactiver7(&self) -> &ISACTIVER7 {
        &self.isactiver7
    }
    ///0x320 - For interrupts ID
    #[inline(always)]
    pub const fn isactiver8(&self) -> &ISACTIVER8 {
        &self.isactiver8
    }
    ///0x380 - For interrupts ID
    #[inline(always)]
    pub const fn icactiver0(&self) -> &ICACTIVER0 {
        &self.icactiver0
    }
    ///0x384 - For interrupts ID
    #[inline(always)]
    pub const fn icactiver1(&self) -> &ICACTIVER1 {
        &self.icactiver1
    }
    ///0x388 - For interrupts ID
    #[inline(always)]
    pub const fn icactiver2(&self) -> &ICACTIVER2 {
        &self.icactiver2
    }
    ///0x38c - For interrupts ID
    #[inline(always)]
    pub const fn icactiver3(&self) -> &ICACTIVER3 {
        &self.icactiver3
    }
    ///0x390 - For interrupts ID
    #[inline(always)]
    pub const fn icactiver4(&self) -> &ICACTIVER4 {
        &self.icactiver4
    }
    ///0x394 - For interrupts ID
    #[inline(always)]
    pub const fn icactiver5(&self) -> &ICACTIVER5 {
        &self.icactiver5
    }
    ///0x398 - For interrupts ID
    #[inline(always)]
    pub const fn icactiver6(&self) -> &ICACTIVER6 {
        &self.icactiver6
    }
    ///0x39c - For interrupts ID
    #[inline(always)]
    pub const fn icactiver7(&self) -> &ICACTIVER7 {
        &self.icactiver7
    }
    ///0x3a0 - For interrupts ID
    #[inline(always)]
    pub const fn icactiver8(&self) -> &ICACTIVER8 {
        &self.icactiver8
    }
    ///0x400 - GICD interrupt priority register 0
    #[inline(always)]
    pub const fn ipriorityr0(&self) -> &IPRIORITYR0 {
        &self.ipriorityr0
    }
    ///0x404 - GICD interrupt priority register 1
    #[inline(always)]
    pub const fn ipriorityr1(&self) -> &IPRIORITYR1 {
        &self.ipriorityr1
    }
    ///0x408 - GICD interrupt priority register 2
    #[inline(always)]
    pub const fn ipriorityr2(&self) -> &IPRIORITYR2 {
        &self.ipriorityr2
    }
    ///0x40c - GICD interrupt priority register 3
    #[inline(always)]
    pub const fn ipriorityr3(&self) -> &IPRIORITYR3 {
        &self.ipriorityr3
    }
    ///0x410 - GICD interrupt priority register 4
    #[inline(always)]
    pub const fn ipriorityr4(&self) -> &IPRIORITYR4 {
        &self.ipriorityr4
    }
    ///0x414 - GICD interrupt priority register 5
    #[inline(always)]
    pub const fn ipriorityr5(&self) -> &IPRIORITYR5 {
        &self.ipriorityr5
    }
    ///0x418 - GICD interrupt priority register 6
    #[inline(always)]
    pub const fn ipriorityr6(&self) -> &IPRIORITYR6 {
        &self.ipriorityr6
    }
    ///0x41c - GICD interrupt priority register 7
    #[inline(always)]
    pub const fn ipriorityr7(&self) -> &IPRIORITYR7 {
        &self.ipriorityr7
    }
    ///0x420 - GICD interrupt priority register 8
    #[inline(always)]
    pub const fn ipriorityr8(&self) -> &IPRIORITYR8 {
        &self.ipriorityr8
    }
    ///0x424 - GICD interrupt priority register 9
    #[inline(always)]
    pub const fn ipriorityr9(&self) -> &IPRIORITYR9 {
        &self.ipriorityr9
    }
    ///0x428 - GICD interrupt priority register 10
    #[inline(always)]
    pub const fn ipriorityr10(&self) -> &IPRIORITYR10 {
        &self.ipriorityr10
    }
    ///0x42c - GICD interrupt priority register 11
    #[inline(always)]
    pub const fn ipriorityr11(&self) -> &IPRIORITYR11 {
        &self.ipriorityr11
    }
    ///0x430 - GICD interrupt priority register 12
    #[inline(always)]
    pub const fn ipriorityr12(&self) -> &IPRIORITYR12 {
        &self.ipriorityr12
    }
    ///0x434 - GICD interrupt priority register 13
    #[inline(always)]
    pub const fn ipriorityr13(&self) -> &IPRIORITYR13 {
        &self.ipriorityr13
    }
    ///0x438 - GICD interrupt priority register 14
    #[inline(always)]
    pub const fn ipriorityr14(&self) -> &IPRIORITYR14 {
        &self.ipriorityr14
    }
    ///0x43c - GICD interrupt priority register 15
    #[inline(always)]
    pub const fn ipriorityr15(&self) -> &IPRIORITYR15 {
        &self.ipriorityr15
    }
    ///0x440 - GICD interrupt priority register 16
    #[inline(always)]
    pub const fn ipriorityr16(&self) -> &IPRIORITYR16 {
        &self.ipriorityr16
    }
    ///0x444 - GICD interrupt priority register 17
    #[inline(always)]
    pub const fn ipriorityr17(&self) -> &IPRIORITYR17 {
        &self.ipriorityr17
    }
    ///0x448 - GICD interrupt priority register 18
    #[inline(always)]
    pub const fn ipriorityr18(&self) -> &IPRIORITYR18 {
        &self.ipriorityr18
    }
    ///0x44c - GICD interrupt priority register 19
    #[inline(always)]
    pub const fn ipriorityr19(&self) -> &IPRIORITYR19 {
        &self.ipriorityr19
    }
    ///0x450 - GICD interrupt priority register 20
    #[inline(always)]
    pub const fn ipriorityr20(&self) -> &IPRIORITYR20 {
        &self.ipriorityr20
    }
    ///0x454 - GICD interrupt priority register 21
    #[inline(always)]
    pub const fn ipriorityr21(&self) -> &IPRIORITYR21 {
        &self.ipriorityr21
    }
    ///0x458 - GICD interrupt priority register 22
    #[inline(always)]
    pub const fn ipriorityr22(&self) -> &IPRIORITYR22 {
        &self.ipriorityr22
    }
    ///0x45c - GICD interrupt priority register 23
    #[inline(always)]
    pub const fn ipriorityr23(&self) -> &IPRIORITYR23 {
        &self.ipriorityr23
    }
    ///0x460 - GICD interrupt priority register 24
    #[inline(always)]
    pub const fn ipriorityr24(&self) -> &IPRIORITYR24 {
        &self.ipriorityr24
    }
    ///0x464 - GICD interrupt priority register 25
    #[inline(always)]
    pub const fn ipriorityr25(&self) -> &IPRIORITYR25 {
        &self.ipriorityr25
    }
    ///0x468 - GICD interrupt priority register 26
    #[inline(always)]
    pub const fn ipriorityr26(&self) -> &IPRIORITYR26 {
        &self.ipriorityr26
    }
    ///0x46c - GICD interrupt priority register 27
    #[inline(always)]
    pub const fn ipriorityr27(&self) -> &IPRIORITYR27 {
        &self.ipriorityr27
    }
    ///0x470 - GICD interrupt priority register 28
    #[inline(always)]
    pub const fn ipriorityr28(&self) -> &IPRIORITYR28 {
        &self.ipriorityr28
    }
    ///0x474 - GICD interrupt priority register 29
    #[inline(always)]
    pub const fn ipriorityr29(&self) -> &IPRIORITYR29 {
        &self.ipriorityr29
    }
    ///0x478 - GICD interrupt priority register 30
    #[inline(always)]
    pub const fn ipriorityr30(&self) -> &IPRIORITYR30 {
        &self.ipriorityr30
    }
    ///0x47c - GICD interrupt priority register 31
    #[inline(always)]
    pub const fn ipriorityr31(&self) -> &IPRIORITYR31 {
        &self.ipriorityr31
    }
    ///0x480 - GICD interrupt priority register 32
    #[inline(always)]
    pub const fn ipriorityr32(&self) -> &IPRIORITYR32 {
        &self.ipriorityr32
    }
    ///0x484 - GICD interrupt priority register 33
    #[inline(always)]
    pub const fn ipriorityr33(&self) -> &IPRIORITYR33 {
        &self.ipriorityr33
    }
    ///0x488 - GICD interrupt priority register 34
    #[inline(always)]
    pub const fn ipriorityr34(&self) -> &IPRIORITYR34 {
        &self.ipriorityr34
    }
    ///0x48c - GICD interrupt priority register 35
    #[inline(always)]
    pub const fn ipriorityr35(&self) -> &IPRIORITYR35 {
        &self.ipriorityr35
    }
    ///0x490 - GICD interrupt priority register 36
    #[inline(always)]
    pub const fn ipriorityr36(&self) -> &IPRIORITYR36 {
        &self.ipriorityr36
    }
    ///0x494 - GICD interrupt priority register 37
    #[inline(always)]
    pub const fn ipriorityr37(&self) -> &IPRIORITYR37 {
        &self.ipriorityr37
    }
    ///0x498 - GICD interrupt priority register 38
    #[inline(always)]
    pub const fn ipriorityr38(&self) -> &IPRIORITYR38 {
        &self.ipriorityr38
    }
    ///0x49c - GICD interrupt priority register 39
    #[inline(always)]
    pub const fn ipriorityr39(&self) -> &IPRIORITYR39 {
        &self.ipriorityr39
    }
    ///0x4a0 - GICD interrupt priority register 40
    #[inline(always)]
    pub const fn ipriorityr40(&self) -> &IPRIORITYR40 {
        &self.ipriorityr40
    }
    ///0x4a4 - GICD interrupt priority register 41
    #[inline(always)]
    pub const fn ipriorityr41(&self) -> &IPRIORITYR41 {
        &self.ipriorityr41
    }
    ///0x4a8 - GICD interrupt priority register 42
    #[inline(always)]
    pub const fn ipriorityr42(&self) -> &IPRIORITYR42 {
        &self.ipriorityr42
    }
    ///0x4ac - GICD interrupt priority register 43
    #[inline(always)]
    pub const fn ipriorityr43(&self) -> &IPRIORITYR43 {
        &self.ipriorityr43
    }
    ///0x4b0 - GICD interrupt priority register 44
    #[inline(always)]
    pub const fn ipriorityr44(&self) -> &IPRIORITYR44 {
        &self.ipriorityr44
    }
    ///0x4b4 - GICD interrupt priority register 45
    #[inline(always)]
    pub const fn ipriorityr45(&self) -> &IPRIORITYR45 {
        &self.ipriorityr45
    }
    ///0x4b8 - GICD interrupt priority register 46
    #[inline(always)]
    pub const fn ipriorityr46(&self) -> &IPRIORITYR46 {
        &self.ipriorityr46
    }
    ///0x4bc - GICD interrupt priority register 47
    #[inline(always)]
    pub const fn ipriorityr47(&self) -> &IPRIORITYR47 {
        &self.ipriorityr47
    }
    ///0x4c0 - GICD interrupt priority register 48
    #[inline(always)]
    pub const fn ipriorityr48(&self) -> &IPRIORITYR48 {
        &self.ipriorityr48
    }
    ///0x4c4 - GICD interrupt priority register 49
    #[inline(always)]
    pub const fn ipriorityr49(&self) -> &IPRIORITYR49 {
        &self.ipriorityr49
    }
    ///0x4c8 - GICD interrupt priority register 50
    #[inline(always)]
    pub const fn ipriorityr50(&self) -> &IPRIORITYR50 {
        &self.ipriorityr50
    }
    ///0x4cc - GICD interrupt priority register 51
    #[inline(always)]
    pub const fn ipriorityr51(&self) -> &IPRIORITYR51 {
        &self.ipriorityr51
    }
    ///0x4d0 - GICD interrupt priority register 52
    #[inline(always)]
    pub const fn ipriorityr52(&self) -> &IPRIORITYR52 {
        &self.ipriorityr52
    }
    ///0x4d4 - GICD interrupt priority register 53
    #[inline(always)]
    pub const fn ipriorityr53(&self) -> &IPRIORITYR53 {
        &self.ipriorityr53
    }
    ///0x4d8 - GICD interrupt priority register 54
    #[inline(always)]
    pub const fn ipriorityr54(&self) -> &IPRIORITYR54 {
        &self.ipriorityr54
    }
    ///0x4dc - GICD interrupt priority register 55
    #[inline(always)]
    pub const fn ipriorityr55(&self) -> &IPRIORITYR55 {
        &self.ipriorityr55
    }
    ///0x4e0 - GICD interrupt priority register 56
    #[inline(always)]
    pub const fn ipriorityr56(&self) -> &IPRIORITYR56 {
        &self.ipriorityr56
    }
    ///0x4e4 - GICD interrupt priority register 57
    #[inline(always)]
    pub const fn ipriorityr57(&self) -> &IPRIORITYR57 {
        &self.ipriorityr57
    }
    ///0x4e8 - GICD interrupt priority register 58
    #[inline(always)]
    pub const fn ipriorityr58(&self) -> &IPRIORITYR58 {
        &self.ipriorityr58
    }
    ///0x4ec - GICD interrupt priority register 59
    #[inline(always)]
    pub const fn ipriorityr59(&self) -> &IPRIORITYR59 {
        &self.ipriorityr59
    }
    ///0x4f0 - GICD interrupt priority register 60
    #[inline(always)]
    pub const fn ipriorityr60(&self) -> &IPRIORITYR60 {
        &self.ipriorityr60
    }
    ///0x4f4 - GICD interrupt priority register 61
    #[inline(always)]
    pub const fn ipriorityr61(&self) -> &IPRIORITYR61 {
        &self.ipriorityr61
    }
    ///0x4f8 - GICD interrupt priority register 62
    #[inline(always)]
    pub const fn ipriorityr62(&self) -> &IPRIORITYR62 {
        &self.ipriorityr62
    }
    ///0x4fc - GICD interrupt priority register 63
    #[inline(always)]
    pub const fn ipriorityr63(&self) -> &IPRIORITYR63 {
        &self.ipriorityr63
    }
    ///0x500 - GICD interrupt priority register 64
    #[inline(always)]
    pub const fn ipriorityr64(&self) -> &IPRIORITYR64 {
        &self.ipriorityr64
    }
    ///0x504 - GICD interrupt priority register 65
    #[inline(always)]
    pub const fn ipriorityr65(&self) -> &IPRIORITYR65 {
        &self.ipriorityr65
    }
    ///0x508 - GICD interrupt priority register 66
    #[inline(always)]
    pub const fn ipriorityr66(&self) -> &IPRIORITYR66 {
        &self.ipriorityr66
    }
    ///0x50c - GICD interrupt priority register 67
    #[inline(always)]
    pub const fn ipriorityr67(&self) -> &IPRIORITYR67 {
        &self.ipriorityr67
    }
    ///0x510 - GICD interrupt priority register 68
    #[inline(always)]
    pub const fn ipriorityr68(&self) -> &IPRIORITYR68 {
        &self.ipriorityr68
    }
    ///0x514 - GICD interrupt priority register 69
    #[inline(always)]
    pub const fn ipriorityr69(&self) -> &IPRIORITYR69 {
        &self.ipriorityr69
    }
    ///0x518 - GICD interrupt priority register 70
    #[inline(always)]
    pub const fn ipriorityr70(&self) -> &IPRIORITYR70 {
        &self.ipriorityr70
    }
    ///0x51c - GICD interrupt priority register 71
    #[inline(always)]
    pub const fn ipriorityr71(&self) -> &IPRIORITYR71 {
        &self.ipriorityr71
    }
    ///0x800 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
    #[inline(always)]
    pub const fn itargetsr0(&self) -> &ITARGETSR0 {
        &self.itargetsr0
    }
    ///0x804 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
    #[inline(always)]
    pub const fn itargetsr1(&self) -> &ITARGETSR1 {
        &self.itargetsr1
    }
    ///0x808 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
    #[inline(always)]
    pub const fn itargetsr2(&self) -> &ITARGETSR2 {
        &self.itargetsr2
    }
    ///0x80c - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
    #[inline(always)]
    pub const fn itargetsr3(&self) -> &ITARGETSR3 {
        &self.itargetsr3
    }
    ///0x810 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
    #[inline(always)]
    pub const fn itargetsr4(&self) -> &ITARGETSR4 {
        &self.itargetsr4
    }
    ///0x814 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
    #[inline(always)]
    pub const fn itargetsr5(&self) -> &ITARGETSR5 {
        &self.itargetsr5
    }
    ///0x818 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
    #[inline(always)]
    pub const fn itargetsr6(&self) -> &ITARGETSR6 {
        &self.itargetsr6
    }
    ///0x81c - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
    #[inline(always)]
    pub const fn itargetsr7(&self) -> &ITARGETSR7 {
        &self.itargetsr7
    }
    ///0x820 - GICD interrupt processor target register 8
    #[inline(always)]
    pub const fn itargetsr8(&self) -> &ITARGETSR8 {
        &self.itargetsr8
    }
    ///0x824 - GICD interrupt processor target register 9
    #[inline(always)]
    pub const fn itargetsr9(&self) -> &ITARGETSR9 {
        &self.itargetsr9
    }
    ///0x828 - GICD interrupt processor target register 10
    #[inline(always)]
    pub const fn itargetsr10(&self) -> &ITARGETSR10 {
        &self.itargetsr10
    }
    ///0x82c - GICD interrupt processor target register 11
    #[inline(always)]
    pub const fn itargetsr11(&self) -> &ITARGETSR11 {
        &self.itargetsr11
    }
    ///0x830 - GICD interrupt processor target register 12
    #[inline(always)]
    pub const fn itargetsr12(&self) -> &ITARGETSR12 {
        &self.itargetsr12
    }
    ///0x834 - GICD interrupt processor target register 13
    #[inline(always)]
    pub const fn itargetsr13(&self) -> &ITARGETSR13 {
        &self.itargetsr13
    }
    ///0x838 - GICD interrupt processor target register 14
    #[inline(always)]
    pub const fn itargetsr14(&self) -> &ITARGETSR14 {
        &self.itargetsr14
    }
    ///0x83c - GICD interrupt processor target register 15
    #[inline(always)]
    pub const fn itargetsr15(&self) -> &ITARGETSR15 {
        &self.itargetsr15
    }
    ///0x840 - GICD interrupt processor target register 16
    #[inline(always)]
    pub const fn itargetsr16(&self) -> &ITARGETSR16 {
        &self.itargetsr16
    }
    ///0x844 - GICD interrupt processor target register 17
    #[inline(always)]
    pub const fn itargetsr17(&self) -> &ITARGETSR17 {
        &self.itargetsr17
    }
    ///0x848 - GICD interrupt processor target register 18
    #[inline(always)]
    pub const fn itargetsr18(&self) -> &ITARGETSR18 {
        &self.itargetsr18
    }
    ///0x84c - GICD interrupt processor target register 19
    #[inline(always)]
    pub const fn itargetsr19(&self) -> &ITARGETSR19 {
        &self.itargetsr19
    }
    ///0x850 - GICD interrupt processor target register 20
    #[inline(always)]
    pub const fn itargetsr20(&self) -> &ITARGETSR20 {
        &self.itargetsr20
    }
    ///0x854 - GICD interrupt processor target register 21
    #[inline(always)]
    pub const fn itargetsr21(&self) -> &ITARGETSR21 {
        &self.itargetsr21
    }
    ///0x858 - GICD interrupt processor target register 22
    #[inline(always)]
    pub const fn itargetsr22(&self) -> &ITARGETSR22 {
        &self.itargetsr22
    }
    ///0x85c - GICD interrupt processor target register 23
    #[inline(always)]
    pub const fn itargetsr23(&self) -> &ITARGETSR23 {
        &self.itargetsr23
    }
    ///0x860 - GICD interrupt processor target register 24
    #[inline(always)]
    pub const fn itargetsr24(&self) -> &ITARGETSR24 {
        &self.itargetsr24
    }
    ///0x864 - GICD interrupt processor target register 25
    #[inline(always)]
    pub const fn itargetsr25(&self) -> &ITARGETSR25 {
        &self.itargetsr25
    }
    ///0x868 - GICD interrupt processor target register 26
    #[inline(always)]
    pub const fn itargetsr26(&self) -> &ITARGETSR26 {
        &self.itargetsr26
    }
    ///0x86c - GICD interrupt processor target register 27
    #[inline(always)]
    pub const fn itargetsr27(&self) -> &ITARGETSR27 {
        &self.itargetsr27
    }
    ///0x870 - GICD interrupt processor target register 28
    #[inline(always)]
    pub const fn itargetsr28(&self) -> &ITARGETSR28 {
        &self.itargetsr28
    }
    ///0x874 - GICD interrupt processor target register 29
    #[inline(always)]
    pub const fn itargetsr29(&self) -> &ITARGETSR29 {
        &self.itargetsr29
    }
    ///0x878 - GICD interrupt processor target register 30
    #[inline(always)]
    pub const fn itargetsr30(&self) -> &ITARGETSR30 {
        &self.itargetsr30
    }
    ///0x87c - GICD interrupt processor target register 31
    #[inline(always)]
    pub const fn itargetsr31(&self) -> &ITARGETSR31 {
        &self.itargetsr31
    }
    ///0x880 - GICD interrupt processor target register 32
    #[inline(always)]
    pub const fn itargetsr32(&self) -> &ITARGETSR32 {
        &self.itargetsr32
    }
    ///0x884 - GICD interrupt processor target register 33
    #[inline(always)]
    pub const fn itargetsr33(&self) -> &ITARGETSR33 {
        &self.itargetsr33
    }
    ///0x888 - GICD interrupt processor target register 34
    #[inline(always)]
    pub const fn itargetsr34(&self) -> &ITARGETSR34 {
        &self.itargetsr34
    }
    ///0x88c - GICD interrupt processor target register 35
    #[inline(always)]
    pub const fn itargetsr35(&self) -> &ITARGETSR35 {
        &self.itargetsr35
    }
    ///0x890 - GICD interrupt processor target register 36
    #[inline(always)]
    pub const fn itargetsr36(&self) -> &ITARGETSR36 {
        &self.itargetsr36
    }
    ///0x894 - GICD interrupt processor target register 37
    #[inline(always)]
    pub const fn itargetsr37(&self) -> &ITARGETSR37 {
        &self.itargetsr37
    }
    ///0x898 - GICD interrupt processor target register 38
    #[inline(always)]
    pub const fn itargetsr38(&self) -> &ITARGETSR38 {
        &self.itargetsr38
    }
    ///0x89c - GICD interrupt processor target register 39
    #[inline(always)]
    pub const fn itargetsr39(&self) -> &ITARGETSR39 {
        &self.itargetsr39
    }
    ///0x8a0 - GICD interrupt processor target register 40
    #[inline(always)]
    pub const fn itargetsr40(&self) -> &ITARGETSR40 {
        &self.itargetsr40
    }
    ///0x8a4 - GICD interrupt processor target register 41
    #[inline(always)]
    pub const fn itargetsr41(&self) -> &ITARGETSR41 {
        &self.itargetsr41
    }
    ///0x8a8 - GICD interrupt processor target register 42
    #[inline(always)]
    pub const fn itargetsr42(&self) -> &ITARGETSR42 {
        &self.itargetsr42
    }
    ///0x8ac - GICD interrupt processor target register 43
    #[inline(always)]
    pub const fn itargetsr43(&self) -> &ITARGETSR43 {
        &self.itargetsr43
    }
    ///0x8b0 - GICD interrupt processor target register 44
    #[inline(always)]
    pub const fn itargetsr44(&self) -> &ITARGETSR44 {
        &self.itargetsr44
    }
    ///0x8b4 - GICD interrupt processor target register 45
    #[inline(always)]
    pub const fn itargetsr45(&self) -> &ITARGETSR45 {
        &self.itargetsr45
    }
    ///0x8b8 - GICD interrupt processor target register 46
    #[inline(always)]
    pub const fn itargetsr46(&self) -> &ITARGETSR46 {
        &self.itargetsr46
    }
    ///0x8bc - GICD interrupt processor target register 47
    #[inline(always)]
    pub const fn itargetsr47(&self) -> &ITARGETSR47 {
        &self.itargetsr47
    }
    ///0x8c0 - GICD interrupt processor target register 48
    #[inline(always)]
    pub const fn itargetsr48(&self) -> &ITARGETSR48 {
        &self.itargetsr48
    }
    ///0x8c4 - GICD interrupt processor target register 49
    #[inline(always)]
    pub const fn itargetsr49(&self) -> &ITARGETSR49 {
        &self.itargetsr49
    }
    ///0x8c8 - GICD interrupt processor target register 50
    #[inline(always)]
    pub const fn itargetsr50(&self) -> &ITARGETSR50 {
        &self.itargetsr50
    }
    ///0x8cc - GICD interrupt processor target register 51
    #[inline(always)]
    pub const fn itargetsr51(&self) -> &ITARGETSR51 {
        &self.itargetsr51
    }
    ///0x8d0 - GICD interrupt processor target register 52
    #[inline(always)]
    pub const fn itargetsr52(&self) -> &ITARGETSR52 {
        &self.itargetsr52
    }
    ///0x8d4 - GICD interrupt processor target register 53
    #[inline(always)]
    pub const fn itargetsr53(&self) -> &ITARGETSR53 {
        &self.itargetsr53
    }
    ///0x8d8 - GICD interrupt processor target register 54
    #[inline(always)]
    pub const fn itargetsr54(&self) -> &ITARGETSR54 {
        &self.itargetsr54
    }
    ///0x8dc - GICD interrupt processor target register 55
    #[inline(always)]
    pub const fn itargetsr55(&self) -> &ITARGETSR55 {
        &self.itargetsr55
    }
    ///0x8e0 - GICD interrupt processor target register 56
    #[inline(always)]
    pub const fn itargetsr56(&self) -> &ITARGETSR56 {
        &self.itargetsr56
    }
    ///0x8e4 - GICD interrupt processor target register 57
    #[inline(always)]
    pub const fn itargetsr57(&self) -> &ITARGETSR57 {
        &self.itargetsr57
    }
    ///0x8e8 - GICD interrupt processor target register 58
    #[inline(always)]
    pub const fn itargetsr58(&self) -> &ITARGETSR58 {
        &self.itargetsr58
    }
    ///0x8ec - GICD interrupt processor target register 59
    #[inline(always)]
    pub const fn itargetsr59(&self) -> &ITARGETSR59 {
        &self.itargetsr59
    }
    ///0x8f0 - GICD interrupt processor target register 60
    #[inline(always)]
    pub const fn itargetsr60(&self) -> &ITARGETSR60 {
        &self.itargetsr60
    }
    ///0x8f4 - GICD interrupt processor target register 61
    #[inline(always)]
    pub const fn itargetsr61(&self) -> &ITARGETSR61 {
        &self.itargetsr61
    }
    ///0x8f8 - GICD interrupt processor target register 62
    #[inline(always)]
    pub const fn itargetsr62(&self) -> &ITARGETSR62 {
        &self.itargetsr62
    }
    ///0x8fc - GICD interrupt processor target register 63
    #[inline(always)]
    pub const fn itargetsr63(&self) -> &ITARGETSR63 {
        &self.itargetsr63
    }
    ///0x900 - GICD interrupt processor target register 64
    #[inline(always)]
    pub const fn itargetsr64(&self) -> &ITARGETSR64 {
        &self.itargetsr64
    }
    ///0x904 - GICD interrupt processor target register 65
    #[inline(always)]
    pub const fn itargetsr65(&self) -> &ITARGETSR65 {
        &self.itargetsr65
    }
    ///0x908 - GICD interrupt processor target register 66
    #[inline(always)]
    pub const fn itargetsr66(&self) -> &ITARGETSR66 {
        &self.itargetsr66
    }
    ///0x90c - GICD interrupt processor target register 67
    #[inline(always)]
    pub const fn itargetsr67(&self) -> &ITARGETSR67 {
        &self.itargetsr67
    }
    ///0x910 - GICD interrupt processor target register 68
    #[inline(always)]
    pub const fn itargetsr68(&self) -> &ITARGETSR68 {
        &self.itargetsr68
    }
    ///0x914 - GICD interrupt processor target register 69
    #[inline(always)]
    pub const fn itargetsr69(&self) -> &ITARGETSR69 {
        &self.itargetsr69
    }
    ///0x918 - GICD interrupt processor target register 70
    #[inline(always)]
    pub const fn itargetsr70(&self) -> &ITARGETSR70 {
        &self.itargetsr70
    }
    ///0x91c - GICD interrupt processor target register 71
    #[inline(always)]
    pub const fn itargetsr71(&self) -> &ITARGETSR71 {
        &self.itargetsr71
    }
    ///0xc00 - GICD interrupt configuration register
    #[inline(always)]
    pub const fn icfgr0(&self) -> &ICFGR0 {
        &self.icfgr0
    }
    ///0xc04 - GICD interrupt configuration register
    #[inline(always)]
    pub const fn icfgr1(&self) -> &ICFGR1 {
        &self.icfgr1
    }
    ///0xc08 - GICD interrupt configuration register 2
    #[inline(always)]
    pub const fn icfgr2(&self) -> &ICFGR2 {
        &self.icfgr2
    }
    ///0xc0c - GICD interrupt configuration register 3
    #[inline(always)]
    pub const fn icfgr3(&self) -> &ICFGR3 {
        &self.icfgr3
    }
    ///0xc10 - GICD interrupt configuration register 4
    #[inline(always)]
    pub const fn icfgr4(&self) -> &ICFGR4 {
        &self.icfgr4
    }
    ///0xc14 - GICD interrupt configuration register 5
    #[inline(always)]
    pub const fn icfgr5(&self) -> &ICFGR5 {
        &self.icfgr5
    }
    ///0xc18 - GICD interrupt configuration register 6
    #[inline(always)]
    pub const fn icfgr6(&self) -> &ICFGR6 {
        &self.icfgr6
    }
    ///0xc1c - GICD interrupt configuration register 7
    #[inline(always)]
    pub const fn icfgr7(&self) -> &ICFGR7 {
        &self.icfgr7
    }
    ///0xc20 - GICD interrupt configuration register 8
    #[inline(always)]
    pub const fn icfgr8(&self) -> &ICFGR8 {
        &self.icfgr8
    }
    ///0xc24 - GICD interrupt configuration register 9
    #[inline(always)]
    pub const fn icfgr9(&self) -> &ICFGR9 {
        &self.icfgr9
    }
    ///0xc28 - GICD interrupt configuration register 10
    #[inline(always)]
    pub const fn icfgr10(&self) -> &ICFGR10 {
        &self.icfgr10
    }
    ///0xc2c - GICD interrupt configuration register 11
    #[inline(always)]
    pub const fn icfgr11(&self) -> &ICFGR11 {
        &self.icfgr11
    }
    ///0xc30 - GICD interrupt configuration register 12
    #[inline(always)]
    pub const fn icfgr12(&self) -> &ICFGR12 {
        &self.icfgr12
    }
    ///0xc34 - GICD interrupt configuration register 13
    #[inline(always)]
    pub const fn icfgr13(&self) -> &ICFGR13 {
        &self.icfgr13
    }
    ///0xc38 - GICD interrupt configuration register 14
    #[inline(always)]
    pub const fn icfgr14(&self) -> &ICFGR14 {
        &self.icfgr14
    }
    ///0xc3c - GICD interrupt configuration register 15
    #[inline(always)]
    pub const fn icfgr15(&self) -> &ICFGR15 {
        &self.icfgr15
    }
    ///0xc40 - GICD interrupt configuration register 16
    #[inline(always)]
    pub const fn icfgr16(&self) -> &ICFGR16 {
        &self.icfgr16
    }
    ///0xc44 - GICD interrupt configuration register 17
    #[inline(always)]
    pub const fn icfgr17(&self) -> &ICFGR17 {
        &self.icfgr17
    }
    ///0xd00 - GICD private peripheral interrupt status register
    #[inline(always)]
    pub const fn ppisr(&self) -> &PPISR {
        &self.ppisr
    }
    ///0xd08 - For interrupts ID = SPI number+32, from SPI \[x*32+31\] to SPI \[x*32\]
    #[inline(always)]
    pub const fn spisr1(&self) -> &SPISR1 {
        &self.spisr1
    }
    ///0xd0c - For interrupts ID
    #[inline(always)]
    pub const fn spisr2(&self) -> &SPISR2 {
        &self.spisr2
    }
    ///0xd10 - For interrupts ID
    #[inline(always)]
    pub const fn spisr3(&self) -> &SPISR3 {
        &self.spisr3
    }
    ///0xd14 - For interrupts ID
    #[inline(always)]
    pub const fn spisr4(&self) -> &SPISR4 {
        &self.spisr4
    }
    ///0xd18 - For interrupts ID
    #[inline(always)]
    pub const fn spisr5(&self) -> &SPISR5 {
        &self.spisr5
    }
    ///0xd1c - For interrupts ID
    #[inline(always)]
    pub const fn spisr6(&self) -> &SPISR6 {
        &self.spisr6
    }
    ///0xd20 - For interrupts ID
    #[inline(always)]
    pub const fn spisr7(&self) -> &SPISR7 {
        &self.spisr7
    }
    ///0xf00 - GICD software generated interrupt register
    #[inline(always)]
    pub const fn sgir(&self) -> &SGIR {
        &self.sgir
    }
    ///0xf10 - For SGI x*4 to SGI x*4+3
    #[inline(always)]
    pub const fn cpendsgir0(&self) -> &CPENDSGIR0 {
        &self.cpendsgir0
    }
    ///0xf14 - For SGI x*4 to SGI x*4+3
    #[inline(always)]
    pub const fn cpendsgir1(&self) -> &CPENDSGIR1 {
        &self.cpendsgir1
    }
    ///0xf18 - For SGI x*4 to SGI x*4+3
    #[inline(always)]
    pub const fn cpendsgir2(&self) -> &CPENDSGIR2 {
        &self.cpendsgir2
    }
    ///0xf1c - For SGI x*4 to SGI x*4+3
    #[inline(always)]
    pub const fn cpendsgir3(&self) -> &CPENDSGIR3 {
        &self.cpendsgir3
    }
    ///0xf20 - For SGI x*4 to SGI x*4+3
    #[inline(always)]
    pub const fn spendsgir0(&self) -> &SPENDSGIR0 {
        &self.spendsgir0
    }
    ///0xf24 - For SGI x*4 to SGI x*4+3
    #[inline(always)]
    pub const fn spendsgir1(&self) -> &SPENDSGIR1 {
        &self.spendsgir1
    }
    ///0xf28 - For SGI x*4 to SGI x*4+3
    #[inline(always)]
    pub const fn spendsgir2(&self) -> &SPENDSGIR2 {
        &self.spendsgir2
    }
    ///0xf2c - For SGI x*4 to SGI x*4+3
    #[inline(always)]
    pub const fn spendsgir3(&self) -> &SPENDSGIR3 {
        &self.spendsgir3
    }
    ///0xfd0 - GICD peripheral ID4 register
    #[inline(always)]
    pub const fn pidr4(&self) -> &PIDR4 {
        &self.pidr4
    }
    ///0xfd4 - GICD peripheral ID5 to ID7 register 5
    #[inline(always)]
    pub const fn pidr5(&self) -> &PIDR5 {
        &self.pidr5
    }
    ///0xfd8 - GICD peripheral ID5 to ID7 register 6
    #[inline(always)]
    pub const fn pidr6(&self) -> &PIDR6 {
        &self.pidr6
    }
    ///0xfdc - GICD peripheral ID5 to ID7 register 7
    #[inline(always)]
    pub const fn pidr7(&self) -> &PIDR7 {
        &self.pidr7
    }
    ///0xfe0 - GICD peripheral ID0 register
    #[inline(always)]
    pub const fn pidr0(&self) -> &PIDR0 {
        &self.pidr0
    }
    ///0xfe4 - GICD peripheral ID1 register
    #[inline(always)]
    pub const fn pidr1(&self) -> &PIDR1 {
        &self.pidr1
    }
    ///0xfe8 - GICD peripheral ID2 register
    #[inline(always)]
    pub const fn pidr2(&self) -> &PIDR2 {
        &self.pidr2
    }
    ///0xfec - GICD peripheral ID3 register
    #[inline(always)]
    pub const fn pidr3(&self) -> &PIDR3 {
        &self.pidr3
    }
    ///0xff0 - GICD component ID0 register
    #[inline(always)]
    pub const fn cidr0(&self) -> &CIDR0 {
        &self.cidr0
    }
    ///0xff4 - GICD component ID1 register
    #[inline(always)]
    pub const fn cidr1(&self) -> &CIDR1 {
        &self.cidr1
    }
    ///0xff8 - GICD component ID2 register
    #[inline(always)]
    pub const fn cidr2(&self) -> &CIDR2 {
        &self.cidr2
    }
    ///0xffc - GICD component ID3 register
    #[inline(always)]
    pub const fn cidr3(&self) -> &CIDR3 {
        &self.cidr3
    }
}
/**CTLR (rw) register accessor: GICD control register

You can [`read`](crate::Reg::read) this register and get [`ctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:CTLR)

For information about available fields see [`mod@ctlr`] module*/
pub type CTLR = crate::Reg<ctlr::CTLRrs>;
///GICD control register
pub mod ctlr;
/**TYPER (r) register accessor: GICD interrupt controller type register

You can [`read`](crate::Reg::read) this register and get [`typer::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:TYPER)

For information about available fields see [`mod@typer`] module*/
pub type TYPER = crate::Reg<typer::TYPERrs>;
///GICD interrupt controller type register
pub mod typer;
/**IIDR (r) register accessor: GICD implementer identification register

You can [`read`](crate::Reg::read) this register and get [`iidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IIDR)

For information about available fields see [`mod@iidr`] module*/
pub type IIDR = crate::Reg<iidr::IIDRrs>;
///GICD implementer identification register
pub mod iidr;
/**IGROUPR0 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`igroupr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igroupr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IGROUPR0)

For information about available fields see [`mod@igroupr0`] module*/
pub type IGROUPR0 = crate::Reg<igroupr0::IGROUPR0rs>;
///For interrupts ID
pub mod igroupr0;
/**IGROUPR1 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`igroupr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igroupr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IGROUPR1)

For information about available fields see [`mod@igroupr1`] module*/
pub type IGROUPR1 = crate::Reg<igroupr1::IGROUPR1rs>;
///For interrupts ID
pub mod igroupr1;
/**IGROUPR2 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`igroupr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igroupr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IGROUPR2)

For information about available fields see [`mod@igroupr2`] module*/
pub type IGROUPR2 = crate::Reg<igroupr2::IGROUPR2rs>;
///For interrupts ID
pub mod igroupr2;
/**IGROUPR3 (rw) register accessor: For interrupts ID = x*32 to ID = x*32+31

You can [`read`](crate::Reg::read) this register and get [`igroupr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igroupr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IGROUPR3)

For information about available fields see [`mod@igroupr3`] module*/
pub type IGROUPR3 = crate::Reg<igroupr3::IGROUPR3rs>;
///For interrupts ID = x*32 to ID = x*32+31
pub mod igroupr3;
/**IGROUPR4 (rw) register accessor: For interrupts ID = x*32 to ID = x*32+31

You can [`read`](crate::Reg::read) this register and get [`igroupr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igroupr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IGROUPR4)

For information about available fields see [`mod@igroupr4`] module*/
pub type IGROUPR4 = crate::Reg<igroupr4::IGROUPR4rs>;
///For interrupts ID = x*32 to ID = x*32+31
pub mod igroupr4;
/**IGROUPR5 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`igroupr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igroupr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IGROUPR5)

For information about available fields see [`mod@igroupr5`] module*/
pub type IGROUPR5 = crate::Reg<igroupr5::IGROUPR5rs>;
///For interrupts ID
pub mod igroupr5;
/**IGROUPR6 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`igroupr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igroupr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IGROUPR6)

For information about available fields see [`mod@igroupr6`] module*/
pub type IGROUPR6 = crate::Reg<igroupr6::IGROUPR6rs>;
///For interrupts ID
pub mod igroupr6;
/**IGROUPR7 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`igroupr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igroupr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IGROUPR7)

For information about available fields see [`mod@igroupr7`] module*/
pub type IGROUPR7 = crate::Reg<igroupr7::IGROUPR7rs>;
///For interrupts ID
pub mod igroupr7;
/**IGROUPR8 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`igroupr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igroupr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IGROUPR8)

For information about available fields see [`mod@igroupr8`] module*/
pub type IGROUPR8 = crate::Reg<igroupr8::IGROUPR8rs>;
///For interrupts ID
pub mod igroupr8;
/**ISENABLER0 (rw) register accessor: For interrupts ID = 0 to ID = 31

You can [`read`](crate::Reg::read) this register and get [`isenabler0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isenabler0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISENABLER0)

For information about available fields see [`mod@isenabler0`] module*/
pub type ISENABLER0 = crate::Reg<isenabler0::ISENABLER0rs>;
///For interrupts ID = 0 to ID = 31
pub mod isenabler0;
/**ISENABLER1 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isenabler1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isenabler1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISENABLER1)

For information about available fields see [`mod@isenabler1`] module*/
pub type ISENABLER1 = crate::Reg<isenabler1::ISENABLER1rs>;
///For interrupts ID
pub mod isenabler1;
/**ISENABLER2 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isenabler2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isenabler2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISENABLER2)

For information about available fields see [`mod@isenabler2`] module*/
pub type ISENABLER2 = crate::Reg<isenabler2::ISENABLER2rs>;
///For interrupts ID
pub mod isenabler2;
/**ISENABLER3 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isenabler3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isenabler3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISENABLER3)

For information about available fields see [`mod@isenabler3`] module*/
pub type ISENABLER3 = crate::Reg<isenabler3::ISENABLER3rs>;
///For interrupts ID
pub mod isenabler3;
/**ISENABLER4 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isenabler4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isenabler4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISENABLER4)

For information about available fields see [`mod@isenabler4`] module*/
pub type ISENABLER4 = crate::Reg<isenabler4::ISENABLER4rs>;
///For interrupts ID
pub mod isenabler4;
/**ISENABLER5 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isenabler5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isenabler5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISENABLER5)

For information about available fields see [`mod@isenabler5`] module*/
pub type ISENABLER5 = crate::Reg<isenabler5::ISENABLER5rs>;
///For interrupts ID
pub mod isenabler5;
/**ISENABLER6 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isenabler6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isenabler6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISENABLER6)

For information about available fields see [`mod@isenabler6`] module*/
pub type ISENABLER6 = crate::Reg<isenabler6::ISENABLER6rs>;
///For interrupts ID
pub mod isenabler6;
/**ISENABLER7 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isenabler7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isenabler7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISENABLER7)

For information about available fields see [`mod@isenabler7`] module*/
pub type ISENABLER7 = crate::Reg<isenabler7::ISENABLER7rs>;
///For interrupts ID
pub mod isenabler7;
/**ISENABLER8 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isenabler8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isenabler8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISENABLER8)

For information about available fields see [`mod@isenabler8`] module*/
pub type ISENABLER8 = crate::Reg<isenabler8::ISENABLER8rs>;
///For interrupts ID
pub mod isenabler8;
/**ICENABLER0 (rw) register accessor: For interrupts ID = 0 to ID = 31

You can [`read`](crate::Reg::read) this register and get [`icenabler0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icenabler0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICENABLER0)

For information about available fields see [`mod@icenabler0`] module*/
pub type ICENABLER0 = crate::Reg<icenabler0::ICENABLER0rs>;
///For interrupts ID = 0 to ID = 31
pub mod icenabler0;
/**ICENABLER1 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icenabler1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icenabler1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICENABLER1)

For information about available fields see [`mod@icenabler1`] module*/
pub type ICENABLER1 = crate::Reg<icenabler1::ICENABLER1rs>;
///For interrupts ID
pub mod icenabler1;
/**ICENABLER2 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icenabler2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icenabler2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICENABLER2)

For information about available fields see [`mod@icenabler2`] module*/
pub type ICENABLER2 = crate::Reg<icenabler2::ICENABLER2rs>;
///For interrupts ID
pub mod icenabler2;
/**ICENABLER3 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icenabler3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icenabler3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICENABLER3)

For information about available fields see [`mod@icenabler3`] module*/
pub type ICENABLER3 = crate::Reg<icenabler3::ICENABLER3rs>;
///For interrupts ID
pub mod icenabler3;
/**ICENABLER4 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icenabler4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icenabler4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICENABLER4)

For information about available fields see [`mod@icenabler4`] module*/
pub type ICENABLER4 = crate::Reg<icenabler4::ICENABLER4rs>;
///For interrupts ID
pub mod icenabler4;
/**ICENABLER5 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icenabler5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icenabler5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICENABLER5)

For information about available fields see [`mod@icenabler5`] module*/
pub type ICENABLER5 = crate::Reg<icenabler5::ICENABLER5rs>;
///For interrupts ID
pub mod icenabler5;
/**ICENABLER6 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icenabler6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icenabler6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICENABLER6)

For information about available fields see [`mod@icenabler6`] module*/
pub type ICENABLER6 = crate::Reg<icenabler6::ICENABLER6rs>;
///For interrupts ID
pub mod icenabler6;
/**ICENABLER7 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icenabler7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icenabler7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICENABLER7)

For information about available fields see [`mod@icenabler7`] module*/
pub type ICENABLER7 = crate::Reg<icenabler7::ICENABLER7rs>;
///For interrupts ID
pub mod icenabler7;
/**ICENABLER8 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icenabler8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icenabler8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICENABLER8)

For information about available fields see [`mod@icenabler8`] module*/
pub type ICENABLER8 = crate::Reg<icenabler8::ICENABLER8rs>;
///For interrupts ID
pub mod icenabler8;
/**ISPENDR0 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISPENDR0)

For information about available fields see [`mod@ispendr0`] module*/
pub type ISPENDR0 = crate::Reg<ispendr0::ISPENDR0rs>;
///For interrupts ID
pub mod ispendr0;
/**ISPENDR1 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISPENDR1)

For information about available fields see [`mod@ispendr1`] module*/
pub type ISPENDR1 = crate::Reg<ispendr1::ISPENDR1rs>;
///For interrupts ID
pub mod ispendr1;
/**ISPENDR2 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISPENDR2)

For information about available fields see [`mod@ispendr2`] module*/
pub type ISPENDR2 = crate::Reg<ispendr2::ISPENDR2rs>;
///For interrupts ID
pub mod ispendr2;
/**ISPENDR3 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISPENDR3)

For information about available fields see [`mod@ispendr3`] module*/
pub type ISPENDR3 = crate::Reg<ispendr3::ISPENDR3rs>;
///For interrupts ID
pub mod ispendr3;
/**ISPENDR4 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISPENDR4)

For information about available fields see [`mod@ispendr4`] module*/
pub type ISPENDR4 = crate::Reg<ispendr4::ISPENDR4rs>;
///For interrupts ID
pub mod ispendr4;
/**ISPENDR5 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISPENDR5)

For information about available fields see [`mod@ispendr5`] module*/
pub type ISPENDR5 = crate::Reg<ispendr5::ISPENDR5rs>;
///For interrupts ID
pub mod ispendr5;
/**ISPENDR6 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISPENDR6)

For information about available fields see [`mod@ispendr6`] module*/
pub type ISPENDR6 = crate::Reg<ispendr6::ISPENDR6rs>;
///For interrupts ID
pub mod ispendr6;
/**ISPENDR7 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISPENDR7)

For information about available fields see [`mod@ispendr7`] module*/
pub type ISPENDR7 = crate::Reg<ispendr7::ISPENDR7rs>;
///For interrupts ID
pub mod ispendr7;
/**ISPENDR8 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISPENDR8)

For information about available fields see [`mod@ispendr8`] module*/
pub type ISPENDR8 = crate::Reg<ispendr8::ISPENDR8rs>;
///For interrupts ID
pub mod ispendr8;
/**ICPENDR0 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICPENDR0)

For information about available fields see [`mod@icpendr0`] module*/
pub type ICPENDR0 = crate::Reg<icpendr0::ICPENDR0rs>;
///For interrupts ID
pub mod icpendr0;
/**ICPENDR1 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICPENDR1)

For information about available fields see [`mod@icpendr1`] module*/
pub type ICPENDR1 = crate::Reg<icpendr1::ICPENDR1rs>;
///For interrupts ID
pub mod icpendr1;
/**ICPENDR2 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICPENDR2)

For information about available fields see [`mod@icpendr2`] module*/
pub type ICPENDR2 = crate::Reg<icpendr2::ICPENDR2rs>;
///For interrupts ID
pub mod icpendr2;
/**ICPENDR3 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICPENDR3)

For information about available fields see [`mod@icpendr3`] module*/
pub type ICPENDR3 = crate::Reg<icpendr3::ICPENDR3rs>;
///For interrupts ID
pub mod icpendr3;
/**ICPENDR4 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICPENDR4)

For information about available fields see [`mod@icpendr4`] module*/
pub type ICPENDR4 = crate::Reg<icpendr4::ICPENDR4rs>;
///For interrupts ID
pub mod icpendr4;
/**ICPENDR5 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICPENDR5)

For information about available fields see [`mod@icpendr5`] module*/
pub type ICPENDR5 = crate::Reg<icpendr5::ICPENDR5rs>;
///For interrupts ID
pub mod icpendr5;
/**ICPENDR6 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICPENDR6)

For information about available fields see [`mod@icpendr6`] module*/
pub type ICPENDR6 = crate::Reg<icpendr6::ICPENDR6rs>;
///For interrupts ID
pub mod icpendr6;
/**ICPENDR7 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICPENDR7)

For information about available fields see [`mod@icpendr7`] module*/
pub type ICPENDR7 = crate::Reg<icpendr7::ICPENDR7rs>;
///For interrupts ID
pub mod icpendr7;
/**ICPENDR8 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icpendr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpendr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICPENDR8)

For information about available fields see [`mod@icpendr8`] module*/
pub type ICPENDR8 = crate::Reg<icpendr8::ICPENDR8rs>;
///For interrupts ID
pub mod icpendr8;
/**ISACTIVER0 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISACTIVER0)

For information about available fields see [`mod@isactiver0`] module*/
pub type ISACTIVER0 = crate::Reg<isactiver0::ISACTIVER0rs>;
///For interrupts ID
pub mod isactiver0;
/**ISACTIVER1 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISACTIVER1)

For information about available fields see [`mod@isactiver1`] module*/
pub type ISACTIVER1 = crate::Reg<isactiver1::ISACTIVER1rs>;
///For interrupts ID
pub mod isactiver1;
/**ISACTIVER2 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISACTIVER2)

For information about available fields see [`mod@isactiver2`] module*/
pub type ISACTIVER2 = crate::Reg<isactiver2::ISACTIVER2rs>;
///For interrupts ID
pub mod isactiver2;
/**ISACTIVER3 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISACTIVER3)

For information about available fields see [`mod@isactiver3`] module*/
pub type ISACTIVER3 = crate::Reg<isactiver3::ISACTIVER3rs>;
///For interrupts ID
pub mod isactiver3;
/**ISACTIVER4 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISACTIVER4)

For information about available fields see [`mod@isactiver4`] module*/
pub type ISACTIVER4 = crate::Reg<isactiver4::ISACTIVER4rs>;
///For interrupts ID
pub mod isactiver4;
/**ISACTIVER5 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISACTIVER5)

For information about available fields see [`mod@isactiver5`] module*/
pub type ISACTIVER5 = crate::Reg<isactiver5::ISACTIVER5rs>;
///For interrupts ID
pub mod isactiver5;
/**ISACTIVER6 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISACTIVER6)

For information about available fields see [`mod@isactiver6`] module*/
pub type ISACTIVER6 = crate::Reg<isactiver6::ISACTIVER6rs>;
///For interrupts ID
pub mod isactiver6;
/**ISACTIVER7 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISACTIVER7)

For information about available fields see [`mod@isactiver7`] module*/
pub type ISACTIVER7 = crate::Reg<isactiver7::ISACTIVER7rs>;
///For interrupts ID
pub mod isactiver7;
/**ISACTIVER8 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISACTIVER8)

For information about available fields see [`mod@isactiver8`] module*/
pub type ISACTIVER8 = crate::Reg<isactiver8::ISACTIVER8rs>;
///For interrupts ID
pub mod isactiver8;
/**ICACTIVER0 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icactiver0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icactiver0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICACTIVER0)

For information about available fields see [`mod@icactiver0`] module*/
pub type ICACTIVER0 = crate::Reg<icactiver0::ICACTIVER0rs>;
///For interrupts ID
pub mod icactiver0;
/**ICACTIVER1 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icactiver1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icactiver1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICACTIVER1)

For information about available fields see [`mod@icactiver1`] module*/
pub type ICACTIVER1 = crate::Reg<icactiver1::ICACTIVER1rs>;
///For interrupts ID
pub mod icactiver1;
/**ICACTIVER2 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icactiver2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icactiver2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICACTIVER2)

For information about available fields see [`mod@icactiver2`] module*/
pub type ICACTIVER2 = crate::Reg<icactiver2::ICACTIVER2rs>;
///For interrupts ID
pub mod icactiver2;
/**ICACTIVER3 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icactiver3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icactiver3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICACTIVER3)

For information about available fields see [`mod@icactiver3`] module*/
pub type ICACTIVER3 = crate::Reg<icactiver3::ICACTIVER3rs>;
///For interrupts ID
pub mod icactiver3;
/**ICACTIVER4 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icactiver4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icactiver4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICACTIVER4)

For information about available fields see [`mod@icactiver4`] module*/
pub type ICACTIVER4 = crate::Reg<icactiver4::ICACTIVER4rs>;
///For interrupts ID
pub mod icactiver4;
/**ICACTIVER5 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icactiver5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icactiver5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICACTIVER5)

For information about available fields see [`mod@icactiver5`] module*/
pub type ICACTIVER5 = crate::Reg<icactiver5::ICACTIVER5rs>;
///For interrupts ID
pub mod icactiver5;
/**ICACTIVER6 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icactiver6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icactiver6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICACTIVER6)

For information about available fields see [`mod@icactiver6`] module*/
pub type ICACTIVER6 = crate::Reg<icactiver6::ICACTIVER6rs>;
///For interrupts ID
pub mod icactiver6;
/**ICACTIVER7 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icactiver7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icactiver7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICACTIVER7)

For information about available fields see [`mod@icactiver7`] module*/
pub type ICACTIVER7 = crate::Reg<icactiver7::ICACTIVER7rs>;
///For interrupts ID
pub mod icactiver7;
/**ICACTIVER8 (rw) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`icactiver8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icactiver8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICACTIVER8)

For information about available fields see [`mod@icactiver8`] module*/
pub type ICACTIVER8 = crate::Reg<icactiver8::ICACTIVER8rs>;
///For interrupts ID
pub mod icactiver8;
/**IPRIORITYR0 (rw) register accessor: GICD interrupt priority register 0

You can [`read`](crate::Reg::read) this register and get [`ipriorityr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR0)

For information about available fields see [`mod@ipriorityr0`] module*/
pub type IPRIORITYR0 = crate::Reg<ipriorityr0::IPRIORITYR0rs>;
///GICD interrupt priority register 0
pub mod ipriorityr0;
/**IPRIORITYR1 (rw) register accessor: GICD interrupt priority register 1

You can [`read`](crate::Reg::read) this register and get [`ipriorityr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR1)

For information about available fields see [`mod@ipriorityr1`] module*/
pub type IPRIORITYR1 = crate::Reg<ipriorityr1::IPRIORITYR1rs>;
///GICD interrupt priority register 1
pub mod ipriorityr1;
/**IPRIORITYR2 (rw) register accessor: GICD interrupt priority register 2

You can [`read`](crate::Reg::read) this register and get [`ipriorityr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR2)

For information about available fields see [`mod@ipriorityr2`] module*/
pub type IPRIORITYR2 = crate::Reg<ipriorityr2::IPRIORITYR2rs>;
///GICD interrupt priority register 2
pub mod ipriorityr2;
/**IPRIORITYR3 (rw) register accessor: GICD interrupt priority register 3

You can [`read`](crate::Reg::read) this register and get [`ipriorityr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR3)

For information about available fields see [`mod@ipriorityr3`] module*/
pub type IPRIORITYR3 = crate::Reg<ipriorityr3::IPRIORITYR3rs>;
///GICD interrupt priority register 3
pub mod ipriorityr3;
/**IPRIORITYR4 (rw) register accessor: GICD interrupt priority register 4

You can [`read`](crate::Reg::read) this register and get [`ipriorityr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR4)

For information about available fields see [`mod@ipriorityr4`] module*/
pub type IPRIORITYR4 = crate::Reg<ipriorityr4::IPRIORITYR4rs>;
///GICD interrupt priority register 4
pub mod ipriorityr4;
/**IPRIORITYR5 (rw) register accessor: GICD interrupt priority register 5

You can [`read`](crate::Reg::read) this register and get [`ipriorityr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR5)

For information about available fields see [`mod@ipriorityr5`] module*/
pub type IPRIORITYR5 = crate::Reg<ipriorityr5::IPRIORITYR5rs>;
///GICD interrupt priority register 5
pub mod ipriorityr5;
/**IPRIORITYR6 (rw) register accessor: GICD interrupt priority register 6

You can [`read`](crate::Reg::read) this register and get [`ipriorityr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR6)

For information about available fields see [`mod@ipriorityr6`] module*/
pub type IPRIORITYR6 = crate::Reg<ipriorityr6::IPRIORITYR6rs>;
///GICD interrupt priority register 6
pub mod ipriorityr6;
/**IPRIORITYR7 (rw) register accessor: GICD interrupt priority register 7

You can [`read`](crate::Reg::read) this register and get [`ipriorityr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR7)

For information about available fields see [`mod@ipriorityr7`] module*/
pub type IPRIORITYR7 = crate::Reg<ipriorityr7::IPRIORITYR7rs>;
///GICD interrupt priority register 7
pub mod ipriorityr7;
/**IPRIORITYR8 (rw) register accessor: GICD interrupt priority register 8

You can [`read`](crate::Reg::read) this register and get [`ipriorityr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR8)

For information about available fields see [`mod@ipriorityr8`] module*/
pub type IPRIORITYR8 = crate::Reg<ipriorityr8::IPRIORITYR8rs>;
///GICD interrupt priority register 8
pub mod ipriorityr8;
/**IPRIORITYR9 (rw) register accessor: GICD interrupt priority register 9

You can [`read`](crate::Reg::read) this register and get [`ipriorityr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR9)

For information about available fields see [`mod@ipriorityr9`] module*/
pub type IPRIORITYR9 = crate::Reg<ipriorityr9::IPRIORITYR9rs>;
///GICD interrupt priority register 9
pub mod ipriorityr9;
/**IPRIORITYR10 (rw) register accessor: GICD interrupt priority register 10

You can [`read`](crate::Reg::read) this register and get [`ipriorityr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR10)

For information about available fields see [`mod@ipriorityr10`] module*/
pub type IPRIORITYR10 = crate::Reg<ipriorityr10::IPRIORITYR10rs>;
///GICD interrupt priority register 10
pub mod ipriorityr10;
/**IPRIORITYR11 (rw) register accessor: GICD interrupt priority register 11

You can [`read`](crate::Reg::read) this register and get [`ipriorityr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR11)

For information about available fields see [`mod@ipriorityr11`] module*/
pub type IPRIORITYR11 = crate::Reg<ipriorityr11::IPRIORITYR11rs>;
///GICD interrupt priority register 11
pub mod ipriorityr11;
/**IPRIORITYR12 (rw) register accessor: GICD interrupt priority register 12

You can [`read`](crate::Reg::read) this register and get [`ipriorityr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR12)

For information about available fields see [`mod@ipriorityr12`] module*/
pub type IPRIORITYR12 = crate::Reg<ipriorityr12::IPRIORITYR12rs>;
///GICD interrupt priority register 12
pub mod ipriorityr12;
/**IPRIORITYR13 (rw) register accessor: GICD interrupt priority register 13

You can [`read`](crate::Reg::read) this register and get [`ipriorityr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR13)

For information about available fields see [`mod@ipriorityr13`] module*/
pub type IPRIORITYR13 = crate::Reg<ipriorityr13::IPRIORITYR13rs>;
///GICD interrupt priority register 13
pub mod ipriorityr13;
/**IPRIORITYR14 (rw) register accessor: GICD interrupt priority register 14

You can [`read`](crate::Reg::read) this register and get [`ipriorityr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR14)

For information about available fields see [`mod@ipriorityr14`] module*/
pub type IPRIORITYR14 = crate::Reg<ipriorityr14::IPRIORITYR14rs>;
///GICD interrupt priority register 14
pub mod ipriorityr14;
/**IPRIORITYR15 (rw) register accessor: GICD interrupt priority register 15

You can [`read`](crate::Reg::read) this register and get [`ipriorityr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR15)

For information about available fields see [`mod@ipriorityr15`] module*/
pub type IPRIORITYR15 = crate::Reg<ipriorityr15::IPRIORITYR15rs>;
///GICD interrupt priority register 15
pub mod ipriorityr15;
/**IPRIORITYR16 (rw) register accessor: GICD interrupt priority register 16

You can [`read`](crate::Reg::read) this register and get [`ipriorityr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR16)

For information about available fields see [`mod@ipriorityr16`] module*/
pub type IPRIORITYR16 = crate::Reg<ipriorityr16::IPRIORITYR16rs>;
///GICD interrupt priority register 16
pub mod ipriorityr16;
/**IPRIORITYR17 (rw) register accessor: GICD interrupt priority register 17

You can [`read`](crate::Reg::read) this register and get [`ipriorityr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR17)

For information about available fields see [`mod@ipriorityr17`] module*/
pub type IPRIORITYR17 = crate::Reg<ipriorityr17::IPRIORITYR17rs>;
///GICD interrupt priority register 17
pub mod ipriorityr17;
/**IPRIORITYR18 (rw) register accessor: GICD interrupt priority register 18

You can [`read`](crate::Reg::read) this register and get [`ipriorityr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR18)

For information about available fields see [`mod@ipriorityr18`] module*/
pub type IPRIORITYR18 = crate::Reg<ipriorityr18::IPRIORITYR18rs>;
///GICD interrupt priority register 18
pub mod ipriorityr18;
/**IPRIORITYR19 (rw) register accessor: GICD interrupt priority register 19

You can [`read`](crate::Reg::read) this register and get [`ipriorityr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR19)

For information about available fields see [`mod@ipriorityr19`] module*/
pub type IPRIORITYR19 = crate::Reg<ipriorityr19::IPRIORITYR19rs>;
///GICD interrupt priority register 19
pub mod ipriorityr19;
/**IPRIORITYR20 (rw) register accessor: GICD interrupt priority register 20

You can [`read`](crate::Reg::read) this register and get [`ipriorityr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR20)

For information about available fields see [`mod@ipriorityr20`] module*/
pub type IPRIORITYR20 = crate::Reg<ipriorityr20::IPRIORITYR20rs>;
///GICD interrupt priority register 20
pub mod ipriorityr20;
/**IPRIORITYR21 (rw) register accessor: GICD interrupt priority register 21

You can [`read`](crate::Reg::read) this register and get [`ipriorityr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR21)

For information about available fields see [`mod@ipriorityr21`] module*/
pub type IPRIORITYR21 = crate::Reg<ipriorityr21::IPRIORITYR21rs>;
///GICD interrupt priority register 21
pub mod ipriorityr21;
/**IPRIORITYR22 (rw) register accessor: GICD interrupt priority register 22

You can [`read`](crate::Reg::read) this register and get [`ipriorityr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR22)

For information about available fields see [`mod@ipriorityr22`] module*/
pub type IPRIORITYR22 = crate::Reg<ipriorityr22::IPRIORITYR22rs>;
///GICD interrupt priority register 22
pub mod ipriorityr22;
/**IPRIORITYR23 (rw) register accessor: GICD interrupt priority register 23

You can [`read`](crate::Reg::read) this register and get [`ipriorityr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR23)

For information about available fields see [`mod@ipriorityr23`] module*/
pub type IPRIORITYR23 = crate::Reg<ipriorityr23::IPRIORITYR23rs>;
///GICD interrupt priority register 23
pub mod ipriorityr23;
/**IPRIORITYR24 (rw) register accessor: GICD interrupt priority register 24

You can [`read`](crate::Reg::read) this register and get [`ipriorityr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR24)

For information about available fields see [`mod@ipriorityr24`] module*/
pub type IPRIORITYR24 = crate::Reg<ipriorityr24::IPRIORITYR24rs>;
///GICD interrupt priority register 24
pub mod ipriorityr24;
/**IPRIORITYR25 (rw) register accessor: GICD interrupt priority register 25

You can [`read`](crate::Reg::read) this register and get [`ipriorityr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR25)

For information about available fields see [`mod@ipriorityr25`] module*/
pub type IPRIORITYR25 = crate::Reg<ipriorityr25::IPRIORITYR25rs>;
///GICD interrupt priority register 25
pub mod ipriorityr25;
/**IPRIORITYR26 (rw) register accessor: GICD interrupt priority register 26

You can [`read`](crate::Reg::read) this register and get [`ipriorityr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR26)

For information about available fields see [`mod@ipriorityr26`] module*/
pub type IPRIORITYR26 = crate::Reg<ipriorityr26::IPRIORITYR26rs>;
///GICD interrupt priority register 26
pub mod ipriorityr26;
/**IPRIORITYR27 (rw) register accessor: GICD interrupt priority register 27

You can [`read`](crate::Reg::read) this register and get [`ipriorityr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR27)

For information about available fields see [`mod@ipriorityr27`] module*/
pub type IPRIORITYR27 = crate::Reg<ipriorityr27::IPRIORITYR27rs>;
///GICD interrupt priority register 27
pub mod ipriorityr27;
/**IPRIORITYR28 (rw) register accessor: GICD interrupt priority register 28

You can [`read`](crate::Reg::read) this register and get [`ipriorityr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR28)

For information about available fields see [`mod@ipriorityr28`] module*/
pub type IPRIORITYR28 = crate::Reg<ipriorityr28::IPRIORITYR28rs>;
///GICD interrupt priority register 28
pub mod ipriorityr28;
/**IPRIORITYR29 (rw) register accessor: GICD interrupt priority register 29

You can [`read`](crate::Reg::read) this register and get [`ipriorityr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR29)

For information about available fields see [`mod@ipriorityr29`] module*/
pub type IPRIORITYR29 = crate::Reg<ipriorityr29::IPRIORITYR29rs>;
///GICD interrupt priority register 29
pub mod ipriorityr29;
/**IPRIORITYR30 (rw) register accessor: GICD interrupt priority register 30

You can [`read`](crate::Reg::read) this register and get [`ipriorityr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR30)

For information about available fields see [`mod@ipriorityr30`] module*/
pub type IPRIORITYR30 = crate::Reg<ipriorityr30::IPRIORITYR30rs>;
///GICD interrupt priority register 30
pub mod ipriorityr30;
/**IPRIORITYR31 (rw) register accessor: GICD interrupt priority register 31

You can [`read`](crate::Reg::read) this register and get [`ipriorityr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR31)

For information about available fields see [`mod@ipriorityr31`] module*/
pub type IPRIORITYR31 = crate::Reg<ipriorityr31::IPRIORITYR31rs>;
///GICD interrupt priority register 31
pub mod ipriorityr31;
/**IPRIORITYR32 (rw) register accessor: GICD interrupt priority register 32

You can [`read`](crate::Reg::read) this register and get [`ipriorityr32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR32)

For information about available fields see [`mod@ipriorityr32`] module*/
pub type IPRIORITYR32 = crate::Reg<ipriorityr32::IPRIORITYR32rs>;
///GICD interrupt priority register 32
pub mod ipriorityr32;
/**IPRIORITYR33 (rw) register accessor: GICD interrupt priority register 33

You can [`read`](crate::Reg::read) this register and get [`ipriorityr33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR33)

For information about available fields see [`mod@ipriorityr33`] module*/
pub type IPRIORITYR33 = crate::Reg<ipriorityr33::IPRIORITYR33rs>;
///GICD interrupt priority register 33
pub mod ipriorityr33;
/**IPRIORITYR34 (rw) register accessor: GICD interrupt priority register 34

You can [`read`](crate::Reg::read) this register and get [`ipriorityr34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR34)

For information about available fields see [`mod@ipriorityr34`] module*/
pub type IPRIORITYR34 = crate::Reg<ipriorityr34::IPRIORITYR34rs>;
///GICD interrupt priority register 34
pub mod ipriorityr34;
/**IPRIORITYR35 (rw) register accessor: GICD interrupt priority register 35

You can [`read`](crate::Reg::read) this register and get [`ipriorityr35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR35)

For information about available fields see [`mod@ipriorityr35`] module*/
pub type IPRIORITYR35 = crate::Reg<ipriorityr35::IPRIORITYR35rs>;
///GICD interrupt priority register 35
pub mod ipriorityr35;
/**IPRIORITYR36 (rw) register accessor: GICD interrupt priority register 36

You can [`read`](crate::Reg::read) this register and get [`ipriorityr36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR36)

For information about available fields see [`mod@ipriorityr36`] module*/
pub type IPRIORITYR36 = crate::Reg<ipriorityr36::IPRIORITYR36rs>;
///GICD interrupt priority register 36
pub mod ipriorityr36;
/**IPRIORITYR37 (rw) register accessor: GICD interrupt priority register 37

You can [`read`](crate::Reg::read) this register and get [`ipriorityr37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR37)

For information about available fields see [`mod@ipriorityr37`] module*/
pub type IPRIORITYR37 = crate::Reg<ipriorityr37::IPRIORITYR37rs>;
///GICD interrupt priority register 37
pub mod ipriorityr37;
/**IPRIORITYR38 (rw) register accessor: GICD interrupt priority register 38

You can [`read`](crate::Reg::read) this register and get [`ipriorityr38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR38)

For information about available fields see [`mod@ipriorityr38`] module*/
pub type IPRIORITYR38 = crate::Reg<ipriorityr38::IPRIORITYR38rs>;
///GICD interrupt priority register 38
pub mod ipriorityr38;
/**IPRIORITYR39 (rw) register accessor: GICD interrupt priority register 39

You can [`read`](crate::Reg::read) this register and get [`ipriorityr39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR39)

For information about available fields see [`mod@ipriorityr39`] module*/
pub type IPRIORITYR39 = crate::Reg<ipriorityr39::IPRIORITYR39rs>;
///GICD interrupt priority register 39
pub mod ipriorityr39;
/**IPRIORITYR40 (rw) register accessor: GICD interrupt priority register 40

You can [`read`](crate::Reg::read) this register and get [`ipriorityr40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR40)

For information about available fields see [`mod@ipriorityr40`] module*/
pub type IPRIORITYR40 = crate::Reg<ipriorityr40::IPRIORITYR40rs>;
///GICD interrupt priority register 40
pub mod ipriorityr40;
/**IPRIORITYR41 (rw) register accessor: GICD interrupt priority register 41

You can [`read`](crate::Reg::read) this register and get [`ipriorityr41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR41)

For information about available fields see [`mod@ipriorityr41`] module*/
pub type IPRIORITYR41 = crate::Reg<ipriorityr41::IPRIORITYR41rs>;
///GICD interrupt priority register 41
pub mod ipriorityr41;
/**IPRIORITYR42 (rw) register accessor: GICD interrupt priority register 42

You can [`read`](crate::Reg::read) this register and get [`ipriorityr42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR42)

For information about available fields see [`mod@ipriorityr42`] module*/
pub type IPRIORITYR42 = crate::Reg<ipriorityr42::IPRIORITYR42rs>;
///GICD interrupt priority register 42
pub mod ipriorityr42;
/**IPRIORITYR43 (rw) register accessor: GICD interrupt priority register 43

You can [`read`](crate::Reg::read) this register and get [`ipriorityr43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR43)

For information about available fields see [`mod@ipriorityr43`] module*/
pub type IPRIORITYR43 = crate::Reg<ipriorityr43::IPRIORITYR43rs>;
///GICD interrupt priority register 43
pub mod ipriorityr43;
/**IPRIORITYR44 (rw) register accessor: GICD interrupt priority register 44

You can [`read`](crate::Reg::read) this register and get [`ipriorityr44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR44)

For information about available fields see [`mod@ipriorityr44`] module*/
pub type IPRIORITYR44 = crate::Reg<ipriorityr44::IPRIORITYR44rs>;
///GICD interrupt priority register 44
pub mod ipriorityr44;
/**IPRIORITYR45 (rw) register accessor: GICD interrupt priority register 45

You can [`read`](crate::Reg::read) this register and get [`ipriorityr45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR45)

For information about available fields see [`mod@ipriorityr45`] module*/
pub type IPRIORITYR45 = crate::Reg<ipriorityr45::IPRIORITYR45rs>;
///GICD interrupt priority register 45
pub mod ipriorityr45;
/**IPRIORITYR46 (rw) register accessor: GICD interrupt priority register 46

You can [`read`](crate::Reg::read) this register and get [`ipriorityr46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR46)

For information about available fields see [`mod@ipriorityr46`] module*/
pub type IPRIORITYR46 = crate::Reg<ipriorityr46::IPRIORITYR46rs>;
///GICD interrupt priority register 46
pub mod ipriorityr46;
/**IPRIORITYR47 (rw) register accessor: GICD interrupt priority register 47

You can [`read`](crate::Reg::read) this register and get [`ipriorityr47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR47)

For information about available fields see [`mod@ipriorityr47`] module*/
pub type IPRIORITYR47 = crate::Reg<ipriorityr47::IPRIORITYR47rs>;
///GICD interrupt priority register 47
pub mod ipriorityr47;
/**IPRIORITYR48 (rw) register accessor: GICD interrupt priority register 48

You can [`read`](crate::Reg::read) this register and get [`ipriorityr48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR48)

For information about available fields see [`mod@ipriorityr48`] module*/
pub type IPRIORITYR48 = crate::Reg<ipriorityr48::IPRIORITYR48rs>;
///GICD interrupt priority register 48
pub mod ipriorityr48;
/**IPRIORITYR49 (rw) register accessor: GICD interrupt priority register 49

You can [`read`](crate::Reg::read) this register and get [`ipriorityr49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR49)

For information about available fields see [`mod@ipriorityr49`] module*/
pub type IPRIORITYR49 = crate::Reg<ipriorityr49::IPRIORITYR49rs>;
///GICD interrupt priority register 49
pub mod ipriorityr49;
/**IPRIORITYR50 (rw) register accessor: GICD interrupt priority register 50

You can [`read`](crate::Reg::read) this register and get [`ipriorityr50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR50)

For information about available fields see [`mod@ipriorityr50`] module*/
pub type IPRIORITYR50 = crate::Reg<ipriorityr50::IPRIORITYR50rs>;
///GICD interrupt priority register 50
pub mod ipriorityr50;
/**IPRIORITYR51 (rw) register accessor: GICD interrupt priority register 51

You can [`read`](crate::Reg::read) this register and get [`ipriorityr51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR51)

For information about available fields see [`mod@ipriorityr51`] module*/
pub type IPRIORITYR51 = crate::Reg<ipriorityr51::IPRIORITYR51rs>;
///GICD interrupt priority register 51
pub mod ipriorityr51;
/**IPRIORITYR52 (rw) register accessor: GICD interrupt priority register 52

You can [`read`](crate::Reg::read) this register and get [`ipriorityr52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR52)

For information about available fields see [`mod@ipriorityr52`] module*/
pub type IPRIORITYR52 = crate::Reg<ipriorityr52::IPRIORITYR52rs>;
///GICD interrupt priority register 52
pub mod ipriorityr52;
/**IPRIORITYR53 (rw) register accessor: GICD interrupt priority register 53

You can [`read`](crate::Reg::read) this register and get [`ipriorityr53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR53)

For information about available fields see [`mod@ipriorityr53`] module*/
pub type IPRIORITYR53 = crate::Reg<ipriorityr53::IPRIORITYR53rs>;
///GICD interrupt priority register 53
pub mod ipriorityr53;
/**IPRIORITYR54 (rw) register accessor: GICD interrupt priority register 54

You can [`read`](crate::Reg::read) this register and get [`ipriorityr54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR54)

For information about available fields see [`mod@ipriorityr54`] module*/
pub type IPRIORITYR54 = crate::Reg<ipriorityr54::IPRIORITYR54rs>;
///GICD interrupt priority register 54
pub mod ipriorityr54;
/**IPRIORITYR55 (rw) register accessor: GICD interrupt priority register 55

You can [`read`](crate::Reg::read) this register and get [`ipriorityr55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR55)

For information about available fields see [`mod@ipriorityr55`] module*/
pub type IPRIORITYR55 = crate::Reg<ipriorityr55::IPRIORITYR55rs>;
///GICD interrupt priority register 55
pub mod ipriorityr55;
/**IPRIORITYR56 (rw) register accessor: GICD interrupt priority register 56

You can [`read`](crate::Reg::read) this register and get [`ipriorityr56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR56)

For information about available fields see [`mod@ipriorityr56`] module*/
pub type IPRIORITYR56 = crate::Reg<ipriorityr56::IPRIORITYR56rs>;
///GICD interrupt priority register 56
pub mod ipriorityr56;
/**IPRIORITYR57 (rw) register accessor: GICD interrupt priority register 57

You can [`read`](crate::Reg::read) this register and get [`ipriorityr57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR57)

For information about available fields see [`mod@ipriorityr57`] module*/
pub type IPRIORITYR57 = crate::Reg<ipriorityr57::IPRIORITYR57rs>;
///GICD interrupt priority register 57
pub mod ipriorityr57;
/**IPRIORITYR58 (rw) register accessor: GICD interrupt priority register 58

You can [`read`](crate::Reg::read) this register and get [`ipriorityr58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR58)

For information about available fields see [`mod@ipriorityr58`] module*/
pub type IPRIORITYR58 = crate::Reg<ipriorityr58::IPRIORITYR58rs>;
///GICD interrupt priority register 58
pub mod ipriorityr58;
/**IPRIORITYR59 (rw) register accessor: GICD interrupt priority register 59

You can [`read`](crate::Reg::read) this register and get [`ipriorityr59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR59)

For information about available fields see [`mod@ipriorityr59`] module*/
pub type IPRIORITYR59 = crate::Reg<ipriorityr59::IPRIORITYR59rs>;
///GICD interrupt priority register 59
pub mod ipriorityr59;
/**IPRIORITYR60 (rw) register accessor: GICD interrupt priority register 60

You can [`read`](crate::Reg::read) this register and get [`ipriorityr60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR60)

For information about available fields see [`mod@ipriorityr60`] module*/
pub type IPRIORITYR60 = crate::Reg<ipriorityr60::IPRIORITYR60rs>;
///GICD interrupt priority register 60
pub mod ipriorityr60;
/**IPRIORITYR61 (rw) register accessor: GICD interrupt priority register 61

You can [`read`](crate::Reg::read) this register and get [`ipriorityr61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR61)

For information about available fields see [`mod@ipriorityr61`] module*/
pub type IPRIORITYR61 = crate::Reg<ipriorityr61::IPRIORITYR61rs>;
///GICD interrupt priority register 61
pub mod ipriorityr61;
/**IPRIORITYR62 (rw) register accessor: GICD interrupt priority register 62

You can [`read`](crate::Reg::read) this register and get [`ipriorityr62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR62)

For information about available fields see [`mod@ipriorityr62`] module*/
pub type IPRIORITYR62 = crate::Reg<ipriorityr62::IPRIORITYR62rs>;
///GICD interrupt priority register 62
pub mod ipriorityr62;
/**IPRIORITYR63 (rw) register accessor: GICD interrupt priority register 63

You can [`read`](crate::Reg::read) this register and get [`ipriorityr63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR63)

For information about available fields see [`mod@ipriorityr63`] module*/
pub type IPRIORITYR63 = crate::Reg<ipriorityr63::IPRIORITYR63rs>;
///GICD interrupt priority register 63
pub mod ipriorityr63;
/**IPRIORITYR64 (rw) register accessor: GICD interrupt priority register 64

You can [`read`](crate::Reg::read) this register and get [`ipriorityr64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR64)

For information about available fields see [`mod@ipriorityr64`] module*/
pub type IPRIORITYR64 = crate::Reg<ipriorityr64::IPRIORITYR64rs>;
///GICD interrupt priority register 64
pub mod ipriorityr64;
/**IPRIORITYR65 (rw) register accessor: GICD interrupt priority register 65

You can [`read`](crate::Reg::read) this register and get [`ipriorityr65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR65)

For information about available fields see [`mod@ipriorityr65`] module*/
pub type IPRIORITYR65 = crate::Reg<ipriorityr65::IPRIORITYR65rs>;
///GICD interrupt priority register 65
pub mod ipriorityr65;
/**IPRIORITYR66 (rw) register accessor: GICD interrupt priority register 66

You can [`read`](crate::Reg::read) this register and get [`ipriorityr66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR66)

For information about available fields see [`mod@ipriorityr66`] module*/
pub type IPRIORITYR66 = crate::Reg<ipriorityr66::IPRIORITYR66rs>;
///GICD interrupt priority register 66
pub mod ipriorityr66;
/**IPRIORITYR67 (rw) register accessor: GICD interrupt priority register 67

You can [`read`](crate::Reg::read) this register and get [`ipriorityr67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR67)

For information about available fields see [`mod@ipriorityr67`] module*/
pub type IPRIORITYR67 = crate::Reg<ipriorityr67::IPRIORITYR67rs>;
///GICD interrupt priority register 67
pub mod ipriorityr67;
/**IPRIORITYR68 (rw) register accessor: GICD interrupt priority register 68

You can [`read`](crate::Reg::read) this register and get [`ipriorityr68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR68)

For information about available fields see [`mod@ipriorityr68`] module*/
pub type IPRIORITYR68 = crate::Reg<ipriorityr68::IPRIORITYR68rs>;
///GICD interrupt priority register 68
pub mod ipriorityr68;
/**IPRIORITYR69 (rw) register accessor: GICD interrupt priority register 69

You can [`read`](crate::Reg::read) this register and get [`ipriorityr69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR69)

For information about available fields see [`mod@ipriorityr69`] module*/
pub type IPRIORITYR69 = crate::Reg<ipriorityr69::IPRIORITYR69rs>;
///GICD interrupt priority register 69
pub mod ipriorityr69;
/**IPRIORITYR70 (rw) register accessor: GICD interrupt priority register 70

You can [`read`](crate::Reg::read) this register and get [`ipriorityr70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR70)

For information about available fields see [`mod@ipriorityr70`] module*/
pub type IPRIORITYR70 = crate::Reg<ipriorityr70::IPRIORITYR70rs>;
///GICD interrupt priority register 70
pub mod ipriorityr70;
/**IPRIORITYR71 (rw) register accessor: GICD interrupt priority register 71

You can [`read`](crate::Reg::read) this register and get [`ipriorityr71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipriorityr71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:IPRIORITYR71)

For information about available fields see [`mod@ipriorityr71`] module*/
pub type IPRIORITYR71 = crate::Reg<ipriorityr71::IPRIORITYR71rs>;
///GICD interrupt priority register 71
pub mod ipriorityr71;
/**ITARGETSR0 (r) register accessor: For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.

You can [`read`](crate::Reg::read) this register and get [`itargetsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR0)

For information about available fields see [`mod@itargetsr0`] module*/
pub type ITARGETSR0 = crate::Reg<itargetsr0::ITARGETSR0rs>;
///For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
pub mod itargetsr0;
/**ITARGETSR1 (r) register accessor: For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.

You can [`read`](crate::Reg::read) this register and get [`itargetsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR1)

For information about available fields see [`mod@itargetsr1`] module*/
pub type ITARGETSR1 = crate::Reg<itargetsr1::ITARGETSR1rs>;
///For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
pub mod itargetsr1;
/**ITARGETSR2 (r) register accessor: For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.

You can [`read`](crate::Reg::read) this register and get [`itargetsr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR2)

For information about available fields see [`mod@itargetsr2`] module*/
pub type ITARGETSR2 = crate::Reg<itargetsr2::ITARGETSR2rs>;
///For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
pub mod itargetsr2;
/**ITARGETSR3 (r) register accessor: For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.

You can [`read`](crate::Reg::read) this register and get [`itargetsr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR3)

For information about available fields see [`mod@itargetsr3`] module*/
pub type ITARGETSR3 = crate::Reg<itargetsr3::ITARGETSR3rs>;
///For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
pub mod itargetsr3;
/**ITARGETSR4 (r) register accessor: For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.

You can [`read`](crate::Reg::read) this register and get [`itargetsr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR4)

For information about available fields see [`mod@itargetsr4`] module*/
pub type ITARGETSR4 = crate::Reg<itargetsr4::ITARGETSR4rs>;
///For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
pub mod itargetsr4;
/**ITARGETSR5 (r) register accessor: For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.

You can [`read`](crate::Reg::read) this register and get [`itargetsr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR5)

For information about available fields see [`mod@itargetsr5`] module*/
pub type ITARGETSR5 = crate::Reg<itargetsr5::ITARGETSR5rs>;
///For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
pub mod itargetsr5;
/**ITARGETSR6 (r) register accessor: For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.

You can [`read`](crate::Reg::read) this register and get [`itargetsr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR6)

For information about available fields see [`mod@itargetsr6`] module*/
pub type ITARGETSR6 = crate::Reg<itargetsr6::ITARGETSR6rs>;
///For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
pub mod itargetsr6;
/**ITARGETSR7 (r) register accessor: For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.

You can [`read`](crate::Reg::read) this register and get [`itargetsr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR7)

For information about available fields see [`mod@itargetsr7`] module*/
pub type ITARGETSR7 = crate::Reg<itargetsr7::ITARGETSR7rs>;
///For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.
pub mod itargetsr7;
/**ITARGETSR8 (rw) register accessor: GICD interrupt processor target register 8

You can [`read`](crate::Reg::read) this register and get [`itargetsr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR8)

For information about available fields see [`mod@itargetsr8`] module*/
pub type ITARGETSR8 = crate::Reg<itargetsr8::ITARGETSR8rs>;
///GICD interrupt processor target register 8
pub mod itargetsr8;
/**ITARGETSR9 (rw) register accessor: GICD interrupt processor target register 9

You can [`read`](crate::Reg::read) this register and get [`itargetsr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR9)

For information about available fields see [`mod@itargetsr9`] module*/
pub type ITARGETSR9 = crate::Reg<itargetsr9::ITARGETSR9rs>;
///GICD interrupt processor target register 9
pub mod itargetsr9;
/**ITARGETSR10 (rw) register accessor: GICD interrupt processor target register 10

You can [`read`](crate::Reg::read) this register and get [`itargetsr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR10)

For information about available fields see [`mod@itargetsr10`] module*/
pub type ITARGETSR10 = crate::Reg<itargetsr10::ITARGETSR10rs>;
///GICD interrupt processor target register 10
pub mod itargetsr10;
/**ITARGETSR11 (rw) register accessor: GICD interrupt processor target register 11

You can [`read`](crate::Reg::read) this register and get [`itargetsr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR11)

For information about available fields see [`mod@itargetsr11`] module*/
pub type ITARGETSR11 = crate::Reg<itargetsr11::ITARGETSR11rs>;
///GICD interrupt processor target register 11
pub mod itargetsr11;
/**ITARGETSR12 (rw) register accessor: GICD interrupt processor target register 12

You can [`read`](crate::Reg::read) this register and get [`itargetsr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR12)

For information about available fields see [`mod@itargetsr12`] module*/
pub type ITARGETSR12 = crate::Reg<itargetsr12::ITARGETSR12rs>;
///GICD interrupt processor target register 12
pub mod itargetsr12;
/**ITARGETSR13 (rw) register accessor: GICD interrupt processor target register 13

You can [`read`](crate::Reg::read) this register and get [`itargetsr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR13)

For information about available fields see [`mod@itargetsr13`] module*/
pub type ITARGETSR13 = crate::Reg<itargetsr13::ITARGETSR13rs>;
///GICD interrupt processor target register 13
pub mod itargetsr13;
/**ITARGETSR14 (rw) register accessor: GICD interrupt processor target register 14

You can [`read`](crate::Reg::read) this register and get [`itargetsr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR14)

For information about available fields see [`mod@itargetsr14`] module*/
pub type ITARGETSR14 = crate::Reg<itargetsr14::ITARGETSR14rs>;
///GICD interrupt processor target register 14
pub mod itargetsr14;
/**ITARGETSR15 (rw) register accessor: GICD interrupt processor target register 15

You can [`read`](crate::Reg::read) this register and get [`itargetsr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR15)

For information about available fields see [`mod@itargetsr15`] module*/
pub type ITARGETSR15 = crate::Reg<itargetsr15::ITARGETSR15rs>;
///GICD interrupt processor target register 15
pub mod itargetsr15;
/**ITARGETSR16 (rw) register accessor: GICD interrupt processor target register 16

You can [`read`](crate::Reg::read) this register and get [`itargetsr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR16)

For information about available fields see [`mod@itargetsr16`] module*/
pub type ITARGETSR16 = crate::Reg<itargetsr16::ITARGETSR16rs>;
///GICD interrupt processor target register 16
pub mod itargetsr16;
/**ITARGETSR17 (rw) register accessor: GICD interrupt processor target register 17

You can [`read`](crate::Reg::read) this register and get [`itargetsr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR17)

For information about available fields see [`mod@itargetsr17`] module*/
pub type ITARGETSR17 = crate::Reg<itargetsr17::ITARGETSR17rs>;
///GICD interrupt processor target register 17
pub mod itargetsr17;
/**ITARGETSR18 (rw) register accessor: GICD interrupt processor target register 18

You can [`read`](crate::Reg::read) this register and get [`itargetsr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR18)

For information about available fields see [`mod@itargetsr18`] module*/
pub type ITARGETSR18 = crate::Reg<itargetsr18::ITARGETSR18rs>;
///GICD interrupt processor target register 18
pub mod itargetsr18;
/**ITARGETSR19 (rw) register accessor: GICD interrupt processor target register 19

You can [`read`](crate::Reg::read) this register and get [`itargetsr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR19)

For information about available fields see [`mod@itargetsr19`] module*/
pub type ITARGETSR19 = crate::Reg<itargetsr19::ITARGETSR19rs>;
///GICD interrupt processor target register 19
pub mod itargetsr19;
/**ITARGETSR20 (rw) register accessor: GICD interrupt processor target register 20

You can [`read`](crate::Reg::read) this register and get [`itargetsr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR20)

For information about available fields see [`mod@itargetsr20`] module*/
pub type ITARGETSR20 = crate::Reg<itargetsr20::ITARGETSR20rs>;
///GICD interrupt processor target register 20
pub mod itargetsr20;
/**ITARGETSR21 (rw) register accessor: GICD interrupt processor target register 21

You can [`read`](crate::Reg::read) this register and get [`itargetsr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR21)

For information about available fields see [`mod@itargetsr21`] module*/
pub type ITARGETSR21 = crate::Reg<itargetsr21::ITARGETSR21rs>;
///GICD interrupt processor target register 21
pub mod itargetsr21;
/**ITARGETSR22 (rw) register accessor: GICD interrupt processor target register 22

You can [`read`](crate::Reg::read) this register and get [`itargetsr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR22)

For information about available fields see [`mod@itargetsr22`] module*/
pub type ITARGETSR22 = crate::Reg<itargetsr22::ITARGETSR22rs>;
///GICD interrupt processor target register 22
pub mod itargetsr22;
/**ITARGETSR23 (rw) register accessor: GICD interrupt processor target register 23

You can [`read`](crate::Reg::read) this register and get [`itargetsr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR23)

For information about available fields see [`mod@itargetsr23`] module*/
pub type ITARGETSR23 = crate::Reg<itargetsr23::ITARGETSR23rs>;
///GICD interrupt processor target register 23
pub mod itargetsr23;
/**ITARGETSR24 (rw) register accessor: GICD interrupt processor target register 24

You can [`read`](crate::Reg::read) this register and get [`itargetsr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR24)

For information about available fields see [`mod@itargetsr24`] module*/
pub type ITARGETSR24 = crate::Reg<itargetsr24::ITARGETSR24rs>;
///GICD interrupt processor target register 24
pub mod itargetsr24;
/**ITARGETSR25 (rw) register accessor: GICD interrupt processor target register 25

You can [`read`](crate::Reg::read) this register and get [`itargetsr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR25)

For information about available fields see [`mod@itargetsr25`] module*/
pub type ITARGETSR25 = crate::Reg<itargetsr25::ITARGETSR25rs>;
///GICD interrupt processor target register 25
pub mod itargetsr25;
/**ITARGETSR26 (rw) register accessor: GICD interrupt processor target register 26

You can [`read`](crate::Reg::read) this register and get [`itargetsr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR26)

For information about available fields see [`mod@itargetsr26`] module*/
pub type ITARGETSR26 = crate::Reg<itargetsr26::ITARGETSR26rs>;
///GICD interrupt processor target register 26
pub mod itargetsr26;
/**ITARGETSR27 (rw) register accessor: GICD interrupt processor target register 27

You can [`read`](crate::Reg::read) this register and get [`itargetsr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR27)

For information about available fields see [`mod@itargetsr27`] module*/
pub type ITARGETSR27 = crate::Reg<itargetsr27::ITARGETSR27rs>;
///GICD interrupt processor target register 27
pub mod itargetsr27;
/**ITARGETSR28 (rw) register accessor: GICD interrupt processor target register 28

You can [`read`](crate::Reg::read) this register and get [`itargetsr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR28)

For information about available fields see [`mod@itargetsr28`] module*/
pub type ITARGETSR28 = crate::Reg<itargetsr28::ITARGETSR28rs>;
///GICD interrupt processor target register 28
pub mod itargetsr28;
/**ITARGETSR29 (rw) register accessor: GICD interrupt processor target register 29

You can [`read`](crate::Reg::read) this register and get [`itargetsr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR29)

For information about available fields see [`mod@itargetsr29`] module*/
pub type ITARGETSR29 = crate::Reg<itargetsr29::ITARGETSR29rs>;
///GICD interrupt processor target register 29
pub mod itargetsr29;
/**ITARGETSR30 (rw) register accessor: GICD interrupt processor target register 30

You can [`read`](crate::Reg::read) this register and get [`itargetsr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR30)

For information about available fields see [`mod@itargetsr30`] module*/
pub type ITARGETSR30 = crate::Reg<itargetsr30::ITARGETSR30rs>;
///GICD interrupt processor target register 30
pub mod itargetsr30;
/**ITARGETSR31 (rw) register accessor: GICD interrupt processor target register 31

You can [`read`](crate::Reg::read) this register and get [`itargetsr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR31)

For information about available fields see [`mod@itargetsr31`] module*/
pub type ITARGETSR31 = crate::Reg<itargetsr31::ITARGETSR31rs>;
///GICD interrupt processor target register 31
pub mod itargetsr31;
/**ITARGETSR32 (rw) register accessor: GICD interrupt processor target register 32

You can [`read`](crate::Reg::read) this register and get [`itargetsr32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR32)

For information about available fields see [`mod@itargetsr32`] module*/
pub type ITARGETSR32 = crate::Reg<itargetsr32::ITARGETSR32rs>;
///GICD interrupt processor target register 32
pub mod itargetsr32;
/**ITARGETSR33 (rw) register accessor: GICD interrupt processor target register 33

You can [`read`](crate::Reg::read) this register and get [`itargetsr33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR33)

For information about available fields see [`mod@itargetsr33`] module*/
pub type ITARGETSR33 = crate::Reg<itargetsr33::ITARGETSR33rs>;
///GICD interrupt processor target register 33
pub mod itargetsr33;
/**ITARGETSR34 (rw) register accessor: GICD interrupt processor target register 34

You can [`read`](crate::Reg::read) this register and get [`itargetsr34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR34)

For information about available fields see [`mod@itargetsr34`] module*/
pub type ITARGETSR34 = crate::Reg<itargetsr34::ITARGETSR34rs>;
///GICD interrupt processor target register 34
pub mod itargetsr34;
/**ITARGETSR35 (rw) register accessor: GICD interrupt processor target register 35

You can [`read`](crate::Reg::read) this register and get [`itargetsr35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR35)

For information about available fields see [`mod@itargetsr35`] module*/
pub type ITARGETSR35 = crate::Reg<itargetsr35::ITARGETSR35rs>;
///GICD interrupt processor target register 35
pub mod itargetsr35;
/**ITARGETSR36 (rw) register accessor: GICD interrupt processor target register 36

You can [`read`](crate::Reg::read) this register and get [`itargetsr36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR36)

For information about available fields see [`mod@itargetsr36`] module*/
pub type ITARGETSR36 = crate::Reg<itargetsr36::ITARGETSR36rs>;
///GICD interrupt processor target register 36
pub mod itargetsr36;
/**ITARGETSR37 (rw) register accessor: GICD interrupt processor target register 37

You can [`read`](crate::Reg::read) this register and get [`itargetsr37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR37)

For information about available fields see [`mod@itargetsr37`] module*/
pub type ITARGETSR37 = crate::Reg<itargetsr37::ITARGETSR37rs>;
///GICD interrupt processor target register 37
pub mod itargetsr37;
/**ITARGETSR38 (rw) register accessor: GICD interrupt processor target register 38

You can [`read`](crate::Reg::read) this register and get [`itargetsr38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR38)

For information about available fields see [`mod@itargetsr38`] module*/
pub type ITARGETSR38 = crate::Reg<itargetsr38::ITARGETSR38rs>;
///GICD interrupt processor target register 38
pub mod itargetsr38;
/**ITARGETSR39 (rw) register accessor: GICD interrupt processor target register 39

You can [`read`](crate::Reg::read) this register and get [`itargetsr39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR39)

For information about available fields see [`mod@itargetsr39`] module*/
pub type ITARGETSR39 = crate::Reg<itargetsr39::ITARGETSR39rs>;
///GICD interrupt processor target register 39
pub mod itargetsr39;
/**ITARGETSR40 (rw) register accessor: GICD interrupt processor target register 40

You can [`read`](crate::Reg::read) this register and get [`itargetsr40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR40)

For information about available fields see [`mod@itargetsr40`] module*/
pub type ITARGETSR40 = crate::Reg<itargetsr40::ITARGETSR40rs>;
///GICD interrupt processor target register 40
pub mod itargetsr40;
/**ITARGETSR41 (rw) register accessor: GICD interrupt processor target register 41

You can [`read`](crate::Reg::read) this register and get [`itargetsr41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR41)

For information about available fields see [`mod@itargetsr41`] module*/
pub type ITARGETSR41 = crate::Reg<itargetsr41::ITARGETSR41rs>;
///GICD interrupt processor target register 41
pub mod itargetsr41;
/**ITARGETSR42 (rw) register accessor: GICD interrupt processor target register 42

You can [`read`](crate::Reg::read) this register and get [`itargetsr42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR42)

For information about available fields see [`mod@itargetsr42`] module*/
pub type ITARGETSR42 = crate::Reg<itargetsr42::ITARGETSR42rs>;
///GICD interrupt processor target register 42
pub mod itargetsr42;
/**ITARGETSR43 (rw) register accessor: GICD interrupt processor target register 43

You can [`read`](crate::Reg::read) this register and get [`itargetsr43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR43)

For information about available fields see [`mod@itargetsr43`] module*/
pub type ITARGETSR43 = crate::Reg<itargetsr43::ITARGETSR43rs>;
///GICD interrupt processor target register 43
pub mod itargetsr43;
/**ITARGETSR44 (rw) register accessor: GICD interrupt processor target register 44

You can [`read`](crate::Reg::read) this register and get [`itargetsr44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR44)

For information about available fields see [`mod@itargetsr44`] module*/
pub type ITARGETSR44 = crate::Reg<itargetsr44::ITARGETSR44rs>;
///GICD interrupt processor target register 44
pub mod itargetsr44;
/**ITARGETSR45 (rw) register accessor: GICD interrupt processor target register 45

You can [`read`](crate::Reg::read) this register and get [`itargetsr45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR45)

For information about available fields see [`mod@itargetsr45`] module*/
pub type ITARGETSR45 = crate::Reg<itargetsr45::ITARGETSR45rs>;
///GICD interrupt processor target register 45
pub mod itargetsr45;
/**ITARGETSR46 (rw) register accessor: GICD interrupt processor target register 46

You can [`read`](crate::Reg::read) this register and get [`itargetsr46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR46)

For information about available fields see [`mod@itargetsr46`] module*/
pub type ITARGETSR46 = crate::Reg<itargetsr46::ITARGETSR46rs>;
///GICD interrupt processor target register 46
pub mod itargetsr46;
/**ITARGETSR47 (rw) register accessor: GICD interrupt processor target register 47

You can [`read`](crate::Reg::read) this register and get [`itargetsr47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR47)

For information about available fields see [`mod@itargetsr47`] module*/
pub type ITARGETSR47 = crate::Reg<itargetsr47::ITARGETSR47rs>;
///GICD interrupt processor target register 47
pub mod itargetsr47;
/**ITARGETSR48 (rw) register accessor: GICD interrupt processor target register 48

You can [`read`](crate::Reg::read) this register and get [`itargetsr48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR48)

For information about available fields see [`mod@itargetsr48`] module*/
pub type ITARGETSR48 = crate::Reg<itargetsr48::ITARGETSR48rs>;
///GICD interrupt processor target register 48
pub mod itargetsr48;
/**ITARGETSR49 (rw) register accessor: GICD interrupt processor target register 49

You can [`read`](crate::Reg::read) this register and get [`itargetsr49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR49)

For information about available fields see [`mod@itargetsr49`] module*/
pub type ITARGETSR49 = crate::Reg<itargetsr49::ITARGETSR49rs>;
///GICD interrupt processor target register 49
pub mod itargetsr49;
/**ITARGETSR50 (rw) register accessor: GICD interrupt processor target register 50

You can [`read`](crate::Reg::read) this register and get [`itargetsr50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR50)

For information about available fields see [`mod@itargetsr50`] module*/
pub type ITARGETSR50 = crate::Reg<itargetsr50::ITARGETSR50rs>;
///GICD interrupt processor target register 50
pub mod itargetsr50;
/**ITARGETSR51 (rw) register accessor: GICD interrupt processor target register 51

You can [`read`](crate::Reg::read) this register and get [`itargetsr51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR51)

For information about available fields see [`mod@itargetsr51`] module*/
pub type ITARGETSR51 = crate::Reg<itargetsr51::ITARGETSR51rs>;
///GICD interrupt processor target register 51
pub mod itargetsr51;
/**ITARGETSR52 (rw) register accessor: GICD interrupt processor target register 52

You can [`read`](crate::Reg::read) this register and get [`itargetsr52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR52)

For information about available fields see [`mod@itargetsr52`] module*/
pub type ITARGETSR52 = crate::Reg<itargetsr52::ITARGETSR52rs>;
///GICD interrupt processor target register 52
pub mod itargetsr52;
/**ITARGETSR53 (rw) register accessor: GICD interrupt processor target register 53

You can [`read`](crate::Reg::read) this register and get [`itargetsr53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR53)

For information about available fields see [`mod@itargetsr53`] module*/
pub type ITARGETSR53 = crate::Reg<itargetsr53::ITARGETSR53rs>;
///GICD interrupt processor target register 53
pub mod itargetsr53;
/**ITARGETSR54 (rw) register accessor: GICD interrupt processor target register 54

You can [`read`](crate::Reg::read) this register and get [`itargetsr54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR54)

For information about available fields see [`mod@itargetsr54`] module*/
pub type ITARGETSR54 = crate::Reg<itargetsr54::ITARGETSR54rs>;
///GICD interrupt processor target register 54
pub mod itargetsr54;
/**ITARGETSR55 (rw) register accessor: GICD interrupt processor target register 55

You can [`read`](crate::Reg::read) this register and get [`itargetsr55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR55)

For information about available fields see [`mod@itargetsr55`] module*/
pub type ITARGETSR55 = crate::Reg<itargetsr55::ITARGETSR55rs>;
///GICD interrupt processor target register 55
pub mod itargetsr55;
/**ITARGETSR56 (rw) register accessor: GICD interrupt processor target register 56

You can [`read`](crate::Reg::read) this register and get [`itargetsr56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR56)

For information about available fields see [`mod@itargetsr56`] module*/
pub type ITARGETSR56 = crate::Reg<itargetsr56::ITARGETSR56rs>;
///GICD interrupt processor target register 56
pub mod itargetsr56;
/**ITARGETSR57 (rw) register accessor: GICD interrupt processor target register 57

You can [`read`](crate::Reg::read) this register and get [`itargetsr57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR57)

For information about available fields see [`mod@itargetsr57`] module*/
pub type ITARGETSR57 = crate::Reg<itargetsr57::ITARGETSR57rs>;
///GICD interrupt processor target register 57
pub mod itargetsr57;
/**ITARGETSR58 (rw) register accessor: GICD interrupt processor target register 58

You can [`read`](crate::Reg::read) this register and get [`itargetsr58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR58)

For information about available fields see [`mod@itargetsr58`] module*/
pub type ITARGETSR58 = crate::Reg<itargetsr58::ITARGETSR58rs>;
///GICD interrupt processor target register 58
pub mod itargetsr58;
/**ITARGETSR59 (rw) register accessor: GICD interrupt processor target register 59

You can [`read`](crate::Reg::read) this register and get [`itargetsr59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR59)

For information about available fields see [`mod@itargetsr59`] module*/
pub type ITARGETSR59 = crate::Reg<itargetsr59::ITARGETSR59rs>;
///GICD interrupt processor target register 59
pub mod itargetsr59;
/**ITARGETSR60 (rw) register accessor: GICD interrupt processor target register 60

You can [`read`](crate::Reg::read) this register and get [`itargetsr60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR60)

For information about available fields see [`mod@itargetsr60`] module*/
pub type ITARGETSR60 = crate::Reg<itargetsr60::ITARGETSR60rs>;
///GICD interrupt processor target register 60
pub mod itargetsr60;
/**ITARGETSR61 (rw) register accessor: GICD interrupt processor target register 61

You can [`read`](crate::Reg::read) this register and get [`itargetsr61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR61)

For information about available fields see [`mod@itargetsr61`] module*/
pub type ITARGETSR61 = crate::Reg<itargetsr61::ITARGETSR61rs>;
///GICD interrupt processor target register 61
pub mod itargetsr61;
/**ITARGETSR62 (rw) register accessor: GICD interrupt processor target register 62

You can [`read`](crate::Reg::read) this register and get [`itargetsr62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR62)

For information about available fields see [`mod@itargetsr62`] module*/
pub type ITARGETSR62 = crate::Reg<itargetsr62::ITARGETSR62rs>;
///GICD interrupt processor target register 62
pub mod itargetsr62;
/**ITARGETSR63 (rw) register accessor: GICD interrupt processor target register 63

You can [`read`](crate::Reg::read) this register and get [`itargetsr63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR63)

For information about available fields see [`mod@itargetsr63`] module*/
pub type ITARGETSR63 = crate::Reg<itargetsr63::ITARGETSR63rs>;
///GICD interrupt processor target register 63
pub mod itargetsr63;
/**ITARGETSR64 (rw) register accessor: GICD interrupt processor target register 64

You can [`read`](crate::Reg::read) this register and get [`itargetsr64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR64)

For information about available fields see [`mod@itargetsr64`] module*/
pub type ITARGETSR64 = crate::Reg<itargetsr64::ITARGETSR64rs>;
///GICD interrupt processor target register 64
pub mod itargetsr64;
/**ITARGETSR65 (rw) register accessor: GICD interrupt processor target register 65

You can [`read`](crate::Reg::read) this register and get [`itargetsr65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR65)

For information about available fields see [`mod@itargetsr65`] module*/
pub type ITARGETSR65 = crate::Reg<itargetsr65::ITARGETSR65rs>;
///GICD interrupt processor target register 65
pub mod itargetsr65;
/**ITARGETSR66 (rw) register accessor: GICD interrupt processor target register 66

You can [`read`](crate::Reg::read) this register and get [`itargetsr66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR66)

For information about available fields see [`mod@itargetsr66`] module*/
pub type ITARGETSR66 = crate::Reg<itargetsr66::ITARGETSR66rs>;
///GICD interrupt processor target register 66
pub mod itargetsr66;
/**ITARGETSR67 (rw) register accessor: GICD interrupt processor target register 67

You can [`read`](crate::Reg::read) this register and get [`itargetsr67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR67)

For information about available fields see [`mod@itargetsr67`] module*/
pub type ITARGETSR67 = crate::Reg<itargetsr67::ITARGETSR67rs>;
///GICD interrupt processor target register 67
pub mod itargetsr67;
/**ITARGETSR68 (rw) register accessor: GICD interrupt processor target register 68

You can [`read`](crate::Reg::read) this register and get [`itargetsr68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR68)

For information about available fields see [`mod@itargetsr68`] module*/
pub type ITARGETSR68 = crate::Reg<itargetsr68::ITARGETSR68rs>;
///GICD interrupt processor target register 68
pub mod itargetsr68;
/**ITARGETSR69 (rw) register accessor: GICD interrupt processor target register 69

You can [`read`](crate::Reg::read) this register and get [`itargetsr69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR69)

For information about available fields see [`mod@itargetsr69`] module*/
pub type ITARGETSR69 = crate::Reg<itargetsr69::ITARGETSR69rs>;
///GICD interrupt processor target register 69
pub mod itargetsr69;
/**ITARGETSR70 (rw) register accessor: GICD interrupt processor target register 70

You can [`read`](crate::Reg::read) this register and get [`itargetsr70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR70)

For information about available fields see [`mod@itargetsr70`] module*/
pub type ITARGETSR70 = crate::Reg<itargetsr70::ITARGETSR70rs>;
///GICD interrupt processor target register 70
pub mod itargetsr70;
/**ITARGETSR71 (rw) register accessor: GICD interrupt processor target register 71

You can [`read`](crate::Reg::read) this register and get [`itargetsr71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itargetsr71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ITARGETSR71)

For information about available fields see [`mod@itargetsr71`] module*/
pub type ITARGETSR71 = crate::Reg<itargetsr71::ITARGETSR71rs>;
///GICD interrupt processor target register 71
pub mod itargetsr71;
/**ICFGR0 (rw) register accessor: GICD interrupt configuration register

You can [`read`](crate::Reg::read) this register and get [`icfgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR0)

For information about available fields see [`mod@icfgr0`] module*/
pub type ICFGR0 = crate::Reg<icfgr0::ICFGR0rs>;
///GICD interrupt configuration register
pub mod icfgr0;
/**ICFGR1 (rw) register accessor: GICD interrupt configuration register

You can [`read`](crate::Reg::read) this register and get [`icfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR1)

For information about available fields see [`mod@icfgr1`] module*/
pub type ICFGR1 = crate::Reg<icfgr1::ICFGR1rs>;
///GICD interrupt configuration register
pub mod icfgr1;
/**ICFGR2 (rw) register accessor: GICD interrupt configuration register 2

You can [`read`](crate::Reg::read) this register and get [`icfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR2)

For information about available fields see [`mod@icfgr2`] module*/
pub type ICFGR2 = crate::Reg<icfgr2::ICFGR2rs>;
///GICD interrupt configuration register 2
pub mod icfgr2;
/**ICFGR3 (rw) register accessor: GICD interrupt configuration register 3

You can [`read`](crate::Reg::read) this register and get [`icfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR3)

For information about available fields see [`mod@icfgr3`] module*/
pub type ICFGR3 = crate::Reg<icfgr3::ICFGR3rs>;
///GICD interrupt configuration register 3
pub mod icfgr3;
/**ICFGR4 (rw) register accessor: GICD interrupt configuration register 4

You can [`read`](crate::Reg::read) this register and get [`icfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR4)

For information about available fields see [`mod@icfgr4`] module*/
pub type ICFGR4 = crate::Reg<icfgr4::ICFGR4rs>;
///GICD interrupt configuration register 4
pub mod icfgr4;
/**ICFGR5 (rw) register accessor: GICD interrupt configuration register 5

You can [`read`](crate::Reg::read) this register and get [`icfgr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR5)

For information about available fields see [`mod@icfgr5`] module*/
pub type ICFGR5 = crate::Reg<icfgr5::ICFGR5rs>;
///GICD interrupt configuration register 5
pub mod icfgr5;
/**ICFGR6 (rw) register accessor: GICD interrupt configuration register 6

You can [`read`](crate::Reg::read) this register and get [`icfgr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR6)

For information about available fields see [`mod@icfgr6`] module*/
pub type ICFGR6 = crate::Reg<icfgr6::ICFGR6rs>;
///GICD interrupt configuration register 6
pub mod icfgr6;
/**ICFGR7 (rw) register accessor: GICD interrupt configuration register 7

You can [`read`](crate::Reg::read) this register and get [`icfgr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR7)

For information about available fields see [`mod@icfgr7`] module*/
pub type ICFGR7 = crate::Reg<icfgr7::ICFGR7rs>;
///GICD interrupt configuration register 7
pub mod icfgr7;
/**ICFGR8 (rw) register accessor: GICD interrupt configuration register 8

You can [`read`](crate::Reg::read) this register and get [`icfgr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR8)

For information about available fields see [`mod@icfgr8`] module*/
pub type ICFGR8 = crate::Reg<icfgr8::ICFGR8rs>;
///GICD interrupt configuration register 8
pub mod icfgr8;
/**ICFGR9 (rw) register accessor: GICD interrupt configuration register 9

You can [`read`](crate::Reg::read) this register and get [`icfgr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR9)

For information about available fields see [`mod@icfgr9`] module*/
pub type ICFGR9 = crate::Reg<icfgr9::ICFGR9rs>;
///GICD interrupt configuration register 9
pub mod icfgr9;
/**ICFGR10 (rw) register accessor: GICD interrupt configuration register 10

You can [`read`](crate::Reg::read) this register and get [`icfgr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR10)

For information about available fields see [`mod@icfgr10`] module*/
pub type ICFGR10 = crate::Reg<icfgr10::ICFGR10rs>;
///GICD interrupt configuration register 10
pub mod icfgr10;
/**ICFGR11 (rw) register accessor: GICD interrupt configuration register 11

You can [`read`](crate::Reg::read) this register and get [`icfgr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR11)

For information about available fields see [`mod@icfgr11`] module*/
pub type ICFGR11 = crate::Reg<icfgr11::ICFGR11rs>;
///GICD interrupt configuration register 11
pub mod icfgr11;
/**ICFGR12 (rw) register accessor: GICD interrupt configuration register 12

You can [`read`](crate::Reg::read) this register and get [`icfgr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR12)

For information about available fields see [`mod@icfgr12`] module*/
pub type ICFGR12 = crate::Reg<icfgr12::ICFGR12rs>;
///GICD interrupt configuration register 12
pub mod icfgr12;
/**ICFGR13 (rw) register accessor: GICD interrupt configuration register 13

You can [`read`](crate::Reg::read) this register and get [`icfgr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR13)

For information about available fields see [`mod@icfgr13`] module*/
pub type ICFGR13 = crate::Reg<icfgr13::ICFGR13rs>;
///GICD interrupt configuration register 13
pub mod icfgr13;
/**ICFGR14 (rw) register accessor: GICD interrupt configuration register 14

You can [`read`](crate::Reg::read) this register and get [`icfgr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR14)

For information about available fields see [`mod@icfgr14`] module*/
pub type ICFGR14 = crate::Reg<icfgr14::ICFGR14rs>;
///GICD interrupt configuration register 14
pub mod icfgr14;
/**ICFGR15 (rw) register accessor: GICD interrupt configuration register 15

You can [`read`](crate::Reg::read) this register and get [`icfgr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR15)

For information about available fields see [`mod@icfgr15`] module*/
pub type ICFGR15 = crate::Reg<icfgr15::ICFGR15rs>;
///GICD interrupt configuration register 15
pub mod icfgr15;
/**ICFGR16 (rw) register accessor: GICD interrupt configuration register 16

You can [`read`](crate::Reg::read) this register and get [`icfgr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR16)

For information about available fields see [`mod@icfgr16`] module*/
pub type ICFGR16 = crate::Reg<icfgr16::ICFGR16rs>;
///GICD interrupt configuration register 16
pub mod icfgr16;
/**ICFGR17 (rw) register accessor: GICD interrupt configuration register 17

You can [`read`](crate::Reg::read) this register and get [`icfgr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ICFGR17)

For information about available fields see [`mod@icfgr17`] module*/
pub type ICFGR17 = crate::Reg<icfgr17::ICFGR17rs>;
///GICD interrupt configuration register 17
pub mod icfgr17;
/**PPISR (r) register accessor: GICD private peripheral interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ppisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:PPISR)

For information about available fields see [`mod@ppisr`] module*/
pub type PPISR = crate::Reg<ppisr::PPISRrs>;
///GICD private peripheral interrupt status register
pub mod ppisr;
/**SPISR1 (r) register accessor: For interrupts ID = SPI number+32, from SPI \[x*32+31\] to SPI \[x*32\]

You can [`read`](crate::Reg::read) this register and get [`spisr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:SPISR1)

For information about available fields see [`mod@spisr1`] module*/
pub type SPISR1 = crate::Reg<spisr1::SPISR1rs>;
///For interrupts ID = SPI number+32, from SPI \[x*32+31\] to SPI \[x*32\]
pub mod spisr1;
/**SPISR2 (r) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`spisr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:SPISR2)

For information about available fields see [`mod@spisr2`] module*/
pub type SPISR2 = crate::Reg<spisr2::SPISR2rs>;
///For interrupts ID
pub mod spisr2;
/**SPISR3 (r) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`spisr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:SPISR3)

For information about available fields see [`mod@spisr3`] module*/
pub type SPISR3 = crate::Reg<spisr3::SPISR3rs>;
///For interrupts ID
pub mod spisr3;
/**SPISR4 (r) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`spisr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:SPISR4)

For information about available fields see [`mod@spisr4`] module*/
pub type SPISR4 = crate::Reg<spisr4::SPISR4rs>;
///For interrupts ID
pub mod spisr4;
/**SPISR5 (r) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`spisr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:SPISR5)

For information about available fields see [`mod@spisr5`] module*/
pub type SPISR5 = crate::Reg<spisr5::SPISR5rs>;
///For interrupts ID
pub mod spisr5;
/**SPISR6 (r) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`spisr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:SPISR6)

For information about available fields see [`mod@spisr6`] module*/
pub type SPISR6 = crate::Reg<spisr6::SPISR6rs>;
///For interrupts ID
pub mod spisr6;
/**SPISR7 (r) register accessor: For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`spisr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:SPISR7)

For information about available fields see [`mod@spisr7`] module*/
pub type SPISR7 = crate::Reg<spisr7::SPISR7rs>;
///For interrupts ID
pub mod spisr7;
/**SGIR (w) register accessor: GICD software generated interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:SGIR)

For information about available fields see [`mod@sgir`] module*/
pub type SGIR = crate::Reg<sgir::SGIRrs>;
///GICD software generated interrupt register
pub mod sgir;
/**CPENDSGIR0 (rw) register accessor: For SGI x*4 to SGI x*4+3

You can [`read`](crate::Reg::read) this register and get [`cpendsgir0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpendsgir0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:CPENDSGIR0)

For information about available fields see [`mod@cpendsgir0`] module*/
pub type CPENDSGIR0 = crate::Reg<cpendsgir0::CPENDSGIR0rs>;
///For SGI x*4 to SGI x*4+3
pub mod cpendsgir0;
/**CPENDSGIR1 (rw) register accessor: For SGI x*4 to SGI x*4+3

You can [`read`](crate::Reg::read) this register and get [`cpendsgir1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpendsgir1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:CPENDSGIR1)

For information about available fields see [`mod@cpendsgir1`] module*/
pub type CPENDSGIR1 = crate::Reg<cpendsgir1::CPENDSGIR1rs>;
///For SGI x*4 to SGI x*4+3
pub mod cpendsgir1;
/**CPENDSGIR2 (rw) register accessor: For SGI x*4 to SGI x*4+3

You can [`read`](crate::Reg::read) this register and get [`cpendsgir2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpendsgir2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:CPENDSGIR2)

For information about available fields see [`mod@cpendsgir2`] module*/
pub type CPENDSGIR2 = crate::Reg<cpendsgir2::CPENDSGIR2rs>;
///For SGI x*4 to SGI x*4+3
pub mod cpendsgir2;
/**CPENDSGIR3 (rw) register accessor: For SGI x*4 to SGI x*4+3

You can [`read`](crate::Reg::read) this register and get [`cpendsgir3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpendsgir3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:CPENDSGIR3)

For information about available fields see [`mod@cpendsgir3`] module*/
pub type CPENDSGIR3 = crate::Reg<cpendsgir3::CPENDSGIR3rs>;
///For SGI x*4 to SGI x*4+3
pub mod cpendsgir3;
/**SPENDSGIR0 (rw) register accessor: For SGI x*4 to SGI x*4+3

You can [`read`](crate::Reg::read) this register and get [`spendsgir0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spendsgir0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:SPENDSGIR0)

For information about available fields see [`mod@spendsgir0`] module*/
pub type SPENDSGIR0 = crate::Reg<spendsgir0::SPENDSGIR0rs>;
///For SGI x*4 to SGI x*4+3
pub mod spendsgir0;
/**SPENDSGIR1 (rw) register accessor: For SGI x*4 to SGI x*4+3

You can [`read`](crate::Reg::read) this register and get [`spendsgir1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spendsgir1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:SPENDSGIR1)

For information about available fields see [`mod@spendsgir1`] module*/
pub type SPENDSGIR1 = crate::Reg<spendsgir1::SPENDSGIR1rs>;
///For SGI x*4 to SGI x*4+3
pub mod spendsgir1;
/**SPENDSGIR2 (rw) register accessor: For SGI x*4 to SGI x*4+3

You can [`read`](crate::Reg::read) this register and get [`spendsgir2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spendsgir2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:SPENDSGIR2)

For information about available fields see [`mod@spendsgir2`] module*/
pub type SPENDSGIR2 = crate::Reg<spendsgir2::SPENDSGIR2rs>;
///For SGI x*4 to SGI x*4+3
pub mod spendsgir2;
/**SPENDSGIR3 (rw) register accessor: For SGI x*4 to SGI x*4+3

You can [`read`](crate::Reg::read) this register and get [`spendsgir3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spendsgir3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:SPENDSGIR3)

For information about available fields see [`mod@spendsgir3`] module*/
pub type SPENDSGIR3 = crate::Reg<spendsgir3::SPENDSGIR3rs>;
///For SGI x*4 to SGI x*4+3
pub mod spendsgir3;
/**PIDR4 (r) register accessor: GICD peripheral ID4 register

You can [`read`](crate::Reg::read) this register and get [`pidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:PIDR4)

For information about available fields see [`mod@pidr4`] module*/
pub type PIDR4 = crate::Reg<pidr4::PIDR4rs>;
///GICD peripheral ID4 register
pub mod pidr4;
/**PIDR5 (r) register accessor: GICD peripheral ID5 to ID7 register 5

You can [`read`](crate::Reg::read) this register and get [`pidr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:PIDR5)

For information about available fields see [`mod@pidr5`] module*/
pub type PIDR5 = crate::Reg<pidr5::PIDR5rs>;
///GICD peripheral ID5 to ID7 register 5
pub mod pidr5;
/**PIDR6 (r) register accessor: GICD peripheral ID5 to ID7 register 6

You can [`read`](crate::Reg::read) this register and get [`pidr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:PIDR6)

For information about available fields see [`mod@pidr6`] module*/
pub type PIDR6 = crate::Reg<pidr6::PIDR6rs>;
///GICD peripheral ID5 to ID7 register 6
pub mod pidr6;
/**PIDR7 (r) register accessor: GICD peripheral ID5 to ID7 register 7

You can [`read`](crate::Reg::read) this register and get [`pidr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:PIDR7)

For information about available fields see [`mod@pidr7`] module*/
pub type PIDR7 = crate::Reg<pidr7::PIDR7rs>;
///GICD peripheral ID5 to ID7 register 7
pub mod pidr7;
/**PIDR0 (r) register accessor: GICD peripheral ID0 register

You can [`read`](crate::Reg::read) this register and get [`pidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:PIDR0)

For information about available fields see [`mod@pidr0`] module*/
pub type PIDR0 = crate::Reg<pidr0::PIDR0rs>;
///GICD peripheral ID0 register
pub mod pidr0;
/**PIDR1 (r) register accessor: GICD peripheral ID1 register

You can [`read`](crate::Reg::read) this register and get [`pidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:PIDR1)

For information about available fields see [`mod@pidr1`] module*/
pub type PIDR1 = crate::Reg<pidr1::PIDR1rs>;
///GICD peripheral ID1 register
pub mod pidr1;
/**PIDR2 (r) register accessor: GICD peripheral ID2 register

You can [`read`](crate::Reg::read) this register and get [`pidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:PIDR2)

For information about available fields see [`mod@pidr2`] module*/
pub type PIDR2 = crate::Reg<pidr2::PIDR2rs>;
///GICD peripheral ID2 register
pub mod pidr2;
/**PIDR3 (r) register accessor: GICD peripheral ID3 register

You can [`read`](crate::Reg::read) this register and get [`pidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:PIDR3)

For information about available fields see [`mod@pidr3`] module*/
pub type PIDR3 = crate::Reg<pidr3::PIDR3rs>;
///GICD peripheral ID3 register
pub mod pidr3;
/**CIDR0 (r) register accessor: GICD component ID0 register

You can [`read`](crate::Reg::read) this register and get [`cidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:CIDR0)

For information about available fields see [`mod@cidr0`] module*/
pub type CIDR0 = crate::Reg<cidr0::CIDR0rs>;
///GICD component ID0 register
pub mod cidr0;
/**CIDR1 (r) register accessor: GICD component ID1 register

You can [`read`](crate::Reg::read) this register and get [`cidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:CIDR1)

For information about available fields see [`mod@cidr1`] module*/
pub type CIDR1 = crate::Reg<cidr1::CIDR1rs>;
///GICD component ID1 register
pub mod cidr1;
/**CIDR2 (r) register accessor: GICD component ID2 register

You can [`read`](crate::Reg::read) this register and get [`cidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:CIDR2)

For information about available fields see [`mod@cidr2`] module*/
pub type CIDR2 = crate::Reg<cidr2::CIDR2rs>;
///GICD component ID2 register
pub mod cidr2;
/**CIDR3 (r) register accessor: GICD component ID3 register

You can [`read`](crate::Reg::read) this register and get [`cidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:CIDR3)

For information about available fields see [`mod@cidr3`] module*/
pub type CIDR3 = crate::Reg<cidr3::CIDR3rs>;
///GICD component ID3 register
pub mod cidr3;
