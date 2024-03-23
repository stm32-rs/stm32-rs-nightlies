#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gicd_ctlr: GICD_CTLR,
    gicd_typer: GICD_TYPER,
    gicd_iidr: GICD_IIDR,
    _reserved3: [u8; 0x74],
    gicd_igroupr0: GICD_IGROUPR0,
    gicd_igroupr1: GICD_IGROUPR1,
    gicd_igroupr2: GICD_IGROUPR2,
    gicd_igroupr3: GICD_IGROUPR3,
    gicd_igroupr4: GICD_IGROUPR4,
    gicd_igroupr5: GICD_IGROUPR5,
    gicd_igroupr6: GICD_IGROUPR6,
    gicd_igroupr7: GICD_IGROUPR7,
    gicd_igroupr8: GICD_IGROUPR8,
    _reserved12: [u8; 0x5c],
    gicd_isenabler0: GICD_ISENABLER0,
    gicd_isenabler1: GICD_ISENABLER1,
    gicd_isenabler2: GICD_ISENABLER2,
    gicd_isenabler3: GICD_ISENABLER3,
    gicd_isenabler4: GICD_ISENABLER4,
    gicd_isenabler5: GICD_ISENABLER5,
    gicd_isenabler6: GICD_ISENABLER6,
    gicd_isenabler7: GICD_ISENABLER7,
    gicd_isenabler8: GICD_ISENABLER8,
    _reserved21: [u8; 0x5c],
    gicd_icenabler0: GICD_ICENABLER0,
    gicd_icenabler1: GICD_ICENABLER1,
    gicd_icenabler2: GICD_ICENABLER2,
    gicd_icenabler3: GICD_ICENABLER3,
    gicd_icenabler4: GICD_ICENABLER4,
    gicd_icenabler5: GICD_ICENABLER5,
    gicd_icenabler6: GICD_ICENABLER6,
    gicd_icenabler7: GICD_ICENABLER7,
    gicd_icenabler8: GICD_ICENABLER8,
    _reserved30: [u8; 0x5c],
    gicd_ispendr0: GICD_ISPENDR0,
    gicd_ispendr1: GICD_ISPENDR1,
    gicd_ispendr2: GICD_ISPENDR2,
    gicd_ispendr3: GICD_ISPENDR3,
    gicd_ispendr4: GICD_ISPENDR4,
    gicd_ispendr5: GICD_ISPENDR5,
    gicd_ispendr6: GICD_ISPENDR6,
    gicd_ispendr7: GICD_ISPENDR7,
    gicd_ispendr8: GICD_ISPENDR8,
    _reserved39: [u8; 0x5c],
    gicd_icpendr0: GICD_ICPENDR0,
    gicd_icpendr1: GICD_ICPENDR1,
    gicd_icpendr2: GICD_ICPENDR2,
    gicd_icpendr3: GICD_ICPENDR3,
    gicd_icpendr4: GICD_ICPENDR4,
    gicd_icpendr5: GICD_ICPENDR5,
    gicd_icpendr6: GICD_ICPENDR6,
    gicd_icpendr7: GICD_ICPENDR7,
    gicd_icpendr8: GICD_ICPENDR8,
    _reserved48: [u8; 0x5c],
    gicd_isactiver0: GICD_ISACTIVER0,
    gicd_isactiver1: GICD_ISACTIVER1,
    gicd_isactiver2: GICD_ISACTIVER2,
    gicd_isactiver3: GICD_ISACTIVER3,
    gicd_isactiver4: GICD_ISACTIVER4,
    gicd_isactiver5: GICD_ISACTIVER5,
    gicd_isactiver6: GICD_ISACTIVER6,
    gicd_isactiver7: GICD_ISACTIVER7,
    gicd_isactiver8: GICD_ISACTIVER8,
    _reserved57: [u8; 0x5c],
    gicd_icactiver0: GICD_ICACTIVER0,
    gicd_icactiver1: GICD_ICACTIVER1,
    gicd_icactiver2: GICD_ICACTIVER2,
    gicd_icactiver3: GICD_ICACTIVER3,
    gicd_icactiver4: GICD_ICACTIVER4,
    gicd_icactiver5: GICD_ICACTIVER5,
    gicd_icactiver6: GICD_ICACTIVER6,
    gicd_icactiver7: GICD_ICACTIVER7,
    gicd_icactiver8: GICD_ICACTIVER8,
    _reserved66: [u8; 0x5c],
    gicd_ipriorityr0: GICD_IPRIORITYR0,
    gicd_ipriorityr1: GICD_IPRIORITYR1,
    gicd_ipriorityr2: GICD_IPRIORITYR2,
    gicd_ipriorityr3: GICD_IPRIORITYR3,
    gicd_ipriorityr4: GICD_IPRIORITYR4,
    gicd_ipriorityr5: GICD_IPRIORITYR5,
    gicd_ipriorityr6: GICD_IPRIORITYR6,
    gicd_ipriorityr7: GICD_IPRIORITYR7,
    gicd_ipriorityr8: GICD_IPRIORITYR8,
    gicd_ipriorityr9: GICD_IPRIORITYR9,
    gicd_ipriorityr10: GICD_IPRIORITYR10,
    gicd_ipriorityr11: GICD_IPRIORITYR11,
    gicd_ipriorityr12: GICD_IPRIORITYR12,
    gicd_ipriorityr13: GICD_IPRIORITYR13,
    gicd_ipriorityr14: GICD_IPRIORITYR14,
    gicd_ipriorityr15: GICD_IPRIORITYR15,
    gicd_ipriorityr16: GICD_IPRIORITYR16,
    gicd_ipriorityr17: GICD_IPRIORITYR17,
    gicd_ipriorityr18: GICD_IPRIORITYR18,
    gicd_ipriorityr19: GICD_IPRIORITYR19,
    gicd_ipriorityr20: GICD_IPRIORITYR20,
    gicd_ipriorityr21: GICD_IPRIORITYR21,
    gicd_ipriorityr22: GICD_IPRIORITYR22,
    gicd_ipriorityr23: GICD_IPRIORITYR23,
    gicd_ipriorityr24: GICD_IPRIORITYR24,
    gicd_ipriorityr25: GICD_IPRIORITYR25,
    gicd_ipriorityr26: GICD_IPRIORITYR26,
    gicd_ipriorityr27: GICD_IPRIORITYR27,
    gicd_ipriorityr28: GICD_IPRIORITYR28,
    gicd_ipriorityr29: GICD_IPRIORITYR29,
    gicd_ipriorityr30: GICD_IPRIORITYR30,
    gicd_ipriorityr31: GICD_IPRIORITYR31,
    gicd_ipriorityr32: GICD_IPRIORITYR32,
    gicd_ipriorityr33: GICD_IPRIORITYR33,
    gicd_ipriorityr34: GICD_IPRIORITYR34,
    gicd_ipriorityr35: GICD_IPRIORITYR35,
    gicd_ipriorityr36: GICD_IPRIORITYR36,
    gicd_ipriorityr37: GICD_IPRIORITYR37,
    gicd_ipriorityr38: GICD_IPRIORITYR38,
    gicd_ipriorityr39: GICD_IPRIORITYR39,
    gicd_ipriorityr40: GICD_IPRIORITYR40,
    gicd_ipriorityr41: GICD_IPRIORITYR41,
    gicd_ipriorityr42: GICD_IPRIORITYR42,
    gicd_ipriorityr43: GICD_IPRIORITYR43,
    gicd_ipriorityr44: GICD_IPRIORITYR44,
    gicd_ipriorityr45: GICD_IPRIORITYR45,
    gicd_ipriorityr46: GICD_IPRIORITYR46,
    gicd_ipriorityr47: GICD_IPRIORITYR47,
    gicd_ipriorityr48: GICD_IPRIORITYR48,
    gicd_ipriorityr49: GICD_IPRIORITYR49,
    gicd_ipriorityr50: GICD_IPRIORITYR50,
    gicd_ipriorityr51: GICD_IPRIORITYR51,
    gicd_ipriorityr52: GICD_IPRIORITYR52,
    gicd_ipriorityr53: GICD_IPRIORITYR53,
    gicd_ipriorityr54: GICD_IPRIORITYR54,
    gicd_ipriorityr55: GICD_IPRIORITYR55,
    gicd_ipriorityr56: GICD_IPRIORITYR56,
    gicd_ipriorityr57: GICD_IPRIORITYR57,
    gicd_ipriorityr58: GICD_IPRIORITYR58,
    gicd_ipriorityr59: GICD_IPRIORITYR59,
    gicd_ipriorityr60: GICD_IPRIORITYR60,
    gicd_ipriorityr61: GICD_IPRIORITYR61,
    gicd_ipriorityr62: GICD_IPRIORITYR62,
    gicd_ipriorityr63: GICD_IPRIORITYR63,
    gicd_ipriorityr64: GICD_IPRIORITYR64,
    gicd_ipriorityr65: GICD_IPRIORITYR65,
    gicd_ipriorityr66: GICD_IPRIORITYR66,
    gicd_ipriorityr67: GICD_IPRIORITYR67,
    gicd_ipriorityr68: GICD_IPRIORITYR68,
    gicd_ipriorityr69: GICD_IPRIORITYR69,
    gicd_ipriorityr70: GICD_IPRIORITYR70,
    gicd_ipriorityr71: GICD_IPRIORITYR71,
    _reserved138: [u8; 0x02e0],
    gicd_itargetsr0: GICD_ITARGETSR0,
    gicd_itargetsr1: GICD_ITARGETSR1,
    gicd_itargetsr2: GICD_ITARGETSR2,
    gicd_itargetsr3: GICD_ITARGETSR3,
    gicd_itargetsr4: GICD_ITARGETSR4,
    gicd_itargetsr5: GICD_ITARGETSR5,
    gicd_itargetsr6: GICD_ITARGETSR6,
    gicd_itargetsr7: GICD_ITARGETSR7,
    gicd_itargetsr8: GICD_ITARGETSR8,
    gicd_itargetsr9: GICD_ITARGETSR9,
    gicd_itargetsr10: GICD_ITARGETSR10,
    gicd_itargetsr11: GICD_ITARGETSR11,
    gicd_itargetsr12: GICD_ITARGETSR12,
    gicd_itargetsr13: GICD_ITARGETSR13,
    gicd_itargetsr14: GICD_ITARGETSR14,
    gicd_itargetsr15: GICD_ITARGETSR15,
    gicd_itargetsr16: GICD_ITARGETSR16,
    gicd_itargetsr17: GICD_ITARGETSR17,
    gicd_itargetsr18: GICD_ITARGETSR18,
    gicd_itargetsr19: GICD_ITARGETSR19,
    gicd_itargetsr20: GICD_ITARGETSR20,
    gicd_itargetsr21: GICD_ITARGETSR21,
    gicd_itargetsr22: GICD_ITARGETSR22,
    gicd_itargetsr23: GICD_ITARGETSR23,
    gicd_itargetsr24: GICD_ITARGETSR24,
    gicd_itargetsr25: GICD_ITARGETSR25,
    gicd_itargetsr26: GICD_ITARGETSR26,
    gicd_itargetsr27: GICD_ITARGETSR27,
    gicd_itargetsr28: GICD_ITARGETSR28,
    gicd_itargetsr29: GICD_ITARGETSR29,
    gicd_itargetsr30: GICD_ITARGETSR30,
    gicd_itargetsr31: GICD_ITARGETSR31,
    gicd_itargetsr32: GICD_ITARGETSR32,
    gicd_itargetsr33: GICD_ITARGETSR33,
    gicd_itargetsr34: GICD_ITARGETSR34,
    gicd_itargetsr35: GICD_ITARGETSR35,
    gicd_itargetsr36: GICD_ITARGETSR36,
    gicd_itargetsr37: GICD_ITARGETSR37,
    gicd_itargetsr38: GICD_ITARGETSR38,
    gicd_itargetsr39: GICD_ITARGETSR39,
    gicd_itargetsr40: GICD_ITARGETSR40,
    gicd_itargetsr41: GICD_ITARGETSR41,
    gicd_itargetsr42: GICD_ITARGETSR42,
    gicd_itargetsr43: GICD_ITARGETSR43,
    gicd_itargetsr44: GICD_ITARGETSR44,
    gicd_itargetsr45: GICD_ITARGETSR45,
    gicd_itargetsr46: GICD_ITARGETSR46,
    gicd_itargetsr47: GICD_ITARGETSR47,
    gicd_itargetsr48: GICD_ITARGETSR48,
    gicd_itargetsr49: GICD_ITARGETSR49,
    gicd_itargetsr50: GICD_ITARGETSR50,
    gicd_itargetsr51: GICD_ITARGETSR51,
    gicd_itargetsr52: GICD_ITARGETSR52,
    gicd_itargetsr53: GICD_ITARGETSR53,
    gicd_itargetsr54: GICD_ITARGETSR54,
    gicd_itargetsr55: GICD_ITARGETSR55,
    gicd_itargetsr56: GICD_ITARGETSR56,
    gicd_itargetsr57: GICD_ITARGETSR57,
    gicd_itargetsr58: GICD_ITARGETSR58,
    gicd_itargetsr59: GICD_ITARGETSR59,
    gicd_itargetsr60: GICD_ITARGETSR60,
    gicd_itargetsr61: GICD_ITARGETSR61,
    gicd_itargetsr62: GICD_ITARGETSR62,
    gicd_itargetsr63: GICD_ITARGETSR63,
    gicd_itargetsr64: GICD_ITARGETSR64,
    gicd_itargetsr65: GICD_ITARGETSR65,
    gicd_itargetsr66: GICD_ITARGETSR66,
    gicd_itargetsr67: GICD_ITARGETSR67,
    gicd_itargetsr68: GICD_ITARGETSR68,
    gicd_itargetsr69: GICD_ITARGETSR69,
    gicd_itargetsr70: GICD_ITARGETSR70,
    gicd_itargetsr71: GICD_ITARGETSR71,
    _reserved210: [u8; 0x02e0],
    gicd_icfgr0: GICD_ICFGR0,
    gicd_icfgr1: GICD_ICFGR1,
    gicd_icfgr2: GICD_ICFGR2,
    gicd_icfgr3: GICD_ICFGR3,
    gicd_icfgr4: GICD_ICFGR4,
    gicd_icfgr5: GICD_ICFGR5,
    gicd_icfgr6: GICD_ICFGR6,
    gicd_icfgr7: GICD_ICFGR7,
    gicd_icfgr8: GICD_ICFGR8,
    gicd_icfgr9: GICD_ICFGR9,
    gicd_icfgr10: GICD_ICFGR10,
    gicd_icfgr11: GICD_ICFGR11,
    gicd_icfgr12: GICD_ICFGR12,
    gicd_icfgr13: GICD_ICFGR13,
    gicd_icfgr14: GICD_ICFGR14,
    gicd_icfgr15: GICD_ICFGR15,
    gicd_icfgr16: GICD_ICFGR16,
    gicd_icfgr17: GICD_ICFGR17,
    _reserved228: [u8; 0xb8],
    gicd_ppisr: GICD_PPISR,
    _reserved229: [u8; 0x04],
    gicd_spisr1: GICD_SPISR1,
    gicd_spisr2: GICD_SPISR2,
    gicd_spisr3: GICD_SPISR3,
    gicd_spisr4: GICD_SPISR4,
    gicd_spisr5: GICD_SPISR5,
    gicd_spisr6: GICD_SPISR6,
    gicd_spisr7: GICD_SPISR7,
    _reserved236: [u8; 0x01dc],
    gicd_sgir: GICD_SGIR,
    _reserved237: [u8; 0x0c],
    gicd_cpendsgir0: GICD_CPENDSGIR0,
    gicd_cpendsgir1: GICD_CPENDSGIR1,
    gicd_cpendsgir2: GICD_CPENDSGIR2,
    gicd_cpendsgir3: GICD_CPENDSGIR3,
    gicd_spendsgir0: GICD_SPENDSGIR0,
    gicd_spendsgir1: GICD_SPENDSGIR1,
    gicd_spendsgir2: GICD_SPENDSGIR2,
    gicd_spendsgir3: GICD_SPENDSGIR3,
    _reserved245: [u8; 0xa0],
    gicd_pidr4: GICD_PIDR4,
    gicd_pidr5: GICD_PIDR5,
    gicd_pidr6: GICD_PIDR6,
    gicd_pidr7: GICD_PIDR7,
    gicd_pidr0: GICD_PIDR0,
    gicd_pidr1: GICD_PIDR1,
    gicd_pidr2: GICD_PIDR2,
    gicd_pidr3: GICD_PIDR3,
    gicd_cidr0: GICD_CIDR0,
    gicd_cidr1: GICD_CIDR1,
    gicd_cidr2: GICD_CIDR2,
    gicd_cidr3: GICD_CIDR3,
}
impl RegisterBlock {
    #[doc = "0x00 - GICD control register"]
    #[inline(always)]
    pub const fn gicd_ctlr(&self) -> &GICD_CTLR {
        &self.gicd_ctlr
    }
    #[doc = "0x04 - GICD interrupt controller type register"]
    #[inline(always)]
    pub const fn gicd_typer(&self) -> &GICD_TYPER {
        &self.gicd_typer
    }
    #[doc = "0x08 - GICD implementer identification register"]
    #[inline(always)]
    pub const fn gicd_iidr(&self) -> &GICD_IIDR {
        &self.gicd_iidr
    }
    #[doc = "0x80 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_igroupr0(&self) -> &GICD_IGROUPR0 {
        &self.gicd_igroupr0
    }
    #[doc = "0x84 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_igroupr1(&self) -> &GICD_IGROUPR1 {
        &self.gicd_igroupr1
    }
    #[doc = "0x88 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_igroupr2(&self) -> &GICD_IGROUPR2 {
        &self.gicd_igroupr2
    }
    #[doc = "0x8c - For interrupts ID = x*32 to ID = x*32+31"]
    #[inline(always)]
    pub const fn gicd_igroupr3(&self) -> &GICD_IGROUPR3 {
        &self.gicd_igroupr3
    }
    #[doc = "0x90 - For interrupts ID = x*32 to ID = x*32+31"]
    #[inline(always)]
    pub const fn gicd_igroupr4(&self) -> &GICD_IGROUPR4 {
        &self.gicd_igroupr4
    }
    #[doc = "0x94 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_igroupr5(&self) -> &GICD_IGROUPR5 {
        &self.gicd_igroupr5
    }
    #[doc = "0x98 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_igroupr6(&self) -> &GICD_IGROUPR6 {
        &self.gicd_igroupr6
    }
    #[doc = "0x9c - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_igroupr7(&self) -> &GICD_IGROUPR7 {
        &self.gicd_igroupr7
    }
    #[doc = "0xa0 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_igroupr8(&self) -> &GICD_IGROUPR8 {
        &self.gicd_igroupr8
    }
    #[doc = "0x100 - For interrupts ID = 0 to ID = 31"]
    #[inline(always)]
    pub const fn gicd_isenabler0(&self) -> &GICD_ISENABLER0 {
        &self.gicd_isenabler0
    }
    #[doc = "0x104 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_isenabler1(&self) -> &GICD_ISENABLER1 {
        &self.gicd_isenabler1
    }
    #[doc = "0x108 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_isenabler2(&self) -> &GICD_ISENABLER2 {
        &self.gicd_isenabler2
    }
    #[doc = "0x10c - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_isenabler3(&self) -> &GICD_ISENABLER3 {
        &self.gicd_isenabler3
    }
    #[doc = "0x110 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_isenabler4(&self) -> &GICD_ISENABLER4 {
        &self.gicd_isenabler4
    }
    #[doc = "0x114 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_isenabler5(&self) -> &GICD_ISENABLER5 {
        &self.gicd_isenabler5
    }
    #[doc = "0x118 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_isenabler6(&self) -> &GICD_ISENABLER6 {
        &self.gicd_isenabler6
    }
    #[doc = "0x11c - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_isenabler7(&self) -> &GICD_ISENABLER7 {
        &self.gicd_isenabler7
    }
    #[doc = "0x120 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_isenabler8(&self) -> &GICD_ISENABLER8 {
        &self.gicd_isenabler8
    }
    #[doc = "0x180 - For interrupts ID = 0 to ID = 31"]
    #[inline(always)]
    pub const fn gicd_icenabler0(&self) -> &GICD_ICENABLER0 {
        &self.gicd_icenabler0
    }
    #[doc = "0x184 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icenabler1(&self) -> &GICD_ICENABLER1 {
        &self.gicd_icenabler1
    }
    #[doc = "0x188 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icenabler2(&self) -> &GICD_ICENABLER2 {
        &self.gicd_icenabler2
    }
    #[doc = "0x18c - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icenabler3(&self) -> &GICD_ICENABLER3 {
        &self.gicd_icenabler3
    }
    #[doc = "0x190 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icenabler4(&self) -> &GICD_ICENABLER4 {
        &self.gicd_icenabler4
    }
    #[doc = "0x194 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icenabler5(&self) -> &GICD_ICENABLER5 {
        &self.gicd_icenabler5
    }
    #[doc = "0x198 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icenabler6(&self) -> &GICD_ICENABLER6 {
        &self.gicd_icenabler6
    }
    #[doc = "0x19c - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icenabler7(&self) -> &GICD_ICENABLER7 {
        &self.gicd_icenabler7
    }
    #[doc = "0x1a0 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icenabler8(&self) -> &GICD_ICENABLER8 {
        &self.gicd_icenabler8
    }
    #[doc = "0x200 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_ispendr0(&self) -> &GICD_ISPENDR0 {
        &self.gicd_ispendr0
    }
    #[doc = "0x204 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_ispendr1(&self) -> &GICD_ISPENDR1 {
        &self.gicd_ispendr1
    }
    #[doc = "0x208 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_ispendr2(&self) -> &GICD_ISPENDR2 {
        &self.gicd_ispendr2
    }
    #[doc = "0x20c - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_ispendr3(&self) -> &GICD_ISPENDR3 {
        &self.gicd_ispendr3
    }
    #[doc = "0x210 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_ispendr4(&self) -> &GICD_ISPENDR4 {
        &self.gicd_ispendr4
    }
    #[doc = "0x214 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_ispendr5(&self) -> &GICD_ISPENDR5 {
        &self.gicd_ispendr5
    }
    #[doc = "0x218 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_ispendr6(&self) -> &GICD_ISPENDR6 {
        &self.gicd_ispendr6
    }
    #[doc = "0x21c - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_ispendr7(&self) -> &GICD_ISPENDR7 {
        &self.gicd_ispendr7
    }
    #[doc = "0x220 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_ispendr8(&self) -> &GICD_ISPENDR8 {
        &self.gicd_ispendr8
    }
    #[doc = "0x280 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icpendr0(&self) -> &GICD_ICPENDR0 {
        &self.gicd_icpendr0
    }
    #[doc = "0x284 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icpendr1(&self) -> &GICD_ICPENDR1 {
        &self.gicd_icpendr1
    }
    #[doc = "0x288 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icpendr2(&self) -> &GICD_ICPENDR2 {
        &self.gicd_icpendr2
    }
    #[doc = "0x28c - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icpendr3(&self) -> &GICD_ICPENDR3 {
        &self.gicd_icpendr3
    }
    #[doc = "0x290 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icpendr4(&self) -> &GICD_ICPENDR4 {
        &self.gicd_icpendr4
    }
    #[doc = "0x294 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icpendr5(&self) -> &GICD_ICPENDR5 {
        &self.gicd_icpendr5
    }
    #[doc = "0x298 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icpendr6(&self) -> &GICD_ICPENDR6 {
        &self.gicd_icpendr6
    }
    #[doc = "0x29c - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icpendr7(&self) -> &GICD_ICPENDR7 {
        &self.gicd_icpendr7
    }
    #[doc = "0x2a0 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icpendr8(&self) -> &GICD_ICPENDR8 {
        &self.gicd_icpendr8
    }
    #[doc = "0x300 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_isactiver0(&self) -> &GICD_ISACTIVER0 {
        &self.gicd_isactiver0
    }
    #[doc = "0x304 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_isactiver1(&self) -> &GICD_ISACTIVER1 {
        &self.gicd_isactiver1
    }
    #[doc = "0x308 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_isactiver2(&self) -> &GICD_ISACTIVER2 {
        &self.gicd_isactiver2
    }
    #[doc = "0x30c - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_isactiver3(&self) -> &GICD_ISACTIVER3 {
        &self.gicd_isactiver3
    }
    #[doc = "0x310 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_isactiver4(&self) -> &GICD_ISACTIVER4 {
        &self.gicd_isactiver4
    }
    #[doc = "0x314 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_isactiver5(&self) -> &GICD_ISACTIVER5 {
        &self.gicd_isactiver5
    }
    #[doc = "0x318 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_isactiver6(&self) -> &GICD_ISACTIVER6 {
        &self.gicd_isactiver6
    }
    #[doc = "0x31c - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_isactiver7(&self) -> &GICD_ISACTIVER7 {
        &self.gicd_isactiver7
    }
    #[doc = "0x320 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_isactiver8(&self) -> &GICD_ISACTIVER8 {
        &self.gicd_isactiver8
    }
    #[doc = "0x380 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icactiver0(&self) -> &GICD_ICACTIVER0 {
        &self.gicd_icactiver0
    }
    #[doc = "0x384 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icactiver1(&self) -> &GICD_ICACTIVER1 {
        &self.gicd_icactiver1
    }
    #[doc = "0x388 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icactiver2(&self) -> &GICD_ICACTIVER2 {
        &self.gicd_icactiver2
    }
    #[doc = "0x38c - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icactiver3(&self) -> &GICD_ICACTIVER3 {
        &self.gicd_icactiver3
    }
    #[doc = "0x390 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icactiver4(&self) -> &GICD_ICACTIVER4 {
        &self.gicd_icactiver4
    }
    #[doc = "0x394 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icactiver5(&self) -> &GICD_ICACTIVER5 {
        &self.gicd_icactiver5
    }
    #[doc = "0x398 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icactiver6(&self) -> &GICD_ICACTIVER6 {
        &self.gicd_icactiver6
    }
    #[doc = "0x39c - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icactiver7(&self) -> &GICD_ICACTIVER7 {
        &self.gicd_icactiver7
    }
    #[doc = "0x3a0 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_icactiver8(&self) -> &GICD_ICACTIVER8 {
        &self.gicd_icactiver8
    }
    #[doc = "0x400 - GICD interrupt priority register 0"]
    #[inline(always)]
    pub const fn gicd_ipriorityr0(&self) -> &GICD_IPRIORITYR0 {
        &self.gicd_ipriorityr0
    }
    #[doc = "0x404 - GICD interrupt priority register 1"]
    #[inline(always)]
    pub const fn gicd_ipriorityr1(&self) -> &GICD_IPRIORITYR1 {
        &self.gicd_ipriorityr1
    }
    #[doc = "0x408 - GICD interrupt priority register 2"]
    #[inline(always)]
    pub const fn gicd_ipriorityr2(&self) -> &GICD_IPRIORITYR2 {
        &self.gicd_ipriorityr2
    }
    #[doc = "0x40c - GICD interrupt priority register 3"]
    #[inline(always)]
    pub const fn gicd_ipriorityr3(&self) -> &GICD_IPRIORITYR3 {
        &self.gicd_ipriorityr3
    }
    #[doc = "0x410 - GICD interrupt priority register 4"]
    #[inline(always)]
    pub const fn gicd_ipriorityr4(&self) -> &GICD_IPRIORITYR4 {
        &self.gicd_ipriorityr4
    }
    #[doc = "0x414 - GICD interrupt priority register 5"]
    #[inline(always)]
    pub const fn gicd_ipriorityr5(&self) -> &GICD_IPRIORITYR5 {
        &self.gicd_ipriorityr5
    }
    #[doc = "0x418 - GICD interrupt priority register 6"]
    #[inline(always)]
    pub const fn gicd_ipriorityr6(&self) -> &GICD_IPRIORITYR6 {
        &self.gicd_ipriorityr6
    }
    #[doc = "0x41c - GICD interrupt priority register 7"]
    #[inline(always)]
    pub const fn gicd_ipriorityr7(&self) -> &GICD_IPRIORITYR7 {
        &self.gicd_ipriorityr7
    }
    #[doc = "0x420 - GICD interrupt priority register 8"]
    #[inline(always)]
    pub const fn gicd_ipriorityr8(&self) -> &GICD_IPRIORITYR8 {
        &self.gicd_ipriorityr8
    }
    #[doc = "0x424 - GICD interrupt priority register 9"]
    #[inline(always)]
    pub const fn gicd_ipriorityr9(&self) -> &GICD_IPRIORITYR9 {
        &self.gicd_ipriorityr9
    }
    #[doc = "0x428 - GICD interrupt priority register 10"]
    #[inline(always)]
    pub const fn gicd_ipriorityr10(&self) -> &GICD_IPRIORITYR10 {
        &self.gicd_ipriorityr10
    }
    #[doc = "0x42c - GICD interrupt priority register 11"]
    #[inline(always)]
    pub const fn gicd_ipriorityr11(&self) -> &GICD_IPRIORITYR11 {
        &self.gicd_ipriorityr11
    }
    #[doc = "0x430 - GICD interrupt priority register 12"]
    #[inline(always)]
    pub const fn gicd_ipriorityr12(&self) -> &GICD_IPRIORITYR12 {
        &self.gicd_ipriorityr12
    }
    #[doc = "0x434 - GICD interrupt priority register 13"]
    #[inline(always)]
    pub const fn gicd_ipriorityr13(&self) -> &GICD_IPRIORITYR13 {
        &self.gicd_ipriorityr13
    }
    #[doc = "0x438 - GICD interrupt priority register 14"]
    #[inline(always)]
    pub const fn gicd_ipriorityr14(&self) -> &GICD_IPRIORITYR14 {
        &self.gicd_ipriorityr14
    }
    #[doc = "0x43c - GICD interrupt priority register 15"]
    #[inline(always)]
    pub const fn gicd_ipriorityr15(&self) -> &GICD_IPRIORITYR15 {
        &self.gicd_ipriorityr15
    }
    #[doc = "0x440 - GICD interrupt priority register 16"]
    #[inline(always)]
    pub const fn gicd_ipriorityr16(&self) -> &GICD_IPRIORITYR16 {
        &self.gicd_ipriorityr16
    }
    #[doc = "0x444 - GICD interrupt priority register 17"]
    #[inline(always)]
    pub const fn gicd_ipriorityr17(&self) -> &GICD_IPRIORITYR17 {
        &self.gicd_ipriorityr17
    }
    #[doc = "0x448 - GICD interrupt priority register 18"]
    #[inline(always)]
    pub const fn gicd_ipriorityr18(&self) -> &GICD_IPRIORITYR18 {
        &self.gicd_ipriorityr18
    }
    #[doc = "0x44c - GICD interrupt priority register 19"]
    #[inline(always)]
    pub const fn gicd_ipriorityr19(&self) -> &GICD_IPRIORITYR19 {
        &self.gicd_ipriorityr19
    }
    #[doc = "0x450 - GICD interrupt priority register 20"]
    #[inline(always)]
    pub const fn gicd_ipriorityr20(&self) -> &GICD_IPRIORITYR20 {
        &self.gicd_ipriorityr20
    }
    #[doc = "0x454 - GICD interrupt priority register 21"]
    #[inline(always)]
    pub const fn gicd_ipriorityr21(&self) -> &GICD_IPRIORITYR21 {
        &self.gicd_ipriorityr21
    }
    #[doc = "0x458 - GICD interrupt priority register 22"]
    #[inline(always)]
    pub const fn gicd_ipriorityr22(&self) -> &GICD_IPRIORITYR22 {
        &self.gicd_ipriorityr22
    }
    #[doc = "0x45c - GICD interrupt priority register 23"]
    #[inline(always)]
    pub const fn gicd_ipriorityr23(&self) -> &GICD_IPRIORITYR23 {
        &self.gicd_ipriorityr23
    }
    #[doc = "0x460 - GICD interrupt priority register 24"]
    #[inline(always)]
    pub const fn gicd_ipriorityr24(&self) -> &GICD_IPRIORITYR24 {
        &self.gicd_ipriorityr24
    }
    #[doc = "0x464 - GICD interrupt priority register 25"]
    #[inline(always)]
    pub const fn gicd_ipriorityr25(&self) -> &GICD_IPRIORITYR25 {
        &self.gicd_ipriorityr25
    }
    #[doc = "0x468 - GICD interrupt priority register 26"]
    #[inline(always)]
    pub const fn gicd_ipriorityr26(&self) -> &GICD_IPRIORITYR26 {
        &self.gicd_ipriorityr26
    }
    #[doc = "0x46c - GICD interrupt priority register 27"]
    #[inline(always)]
    pub const fn gicd_ipriorityr27(&self) -> &GICD_IPRIORITYR27 {
        &self.gicd_ipriorityr27
    }
    #[doc = "0x470 - GICD interrupt priority register 28"]
    #[inline(always)]
    pub const fn gicd_ipriorityr28(&self) -> &GICD_IPRIORITYR28 {
        &self.gicd_ipriorityr28
    }
    #[doc = "0x474 - GICD interrupt priority register 29"]
    #[inline(always)]
    pub const fn gicd_ipriorityr29(&self) -> &GICD_IPRIORITYR29 {
        &self.gicd_ipriorityr29
    }
    #[doc = "0x478 - GICD interrupt priority register 30"]
    #[inline(always)]
    pub const fn gicd_ipriorityr30(&self) -> &GICD_IPRIORITYR30 {
        &self.gicd_ipriorityr30
    }
    #[doc = "0x47c - GICD interrupt priority register 31"]
    #[inline(always)]
    pub const fn gicd_ipriorityr31(&self) -> &GICD_IPRIORITYR31 {
        &self.gicd_ipriorityr31
    }
    #[doc = "0x480 - GICD interrupt priority register 32"]
    #[inline(always)]
    pub const fn gicd_ipriorityr32(&self) -> &GICD_IPRIORITYR32 {
        &self.gicd_ipriorityr32
    }
    #[doc = "0x484 - GICD interrupt priority register 33"]
    #[inline(always)]
    pub const fn gicd_ipriorityr33(&self) -> &GICD_IPRIORITYR33 {
        &self.gicd_ipriorityr33
    }
    #[doc = "0x488 - GICD interrupt priority register 34"]
    #[inline(always)]
    pub const fn gicd_ipriorityr34(&self) -> &GICD_IPRIORITYR34 {
        &self.gicd_ipriorityr34
    }
    #[doc = "0x48c - GICD interrupt priority register 35"]
    #[inline(always)]
    pub const fn gicd_ipriorityr35(&self) -> &GICD_IPRIORITYR35 {
        &self.gicd_ipriorityr35
    }
    #[doc = "0x490 - GICD interrupt priority register 36"]
    #[inline(always)]
    pub const fn gicd_ipriorityr36(&self) -> &GICD_IPRIORITYR36 {
        &self.gicd_ipriorityr36
    }
    #[doc = "0x494 - GICD interrupt priority register 37"]
    #[inline(always)]
    pub const fn gicd_ipriorityr37(&self) -> &GICD_IPRIORITYR37 {
        &self.gicd_ipriorityr37
    }
    #[doc = "0x498 - GICD interrupt priority register 38"]
    #[inline(always)]
    pub const fn gicd_ipriorityr38(&self) -> &GICD_IPRIORITYR38 {
        &self.gicd_ipriorityr38
    }
    #[doc = "0x49c - GICD interrupt priority register 39"]
    #[inline(always)]
    pub const fn gicd_ipriorityr39(&self) -> &GICD_IPRIORITYR39 {
        &self.gicd_ipriorityr39
    }
    #[doc = "0x4a0 - GICD interrupt priority register 40"]
    #[inline(always)]
    pub const fn gicd_ipriorityr40(&self) -> &GICD_IPRIORITYR40 {
        &self.gicd_ipriorityr40
    }
    #[doc = "0x4a4 - GICD interrupt priority register 41"]
    #[inline(always)]
    pub const fn gicd_ipriorityr41(&self) -> &GICD_IPRIORITYR41 {
        &self.gicd_ipriorityr41
    }
    #[doc = "0x4a8 - GICD interrupt priority register 42"]
    #[inline(always)]
    pub const fn gicd_ipriorityr42(&self) -> &GICD_IPRIORITYR42 {
        &self.gicd_ipriorityr42
    }
    #[doc = "0x4ac - GICD interrupt priority register 43"]
    #[inline(always)]
    pub const fn gicd_ipriorityr43(&self) -> &GICD_IPRIORITYR43 {
        &self.gicd_ipriorityr43
    }
    #[doc = "0x4b0 - GICD interrupt priority register 44"]
    #[inline(always)]
    pub const fn gicd_ipriorityr44(&self) -> &GICD_IPRIORITYR44 {
        &self.gicd_ipriorityr44
    }
    #[doc = "0x4b4 - GICD interrupt priority register 45"]
    #[inline(always)]
    pub const fn gicd_ipriorityr45(&self) -> &GICD_IPRIORITYR45 {
        &self.gicd_ipriorityr45
    }
    #[doc = "0x4b8 - GICD interrupt priority register 46"]
    #[inline(always)]
    pub const fn gicd_ipriorityr46(&self) -> &GICD_IPRIORITYR46 {
        &self.gicd_ipriorityr46
    }
    #[doc = "0x4bc - GICD interrupt priority register 47"]
    #[inline(always)]
    pub const fn gicd_ipriorityr47(&self) -> &GICD_IPRIORITYR47 {
        &self.gicd_ipriorityr47
    }
    #[doc = "0x4c0 - GICD interrupt priority register 48"]
    #[inline(always)]
    pub const fn gicd_ipriorityr48(&self) -> &GICD_IPRIORITYR48 {
        &self.gicd_ipriorityr48
    }
    #[doc = "0x4c4 - GICD interrupt priority register 49"]
    #[inline(always)]
    pub const fn gicd_ipriorityr49(&self) -> &GICD_IPRIORITYR49 {
        &self.gicd_ipriorityr49
    }
    #[doc = "0x4c8 - GICD interrupt priority register 50"]
    #[inline(always)]
    pub const fn gicd_ipriorityr50(&self) -> &GICD_IPRIORITYR50 {
        &self.gicd_ipriorityr50
    }
    #[doc = "0x4cc - GICD interrupt priority register 51"]
    #[inline(always)]
    pub const fn gicd_ipriorityr51(&self) -> &GICD_IPRIORITYR51 {
        &self.gicd_ipriorityr51
    }
    #[doc = "0x4d0 - GICD interrupt priority register 52"]
    #[inline(always)]
    pub const fn gicd_ipriorityr52(&self) -> &GICD_IPRIORITYR52 {
        &self.gicd_ipriorityr52
    }
    #[doc = "0x4d4 - GICD interrupt priority register 53"]
    #[inline(always)]
    pub const fn gicd_ipriorityr53(&self) -> &GICD_IPRIORITYR53 {
        &self.gicd_ipriorityr53
    }
    #[doc = "0x4d8 - GICD interrupt priority register 54"]
    #[inline(always)]
    pub const fn gicd_ipriorityr54(&self) -> &GICD_IPRIORITYR54 {
        &self.gicd_ipriorityr54
    }
    #[doc = "0x4dc - GICD interrupt priority register 55"]
    #[inline(always)]
    pub const fn gicd_ipriorityr55(&self) -> &GICD_IPRIORITYR55 {
        &self.gicd_ipriorityr55
    }
    #[doc = "0x4e0 - GICD interrupt priority register 56"]
    #[inline(always)]
    pub const fn gicd_ipriorityr56(&self) -> &GICD_IPRIORITYR56 {
        &self.gicd_ipriorityr56
    }
    #[doc = "0x4e4 - GICD interrupt priority register 57"]
    #[inline(always)]
    pub const fn gicd_ipriorityr57(&self) -> &GICD_IPRIORITYR57 {
        &self.gicd_ipriorityr57
    }
    #[doc = "0x4e8 - GICD interrupt priority register 58"]
    #[inline(always)]
    pub const fn gicd_ipriorityr58(&self) -> &GICD_IPRIORITYR58 {
        &self.gicd_ipriorityr58
    }
    #[doc = "0x4ec - GICD interrupt priority register 59"]
    #[inline(always)]
    pub const fn gicd_ipriorityr59(&self) -> &GICD_IPRIORITYR59 {
        &self.gicd_ipriorityr59
    }
    #[doc = "0x4f0 - GICD interrupt priority register 60"]
    #[inline(always)]
    pub const fn gicd_ipriorityr60(&self) -> &GICD_IPRIORITYR60 {
        &self.gicd_ipriorityr60
    }
    #[doc = "0x4f4 - GICD interrupt priority register 61"]
    #[inline(always)]
    pub const fn gicd_ipriorityr61(&self) -> &GICD_IPRIORITYR61 {
        &self.gicd_ipriorityr61
    }
    #[doc = "0x4f8 - GICD interrupt priority register 62"]
    #[inline(always)]
    pub const fn gicd_ipriorityr62(&self) -> &GICD_IPRIORITYR62 {
        &self.gicd_ipriorityr62
    }
    #[doc = "0x4fc - GICD interrupt priority register 63"]
    #[inline(always)]
    pub const fn gicd_ipriorityr63(&self) -> &GICD_IPRIORITYR63 {
        &self.gicd_ipriorityr63
    }
    #[doc = "0x500 - GICD interrupt priority register 64"]
    #[inline(always)]
    pub const fn gicd_ipriorityr64(&self) -> &GICD_IPRIORITYR64 {
        &self.gicd_ipriorityr64
    }
    #[doc = "0x504 - GICD interrupt priority register 65"]
    #[inline(always)]
    pub const fn gicd_ipriorityr65(&self) -> &GICD_IPRIORITYR65 {
        &self.gicd_ipriorityr65
    }
    #[doc = "0x508 - GICD interrupt priority register 66"]
    #[inline(always)]
    pub const fn gicd_ipriorityr66(&self) -> &GICD_IPRIORITYR66 {
        &self.gicd_ipriorityr66
    }
    #[doc = "0x50c - GICD interrupt priority register 67"]
    #[inline(always)]
    pub const fn gicd_ipriorityr67(&self) -> &GICD_IPRIORITYR67 {
        &self.gicd_ipriorityr67
    }
    #[doc = "0x510 - GICD interrupt priority register 68"]
    #[inline(always)]
    pub const fn gicd_ipriorityr68(&self) -> &GICD_IPRIORITYR68 {
        &self.gicd_ipriorityr68
    }
    #[doc = "0x514 - GICD interrupt priority register 69"]
    #[inline(always)]
    pub const fn gicd_ipriorityr69(&self) -> &GICD_IPRIORITYR69 {
        &self.gicd_ipriorityr69
    }
    #[doc = "0x518 - GICD interrupt priority register 70"]
    #[inline(always)]
    pub const fn gicd_ipriorityr70(&self) -> &GICD_IPRIORITYR70 {
        &self.gicd_ipriorityr70
    }
    #[doc = "0x51c - GICD interrupt priority register 71"]
    #[inline(always)]
    pub const fn gicd_ipriorityr71(&self) -> &GICD_IPRIORITYR71 {
        &self.gicd_ipriorityr71
    }
    #[doc = "0x800 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    #[inline(always)]
    pub const fn gicd_itargetsr0(&self) -> &GICD_ITARGETSR0 {
        &self.gicd_itargetsr0
    }
    #[doc = "0x804 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    #[inline(always)]
    pub const fn gicd_itargetsr1(&self) -> &GICD_ITARGETSR1 {
        &self.gicd_itargetsr1
    }
    #[doc = "0x808 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    #[inline(always)]
    pub const fn gicd_itargetsr2(&self) -> &GICD_ITARGETSR2 {
        &self.gicd_itargetsr2
    }
    #[doc = "0x80c - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    #[inline(always)]
    pub const fn gicd_itargetsr3(&self) -> &GICD_ITARGETSR3 {
        &self.gicd_itargetsr3
    }
    #[doc = "0x810 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    #[inline(always)]
    pub const fn gicd_itargetsr4(&self) -> &GICD_ITARGETSR4 {
        &self.gicd_itargetsr4
    }
    #[doc = "0x814 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    #[inline(always)]
    pub const fn gicd_itargetsr5(&self) -> &GICD_ITARGETSR5 {
        &self.gicd_itargetsr5
    }
    #[doc = "0x818 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    #[inline(always)]
    pub const fn gicd_itargetsr6(&self) -> &GICD_ITARGETSR6 {
        &self.gicd_itargetsr6
    }
    #[doc = "0x81c - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    #[inline(always)]
    pub const fn gicd_itargetsr7(&self) -> &GICD_ITARGETSR7 {
        &self.gicd_itargetsr7
    }
    #[doc = "0x820 - GICD interrupt processor target register 8"]
    #[inline(always)]
    pub const fn gicd_itargetsr8(&self) -> &GICD_ITARGETSR8 {
        &self.gicd_itargetsr8
    }
    #[doc = "0x824 - GICD interrupt processor target register 9"]
    #[inline(always)]
    pub const fn gicd_itargetsr9(&self) -> &GICD_ITARGETSR9 {
        &self.gicd_itargetsr9
    }
    #[doc = "0x828 - GICD interrupt processor target register 10"]
    #[inline(always)]
    pub const fn gicd_itargetsr10(&self) -> &GICD_ITARGETSR10 {
        &self.gicd_itargetsr10
    }
    #[doc = "0x82c - GICD interrupt processor target register 11"]
    #[inline(always)]
    pub const fn gicd_itargetsr11(&self) -> &GICD_ITARGETSR11 {
        &self.gicd_itargetsr11
    }
    #[doc = "0x830 - GICD interrupt processor target register 12"]
    #[inline(always)]
    pub const fn gicd_itargetsr12(&self) -> &GICD_ITARGETSR12 {
        &self.gicd_itargetsr12
    }
    #[doc = "0x834 - GICD interrupt processor target register 13"]
    #[inline(always)]
    pub const fn gicd_itargetsr13(&self) -> &GICD_ITARGETSR13 {
        &self.gicd_itargetsr13
    }
    #[doc = "0x838 - GICD interrupt processor target register 14"]
    #[inline(always)]
    pub const fn gicd_itargetsr14(&self) -> &GICD_ITARGETSR14 {
        &self.gicd_itargetsr14
    }
    #[doc = "0x83c - GICD interrupt processor target register 15"]
    #[inline(always)]
    pub const fn gicd_itargetsr15(&self) -> &GICD_ITARGETSR15 {
        &self.gicd_itargetsr15
    }
    #[doc = "0x840 - GICD interrupt processor target register 16"]
    #[inline(always)]
    pub const fn gicd_itargetsr16(&self) -> &GICD_ITARGETSR16 {
        &self.gicd_itargetsr16
    }
    #[doc = "0x844 - GICD interrupt processor target register 17"]
    #[inline(always)]
    pub const fn gicd_itargetsr17(&self) -> &GICD_ITARGETSR17 {
        &self.gicd_itargetsr17
    }
    #[doc = "0x848 - GICD interrupt processor target register 18"]
    #[inline(always)]
    pub const fn gicd_itargetsr18(&self) -> &GICD_ITARGETSR18 {
        &self.gicd_itargetsr18
    }
    #[doc = "0x84c - GICD interrupt processor target register 19"]
    #[inline(always)]
    pub const fn gicd_itargetsr19(&self) -> &GICD_ITARGETSR19 {
        &self.gicd_itargetsr19
    }
    #[doc = "0x850 - GICD interrupt processor target register 20"]
    #[inline(always)]
    pub const fn gicd_itargetsr20(&self) -> &GICD_ITARGETSR20 {
        &self.gicd_itargetsr20
    }
    #[doc = "0x854 - GICD interrupt processor target register 21"]
    #[inline(always)]
    pub const fn gicd_itargetsr21(&self) -> &GICD_ITARGETSR21 {
        &self.gicd_itargetsr21
    }
    #[doc = "0x858 - GICD interrupt processor target register 22"]
    #[inline(always)]
    pub const fn gicd_itargetsr22(&self) -> &GICD_ITARGETSR22 {
        &self.gicd_itargetsr22
    }
    #[doc = "0x85c - GICD interrupt processor target register 23"]
    #[inline(always)]
    pub const fn gicd_itargetsr23(&self) -> &GICD_ITARGETSR23 {
        &self.gicd_itargetsr23
    }
    #[doc = "0x860 - GICD interrupt processor target register 24"]
    #[inline(always)]
    pub const fn gicd_itargetsr24(&self) -> &GICD_ITARGETSR24 {
        &self.gicd_itargetsr24
    }
    #[doc = "0x864 - GICD interrupt processor target register 25"]
    #[inline(always)]
    pub const fn gicd_itargetsr25(&self) -> &GICD_ITARGETSR25 {
        &self.gicd_itargetsr25
    }
    #[doc = "0x868 - GICD interrupt processor target register 26"]
    #[inline(always)]
    pub const fn gicd_itargetsr26(&self) -> &GICD_ITARGETSR26 {
        &self.gicd_itargetsr26
    }
    #[doc = "0x86c - GICD interrupt processor target register 27"]
    #[inline(always)]
    pub const fn gicd_itargetsr27(&self) -> &GICD_ITARGETSR27 {
        &self.gicd_itargetsr27
    }
    #[doc = "0x870 - GICD interrupt processor target register 28"]
    #[inline(always)]
    pub const fn gicd_itargetsr28(&self) -> &GICD_ITARGETSR28 {
        &self.gicd_itargetsr28
    }
    #[doc = "0x874 - GICD interrupt processor target register 29"]
    #[inline(always)]
    pub const fn gicd_itargetsr29(&self) -> &GICD_ITARGETSR29 {
        &self.gicd_itargetsr29
    }
    #[doc = "0x878 - GICD interrupt processor target register 30"]
    #[inline(always)]
    pub const fn gicd_itargetsr30(&self) -> &GICD_ITARGETSR30 {
        &self.gicd_itargetsr30
    }
    #[doc = "0x87c - GICD interrupt processor target register 31"]
    #[inline(always)]
    pub const fn gicd_itargetsr31(&self) -> &GICD_ITARGETSR31 {
        &self.gicd_itargetsr31
    }
    #[doc = "0x880 - GICD interrupt processor target register 32"]
    #[inline(always)]
    pub const fn gicd_itargetsr32(&self) -> &GICD_ITARGETSR32 {
        &self.gicd_itargetsr32
    }
    #[doc = "0x884 - GICD interrupt processor target register 33"]
    #[inline(always)]
    pub const fn gicd_itargetsr33(&self) -> &GICD_ITARGETSR33 {
        &self.gicd_itargetsr33
    }
    #[doc = "0x888 - GICD interrupt processor target register 34"]
    #[inline(always)]
    pub const fn gicd_itargetsr34(&self) -> &GICD_ITARGETSR34 {
        &self.gicd_itargetsr34
    }
    #[doc = "0x88c - GICD interrupt processor target register 35"]
    #[inline(always)]
    pub const fn gicd_itargetsr35(&self) -> &GICD_ITARGETSR35 {
        &self.gicd_itargetsr35
    }
    #[doc = "0x890 - GICD interrupt processor target register 36"]
    #[inline(always)]
    pub const fn gicd_itargetsr36(&self) -> &GICD_ITARGETSR36 {
        &self.gicd_itargetsr36
    }
    #[doc = "0x894 - GICD interrupt processor target register 37"]
    #[inline(always)]
    pub const fn gicd_itargetsr37(&self) -> &GICD_ITARGETSR37 {
        &self.gicd_itargetsr37
    }
    #[doc = "0x898 - GICD interrupt processor target register 38"]
    #[inline(always)]
    pub const fn gicd_itargetsr38(&self) -> &GICD_ITARGETSR38 {
        &self.gicd_itargetsr38
    }
    #[doc = "0x89c - GICD interrupt processor target register 39"]
    #[inline(always)]
    pub const fn gicd_itargetsr39(&self) -> &GICD_ITARGETSR39 {
        &self.gicd_itargetsr39
    }
    #[doc = "0x8a0 - GICD interrupt processor target register 40"]
    #[inline(always)]
    pub const fn gicd_itargetsr40(&self) -> &GICD_ITARGETSR40 {
        &self.gicd_itargetsr40
    }
    #[doc = "0x8a4 - GICD interrupt processor target register 41"]
    #[inline(always)]
    pub const fn gicd_itargetsr41(&self) -> &GICD_ITARGETSR41 {
        &self.gicd_itargetsr41
    }
    #[doc = "0x8a8 - GICD interrupt processor target register 42"]
    #[inline(always)]
    pub const fn gicd_itargetsr42(&self) -> &GICD_ITARGETSR42 {
        &self.gicd_itargetsr42
    }
    #[doc = "0x8ac - GICD interrupt processor target register 43"]
    #[inline(always)]
    pub const fn gicd_itargetsr43(&self) -> &GICD_ITARGETSR43 {
        &self.gicd_itargetsr43
    }
    #[doc = "0x8b0 - GICD interrupt processor target register 44"]
    #[inline(always)]
    pub const fn gicd_itargetsr44(&self) -> &GICD_ITARGETSR44 {
        &self.gicd_itargetsr44
    }
    #[doc = "0x8b4 - GICD interrupt processor target register 45"]
    #[inline(always)]
    pub const fn gicd_itargetsr45(&self) -> &GICD_ITARGETSR45 {
        &self.gicd_itargetsr45
    }
    #[doc = "0x8b8 - GICD interrupt processor target register 46"]
    #[inline(always)]
    pub const fn gicd_itargetsr46(&self) -> &GICD_ITARGETSR46 {
        &self.gicd_itargetsr46
    }
    #[doc = "0x8bc - GICD interrupt processor target register 47"]
    #[inline(always)]
    pub const fn gicd_itargetsr47(&self) -> &GICD_ITARGETSR47 {
        &self.gicd_itargetsr47
    }
    #[doc = "0x8c0 - GICD interrupt processor target register 48"]
    #[inline(always)]
    pub const fn gicd_itargetsr48(&self) -> &GICD_ITARGETSR48 {
        &self.gicd_itargetsr48
    }
    #[doc = "0x8c4 - GICD interrupt processor target register 49"]
    #[inline(always)]
    pub const fn gicd_itargetsr49(&self) -> &GICD_ITARGETSR49 {
        &self.gicd_itargetsr49
    }
    #[doc = "0x8c8 - GICD interrupt processor target register 50"]
    #[inline(always)]
    pub const fn gicd_itargetsr50(&self) -> &GICD_ITARGETSR50 {
        &self.gicd_itargetsr50
    }
    #[doc = "0x8cc - GICD interrupt processor target register 51"]
    #[inline(always)]
    pub const fn gicd_itargetsr51(&self) -> &GICD_ITARGETSR51 {
        &self.gicd_itargetsr51
    }
    #[doc = "0x8d0 - GICD interrupt processor target register 52"]
    #[inline(always)]
    pub const fn gicd_itargetsr52(&self) -> &GICD_ITARGETSR52 {
        &self.gicd_itargetsr52
    }
    #[doc = "0x8d4 - GICD interrupt processor target register 53"]
    #[inline(always)]
    pub const fn gicd_itargetsr53(&self) -> &GICD_ITARGETSR53 {
        &self.gicd_itargetsr53
    }
    #[doc = "0x8d8 - GICD interrupt processor target register 54"]
    #[inline(always)]
    pub const fn gicd_itargetsr54(&self) -> &GICD_ITARGETSR54 {
        &self.gicd_itargetsr54
    }
    #[doc = "0x8dc - GICD interrupt processor target register 55"]
    #[inline(always)]
    pub const fn gicd_itargetsr55(&self) -> &GICD_ITARGETSR55 {
        &self.gicd_itargetsr55
    }
    #[doc = "0x8e0 - GICD interrupt processor target register 56"]
    #[inline(always)]
    pub const fn gicd_itargetsr56(&self) -> &GICD_ITARGETSR56 {
        &self.gicd_itargetsr56
    }
    #[doc = "0x8e4 - GICD interrupt processor target register 57"]
    #[inline(always)]
    pub const fn gicd_itargetsr57(&self) -> &GICD_ITARGETSR57 {
        &self.gicd_itargetsr57
    }
    #[doc = "0x8e8 - GICD interrupt processor target register 58"]
    #[inline(always)]
    pub const fn gicd_itargetsr58(&self) -> &GICD_ITARGETSR58 {
        &self.gicd_itargetsr58
    }
    #[doc = "0x8ec - GICD interrupt processor target register 59"]
    #[inline(always)]
    pub const fn gicd_itargetsr59(&self) -> &GICD_ITARGETSR59 {
        &self.gicd_itargetsr59
    }
    #[doc = "0x8f0 - GICD interrupt processor target register 60"]
    #[inline(always)]
    pub const fn gicd_itargetsr60(&self) -> &GICD_ITARGETSR60 {
        &self.gicd_itargetsr60
    }
    #[doc = "0x8f4 - GICD interrupt processor target register 61"]
    #[inline(always)]
    pub const fn gicd_itargetsr61(&self) -> &GICD_ITARGETSR61 {
        &self.gicd_itargetsr61
    }
    #[doc = "0x8f8 - GICD interrupt processor target register 62"]
    #[inline(always)]
    pub const fn gicd_itargetsr62(&self) -> &GICD_ITARGETSR62 {
        &self.gicd_itargetsr62
    }
    #[doc = "0x8fc - GICD interrupt processor target register 63"]
    #[inline(always)]
    pub const fn gicd_itargetsr63(&self) -> &GICD_ITARGETSR63 {
        &self.gicd_itargetsr63
    }
    #[doc = "0x900 - GICD interrupt processor target register 64"]
    #[inline(always)]
    pub const fn gicd_itargetsr64(&self) -> &GICD_ITARGETSR64 {
        &self.gicd_itargetsr64
    }
    #[doc = "0x904 - GICD interrupt processor target register 65"]
    #[inline(always)]
    pub const fn gicd_itargetsr65(&self) -> &GICD_ITARGETSR65 {
        &self.gicd_itargetsr65
    }
    #[doc = "0x908 - GICD interrupt processor target register 66"]
    #[inline(always)]
    pub const fn gicd_itargetsr66(&self) -> &GICD_ITARGETSR66 {
        &self.gicd_itargetsr66
    }
    #[doc = "0x90c - GICD interrupt processor target register 67"]
    #[inline(always)]
    pub const fn gicd_itargetsr67(&self) -> &GICD_ITARGETSR67 {
        &self.gicd_itargetsr67
    }
    #[doc = "0x910 - GICD interrupt processor target register 68"]
    #[inline(always)]
    pub const fn gicd_itargetsr68(&self) -> &GICD_ITARGETSR68 {
        &self.gicd_itargetsr68
    }
    #[doc = "0x914 - GICD interrupt processor target register 69"]
    #[inline(always)]
    pub const fn gicd_itargetsr69(&self) -> &GICD_ITARGETSR69 {
        &self.gicd_itargetsr69
    }
    #[doc = "0x918 - GICD interrupt processor target register 70"]
    #[inline(always)]
    pub const fn gicd_itargetsr70(&self) -> &GICD_ITARGETSR70 {
        &self.gicd_itargetsr70
    }
    #[doc = "0x91c - GICD interrupt processor target register 71"]
    #[inline(always)]
    pub const fn gicd_itargetsr71(&self) -> &GICD_ITARGETSR71 {
        &self.gicd_itargetsr71
    }
    #[doc = "0xc00 - GICD interrupt configuration register"]
    #[inline(always)]
    pub const fn gicd_icfgr0(&self) -> &GICD_ICFGR0 {
        &self.gicd_icfgr0
    }
    #[doc = "0xc04 - GICD interrupt configuration register"]
    #[inline(always)]
    pub const fn gicd_icfgr1(&self) -> &GICD_ICFGR1 {
        &self.gicd_icfgr1
    }
    #[doc = "0xc08 - GICD interrupt configuration register 2"]
    #[inline(always)]
    pub const fn gicd_icfgr2(&self) -> &GICD_ICFGR2 {
        &self.gicd_icfgr2
    }
    #[doc = "0xc0c - GICD interrupt configuration register 3"]
    #[inline(always)]
    pub const fn gicd_icfgr3(&self) -> &GICD_ICFGR3 {
        &self.gicd_icfgr3
    }
    #[doc = "0xc10 - GICD interrupt configuration register 4"]
    #[inline(always)]
    pub const fn gicd_icfgr4(&self) -> &GICD_ICFGR4 {
        &self.gicd_icfgr4
    }
    #[doc = "0xc14 - GICD interrupt configuration register 5"]
    #[inline(always)]
    pub const fn gicd_icfgr5(&self) -> &GICD_ICFGR5 {
        &self.gicd_icfgr5
    }
    #[doc = "0xc18 - GICD interrupt configuration register 6"]
    #[inline(always)]
    pub const fn gicd_icfgr6(&self) -> &GICD_ICFGR6 {
        &self.gicd_icfgr6
    }
    #[doc = "0xc1c - GICD interrupt configuration register 7"]
    #[inline(always)]
    pub const fn gicd_icfgr7(&self) -> &GICD_ICFGR7 {
        &self.gicd_icfgr7
    }
    #[doc = "0xc20 - GICD interrupt configuration register 8"]
    #[inline(always)]
    pub const fn gicd_icfgr8(&self) -> &GICD_ICFGR8 {
        &self.gicd_icfgr8
    }
    #[doc = "0xc24 - GICD interrupt configuration register 9"]
    #[inline(always)]
    pub const fn gicd_icfgr9(&self) -> &GICD_ICFGR9 {
        &self.gicd_icfgr9
    }
    #[doc = "0xc28 - GICD interrupt configuration register 10"]
    #[inline(always)]
    pub const fn gicd_icfgr10(&self) -> &GICD_ICFGR10 {
        &self.gicd_icfgr10
    }
    #[doc = "0xc2c - GICD interrupt configuration register 11"]
    #[inline(always)]
    pub const fn gicd_icfgr11(&self) -> &GICD_ICFGR11 {
        &self.gicd_icfgr11
    }
    #[doc = "0xc30 - GICD interrupt configuration register 12"]
    #[inline(always)]
    pub const fn gicd_icfgr12(&self) -> &GICD_ICFGR12 {
        &self.gicd_icfgr12
    }
    #[doc = "0xc34 - GICD interrupt configuration register 13"]
    #[inline(always)]
    pub const fn gicd_icfgr13(&self) -> &GICD_ICFGR13 {
        &self.gicd_icfgr13
    }
    #[doc = "0xc38 - GICD interrupt configuration register 14"]
    #[inline(always)]
    pub const fn gicd_icfgr14(&self) -> &GICD_ICFGR14 {
        &self.gicd_icfgr14
    }
    #[doc = "0xc3c - GICD interrupt configuration register 15"]
    #[inline(always)]
    pub const fn gicd_icfgr15(&self) -> &GICD_ICFGR15 {
        &self.gicd_icfgr15
    }
    #[doc = "0xc40 - GICD interrupt configuration register 16"]
    #[inline(always)]
    pub const fn gicd_icfgr16(&self) -> &GICD_ICFGR16 {
        &self.gicd_icfgr16
    }
    #[doc = "0xc44 - GICD interrupt configuration register 17"]
    #[inline(always)]
    pub const fn gicd_icfgr17(&self) -> &GICD_ICFGR17 {
        &self.gicd_icfgr17
    }
    #[doc = "0xd00 - GICD private peripheral interrupt status register"]
    #[inline(always)]
    pub const fn gicd_ppisr(&self) -> &GICD_PPISR {
        &self.gicd_ppisr
    }
    #[doc = "0xd08 - For interrupts ID = SPI number+32, from SPI \\[x*32+31\\]
to SPI \\[x*32\\]"]
    #[inline(always)]
    pub const fn gicd_spisr1(&self) -> &GICD_SPISR1 {
        &self.gicd_spisr1
    }
    #[doc = "0xd0c - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_spisr2(&self) -> &GICD_SPISR2 {
        &self.gicd_spisr2
    }
    #[doc = "0xd10 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_spisr3(&self) -> &GICD_SPISR3 {
        &self.gicd_spisr3
    }
    #[doc = "0xd14 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_spisr4(&self) -> &GICD_SPISR4 {
        &self.gicd_spisr4
    }
    #[doc = "0xd18 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_spisr5(&self) -> &GICD_SPISR5 {
        &self.gicd_spisr5
    }
    #[doc = "0xd1c - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_spisr6(&self) -> &GICD_SPISR6 {
        &self.gicd_spisr6
    }
    #[doc = "0xd20 - For interrupts ID"]
    #[inline(always)]
    pub const fn gicd_spisr7(&self) -> &GICD_SPISR7 {
        &self.gicd_spisr7
    }
    #[doc = "0xf00 - GICD software generated interrupt register"]
    #[inline(always)]
    pub const fn gicd_sgir(&self) -> &GICD_SGIR {
        &self.gicd_sgir
    }
    #[doc = "0xf10 - For SGI x*4 to SGI x*4+3"]
    #[inline(always)]
    pub const fn gicd_cpendsgir0(&self) -> &GICD_CPENDSGIR0 {
        &self.gicd_cpendsgir0
    }
    #[doc = "0xf14 - For SGI x*4 to SGI x*4+3"]
    #[inline(always)]
    pub const fn gicd_cpendsgir1(&self) -> &GICD_CPENDSGIR1 {
        &self.gicd_cpendsgir1
    }
    #[doc = "0xf18 - For SGI x*4 to SGI x*4+3"]
    #[inline(always)]
    pub const fn gicd_cpendsgir2(&self) -> &GICD_CPENDSGIR2 {
        &self.gicd_cpendsgir2
    }
    #[doc = "0xf1c - For SGI x*4 to SGI x*4+3"]
    #[inline(always)]
    pub const fn gicd_cpendsgir3(&self) -> &GICD_CPENDSGIR3 {
        &self.gicd_cpendsgir3
    }
    #[doc = "0xf20 - For SGI x*4 to SGI x*4+3"]
    #[inline(always)]
    pub const fn gicd_spendsgir0(&self) -> &GICD_SPENDSGIR0 {
        &self.gicd_spendsgir0
    }
    #[doc = "0xf24 - For SGI x*4 to SGI x*4+3"]
    #[inline(always)]
    pub const fn gicd_spendsgir1(&self) -> &GICD_SPENDSGIR1 {
        &self.gicd_spendsgir1
    }
    #[doc = "0xf28 - For SGI x*4 to SGI x*4+3"]
    #[inline(always)]
    pub const fn gicd_spendsgir2(&self) -> &GICD_SPENDSGIR2 {
        &self.gicd_spendsgir2
    }
    #[doc = "0xf2c - For SGI x*4 to SGI x*4+3"]
    #[inline(always)]
    pub const fn gicd_spendsgir3(&self) -> &GICD_SPENDSGIR3 {
        &self.gicd_spendsgir3
    }
    #[doc = "0xfd0 - GICD peripheral ID4 register"]
    #[inline(always)]
    pub const fn gicd_pidr4(&self) -> &GICD_PIDR4 {
        &self.gicd_pidr4
    }
    #[doc = "0xfd4 - GICD peripheral ID5 to ID7 register 5"]
    #[inline(always)]
    pub const fn gicd_pidr5(&self) -> &GICD_PIDR5 {
        &self.gicd_pidr5
    }
    #[doc = "0xfd8 - GICD peripheral ID5 to ID7 register 6"]
    #[inline(always)]
    pub const fn gicd_pidr6(&self) -> &GICD_PIDR6 {
        &self.gicd_pidr6
    }
    #[doc = "0xfdc - GICD peripheral ID5 to ID7 register 7"]
    #[inline(always)]
    pub const fn gicd_pidr7(&self) -> &GICD_PIDR7 {
        &self.gicd_pidr7
    }
    #[doc = "0xfe0 - GICD peripheral ID0 register"]
    #[inline(always)]
    pub const fn gicd_pidr0(&self) -> &GICD_PIDR0 {
        &self.gicd_pidr0
    }
    #[doc = "0xfe4 - GICD peripheral ID1 register"]
    #[inline(always)]
    pub const fn gicd_pidr1(&self) -> &GICD_PIDR1 {
        &self.gicd_pidr1
    }
    #[doc = "0xfe8 - GICD peripheral ID2 register"]
    #[inline(always)]
    pub const fn gicd_pidr2(&self) -> &GICD_PIDR2 {
        &self.gicd_pidr2
    }
    #[doc = "0xfec - GICD peripheral ID3 register"]
    #[inline(always)]
    pub const fn gicd_pidr3(&self) -> &GICD_PIDR3 {
        &self.gicd_pidr3
    }
    #[doc = "0xff0 - GICD component ID0 register"]
    #[inline(always)]
    pub const fn gicd_cidr0(&self) -> &GICD_CIDR0 {
        &self.gicd_cidr0
    }
    #[doc = "0xff4 - GICD component ID1 register"]
    #[inline(always)]
    pub const fn gicd_cidr1(&self) -> &GICD_CIDR1 {
        &self.gicd_cidr1
    }
    #[doc = "0xff8 - GICD component ID2 register"]
    #[inline(always)]
    pub const fn gicd_cidr2(&self) -> &GICD_CIDR2 {
        &self.gicd_cidr2
    }
    #[doc = "0xffc - GICD component ID3 register"]
    #[inline(always)]
    pub const fn gicd_cidr3(&self) -> &GICD_CIDR3 {
        &self.gicd_cidr3
    }
}
#[doc = "GICD_CTLR (rw) register accessor: GICD control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ctlr`]
module"]
pub type GICD_CTLR = crate::Reg<gicd_ctlr::GICD_CTLRrs>;
#[doc = "GICD control register"]
pub mod gicd_ctlr;
#[doc = "GICD_TYPER (r) register accessor: GICD interrupt controller type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_typer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_typer`]
module"]
pub type GICD_TYPER = crate::Reg<gicd_typer::GICD_TYPERrs>;
#[doc = "GICD interrupt controller type register"]
pub mod gicd_typer;
#[doc = "GICD_IIDR (r) register accessor: GICD implementer identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_iidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_iidr`]
module"]
pub type GICD_IIDR = crate::Reg<gicd_iidr::GICD_IIDRrs>;
#[doc = "GICD implementer identification register"]
pub mod gicd_iidr;
#[doc = "GICD_IGROUPR0 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr0`]
module"]
pub type GICD_IGROUPR0 = crate::Reg<gicd_igroupr0::GICD_IGROUPR0rs>;
#[doc = "For interrupts ID"]
pub mod gicd_igroupr0;
#[doc = "GICD_IGROUPR1 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr1`]
module"]
pub type GICD_IGROUPR1 = crate::Reg<gicd_igroupr1::GICD_IGROUPR1rs>;
#[doc = "For interrupts ID"]
pub mod gicd_igroupr1;
#[doc = "GICD_IGROUPR2 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr2`]
module"]
pub type GICD_IGROUPR2 = crate::Reg<gicd_igroupr2::GICD_IGROUPR2rs>;
#[doc = "For interrupts ID"]
pub mod gicd_igroupr2;
#[doc = "GICD_IGROUPR3 (rw) register accessor: For interrupts ID = x*32 to ID = x*32+31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr3`]
module"]
pub type GICD_IGROUPR3 = crate::Reg<gicd_igroupr3::GICD_IGROUPR3rs>;
#[doc = "For interrupts ID = x*32 to ID = x*32+31"]
pub mod gicd_igroupr3;
#[doc = "GICD_IGROUPR4 (rw) register accessor: For interrupts ID = x*32 to ID = x*32+31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr4`]
module"]
pub type GICD_IGROUPR4 = crate::Reg<gicd_igroupr4::GICD_IGROUPR4rs>;
#[doc = "For interrupts ID = x*32 to ID = x*32+31"]
pub mod gicd_igroupr4;
#[doc = "GICD_IGROUPR5 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr5`]
module"]
pub type GICD_IGROUPR5 = crate::Reg<gicd_igroupr5::GICD_IGROUPR5rs>;
#[doc = "For interrupts ID"]
pub mod gicd_igroupr5;
#[doc = "GICD_IGROUPR6 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr6`]
module"]
pub type GICD_IGROUPR6 = crate::Reg<gicd_igroupr6::GICD_IGROUPR6rs>;
#[doc = "For interrupts ID"]
pub mod gicd_igroupr6;
#[doc = "GICD_IGROUPR7 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr7`]
module"]
pub type GICD_IGROUPR7 = crate::Reg<gicd_igroupr7::GICD_IGROUPR7rs>;
#[doc = "For interrupts ID"]
pub mod gicd_igroupr7;
#[doc = "GICD_IGROUPR8 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr8`]
module"]
pub type GICD_IGROUPR8 = crate::Reg<gicd_igroupr8::GICD_IGROUPR8rs>;
#[doc = "For interrupts ID"]
pub mod gicd_igroupr8;
#[doc = "GICD_ISENABLER0 (rw) register accessor: For interrupts ID = 0 to ID = 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isenabler0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isenabler0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isenabler0`]
module"]
pub type GICD_ISENABLER0 = crate::Reg<gicd_isenabler0::GICD_ISENABLER0rs>;
#[doc = "For interrupts ID = 0 to ID = 31"]
pub mod gicd_isenabler0;
#[doc = "GICD_ISENABLER1 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isenabler1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isenabler1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isenabler1`]
module"]
pub type GICD_ISENABLER1 = crate::Reg<gicd_isenabler1::GICD_ISENABLER1rs>;
#[doc = "For interrupts ID"]
pub mod gicd_isenabler1;
#[doc = "GICD_ISENABLER2 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isenabler2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isenabler2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isenabler2`]
module"]
pub type GICD_ISENABLER2 = crate::Reg<gicd_isenabler2::GICD_ISENABLER2rs>;
#[doc = "For interrupts ID"]
pub mod gicd_isenabler2;
#[doc = "GICD_ISENABLER3 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isenabler3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isenabler3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isenabler3`]
module"]
pub type GICD_ISENABLER3 = crate::Reg<gicd_isenabler3::GICD_ISENABLER3rs>;
#[doc = "For interrupts ID"]
pub mod gicd_isenabler3;
#[doc = "GICD_ISENABLER4 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isenabler4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isenabler4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isenabler4`]
module"]
pub type GICD_ISENABLER4 = crate::Reg<gicd_isenabler4::GICD_ISENABLER4rs>;
#[doc = "For interrupts ID"]
pub mod gicd_isenabler4;
#[doc = "GICD_ISENABLER5 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isenabler5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isenabler5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isenabler5`]
module"]
pub type GICD_ISENABLER5 = crate::Reg<gicd_isenabler5::GICD_ISENABLER5rs>;
#[doc = "For interrupts ID"]
pub mod gicd_isenabler5;
#[doc = "GICD_ISENABLER6 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isenabler6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isenabler6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isenabler6`]
module"]
pub type GICD_ISENABLER6 = crate::Reg<gicd_isenabler6::GICD_ISENABLER6rs>;
#[doc = "For interrupts ID"]
pub mod gicd_isenabler6;
#[doc = "GICD_ISENABLER7 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isenabler7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isenabler7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isenabler7`]
module"]
pub type GICD_ISENABLER7 = crate::Reg<gicd_isenabler7::GICD_ISENABLER7rs>;
#[doc = "For interrupts ID"]
pub mod gicd_isenabler7;
#[doc = "GICD_ISENABLER8 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isenabler8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isenabler8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isenabler8`]
module"]
pub type GICD_ISENABLER8 = crate::Reg<gicd_isenabler8::GICD_ISENABLER8rs>;
#[doc = "For interrupts ID"]
pub mod gicd_isenabler8;
#[doc = "GICD_ICENABLER0 (rw) register accessor: For interrupts ID = 0 to ID = 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler0`]
module"]
pub type GICD_ICENABLER0 = crate::Reg<gicd_icenabler0::GICD_ICENABLER0rs>;
#[doc = "For interrupts ID = 0 to ID = 31"]
pub mod gicd_icenabler0;
#[doc = "GICD_ICENABLER1 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler1`]
module"]
pub type GICD_ICENABLER1 = crate::Reg<gicd_icenabler1::GICD_ICENABLER1rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icenabler1;
#[doc = "GICD_ICENABLER2 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler2`]
module"]
pub type GICD_ICENABLER2 = crate::Reg<gicd_icenabler2::GICD_ICENABLER2rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icenabler2;
#[doc = "GICD_ICENABLER3 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler3`]
module"]
pub type GICD_ICENABLER3 = crate::Reg<gicd_icenabler3::GICD_ICENABLER3rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icenabler3;
#[doc = "GICD_ICENABLER4 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler4`]
module"]
pub type GICD_ICENABLER4 = crate::Reg<gicd_icenabler4::GICD_ICENABLER4rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icenabler4;
#[doc = "GICD_ICENABLER5 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler5`]
module"]
pub type GICD_ICENABLER5 = crate::Reg<gicd_icenabler5::GICD_ICENABLER5rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icenabler5;
#[doc = "GICD_ICENABLER6 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler6`]
module"]
pub type GICD_ICENABLER6 = crate::Reg<gicd_icenabler6::GICD_ICENABLER6rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icenabler6;
#[doc = "GICD_ICENABLER7 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler7`]
module"]
pub type GICD_ICENABLER7 = crate::Reg<gicd_icenabler7::GICD_ICENABLER7rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icenabler7;
#[doc = "GICD_ICENABLER8 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler8`]
module"]
pub type GICD_ICENABLER8 = crate::Reg<gicd_icenabler8::GICD_ICENABLER8rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icenabler8;
#[doc = "GICD_ISPENDR0 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr0`]
module"]
pub type GICD_ISPENDR0 = crate::Reg<gicd_ispendr0::GICD_ISPENDR0rs>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr0;
#[doc = "GICD_ISPENDR1 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr1`]
module"]
pub type GICD_ISPENDR1 = crate::Reg<gicd_ispendr1::GICD_ISPENDR1rs>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr1;
#[doc = "GICD_ISPENDR2 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr2`]
module"]
pub type GICD_ISPENDR2 = crate::Reg<gicd_ispendr2::GICD_ISPENDR2rs>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr2;
#[doc = "GICD_ISPENDR3 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr3`]
module"]
pub type GICD_ISPENDR3 = crate::Reg<gicd_ispendr3::GICD_ISPENDR3rs>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr3;
#[doc = "GICD_ISPENDR4 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr4`]
module"]
pub type GICD_ISPENDR4 = crate::Reg<gicd_ispendr4::GICD_ISPENDR4rs>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr4;
#[doc = "GICD_ISPENDR5 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr5`]
module"]
pub type GICD_ISPENDR5 = crate::Reg<gicd_ispendr5::GICD_ISPENDR5rs>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr5;
#[doc = "GICD_ISPENDR6 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr6`]
module"]
pub type GICD_ISPENDR6 = crate::Reg<gicd_ispendr6::GICD_ISPENDR6rs>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr6;
#[doc = "GICD_ISPENDR7 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr7`]
module"]
pub type GICD_ISPENDR7 = crate::Reg<gicd_ispendr7::GICD_ISPENDR7rs>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr7;
#[doc = "GICD_ISPENDR8 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr8`]
module"]
pub type GICD_ISPENDR8 = crate::Reg<gicd_ispendr8::GICD_ISPENDR8rs>;
#[doc = "For interrupts ID"]
pub mod gicd_ispendr8;
#[doc = "GICD_ICPENDR0 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr0`]
module"]
pub type GICD_ICPENDR0 = crate::Reg<gicd_icpendr0::GICD_ICPENDR0rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr0;
#[doc = "GICD_ICPENDR1 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr1`]
module"]
pub type GICD_ICPENDR1 = crate::Reg<gicd_icpendr1::GICD_ICPENDR1rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr1;
#[doc = "GICD_ICPENDR2 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr2`]
module"]
pub type GICD_ICPENDR2 = crate::Reg<gicd_icpendr2::GICD_ICPENDR2rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr2;
#[doc = "GICD_ICPENDR3 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr3`]
module"]
pub type GICD_ICPENDR3 = crate::Reg<gicd_icpendr3::GICD_ICPENDR3rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr3;
#[doc = "GICD_ICPENDR4 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr4`]
module"]
pub type GICD_ICPENDR4 = crate::Reg<gicd_icpendr4::GICD_ICPENDR4rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr4;
#[doc = "GICD_ICPENDR5 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr5`]
module"]
pub type GICD_ICPENDR5 = crate::Reg<gicd_icpendr5::GICD_ICPENDR5rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr5;
#[doc = "GICD_ICPENDR6 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr6`]
module"]
pub type GICD_ICPENDR6 = crate::Reg<gicd_icpendr6::GICD_ICPENDR6rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr6;
#[doc = "GICD_ICPENDR7 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr7`]
module"]
pub type GICD_ICPENDR7 = crate::Reg<gicd_icpendr7::GICD_ICPENDR7rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr7;
#[doc = "GICD_ICPENDR8 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr8`]
module"]
pub type GICD_ICPENDR8 = crate::Reg<gicd_icpendr8::GICD_ICPENDR8rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icpendr8;
#[doc = "GICD_ISACTIVER0 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver0`]
module"]
pub type GICD_ISACTIVER0 = crate::Reg<gicd_isactiver0::GICD_ISACTIVER0rs>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver0;
#[doc = "GICD_ISACTIVER1 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver1`]
module"]
pub type GICD_ISACTIVER1 = crate::Reg<gicd_isactiver1::GICD_ISACTIVER1rs>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver1;
#[doc = "GICD_ISACTIVER2 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver2`]
module"]
pub type GICD_ISACTIVER2 = crate::Reg<gicd_isactiver2::GICD_ISACTIVER2rs>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver2;
#[doc = "GICD_ISACTIVER3 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver3`]
module"]
pub type GICD_ISACTIVER3 = crate::Reg<gicd_isactiver3::GICD_ISACTIVER3rs>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver3;
#[doc = "GICD_ISACTIVER4 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver4`]
module"]
pub type GICD_ISACTIVER4 = crate::Reg<gicd_isactiver4::GICD_ISACTIVER4rs>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver4;
#[doc = "GICD_ISACTIVER5 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver5`]
module"]
pub type GICD_ISACTIVER5 = crate::Reg<gicd_isactiver5::GICD_ISACTIVER5rs>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver5;
#[doc = "GICD_ISACTIVER6 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver6`]
module"]
pub type GICD_ISACTIVER6 = crate::Reg<gicd_isactiver6::GICD_ISACTIVER6rs>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver6;
#[doc = "GICD_ISACTIVER7 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver7`]
module"]
pub type GICD_ISACTIVER7 = crate::Reg<gicd_isactiver7::GICD_ISACTIVER7rs>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver7;
#[doc = "GICD_ISACTIVER8 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver8`]
module"]
pub type GICD_ISACTIVER8 = crate::Reg<gicd_isactiver8::GICD_ISACTIVER8rs>;
#[doc = "For interrupts ID"]
pub mod gicd_isactiver8;
#[doc = "GICD_ICACTIVER0 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver0`]
module"]
pub type GICD_ICACTIVER0 = crate::Reg<gicd_icactiver0::GICD_ICACTIVER0rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver0;
#[doc = "GICD_ICACTIVER1 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver1`]
module"]
pub type GICD_ICACTIVER1 = crate::Reg<gicd_icactiver1::GICD_ICACTIVER1rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver1;
#[doc = "GICD_ICACTIVER2 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver2`]
module"]
pub type GICD_ICACTIVER2 = crate::Reg<gicd_icactiver2::GICD_ICACTIVER2rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver2;
#[doc = "GICD_ICACTIVER3 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver3`]
module"]
pub type GICD_ICACTIVER3 = crate::Reg<gicd_icactiver3::GICD_ICACTIVER3rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver3;
#[doc = "GICD_ICACTIVER4 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver4`]
module"]
pub type GICD_ICACTIVER4 = crate::Reg<gicd_icactiver4::GICD_ICACTIVER4rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver4;
#[doc = "GICD_ICACTIVER5 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver5`]
module"]
pub type GICD_ICACTIVER5 = crate::Reg<gicd_icactiver5::GICD_ICACTIVER5rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver5;
#[doc = "GICD_ICACTIVER6 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver6`]
module"]
pub type GICD_ICACTIVER6 = crate::Reg<gicd_icactiver6::GICD_ICACTIVER6rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver6;
#[doc = "GICD_ICACTIVER7 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver7`]
module"]
pub type GICD_ICACTIVER7 = crate::Reg<gicd_icactiver7::GICD_ICACTIVER7rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver7;
#[doc = "GICD_ICACTIVER8 (rw) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver8`]
module"]
pub type GICD_ICACTIVER8 = crate::Reg<gicd_icactiver8::GICD_ICACTIVER8rs>;
#[doc = "For interrupts ID"]
pub mod gicd_icactiver8;
#[doc = "GICD_IPRIORITYR0 (rw) register accessor: GICD interrupt priority register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr0`]
module"]
pub type GICD_IPRIORITYR0 = crate::Reg<gicd_ipriorityr0::GICD_IPRIORITYR0rs>;
#[doc = "GICD interrupt priority register 0"]
pub mod gicd_ipriorityr0;
#[doc = "GICD_IPRIORITYR1 (rw) register accessor: GICD interrupt priority register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr1`]
module"]
pub type GICD_IPRIORITYR1 = crate::Reg<gicd_ipriorityr1::GICD_IPRIORITYR1rs>;
#[doc = "GICD interrupt priority register 1"]
pub mod gicd_ipriorityr1;
#[doc = "GICD_IPRIORITYR2 (rw) register accessor: GICD interrupt priority register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr2`]
module"]
pub type GICD_IPRIORITYR2 = crate::Reg<gicd_ipriorityr2::GICD_IPRIORITYR2rs>;
#[doc = "GICD interrupt priority register 2"]
pub mod gicd_ipriorityr2;
#[doc = "GICD_IPRIORITYR3 (rw) register accessor: GICD interrupt priority register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr3`]
module"]
pub type GICD_IPRIORITYR3 = crate::Reg<gicd_ipriorityr3::GICD_IPRIORITYR3rs>;
#[doc = "GICD interrupt priority register 3"]
pub mod gicd_ipriorityr3;
#[doc = "GICD_IPRIORITYR4 (rw) register accessor: GICD interrupt priority register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr4`]
module"]
pub type GICD_IPRIORITYR4 = crate::Reg<gicd_ipriorityr4::GICD_IPRIORITYR4rs>;
#[doc = "GICD interrupt priority register 4"]
pub mod gicd_ipriorityr4;
#[doc = "GICD_IPRIORITYR5 (rw) register accessor: GICD interrupt priority register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr5`]
module"]
pub type GICD_IPRIORITYR5 = crate::Reg<gicd_ipriorityr5::GICD_IPRIORITYR5rs>;
#[doc = "GICD interrupt priority register 5"]
pub mod gicd_ipriorityr5;
#[doc = "GICD_IPRIORITYR6 (rw) register accessor: GICD interrupt priority register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr6`]
module"]
pub type GICD_IPRIORITYR6 = crate::Reg<gicd_ipriorityr6::GICD_IPRIORITYR6rs>;
#[doc = "GICD interrupt priority register 6"]
pub mod gicd_ipriorityr6;
#[doc = "GICD_IPRIORITYR7 (rw) register accessor: GICD interrupt priority register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr7`]
module"]
pub type GICD_IPRIORITYR7 = crate::Reg<gicd_ipriorityr7::GICD_IPRIORITYR7rs>;
#[doc = "GICD interrupt priority register 7"]
pub mod gicd_ipriorityr7;
#[doc = "GICD_IPRIORITYR8 (rw) register accessor: GICD interrupt priority register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr8`]
module"]
pub type GICD_IPRIORITYR8 = crate::Reg<gicd_ipriorityr8::GICD_IPRIORITYR8rs>;
#[doc = "GICD interrupt priority register 8"]
pub mod gicd_ipriorityr8;
#[doc = "GICD_IPRIORITYR9 (rw) register accessor: GICD interrupt priority register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr9`]
module"]
pub type GICD_IPRIORITYR9 = crate::Reg<gicd_ipriorityr9::GICD_IPRIORITYR9rs>;
#[doc = "GICD interrupt priority register 9"]
pub mod gicd_ipriorityr9;
#[doc = "GICD_IPRIORITYR10 (rw) register accessor: GICD interrupt priority register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr10`]
module"]
pub type GICD_IPRIORITYR10 = crate::Reg<gicd_ipriorityr10::GICD_IPRIORITYR10rs>;
#[doc = "GICD interrupt priority register 10"]
pub mod gicd_ipriorityr10;
#[doc = "GICD_IPRIORITYR11 (rw) register accessor: GICD interrupt priority register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr11`]
module"]
pub type GICD_IPRIORITYR11 = crate::Reg<gicd_ipriorityr11::GICD_IPRIORITYR11rs>;
#[doc = "GICD interrupt priority register 11"]
pub mod gicd_ipriorityr11;
#[doc = "GICD_IPRIORITYR12 (rw) register accessor: GICD interrupt priority register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr12`]
module"]
pub type GICD_IPRIORITYR12 = crate::Reg<gicd_ipriorityr12::GICD_IPRIORITYR12rs>;
#[doc = "GICD interrupt priority register 12"]
pub mod gicd_ipriorityr12;
#[doc = "GICD_IPRIORITYR13 (rw) register accessor: GICD interrupt priority register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr13`]
module"]
pub type GICD_IPRIORITYR13 = crate::Reg<gicd_ipriorityr13::GICD_IPRIORITYR13rs>;
#[doc = "GICD interrupt priority register 13"]
pub mod gicd_ipriorityr13;
#[doc = "GICD_IPRIORITYR14 (rw) register accessor: GICD interrupt priority register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr14`]
module"]
pub type GICD_IPRIORITYR14 = crate::Reg<gicd_ipriorityr14::GICD_IPRIORITYR14rs>;
#[doc = "GICD interrupt priority register 14"]
pub mod gicd_ipriorityr14;
#[doc = "GICD_IPRIORITYR15 (rw) register accessor: GICD interrupt priority register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr15`]
module"]
pub type GICD_IPRIORITYR15 = crate::Reg<gicd_ipriorityr15::GICD_IPRIORITYR15rs>;
#[doc = "GICD interrupt priority register 15"]
pub mod gicd_ipriorityr15;
#[doc = "GICD_IPRIORITYR16 (rw) register accessor: GICD interrupt priority register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr16`]
module"]
pub type GICD_IPRIORITYR16 = crate::Reg<gicd_ipriorityr16::GICD_IPRIORITYR16rs>;
#[doc = "GICD interrupt priority register 16"]
pub mod gicd_ipriorityr16;
#[doc = "GICD_IPRIORITYR17 (rw) register accessor: GICD interrupt priority register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr17`]
module"]
pub type GICD_IPRIORITYR17 = crate::Reg<gicd_ipriorityr17::GICD_IPRIORITYR17rs>;
#[doc = "GICD interrupt priority register 17"]
pub mod gicd_ipriorityr17;
#[doc = "GICD_IPRIORITYR18 (rw) register accessor: GICD interrupt priority register 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr18`]
module"]
pub type GICD_IPRIORITYR18 = crate::Reg<gicd_ipriorityr18::GICD_IPRIORITYR18rs>;
#[doc = "GICD interrupt priority register 18"]
pub mod gicd_ipriorityr18;
#[doc = "GICD_IPRIORITYR19 (rw) register accessor: GICD interrupt priority register 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr19`]
module"]
pub type GICD_IPRIORITYR19 = crate::Reg<gicd_ipriorityr19::GICD_IPRIORITYR19rs>;
#[doc = "GICD interrupt priority register 19"]
pub mod gicd_ipriorityr19;
#[doc = "GICD_IPRIORITYR20 (rw) register accessor: GICD interrupt priority register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr20`]
module"]
pub type GICD_IPRIORITYR20 = crate::Reg<gicd_ipriorityr20::GICD_IPRIORITYR20rs>;
#[doc = "GICD interrupt priority register 20"]
pub mod gicd_ipriorityr20;
#[doc = "GICD_IPRIORITYR21 (rw) register accessor: GICD interrupt priority register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr21`]
module"]
pub type GICD_IPRIORITYR21 = crate::Reg<gicd_ipriorityr21::GICD_IPRIORITYR21rs>;
#[doc = "GICD interrupt priority register 21"]
pub mod gicd_ipriorityr21;
#[doc = "GICD_IPRIORITYR22 (rw) register accessor: GICD interrupt priority register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr22`]
module"]
pub type GICD_IPRIORITYR22 = crate::Reg<gicd_ipriorityr22::GICD_IPRIORITYR22rs>;
#[doc = "GICD interrupt priority register 22"]
pub mod gicd_ipriorityr22;
#[doc = "GICD_IPRIORITYR23 (rw) register accessor: GICD interrupt priority register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr23`]
module"]
pub type GICD_IPRIORITYR23 = crate::Reg<gicd_ipriorityr23::GICD_IPRIORITYR23rs>;
#[doc = "GICD interrupt priority register 23"]
pub mod gicd_ipriorityr23;
#[doc = "GICD_IPRIORITYR24 (rw) register accessor: GICD interrupt priority register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr24`]
module"]
pub type GICD_IPRIORITYR24 = crate::Reg<gicd_ipriorityr24::GICD_IPRIORITYR24rs>;
#[doc = "GICD interrupt priority register 24"]
pub mod gicd_ipriorityr24;
#[doc = "GICD_IPRIORITYR25 (rw) register accessor: GICD interrupt priority register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr25`]
module"]
pub type GICD_IPRIORITYR25 = crate::Reg<gicd_ipriorityr25::GICD_IPRIORITYR25rs>;
#[doc = "GICD interrupt priority register 25"]
pub mod gicd_ipriorityr25;
#[doc = "GICD_IPRIORITYR26 (rw) register accessor: GICD interrupt priority register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr26`]
module"]
pub type GICD_IPRIORITYR26 = crate::Reg<gicd_ipriorityr26::GICD_IPRIORITYR26rs>;
#[doc = "GICD interrupt priority register 26"]
pub mod gicd_ipriorityr26;
#[doc = "GICD_IPRIORITYR27 (rw) register accessor: GICD interrupt priority register 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr27`]
module"]
pub type GICD_IPRIORITYR27 = crate::Reg<gicd_ipriorityr27::GICD_IPRIORITYR27rs>;
#[doc = "GICD interrupt priority register 27"]
pub mod gicd_ipriorityr27;
#[doc = "GICD_IPRIORITYR28 (rw) register accessor: GICD interrupt priority register 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr28`]
module"]
pub type GICD_IPRIORITYR28 = crate::Reg<gicd_ipriorityr28::GICD_IPRIORITYR28rs>;
#[doc = "GICD interrupt priority register 28"]
pub mod gicd_ipriorityr28;
#[doc = "GICD_IPRIORITYR29 (rw) register accessor: GICD interrupt priority register 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr29`]
module"]
pub type GICD_IPRIORITYR29 = crate::Reg<gicd_ipriorityr29::GICD_IPRIORITYR29rs>;
#[doc = "GICD interrupt priority register 29"]
pub mod gicd_ipriorityr29;
#[doc = "GICD_IPRIORITYR30 (rw) register accessor: GICD interrupt priority register 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr30`]
module"]
pub type GICD_IPRIORITYR30 = crate::Reg<gicd_ipriorityr30::GICD_IPRIORITYR30rs>;
#[doc = "GICD interrupt priority register 30"]
pub mod gicd_ipriorityr30;
#[doc = "GICD_IPRIORITYR31 (rw) register accessor: GICD interrupt priority register 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr31`]
module"]
pub type GICD_IPRIORITYR31 = crate::Reg<gicd_ipriorityr31::GICD_IPRIORITYR31rs>;
#[doc = "GICD interrupt priority register 31"]
pub mod gicd_ipriorityr31;
#[doc = "GICD_IPRIORITYR32 (rw) register accessor: GICD interrupt priority register 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr32`]
module"]
pub type GICD_IPRIORITYR32 = crate::Reg<gicd_ipriorityr32::GICD_IPRIORITYR32rs>;
#[doc = "GICD interrupt priority register 32"]
pub mod gicd_ipriorityr32;
#[doc = "GICD_IPRIORITYR33 (rw) register accessor: GICD interrupt priority register 33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr33`]
module"]
pub type GICD_IPRIORITYR33 = crate::Reg<gicd_ipriorityr33::GICD_IPRIORITYR33rs>;
#[doc = "GICD interrupt priority register 33"]
pub mod gicd_ipriorityr33;
#[doc = "GICD_IPRIORITYR34 (rw) register accessor: GICD interrupt priority register 34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr34`]
module"]
pub type GICD_IPRIORITYR34 = crate::Reg<gicd_ipriorityr34::GICD_IPRIORITYR34rs>;
#[doc = "GICD interrupt priority register 34"]
pub mod gicd_ipriorityr34;
#[doc = "GICD_IPRIORITYR35 (rw) register accessor: GICD interrupt priority register 35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr35`]
module"]
pub type GICD_IPRIORITYR35 = crate::Reg<gicd_ipriorityr35::GICD_IPRIORITYR35rs>;
#[doc = "GICD interrupt priority register 35"]
pub mod gicd_ipriorityr35;
#[doc = "GICD_IPRIORITYR36 (rw) register accessor: GICD interrupt priority register 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr36`]
module"]
pub type GICD_IPRIORITYR36 = crate::Reg<gicd_ipriorityr36::GICD_IPRIORITYR36rs>;
#[doc = "GICD interrupt priority register 36"]
pub mod gicd_ipriorityr36;
#[doc = "GICD_IPRIORITYR37 (rw) register accessor: GICD interrupt priority register 37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr37`]
module"]
pub type GICD_IPRIORITYR37 = crate::Reg<gicd_ipriorityr37::GICD_IPRIORITYR37rs>;
#[doc = "GICD interrupt priority register 37"]
pub mod gicd_ipriorityr37;
#[doc = "GICD_IPRIORITYR38 (rw) register accessor: GICD interrupt priority register 38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr38`]
module"]
pub type GICD_IPRIORITYR38 = crate::Reg<gicd_ipriorityr38::GICD_IPRIORITYR38rs>;
#[doc = "GICD interrupt priority register 38"]
pub mod gicd_ipriorityr38;
#[doc = "GICD_IPRIORITYR39 (rw) register accessor: GICD interrupt priority register 39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr39`]
module"]
pub type GICD_IPRIORITYR39 = crate::Reg<gicd_ipriorityr39::GICD_IPRIORITYR39rs>;
#[doc = "GICD interrupt priority register 39"]
pub mod gicd_ipriorityr39;
#[doc = "GICD_IPRIORITYR40 (rw) register accessor: GICD interrupt priority register 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr40`]
module"]
pub type GICD_IPRIORITYR40 = crate::Reg<gicd_ipriorityr40::GICD_IPRIORITYR40rs>;
#[doc = "GICD interrupt priority register 40"]
pub mod gicd_ipriorityr40;
#[doc = "GICD_IPRIORITYR41 (rw) register accessor: GICD interrupt priority register 41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr41`]
module"]
pub type GICD_IPRIORITYR41 = crate::Reg<gicd_ipriorityr41::GICD_IPRIORITYR41rs>;
#[doc = "GICD interrupt priority register 41"]
pub mod gicd_ipriorityr41;
#[doc = "GICD_IPRIORITYR42 (rw) register accessor: GICD interrupt priority register 42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr42`]
module"]
pub type GICD_IPRIORITYR42 = crate::Reg<gicd_ipriorityr42::GICD_IPRIORITYR42rs>;
#[doc = "GICD interrupt priority register 42"]
pub mod gicd_ipriorityr42;
#[doc = "GICD_IPRIORITYR43 (rw) register accessor: GICD interrupt priority register 43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr43`]
module"]
pub type GICD_IPRIORITYR43 = crate::Reg<gicd_ipriorityr43::GICD_IPRIORITYR43rs>;
#[doc = "GICD interrupt priority register 43"]
pub mod gicd_ipriorityr43;
#[doc = "GICD_IPRIORITYR44 (rw) register accessor: GICD interrupt priority register 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr44`]
module"]
pub type GICD_IPRIORITYR44 = crate::Reg<gicd_ipriorityr44::GICD_IPRIORITYR44rs>;
#[doc = "GICD interrupt priority register 44"]
pub mod gicd_ipriorityr44;
#[doc = "GICD_IPRIORITYR45 (rw) register accessor: GICD interrupt priority register 45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr45`]
module"]
pub type GICD_IPRIORITYR45 = crate::Reg<gicd_ipriorityr45::GICD_IPRIORITYR45rs>;
#[doc = "GICD interrupt priority register 45"]
pub mod gicd_ipriorityr45;
#[doc = "GICD_IPRIORITYR46 (rw) register accessor: GICD interrupt priority register 46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr46`]
module"]
pub type GICD_IPRIORITYR46 = crate::Reg<gicd_ipriorityr46::GICD_IPRIORITYR46rs>;
#[doc = "GICD interrupt priority register 46"]
pub mod gicd_ipriorityr46;
#[doc = "GICD_IPRIORITYR47 (rw) register accessor: GICD interrupt priority register 47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr47`]
module"]
pub type GICD_IPRIORITYR47 = crate::Reg<gicd_ipriorityr47::GICD_IPRIORITYR47rs>;
#[doc = "GICD interrupt priority register 47"]
pub mod gicd_ipriorityr47;
#[doc = "GICD_IPRIORITYR48 (rw) register accessor: GICD interrupt priority register 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr48`]
module"]
pub type GICD_IPRIORITYR48 = crate::Reg<gicd_ipriorityr48::GICD_IPRIORITYR48rs>;
#[doc = "GICD interrupt priority register 48"]
pub mod gicd_ipriorityr48;
#[doc = "GICD_IPRIORITYR49 (rw) register accessor: GICD interrupt priority register 49\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr49`]
module"]
pub type GICD_IPRIORITYR49 = crate::Reg<gicd_ipriorityr49::GICD_IPRIORITYR49rs>;
#[doc = "GICD interrupt priority register 49"]
pub mod gicd_ipriorityr49;
#[doc = "GICD_IPRIORITYR50 (rw) register accessor: GICD interrupt priority register 50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr50`]
module"]
pub type GICD_IPRIORITYR50 = crate::Reg<gicd_ipriorityr50::GICD_IPRIORITYR50rs>;
#[doc = "GICD interrupt priority register 50"]
pub mod gicd_ipriorityr50;
#[doc = "GICD_IPRIORITYR51 (rw) register accessor: GICD interrupt priority register 51\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr51`]
module"]
pub type GICD_IPRIORITYR51 = crate::Reg<gicd_ipriorityr51::GICD_IPRIORITYR51rs>;
#[doc = "GICD interrupt priority register 51"]
pub mod gicd_ipriorityr51;
#[doc = "GICD_IPRIORITYR52 (rw) register accessor: GICD interrupt priority register 52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr52`]
module"]
pub type GICD_IPRIORITYR52 = crate::Reg<gicd_ipriorityr52::GICD_IPRIORITYR52rs>;
#[doc = "GICD interrupt priority register 52"]
pub mod gicd_ipriorityr52;
#[doc = "GICD_IPRIORITYR53 (rw) register accessor: GICD interrupt priority register 53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr53`]
module"]
pub type GICD_IPRIORITYR53 = crate::Reg<gicd_ipriorityr53::GICD_IPRIORITYR53rs>;
#[doc = "GICD interrupt priority register 53"]
pub mod gicd_ipriorityr53;
#[doc = "GICD_IPRIORITYR54 (rw) register accessor: GICD interrupt priority register 54\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr54`]
module"]
pub type GICD_IPRIORITYR54 = crate::Reg<gicd_ipriorityr54::GICD_IPRIORITYR54rs>;
#[doc = "GICD interrupt priority register 54"]
pub mod gicd_ipriorityr54;
#[doc = "GICD_IPRIORITYR55 (rw) register accessor: GICD interrupt priority register 55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr55`]
module"]
pub type GICD_IPRIORITYR55 = crate::Reg<gicd_ipriorityr55::GICD_IPRIORITYR55rs>;
#[doc = "GICD interrupt priority register 55"]
pub mod gicd_ipriorityr55;
#[doc = "GICD_IPRIORITYR56 (rw) register accessor: GICD interrupt priority register 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr56`]
module"]
pub type GICD_IPRIORITYR56 = crate::Reg<gicd_ipriorityr56::GICD_IPRIORITYR56rs>;
#[doc = "GICD interrupt priority register 56"]
pub mod gicd_ipriorityr56;
#[doc = "GICD_IPRIORITYR57 (rw) register accessor: GICD interrupt priority register 57\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr57::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr57`]
module"]
pub type GICD_IPRIORITYR57 = crate::Reg<gicd_ipriorityr57::GICD_IPRIORITYR57rs>;
#[doc = "GICD interrupt priority register 57"]
pub mod gicd_ipriorityr57;
#[doc = "GICD_IPRIORITYR58 (rw) register accessor: GICD interrupt priority register 58\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr58::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr58`]
module"]
pub type GICD_IPRIORITYR58 = crate::Reg<gicd_ipriorityr58::GICD_IPRIORITYR58rs>;
#[doc = "GICD interrupt priority register 58"]
pub mod gicd_ipriorityr58;
#[doc = "GICD_IPRIORITYR59 (rw) register accessor: GICD interrupt priority register 59\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr59`]
module"]
pub type GICD_IPRIORITYR59 = crate::Reg<gicd_ipriorityr59::GICD_IPRIORITYR59rs>;
#[doc = "GICD interrupt priority register 59"]
pub mod gicd_ipriorityr59;
#[doc = "GICD_IPRIORITYR60 (rw) register accessor: GICD interrupt priority register 60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr60::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr60`]
module"]
pub type GICD_IPRIORITYR60 = crate::Reg<gicd_ipriorityr60::GICD_IPRIORITYR60rs>;
#[doc = "GICD interrupt priority register 60"]
pub mod gicd_ipriorityr60;
#[doc = "GICD_IPRIORITYR61 (rw) register accessor: GICD interrupt priority register 61\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr61::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr61`]
module"]
pub type GICD_IPRIORITYR61 = crate::Reg<gicd_ipriorityr61::GICD_IPRIORITYR61rs>;
#[doc = "GICD interrupt priority register 61"]
pub mod gicd_ipriorityr61;
#[doc = "GICD_IPRIORITYR62 (rw) register accessor: GICD interrupt priority register 62\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr62::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr62`]
module"]
pub type GICD_IPRIORITYR62 = crate::Reg<gicd_ipriorityr62::GICD_IPRIORITYR62rs>;
#[doc = "GICD interrupt priority register 62"]
pub mod gicd_ipriorityr62;
#[doc = "GICD_IPRIORITYR63 (rw) register accessor: GICD interrupt priority register 63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr63`]
module"]
pub type GICD_IPRIORITYR63 = crate::Reg<gicd_ipriorityr63::GICD_IPRIORITYR63rs>;
#[doc = "GICD interrupt priority register 63"]
pub mod gicd_ipriorityr63;
#[doc = "GICD_IPRIORITYR64 (rw) register accessor: GICD interrupt priority register 64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr64`]
module"]
pub type GICD_IPRIORITYR64 = crate::Reg<gicd_ipriorityr64::GICD_IPRIORITYR64rs>;
#[doc = "GICD interrupt priority register 64"]
pub mod gicd_ipriorityr64;
#[doc = "GICD_IPRIORITYR65 (rw) register accessor: GICD interrupt priority register 65\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr65::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr65::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr65`]
module"]
pub type GICD_IPRIORITYR65 = crate::Reg<gicd_ipriorityr65::GICD_IPRIORITYR65rs>;
#[doc = "GICD interrupt priority register 65"]
pub mod gicd_ipriorityr65;
#[doc = "GICD_IPRIORITYR66 (rw) register accessor: GICD interrupt priority register 66\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr66::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr66::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr66`]
module"]
pub type GICD_IPRIORITYR66 = crate::Reg<gicd_ipriorityr66::GICD_IPRIORITYR66rs>;
#[doc = "GICD interrupt priority register 66"]
pub mod gicd_ipriorityr66;
#[doc = "GICD_IPRIORITYR67 (rw) register accessor: GICD interrupt priority register 67\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr67`]
module"]
pub type GICD_IPRIORITYR67 = crate::Reg<gicd_ipriorityr67::GICD_IPRIORITYR67rs>;
#[doc = "GICD interrupt priority register 67"]
pub mod gicd_ipriorityr67;
#[doc = "GICD_IPRIORITYR68 (rw) register accessor: GICD interrupt priority register 68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr68::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr68::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr68`]
module"]
pub type GICD_IPRIORITYR68 = crate::Reg<gicd_ipriorityr68::GICD_IPRIORITYR68rs>;
#[doc = "GICD interrupt priority register 68"]
pub mod gicd_ipriorityr68;
#[doc = "GICD_IPRIORITYR69 (rw) register accessor: GICD interrupt priority register 69\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr69::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr69::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr69`]
module"]
pub type GICD_IPRIORITYR69 = crate::Reg<gicd_ipriorityr69::GICD_IPRIORITYR69rs>;
#[doc = "GICD interrupt priority register 69"]
pub mod gicd_ipriorityr69;
#[doc = "GICD_IPRIORITYR70 (rw) register accessor: GICD interrupt priority register 70\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr70::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr70::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr70`]
module"]
pub type GICD_IPRIORITYR70 = crate::Reg<gicd_ipriorityr70::GICD_IPRIORITYR70rs>;
#[doc = "GICD interrupt priority register 70"]
pub mod gicd_ipriorityr70;
#[doc = "GICD_IPRIORITYR71 (rw) register accessor: GICD interrupt priority register 71\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr71::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr71::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ipriorityr71`]
module"]
pub type GICD_IPRIORITYR71 = crate::Reg<gicd_ipriorityr71::GICD_IPRIORITYR71rs>;
#[doc = "GICD interrupt priority register 71"]
pub mod gicd_ipriorityr71;
#[doc = "GICD_ITARGETSR0 (r) register accessor: For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr0`]
module"]
pub type GICD_ITARGETSR0 = crate::Reg<gicd_itargetsr0::GICD_ITARGETSR0rs>;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr0;
#[doc = "GICD_ITARGETSR1 (r) register accessor: For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr1`]
module"]
pub type GICD_ITARGETSR1 = crate::Reg<gicd_itargetsr1::GICD_ITARGETSR1rs>;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr1;
#[doc = "GICD_ITARGETSR2 (r) register accessor: For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr2`]
module"]
pub type GICD_ITARGETSR2 = crate::Reg<gicd_itargetsr2::GICD_ITARGETSR2rs>;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr2;
#[doc = "GICD_ITARGETSR3 (r) register accessor: For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr3`]
module"]
pub type GICD_ITARGETSR3 = crate::Reg<gicd_itargetsr3::GICD_ITARGETSR3rs>;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr3;
#[doc = "GICD_ITARGETSR4 (r) register accessor: For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr4`]
module"]
pub type GICD_ITARGETSR4 = crate::Reg<gicd_itargetsr4::GICD_ITARGETSR4rs>;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr4;
#[doc = "GICD_ITARGETSR5 (r) register accessor: For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr5`]
module"]
pub type GICD_ITARGETSR5 = crate::Reg<gicd_itargetsr5::GICD_ITARGETSR5rs>;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr5;
#[doc = "GICD_ITARGETSR6 (r) register accessor: For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr6`]
module"]
pub type GICD_ITARGETSR6 = crate::Reg<gicd_itargetsr6::GICD_ITARGETSR6rs>;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr6;
#[doc = "GICD_ITARGETSR7 (r) register accessor: For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr7`]
module"]
pub type GICD_ITARGETSR7 = crate::Reg<gicd_itargetsr7::GICD_ITARGETSR7rs>;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr7;
#[doc = "GICD_ITARGETSR8 (rw) register accessor: GICD interrupt processor target register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr8`]
module"]
pub type GICD_ITARGETSR8 = crate::Reg<gicd_itargetsr8::GICD_ITARGETSR8rs>;
#[doc = "GICD interrupt processor target register 8"]
pub mod gicd_itargetsr8;
#[doc = "GICD_ITARGETSR9 (rw) register accessor: GICD interrupt processor target register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr9`]
module"]
pub type GICD_ITARGETSR9 = crate::Reg<gicd_itargetsr9::GICD_ITARGETSR9rs>;
#[doc = "GICD interrupt processor target register 9"]
pub mod gicd_itargetsr9;
#[doc = "GICD_ITARGETSR10 (rw) register accessor: GICD interrupt processor target register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr10`]
module"]
pub type GICD_ITARGETSR10 = crate::Reg<gicd_itargetsr10::GICD_ITARGETSR10rs>;
#[doc = "GICD interrupt processor target register 10"]
pub mod gicd_itargetsr10;
#[doc = "GICD_ITARGETSR11 (rw) register accessor: GICD interrupt processor target register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr11`]
module"]
pub type GICD_ITARGETSR11 = crate::Reg<gicd_itargetsr11::GICD_ITARGETSR11rs>;
#[doc = "GICD interrupt processor target register 11"]
pub mod gicd_itargetsr11;
#[doc = "GICD_ITARGETSR12 (rw) register accessor: GICD interrupt processor target register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr12`]
module"]
pub type GICD_ITARGETSR12 = crate::Reg<gicd_itargetsr12::GICD_ITARGETSR12rs>;
#[doc = "GICD interrupt processor target register 12"]
pub mod gicd_itargetsr12;
#[doc = "GICD_ITARGETSR13 (rw) register accessor: GICD interrupt processor target register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr13`]
module"]
pub type GICD_ITARGETSR13 = crate::Reg<gicd_itargetsr13::GICD_ITARGETSR13rs>;
#[doc = "GICD interrupt processor target register 13"]
pub mod gicd_itargetsr13;
#[doc = "GICD_ITARGETSR14 (rw) register accessor: GICD interrupt processor target register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr14`]
module"]
pub type GICD_ITARGETSR14 = crate::Reg<gicd_itargetsr14::GICD_ITARGETSR14rs>;
#[doc = "GICD interrupt processor target register 14"]
pub mod gicd_itargetsr14;
#[doc = "GICD_ITARGETSR15 (rw) register accessor: GICD interrupt processor target register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr15`]
module"]
pub type GICD_ITARGETSR15 = crate::Reg<gicd_itargetsr15::GICD_ITARGETSR15rs>;
#[doc = "GICD interrupt processor target register 15"]
pub mod gicd_itargetsr15;
#[doc = "GICD_ITARGETSR16 (rw) register accessor: GICD interrupt processor target register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr16`]
module"]
pub type GICD_ITARGETSR16 = crate::Reg<gicd_itargetsr16::GICD_ITARGETSR16rs>;
#[doc = "GICD interrupt processor target register 16"]
pub mod gicd_itargetsr16;
#[doc = "GICD_ITARGETSR17 (rw) register accessor: GICD interrupt processor target register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr17`]
module"]
pub type GICD_ITARGETSR17 = crate::Reg<gicd_itargetsr17::GICD_ITARGETSR17rs>;
#[doc = "GICD interrupt processor target register 17"]
pub mod gicd_itargetsr17;
#[doc = "GICD_ITARGETSR18 (rw) register accessor: GICD interrupt processor target register 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr18`]
module"]
pub type GICD_ITARGETSR18 = crate::Reg<gicd_itargetsr18::GICD_ITARGETSR18rs>;
#[doc = "GICD interrupt processor target register 18"]
pub mod gicd_itargetsr18;
#[doc = "GICD_ITARGETSR19 (rw) register accessor: GICD interrupt processor target register 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr19`]
module"]
pub type GICD_ITARGETSR19 = crate::Reg<gicd_itargetsr19::GICD_ITARGETSR19rs>;
#[doc = "GICD interrupt processor target register 19"]
pub mod gicd_itargetsr19;
#[doc = "GICD_ITARGETSR20 (rw) register accessor: GICD interrupt processor target register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr20`]
module"]
pub type GICD_ITARGETSR20 = crate::Reg<gicd_itargetsr20::GICD_ITARGETSR20rs>;
#[doc = "GICD interrupt processor target register 20"]
pub mod gicd_itargetsr20;
#[doc = "GICD_ITARGETSR21 (rw) register accessor: GICD interrupt processor target register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr21`]
module"]
pub type GICD_ITARGETSR21 = crate::Reg<gicd_itargetsr21::GICD_ITARGETSR21rs>;
#[doc = "GICD interrupt processor target register 21"]
pub mod gicd_itargetsr21;
#[doc = "GICD_ITARGETSR22 (rw) register accessor: GICD interrupt processor target register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr22`]
module"]
pub type GICD_ITARGETSR22 = crate::Reg<gicd_itargetsr22::GICD_ITARGETSR22rs>;
#[doc = "GICD interrupt processor target register 22"]
pub mod gicd_itargetsr22;
#[doc = "GICD_ITARGETSR23 (rw) register accessor: GICD interrupt processor target register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr23`]
module"]
pub type GICD_ITARGETSR23 = crate::Reg<gicd_itargetsr23::GICD_ITARGETSR23rs>;
#[doc = "GICD interrupt processor target register 23"]
pub mod gicd_itargetsr23;
#[doc = "GICD_ITARGETSR24 (rw) register accessor: GICD interrupt processor target register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr24`]
module"]
pub type GICD_ITARGETSR24 = crate::Reg<gicd_itargetsr24::GICD_ITARGETSR24rs>;
#[doc = "GICD interrupt processor target register 24"]
pub mod gicd_itargetsr24;
#[doc = "GICD_ITARGETSR25 (rw) register accessor: GICD interrupt processor target register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr25`]
module"]
pub type GICD_ITARGETSR25 = crate::Reg<gicd_itargetsr25::GICD_ITARGETSR25rs>;
#[doc = "GICD interrupt processor target register 25"]
pub mod gicd_itargetsr25;
#[doc = "GICD_ITARGETSR26 (rw) register accessor: GICD interrupt processor target register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr26`]
module"]
pub type GICD_ITARGETSR26 = crate::Reg<gicd_itargetsr26::GICD_ITARGETSR26rs>;
#[doc = "GICD interrupt processor target register 26"]
pub mod gicd_itargetsr26;
#[doc = "GICD_ITARGETSR27 (rw) register accessor: GICD interrupt processor target register 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr27`]
module"]
pub type GICD_ITARGETSR27 = crate::Reg<gicd_itargetsr27::GICD_ITARGETSR27rs>;
#[doc = "GICD interrupt processor target register 27"]
pub mod gicd_itargetsr27;
#[doc = "GICD_ITARGETSR28 (rw) register accessor: GICD interrupt processor target register 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr28`]
module"]
pub type GICD_ITARGETSR28 = crate::Reg<gicd_itargetsr28::GICD_ITARGETSR28rs>;
#[doc = "GICD interrupt processor target register 28"]
pub mod gicd_itargetsr28;
#[doc = "GICD_ITARGETSR29 (rw) register accessor: GICD interrupt processor target register 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr29`]
module"]
pub type GICD_ITARGETSR29 = crate::Reg<gicd_itargetsr29::GICD_ITARGETSR29rs>;
#[doc = "GICD interrupt processor target register 29"]
pub mod gicd_itargetsr29;
#[doc = "GICD_ITARGETSR30 (rw) register accessor: GICD interrupt processor target register 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr30`]
module"]
pub type GICD_ITARGETSR30 = crate::Reg<gicd_itargetsr30::GICD_ITARGETSR30rs>;
#[doc = "GICD interrupt processor target register 30"]
pub mod gicd_itargetsr30;
#[doc = "GICD_ITARGETSR31 (rw) register accessor: GICD interrupt processor target register 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr31`]
module"]
pub type GICD_ITARGETSR31 = crate::Reg<gicd_itargetsr31::GICD_ITARGETSR31rs>;
#[doc = "GICD interrupt processor target register 31"]
pub mod gicd_itargetsr31;
#[doc = "GICD_ITARGETSR32 (rw) register accessor: GICD interrupt processor target register 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr32`]
module"]
pub type GICD_ITARGETSR32 = crate::Reg<gicd_itargetsr32::GICD_ITARGETSR32rs>;
#[doc = "GICD interrupt processor target register 32"]
pub mod gicd_itargetsr32;
#[doc = "GICD_ITARGETSR33 (rw) register accessor: GICD interrupt processor target register 33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr33`]
module"]
pub type GICD_ITARGETSR33 = crate::Reg<gicd_itargetsr33::GICD_ITARGETSR33rs>;
#[doc = "GICD interrupt processor target register 33"]
pub mod gicd_itargetsr33;
#[doc = "GICD_ITARGETSR34 (rw) register accessor: GICD interrupt processor target register 34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr34`]
module"]
pub type GICD_ITARGETSR34 = crate::Reg<gicd_itargetsr34::GICD_ITARGETSR34rs>;
#[doc = "GICD interrupt processor target register 34"]
pub mod gicd_itargetsr34;
#[doc = "GICD_ITARGETSR35 (rw) register accessor: GICD interrupt processor target register 35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr35`]
module"]
pub type GICD_ITARGETSR35 = crate::Reg<gicd_itargetsr35::GICD_ITARGETSR35rs>;
#[doc = "GICD interrupt processor target register 35"]
pub mod gicd_itargetsr35;
#[doc = "GICD_ITARGETSR36 (rw) register accessor: GICD interrupt processor target register 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr36`]
module"]
pub type GICD_ITARGETSR36 = crate::Reg<gicd_itargetsr36::GICD_ITARGETSR36rs>;
#[doc = "GICD interrupt processor target register 36"]
pub mod gicd_itargetsr36;
#[doc = "GICD_ITARGETSR37 (rw) register accessor: GICD interrupt processor target register 37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr37`]
module"]
pub type GICD_ITARGETSR37 = crate::Reg<gicd_itargetsr37::GICD_ITARGETSR37rs>;
#[doc = "GICD interrupt processor target register 37"]
pub mod gicd_itargetsr37;
#[doc = "GICD_ITARGETSR38 (rw) register accessor: GICD interrupt processor target register 38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr38`]
module"]
pub type GICD_ITARGETSR38 = crate::Reg<gicd_itargetsr38::GICD_ITARGETSR38rs>;
#[doc = "GICD interrupt processor target register 38"]
pub mod gicd_itargetsr38;
#[doc = "GICD_ITARGETSR39 (rw) register accessor: GICD interrupt processor target register 39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr39`]
module"]
pub type GICD_ITARGETSR39 = crate::Reg<gicd_itargetsr39::GICD_ITARGETSR39rs>;
#[doc = "GICD interrupt processor target register 39"]
pub mod gicd_itargetsr39;
#[doc = "GICD_ITARGETSR40 (rw) register accessor: GICD interrupt processor target register 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr40`]
module"]
pub type GICD_ITARGETSR40 = crate::Reg<gicd_itargetsr40::GICD_ITARGETSR40rs>;
#[doc = "GICD interrupt processor target register 40"]
pub mod gicd_itargetsr40;
#[doc = "GICD_ITARGETSR41 (rw) register accessor: GICD interrupt processor target register 41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr41`]
module"]
pub type GICD_ITARGETSR41 = crate::Reg<gicd_itargetsr41::GICD_ITARGETSR41rs>;
#[doc = "GICD interrupt processor target register 41"]
pub mod gicd_itargetsr41;
#[doc = "GICD_ITARGETSR42 (rw) register accessor: GICD interrupt processor target register 42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr42`]
module"]
pub type GICD_ITARGETSR42 = crate::Reg<gicd_itargetsr42::GICD_ITARGETSR42rs>;
#[doc = "GICD interrupt processor target register 42"]
pub mod gicd_itargetsr42;
#[doc = "GICD_ITARGETSR43 (rw) register accessor: GICD interrupt processor target register 43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr43`]
module"]
pub type GICD_ITARGETSR43 = crate::Reg<gicd_itargetsr43::GICD_ITARGETSR43rs>;
#[doc = "GICD interrupt processor target register 43"]
pub mod gicd_itargetsr43;
#[doc = "GICD_ITARGETSR44 (rw) register accessor: GICD interrupt processor target register 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr44`]
module"]
pub type GICD_ITARGETSR44 = crate::Reg<gicd_itargetsr44::GICD_ITARGETSR44rs>;
#[doc = "GICD interrupt processor target register 44"]
pub mod gicd_itargetsr44;
#[doc = "GICD_ITARGETSR45 (rw) register accessor: GICD interrupt processor target register 45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr45`]
module"]
pub type GICD_ITARGETSR45 = crate::Reg<gicd_itargetsr45::GICD_ITARGETSR45rs>;
#[doc = "GICD interrupt processor target register 45"]
pub mod gicd_itargetsr45;
#[doc = "GICD_ITARGETSR46 (rw) register accessor: GICD interrupt processor target register 46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr46`]
module"]
pub type GICD_ITARGETSR46 = crate::Reg<gicd_itargetsr46::GICD_ITARGETSR46rs>;
#[doc = "GICD interrupt processor target register 46"]
pub mod gicd_itargetsr46;
#[doc = "GICD_ITARGETSR47 (rw) register accessor: GICD interrupt processor target register 47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr47`]
module"]
pub type GICD_ITARGETSR47 = crate::Reg<gicd_itargetsr47::GICD_ITARGETSR47rs>;
#[doc = "GICD interrupt processor target register 47"]
pub mod gicd_itargetsr47;
#[doc = "GICD_ITARGETSR48 (rw) register accessor: GICD interrupt processor target register 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr48`]
module"]
pub type GICD_ITARGETSR48 = crate::Reg<gicd_itargetsr48::GICD_ITARGETSR48rs>;
#[doc = "GICD interrupt processor target register 48"]
pub mod gicd_itargetsr48;
#[doc = "GICD_ITARGETSR49 (rw) register accessor: GICD interrupt processor target register 49\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr49`]
module"]
pub type GICD_ITARGETSR49 = crate::Reg<gicd_itargetsr49::GICD_ITARGETSR49rs>;
#[doc = "GICD interrupt processor target register 49"]
pub mod gicd_itargetsr49;
#[doc = "GICD_ITARGETSR50 (rw) register accessor: GICD interrupt processor target register 50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr50`]
module"]
pub type GICD_ITARGETSR50 = crate::Reg<gicd_itargetsr50::GICD_ITARGETSR50rs>;
#[doc = "GICD interrupt processor target register 50"]
pub mod gicd_itargetsr50;
#[doc = "GICD_ITARGETSR51 (rw) register accessor: GICD interrupt processor target register 51\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr51`]
module"]
pub type GICD_ITARGETSR51 = crate::Reg<gicd_itargetsr51::GICD_ITARGETSR51rs>;
#[doc = "GICD interrupt processor target register 51"]
pub mod gicd_itargetsr51;
#[doc = "GICD_ITARGETSR52 (rw) register accessor: GICD interrupt processor target register 52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr52`]
module"]
pub type GICD_ITARGETSR52 = crate::Reg<gicd_itargetsr52::GICD_ITARGETSR52rs>;
#[doc = "GICD interrupt processor target register 52"]
pub mod gicd_itargetsr52;
#[doc = "GICD_ITARGETSR53 (rw) register accessor: GICD interrupt processor target register 53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr53`]
module"]
pub type GICD_ITARGETSR53 = crate::Reg<gicd_itargetsr53::GICD_ITARGETSR53rs>;
#[doc = "GICD interrupt processor target register 53"]
pub mod gicd_itargetsr53;
#[doc = "GICD_ITARGETSR54 (rw) register accessor: GICD interrupt processor target register 54\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr54`]
module"]
pub type GICD_ITARGETSR54 = crate::Reg<gicd_itargetsr54::GICD_ITARGETSR54rs>;
#[doc = "GICD interrupt processor target register 54"]
pub mod gicd_itargetsr54;
#[doc = "GICD_ITARGETSR55 (rw) register accessor: GICD interrupt processor target register 55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr55`]
module"]
pub type GICD_ITARGETSR55 = crate::Reg<gicd_itargetsr55::GICD_ITARGETSR55rs>;
#[doc = "GICD interrupt processor target register 55"]
pub mod gicd_itargetsr55;
#[doc = "GICD_ITARGETSR56 (rw) register accessor: GICD interrupt processor target register 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr56`]
module"]
pub type GICD_ITARGETSR56 = crate::Reg<gicd_itargetsr56::GICD_ITARGETSR56rs>;
#[doc = "GICD interrupt processor target register 56"]
pub mod gicd_itargetsr56;
#[doc = "GICD_ITARGETSR57 (rw) register accessor: GICD interrupt processor target register 57\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr57::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr57`]
module"]
pub type GICD_ITARGETSR57 = crate::Reg<gicd_itargetsr57::GICD_ITARGETSR57rs>;
#[doc = "GICD interrupt processor target register 57"]
pub mod gicd_itargetsr57;
#[doc = "GICD_ITARGETSR58 (rw) register accessor: GICD interrupt processor target register 58\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr58::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr58`]
module"]
pub type GICD_ITARGETSR58 = crate::Reg<gicd_itargetsr58::GICD_ITARGETSR58rs>;
#[doc = "GICD interrupt processor target register 58"]
pub mod gicd_itargetsr58;
#[doc = "GICD_ITARGETSR59 (rw) register accessor: GICD interrupt processor target register 59\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr59`]
module"]
pub type GICD_ITARGETSR59 = crate::Reg<gicd_itargetsr59::GICD_ITARGETSR59rs>;
#[doc = "GICD interrupt processor target register 59"]
pub mod gicd_itargetsr59;
#[doc = "GICD_ITARGETSR60 (rw) register accessor: GICD interrupt processor target register 60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr60::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr60`]
module"]
pub type GICD_ITARGETSR60 = crate::Reg<gicd_itargetsr60::GICD_ITARGETSR60rs>;
#[doc = "GICD interrupt processor target register 60"]
pub mod gicd_itargetsr60;
#[doc = "GICD_ITARGETSR61 (rw) register accessor: GICD interrupt processor target register 61\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr61::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr61`]
module"]
pub type GICD_ITARGETSR61 = crate::Reg<gicd_itargetsr61::GICD_ITARGETSR61rs>;
#[doc = "GICD interrupt processor target register 61"]
pub mod gicd_itargetsr61;
#[doc = "GICD_ITARGETSR62 (rw) register accessor: GICD interrupt processor target register 62\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr62::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr62`]
module"]
pub type GICD_ITARGETSR62 = crate::Reg<gicd_itargetsr62::GICD_ITARGETSR62rs>;
#[doc = "GICD interrupt processor target register 62"]
pub mod gicd_itargetsr62;
#[doc = "GICD_ITARGETSR63 (rw) register accessor: GICD interrupt processor target register 63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr63`]
module"]
pub type GICD_ITARGETSR63 = crate::Reg<gicd_itargetsr63::GICD_ITARGETSR63rs>;
#[doc = "GICD interrupt processor target register 63"]
pub mod gicd_itargetsr63;
#[doc = "GICD_ITARGETSR64 (rw) register accessor: GICD interrupt processor target register 64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr64`]
module"]
pub type GICD_ITARGETSR64 = crate::Reg<gicd_itargetsr64::GICD_ITARGETSR64rs>;
#[doc = "GICD interrupt processor target register 64"]
pub mod gicd_itargetsr64;
#[doc = "GICD_ITARGETSR65 (rw) register accessor: GICD interrupt processor target register 65\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr65::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr65::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr65`]
module"]
pub type GICD_ITARGETSR65 = crate::Reg<gicd_itargetsr65::GICD_ITARGETSR65rs>;
#[doc = "GICD interrupt processor target register 65"]
pub mod gicd_itargetsr65;
#[doc = "GICD_ITARGETSR66 (rw) register accessor: GICD interrupt processor target register 66\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr66::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr66::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr66`]
module"]
pub type GICD_ITARGETSR66 = crate::Reg<gicd_itargetsr66::GICD_ITARGETSR66rs>;
#[doc = "GICD interrupt processor target register 66"]
pub mod gicd_itargetsr66;
#[doc = "GICD_ITARGETSR67 (rw) register accessor: GICD interrupt processor target register 67\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr67`]
module"]
pub type GICD_ITARGETSR67 = crate::Reg<gicd_itargetsr67::GICD_ITARGETSR67rs>;
#[doc = "GICD interrupt processor target register 67"]
pub mod gicd_itargetsr67;
#[doc = "GICD_ITARGETSR68 (rw) register accessor: GICD interrupt processor target register 68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr68::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr68::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr68`]
module"]
pub type GICD_ITARGETSR68 = crate::Reg<gicd_itargetsr68::GICD_ITARGETSR68rs>;
#[doc = "GICD interrupt processor target register 68"]
pub mod gicd_itargetsr68;
#[doc = "GICD_ITARGETSR69 (rw) register accessor: GICD interrupt processor target register 69\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr69::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr69::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr69`]
module"]
pub type GICD_ITARGETSR69 = crate::Reg<gicd_itargetsr69::GICD_ITARGETSR69rs>;
#[doc = "GICD interrupt processor target register 69"]
pub mod gicd_itargetsr69;
#[doc = "GICD_ITARGETSR70 (rw) register accessor: GICD interrupt processor target register 70\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr70::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr70::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr70`]
module"]
pub type GICD_ITARGETSR70 = crate::Reg<gicd_itargetsr70::GICD_ITARGETSR70rs>;
#[doc = "GICD interrupt processor target register 70"]
pub mod gicd_itargetsr70;
#[doc = "GICD_ITARGETSR71 (rw) register accessor: GICD interrupt processor target register 71\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr71::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr71::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_itargetsr71`]
module"]
pub type GICD_ITARGETSR71 = crate::Reg<gicd_itargetsr71::GICD_ITARGETSR71rs>;
#[doc = "GICD interrupt processor target register 71"]
pub mod gicd_itargetsr71;
#[doc = "GICD_ICFGR0 (rw) register accessor: GICD interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr0`]
module"]
pub type GICD_ICFGR0 = crate::Reg<gicd_icfgr0::GICD_ICFGR0rs>;
#[doc = "GICD interrupt configuration register"]
pub mod gicd_icfgr0;
#[doc = "GICD_ICFGR1 (rw) register accessor: GICD interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr1`]
module"]
pub type GICD_ICFGR1 = crate::Reg<gicd_icfgr1::GICD_ICFGR1rs>;
#[doc = "GICD interrupt configuration register"]
pub mod gicd_icfgr1;
#[doc = "GICD_ICFGR2 (rw) register accessor: GICD interrupt configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr2`]
module"]
pub type GICD_ICFGR2 = crate::Reg<gicd_icfgr2::GICD_ICFGR2rs>;
#[doc = "GICD interrupt configuration register 2"]
pub mod gicd_icfgr2;
#[doc = "GICD_ICFGR3 (rw) register accessor: GICD interrupt configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr3`]
module"]
pub type GICD_ICFGR3 = crate::Reg<gicd_icfgr3::GICD_ICFGR3rs>;
#[doc = "GICD interrupt configuration register 3"]
pub mod gicd_icfgr3;
#[doc = "GICD_ICFGR4 (rw) register accessor: GICD interrupt configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr4`]
module"]
pub type GICD_ICFGR4 = crate::Reg<gicd_icfgr4::GICD_ICFGR4rs>;
#[doc = "GICD interrupt configuration register 4"]
pub mod gicd_icfgr4;
#[doc = "GICD_ICFGR5 (rw) register accessor: GICD interrupt configuration register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr5`]
module"]
pub type GICD_ICFGR5 = crate::Reg<gicd_icfgr5::GICD_ICFGR5rs>;
#[doc = "GICD interrupt configuration register 5"]
pub mod gicd_icfgr5;
#[doc = "GICD_ICFGR6 (rw) register accessor: GICD interrupt configuration register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr6`]
module"]
pub type GICD_ICFGR6 = crate::Reg<gicd_icfgr6::GICD_ICFGR6rs>;
#[doc = "GICD interrupt configuration register 6"]
pub mod gicd_icfgr6;
#[doc = "GICD_ICFGR7 (rw) register accessor: GICD interrupt configuration register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr7`]
module"]
pub type GICD_ICFGR7 = crate::Reg<gicd_icfgr7::GICD_ICFGR7rs>;
#[doc = "GICD interrupt configuration register 7"]
pub mod gicd_icfgr7;
#[doc = "GICD_ICFGR8 (rw) register accessor: GICD interrupt configuration register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr8`]
module"]
pub type GICD_ICFGR8 = crate::Reg<gicd_icfgr8::GICD_ICFGR8rs>;
#[doc = "GICD interrupt configuration register 8"]
pub mod gicd_icfgr8;
#[doc = "GICD_ICFGR9 (rw) register accessor: GICD interrupt configuration register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr9`]
module"]
pub type GICD_ICFGR9 = crate::Reg<gicd_icfgr9::GICD_ICFGR9rs>;
#[doc = "GICD interrupt configuration register 9"]
pub mod gicd_icfgr9;
#[doc = "GICD_ICFGR10 (rw) register accessor: GICD interrupt configuration register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr10`]
module"]
pub type GICD_ICFGR10 = crate::Reg<gicd_icfgr10::GICD_ICFGR10rs>;
#[doc = "GICD interrupt configuration register 10"]
pub mod gicd_icfgr10;
#[doc = "GICD_ICFGR11 (rw) register accessor: GICD interrupt configuration register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr11`]
module"]
pub type GICD_ICFGR11 = crate::Reg<gicd_icfgr11::GICD_ICFGR11rs>;
#[doc = "GICD interrupt configuration register 11"]
pub mod gicd_icfgr11;
#[doc = "GICD_ICFGR12 (rw) register accessor: GICD interrupt configuration register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr12`]
module"]
pub type GICD_ICFGR12 = crate::Reg<gicd_icfgr12::GICD_ICFGR12rs>;
#[doc = "GICD interrupt configuration register 12"]
pub mod gicd_icfgr12;
#[doc = "GICD_ICFGR13 (rw) register accessor: GICD interrupt configuration register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr13`]
module"]
pub type GICD_ICFGR13 = crate::Reg<gicd_icfgr13::GICD_ICFGR13rs>;
#[doc = "GICD interrupt configuration register 13"]
pub mod gicd_icfgr13;
#[doc = "GICD_ICFGR14 (rw) register accessor: GICD interrupt configuration register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr14`]
module"]
pub type GICD_ICFGR14 = crate::Reg<gicd_icfgr14::GICD_ICFGR14rs>;
#[doc = "GICD interrupt configuration register 14"]
pub mod gicd_icfgr14;
#[doc = "GICD_ICFGR15 (rw) register accessor: GICD interrupt configuration register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr15`]
module"]
pub type GICD_ICFGR15 = crate::Reg<gicd_icfgr15::GICD_ICFGR15rs>;
#[doc = "GICD interrupt configuration register 15"]
pub mod gicd_icfgr15;
#[doc = "GICD_ICFGR16 (rw) register accessor: GICD interrupt configuration register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr16`]
module"]
pub type GICD_ICFGR16 = crate::Reg<gicd_icfgr16::GICD_ICFGR16rs>;
#[doc = "GICD interrupt configuration register 16"]
pub mod gicd_icfgr16;
#[doc = "GICD_ICFGR17 (rw) register accessor: GICD interrupt configuration register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icfgr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icfgr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icfgr17`]
module"]
pub type GICD_ICFGR17 = crate::Reg<gicd_icfgr17::GICD_ICFGR17rs>;
#[doc = "GICD interrupt configuration register 17"]
pub mod gicd_icfgr17;
#[doc = "GICD_PPISR (r) register accessor: GICD private peripheral interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ppisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ppisr`]
module"]
pub type GICD_PPISR = crate::Reg<gicd_ppisr::GICD_PPISRrs>;
#[doc = "GICD private peripheral interrupt status register"]
pub mod gicd_ppisr;
#[doc = "GICD_SPISR1 (r) register accessor: For interrupts ID = SPI number+32, from SPI \\[x*32+31\\]
to SPI \\[x*32\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr1`]
module"]
pub type GICD_SPISR1 = crate::Reg<gicd_spisr1::GICD_SPISR1rs>;
#[doc = "For interrupts ID = SPI number+32, from SPI \\[x*32+31\\]
to SPI \\[x*32\\]"]
pub mod gicd_spisr1;
#[doc = "GICD_SPISR2 (r) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr2`]
module"]
pub type GICD_SPISR2 = crate::Reg<gicd_spisr2::GICD_SPISR2rs>;
#[doc = "For interrupts ID"]
pub mod gicd_spisr2;
#[doc = "GICD_SPISR3 (r) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr3`]
module"]
pub type GICD_SPISR3 = crate::Reg<gicd_spisr3::GICD_SPISR3rs>;
#[doc = "For interrupts ID"]
pub mod gicd_spisr3;
#[doc = "GICD_SPISR4 (r) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr4`]
module"]
pub type GICD_SPISR4 = crate::Reg<gicd_spisr4::GICD_SPISR4rs>;
#[doc = "For interrupts ID"]
pub mod gicd_spisr4;
#[doc = "GICD_SPISR5 (r) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr5`]
module"]
pub type GICD_SPISR5 = crate::Reg<gicd_spisr5::GICD_SPISR5rs>;
#[doc = "For interrupts ID"]
pub mod gicd_spisr5;
#[doc = "GICD_SPISR6 (r) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr6`]
module"]
pub type GICD_SPISR6 = crate::Reg<gicd_spisr6::GICD_SPISR6rs>;
#[doc = "For interrupts ID"]
pub mod gicd_spisr6;
#[doc = "GICD_SPISR7 (r) register accessor: For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr7`]
module"]
pub type GICD_SPISR7 = crate::Reg<gicd_spisr7::GICD_SPISR7rs>;
#[doc = "For interrupts ID"]
pub mod gicd_spisr7;
#[doc = "GICD_SGIR (w) register accessor: GICD software generated interrupt register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_sgir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_sgir`]
module"]
pub type GICD_SGIR = crate::Reg<gicd_sgir::GICD_SGIRrs>;
#[doc = "GICD software generated interrupt register"]
pub mod gicd_sgir;
#[doc = "GICD_CPENDSGIR0 (rw) register accessor: For SGI x*4 to SGI x*4+3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cpendsgir0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_cpendsgir0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cpendsgir0`]
module"]
pub type GICD_CPENDSGIR0 = crate::Reg<gicd_cpendsgir0::GICD_CPENDSGIR0rs>;
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_cpendsgir0;
#[doc = "GICD_CPENDSGIR1 (rw) register accessor: For SGI x*4 to SGI x*4+3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cpendsgir1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_cpendsgir1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cpendsgir1`]
module"]
pub type GICD_CPENDSGIR1 = crate::Reg<gicd_cpendsgir1::GICD_CPENDSGIR1rs>;
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_cpendsgir1;
#[doc = "GICD_CPENDSGIR2 (rw) register accessor: For SGI x*4 to SGI x*4+3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cpendsgir2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_cpendsgir2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cpendsgir2`]
module"]
pub type GICD_CPENDSGIR2 = crate::Reg<gicd_cpendsgir2::GICD_CPENDSGIR2rs>;
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_cpendsgir2;
#[doc = "GICD_CPENDSGIR3 (rw) register accessor: For SGI x*4 to SGI x*4+3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cpendsgir3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_cpendsgir3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cpendsgir3`]
module"]
pub type GICD_CPENDSGIR3 = crate::Reg<gicd_cpendsgir3::GICD_CPENDSGIR3rs>;
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_cpendsgir3;
#[doc = "GICD_SPENDSGIR0 (rw) register accessor: For SGI x*4 to SGI x*4+3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spendsgir0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_spendsgir0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spendsgir0`]
module"]
pub type GICD_SPENDSGIR0 = crate::Reg<gicd_spendsgir0::GICD_SPENDSGIR0rs>;
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_spendsgir0;
#[doc = "GICD_SPENDSGIR1 (rw) register accessor: For SGI x*4 to SGI x*4+3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spendsgir1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_spendsgir1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spendsgir1`]
module"]
pub type GICD_SPENDSGIR1 = crate::Reg<gicd_spendsgir1::GICD_SPENDSGIR1rs>;
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_spendsgir1;
#[doc = "GICD_SPENDSGIR2 (rw) register accessor: For SGI x*4 to SGI x*4+3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spendsgir2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_spendsgir2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spendsgir2`]
module"]
pub type GICD_SPENDSGIR2 = crate::Reg<gicd_spendsgir2::GICD_SPENDSGIR2rs>;
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_spendsgir2;
#[doc = "GICD_SPENDSGIR3 (rw) register accessor: For SGI x*4 to SGI x*4+3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spendsgir3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_spendsgir3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spendsgir3`]
module"]
pub type GICD_SPENDSGIR3 = crate::Reg<gicd_spendsgir3::GICD_SPENDSGIR3rs>;
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_spendsgir3;
#[doc = "GICD_PIDR4 (r) register accessor: GICD peripheral ID4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr4`]
module"]
pub type GICD_PIDR4 = crate::Reg<gicd_pidr4::GICD_PIDR4rs>;
#[doc = "GICD peripheral ID4 register"]
pub mod gicd_pidr4;
#[doc = "GICD_PIDR5 (r) register accessor: GICD peripheral ID5 to ID7 register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr5`]
module"]
pub type GICD_PIDR5 = crate::Reg<gicd_pidr5::GICD_PIDR5rs>;
#[doc = "GICD peripheral ID5 to ID7 register 5"]
pub mod gicd_pidr5;
#[doc = "GICD_PIDR6 (r) register accessor: GICD peripheral ID5 to ID7 register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr6`]
module"]
pub type GICD_PIDR6 = crate::Reg<gicd_pidr6::GICD_PIDR6rs>;
#[doc = "GICD peripheral ID5 to ID7 register 6"]
pub mod gicd_pidr6;
#[doc = "GICD_PIDR7 (r) register accessor: GICD peripheral ID5 to ID7 register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr7`]
module"]
pub type GICD_PIDR7 = crate::Reg<gicd_pidr7::GICD_PIDR7rs>;
#[doc = "GICD peripheral ID5 to ID7 register 7"]
pub mod gicd_pidr7;
#[doc = "GICD_PIDR0 (r) register accessor: GICD peripheral ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr0`]
module"]
pub type GICD_PIDR0 = crate::Reg<gicd_pidr0::GICD_PIDR0rs>;
#[doc = "GICD peripheral ID0 register"]
pub mod gicd_pidr0;
#[doc = "GICD_PIDR1 (r) register accessor: GICD peripheral ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr1`]
module"]
pub type GICD_PIDR1 = crate::Reg<gicd_pidr1::GICD_PIDR1rs>;
#[doc = "GICD peripheral ID1 register"]
pub mod gicd_pidr1;
#[doc = "GICD_PIDR2 (r) register accessor: GICD peripheral ID2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr2`]
module"]
pub type GICD_PIDR2 = crate::Reg<gicd_pidr2::GICD_PIDR2rs>;
#[doc = "GICD peripheral ID2 register"]
pub mod gicd_pidr2;
#[doc = "GICD_PIDR3 (r) register accessor: GICD peripheral ID3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr3`]
module"]
pub type GICD_PIDR3 = crate::Reg<gicd_pidr3::GICD_PIDR3rs>;
#[doc = "GICD peripheral ID3 register"]
pub mod gicd_pidr3;
#[doc = "GICD_CIDR0 (r) register accessor: GICD component ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cidr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cidr0`]
module"]
pub type GICD_CIDR0 = crate::Reg<gicd_cidr0::GICD_CIDR0rs>;
#[doc = "GICD component ID0 register"]
pub mod gicd_cidr0;
#[doc = "GICD_CIDR1 (r) register accessor: GICD component ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cidr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cidr1`]
module"]
pub type GICD_CIDR1 = crate::Reg<gicd_cidr1::GICD_CIDR1rs>;
#[doc = "GICD component ID1 register"]
pub mod gicd_cidr1;
#[doc = "GICD_CIDR2 (r) register accessor: GICD component ID2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cidr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cidr2`]
module"]
pub type GICD_CIDR2 = crate::Reg<gicd_cidr2::GICD_CIDR2rs>;
#[doc = "GICD component ID2 register"]
pub mod gicd_cidr2;
#[doc = "GICD_CIDR3 (r) register accessor: GICD component ID3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cidr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cidr3`]
module"]
pub type GICD_CIDR3 = crate::Reg<gicd_cidr3::GICD_CIDR3rs>;
#[doc = "GICD component ID3 register"]
pub mod gicd_cidr3;
