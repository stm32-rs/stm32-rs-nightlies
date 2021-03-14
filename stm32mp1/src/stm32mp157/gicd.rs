#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GICD control register"]
    pub gicd_ctlr: GICD_CTLR,
    #[doc = "0x04 - GICD interrupt controller type register"]
    pub gicd_typer: GICD_TYPER,
    #[doc = "0x08 - GICD implementer identification register"]
    pub gicd_iidr: GICD_IIDR,
    _reserved3: [u8; 116usize],
    #[doc = "0x80 - For interrupts ID"]
    pub gicd_igroupr0: GICD_IGROUPR0,
    #[doc = "0x84 - For interrupts ID"]
    pub gicd_igroupr1: GICD_IGROUPR1,
    #[doc = "0x88 - For interrupts ID"]
    pub gicd_igroupr2: GICD_IGROUPR2,
    #[doc = "0x8c - For interrupts ID = x*32 to ID = x*32+31"]
    pub gicd_igroupr3: GICD_IGROUPR3,
    #[doc = "0x90 - For interrupts ID = x*32 to ID = x*32+31"]
    pub gicd_igroupr4: GICD_IGROUPR4,
    #[doc = "0x94 - For interrupts ID"]
    pub gicd_igroupr5: GICD_IGROUPR5,
    #[doc = "0x98 - For interrupts ID"]
    pub gicd_igroupr6: GICD_IGROUPR6,
    #[doc = "0x9c - For interrupts ID"]
    pub gicd_igroupr7: GICD_IGROUPR7,
    #[doc = "0xa0 - For interrupts ID"]
    pub gicd_igroupr8: GICD_IGROUPR8,
    _reserved12: [u8; 92usize],
    #[doc = "0x100 - For interrupts ID = 0 to ID = 31"]
    pub gicd_isenabler0: GICD_ISENABLER0,
    #[doc = "0x104 - For interrupts ID"]
    pub gicd_isenabler1: GICD_ISENABLER1,
    #[doc = "0x108 - For interrupts ID"]
    pub gicd_isenabler2: GICD_ISENABLER2,
    #[doc = "0x10c - For interrupts ID"]
    pub gicd_isenabler3: GICD_ISENABLER3,
    #[doc = "0x110 - For interrupts ID"]
    pub gicd_isenabler4: GICD_ISENABLER4,
    #[doc = "0x114 - For interrupts ID"]
    pub gicd_isenabler5: GICD_ISENABLER5,
    #[doc = "0x118 - For interrupts ID"]
    pub gicd_isenabler6: GICD_ISENABLER6,
    #[doc = "0x11c - For interrupts ID"]
    pub gicd_isenabler7: GICD_ISENABLER7,
    #[doc = "0x120 - For interrupts ID"]
    pub gicd_isenabler8: GICD_ISENABLER8,
    _reserved21: [u8; 92usize],
    #[doc = "0x180 - For interrupts ID = 0 to ID = 31"]
    pub gicd_icenabler0: GICD_ICENABLER0,
    #[doc = "0x184 - For interrupts ID"]
    pub gicd_icenabler1: GICD_ICENABLER1,
    #[doc = "0x188 - For interrupts ID"]
    pub gicd_icenabler2: GICD_ICENABLER2,
    #[doc = "0x18c - For interrupts ID"]
    pub gicd_icenabler3: GICD_ICENABLER3,
    #[doc = "0x190 - For interrupts ID"]
    pub gicd_icenabler4: GICD_ICENABLER4,
    #[doc = "0x194 - For interrupts ID"]
    pub gicd_icenabler5: GICD_ICENABLER5,
    #[doc = "0x198 - For interrupts ID"]
    pub gicd_icenabler6: GICD_ICENABLER6,
    #[doc = "0x19c - For interrupts ID"]
    pub gicd_icenabler7: GICD_ICENABLER7,
    #[doc = "0x1a0 - For interrupts ID"]
    pub gicd_icenabler8: GICD_ICENABLER8,
    _reserved30: [u8; 92usize],
    #[doc = "0x200 - For interrupts ID"]
    pub gicd_ispendr0: GICD_ISPENDR0,
    #[doc = "0x204 - For interrupts ID"]
    pub gicd_ispendr1: GICD_ISPENDR1,
    #[doc = "0x208 - For interrupts ID"]
    pub gicd_ispendr2: GICD_ISPENDR2,
    #[doc = "0x20c - For interrupts ID"]
    pub gicd_ispendr3: GICD_ISPENDR3,
    #[doc = "0x210 - For interrupts ID"]
    pub gicd_ispendr4: GICD_ISPENDR4,
    #[doc = "0x214 - For interrupts ID"]
    pub gicd_ispendr5: GICD_ISPENDR5,
    #[doc = "0x218 - For interrupts ID"]
    pub gicd_ispendr6: GICD_ISPENDR6,
    #[doc = "0x21c - For interrupts ID"]
    pub gicd_ispendr7: GICD_ISPENDR7,
    #[doc = "0x220 - For interrupts ID"]
    pub gicd_ispendr8: GICD_ISPENDR8,
    _reserved39: [u8; 92usize],
    #[doc = "0x280 - For interrupts ID"]
    pub gicd_icpendr0: GICD_ICPENDR0,
    #[doc = "0x284 - For interrupts ID"]
    pub gicd_icpendr1: GICD_ICPENDR1,
    #[doc = "0x288 - For interrupts ID"]
    pub gicd_icpendr2: GICD_ICPENDR2,
    #[doc = "0x28c - For interrupts ID"]
    pub gicd_icpendr3: GICD_ICPENDR3,
    #[doc = "0x290 - For interrupts ID"]
    pub gicd_icpendr4: GICD_ICPENDR4,
    #[doc = "0x294 - For interrupts ID"]
    pub gicd_icpendr5: GICD_ICPENDR5,
    #[doc = "0x298 - For interrupts ID"]
    pub gicd_icpendr6: GICD_ICPENDR6,
    #[doc = "0x29c - For interrupts ID"]
    pub gicd_icpendr7: GICD_ICPENDR7,
    #[doc = "0x2a0 - For interrupts ID"]
    pub gicd_icpendr8: GICD_ICPENDR8,
    _reserved48: [u8; 92usize],
    #[doc = "0x300 - For interrupts ID"]
    pub gicd_isactiver0: GICD_ISACTIVER0,
    #[doc = "0x304 - For interrupts ID"]
    pub gicd_isactiver1: GICD_ISACTIVER1,
    #[doc = "0x308 - For interrupts ID"]
    pub gicd_isactiver2: GICD_ISACTIVER2,
    #[doc = "0x30c - For interrupts ID"]
    pub gicd_isactiver3: GICD_ISACTIVER3,
    #[doc = "0x310 - For interrupts ID"]
    pub gicd_isactiver4: GICD_ISACTIVER4,
    #[doc = "0x314 - For interrupts ID"]
    pub gicd_isactiver5: GICD_ISACTIVER5,
    #[doc = "0x318 - For interrupts ID"]
    pub gicd_isactiver6: GICD_ISACTIVER6,
    #[doc = "0x31c - For interrupts ID"]
    pub gicd_isactiver7: GICD_ISACTIVER7,
    #[doc = "0x320 - For interrupts ID"]
    pub gicd_isactiver8: GICD_ISACTIVER8,
    _reserved57: [u8; 92usize],
    #[doc = "0x380 - For interrupts ID"]
    pub gicd_icactiver0: GICD_ICACTIVER0,
    #[doc = "0x384 - For interrupts ID"]
    pub gicd_icactiver1: GICD_ICACTIVER1,
    #[doc = "0x388 - For interrupts ID"]
    pub gicd_icactiver2: GICD_ICACTIVER2,
    #[doc = "0x38c - For interrupts ID"]
    pub gicd_icactiver3: GICD_ICACTIVER3,
    #[doc = "0x390 - For interrupts ID"]
    pub gicd_icactiver4: GICD_ICACTIVER4,
    #[doc = "0x394 - For interrupts ID"]
    pub gicd_icactiver5: GICD_ICACTIVER5,
    #[doc = "0x398 - For interrupts ID"]
    pub gicd_icactiver6: GICD_ICACTIVER6,
    #[doc = "0x39c - For interrupts ID"]
    pub gicd_icactiver7: GICD_ICACTIVER7,
    #[doc = "0x3a0 - For interrupts ID"]
    pub gicd_icactiver8: GICD_ICACTIVER8,
    _reserved66: [u8; 92usize],
    #[doc = "0x400 - GICD interrupt priority register 0"]
    pub gicd_ipriorityr0: GICD_IPRIORITYR0,
    #[doc = "0x404 - GICD interrupt priority register 1"]
    pub gicd_ipriorityr1: GICD_IPRIORITYR1,
    #[doc = "0x408 - GICD interrupt priority register 2"]
    pub gicd_ipriorityr2: GICD_IPRIORITYR2,
    #[doc = "0x40c - GICD interrupt priority register 3"]
    pub gicd_ipriorityr3: GICD_IPRIORITYR3,
    #[doc = "0x410 - GICD interrupt priority register 4"]
    pub gicd_ipriorityr4: GICD_IPRIORITYR4,
    #[doc = "0x414 - GICD interrupt priority register 5"]
    pub gicd_ipriorityr5: GICD_IPRIORITYR5,
    #[doc = "0x418 - GICD interrupt priority register 6"]
    pub gicd_ipriorityr6: GICD_IPRIORITYR6,
    #[doc = "0x41c - GICD interrupt priority register 7"]
    pub gicd_ipriorityr7: GICD_IPRIORITYR7,
    #[doc = "0x420 - GICD interrupt priority register 8"]
    pub gicd_ipriorityr8: GICD_IPRIORITYR8,
    #[doc = "0x424 - GICD interrupt priority register 9"]
    pub gicd_ipriorityr9: GICD_IPRIORITYR9,
    #[doc = "0x428 - GICD interrupt priority register 10"]
    pub gicd_ipriorityr10: GICD_IPRIORITYR10,
    #[doc = "0x42c - GICD interrupt priority register 11"]
    pub gicd_ipriorityr11: GICD_IPRIORITYR11,
    #[doc = "0x430 - GICD interrupt priority register 12"]
    pub gicd_ipriorityr12: GICD_IPRIORITYR12,
    #[doc = "0x434 - GICD interrupt priority register 13"]
    pub gicd_ipriorityr13: GICD_IPRIORITYR13,
    #[doc = "0x438 - GICD interrupt priority register 14"]
    pub gicd_ipriorityr14: GICD_IPRIORITYR14,
    #[doc = "0x43c - GICD interrupt priority register 15"]
    pub gicd_ipriorityr15: GICD_IPRIORITYR15,
    #[doc = "0x440 - GICD interrupt priority register 16"]
    pub gicd_ipriorityr16: GICD_IPRIORITYR16,
    #[doc = "0x444 - GICD interrupt priority register 17"]
    pub gicd_ipriorityr17: GICD_IPRIORITYR17,
    #[doc = "0x448 - GICD interrupt priority register 18"]
    pub gicd_ipriorityr18: GICD_IPRIORITYR18,
    #[doc = "0x44c - GICD interrupt priority register 19"]
    pub gicd_ipriorityr19: GICD_IPRIORITYR19,
    #[doc = "0x450 - GICD interrupt priority register 20"]
    pub gicd_ipriorityr20: GICD_IPRIORITYR20,
    #[doc = "0x454 - GICD interrupt priority register 21"]
    pub gicd_ipriorityr21: GICD_IPRIORITYR21,
    #[doc = "0x458 - GICD interrupt priority register 22"]
    pub gicd_ipriorityr22: GICD_IPRIORITYR22,
    #[doc = "0x45c - GICD interrupt priority register 23"]
    pub gicd_ipriorityr23: GICD_IPRIORITYR23,
    #[doc = "0x460 - GICD interrupt priority register 24"]
    pub gicd_ipriorityr24: GICD_IPRIORITYR24,
    #[doc = "0x464 - GICD interrupt priority register 25"]
    pub gicd_ipriorityr25: GICD_IPRIORITYR25,
    #[doc = "0x468 - GICD interrupt priority register 26"]
    pub gicd_ipriorityr26: GICD_IPRIORITYR26,
    #[doc = "0x46c - GICD interrupt priority register 27"]
    pub gicd_ipriorityr27: GICD_IPRIORITYR27,
    #[doc = "0x470 - GICD interrupt priority register 28"]
    pub gicd_ipriorityr28: GICD_IPRIORITYR28,
    #[doc = "0x474 - GICD interrupt priority register 29"]
    pub gicd_ipriorityr29: GICD_IPRIORITYR29,
    #[doc = "0x478 - GICD interrupt priority register 30"]
    pub gicd_ipriorityr30: GICD_IPRIORITYR30,
    #[doc = "0x47c - GICD interrupt priority register 31"]
    pub gicd_ipriorityr31: GICD_IPRIORITYR31,
    #[doc = "0x480 - GICD interrupt priority register 32"]
    pub gicd_ipriorityr32: GICD_IPRIORITYR32,
    #[doc = "0x484 - GICD interrupt priority register 33"]
    pub gicd_ipriorityr33: GICD_IPRIORITYR33,
    #[doc = "0x488 - GICD interrupt priority register 34"]
    pub gicd_ipriorityr34: GICD_IPRIORITYR34,
    #[doc = "0x48c - GICD interrupt priority register 35"]
    pub gicd_ipriorityr35: GICD_IPRIORITYR35,
    #[doc = "0x490 - GICD interrupt priority register 36"]
    pub gicd_ipriorityr36: GICD_IPRIORITYR36,
    #[doc = "0x494 - GICD interrupt priority register 37"]
    pub gicd_ipriorityr37: GICD_IPRIORITYR37,
    #[doc = "0x498 - GICD interrupt priority register 38"]
    pub gicd_ipriorityr38: GICD_IPRIORITYR38,
    #[doc = "0x49c - GICD interrupt priority register 39"]
    pub gicd_ipriorityr39: GICD_IPRIORITYR39,
    #[doc = "0x4a0 - GICD interrupt priority register 40"]
    pub gicd_ipriorityr40: GICD_IPRIORITYR40,
    #[doc = "0x4a4 - GICD interrupt priority register 41"]
    pub gicd_ipriorityr41: GICD_IPRIORITYR41,
    #[doc = "0x4a8 - GICD interrupt priority register 42"]
    pub gicd_ipriorityr42: GICD_IPRIORITYR42,
    #[doc = "0x4ac - GICD interrupt priority register 43"]
    pub gicd_ipriorityr43: GICD_IPRIORITYR43,
    #[doc = "0x4b0 - GICD interrupt priority register 44"]
    pub gicd_ipriorityr44: GICD_IPRIORITYR44,
    #[doc = "0x4b4 - GICD interrupt priority register 45"]
    pub gicd_ipriorityr45: GICD_IPRIORITYR45,
    #[doc = "0x4b8 - GICD interrupt priority register 46"]
    pub gicd_ipriorityr46: GICD_IPRIORITYR46,
    #[doc = "0x4bc - GICD interrupt priority register 47"]
    pub gicd_ipriorityr47: GICD_IPRIORITYR47,
    #[doc = "0x4c0 - GICD interrupt priority register 48"]
    pub gicd_ipriorityr48: GICD_IPRIORITYR48,
    #[doc = "0x4c4 - GICD interrupt priority register 49"]
    pub gicd_ipriorityr49: GICD_IPRIORITYR49,
    #[doc = "0x4c8 - GICD interrupt priority register 50"]
    pub gicd_ipriorityr50: GICD_IPRIORITYR50,
    #[doc = "0x4cc - GICD interrupt priority register 51"]
    pub gicd_ipriorityr51: GICD_IPRIORITYR51,
    #[doc = "0x4d0 - GICD interrupt priority register 52"]
    pub gicd_ipriorityr52: GICD_IPRIORITYR52,
    #[doc = "0x4d4 - GICD interrupt priority register 53"]
    pub gicd_ipriorityr53: GICD_IPRIORITYR53,
    #[doc = "0x4d8 - GICD interrupt priority register 54"]
    pub gicd_ipriorityr54: GICD_IPRIORITYR54,
    #[doc = "0x4dc - GICD interrupt priority register 55"]
    pub gicd_ipriorityr55: GICD_IPRIORITYR55,
    #[doc = "0x4e0 - GICD interrupt priority register 56"]
    pub gicd_ipriorityr56: GICD_IPRIORITYR56,
    #[doc = "0x4e4 - GICD interrupt priority register 57"]
    pub gicd_ipriorityr57: GICD_IPRIORITYR57,
    #[doc = "0x4e8 - GICD interrupt priority register 58"]
    pub gicd_ipriorityr58: GICD_IPRIORITYR58,
    #[doc = "0x4ec - GICD interrupt priority register 59"]
    pub gicd_ipriorityr59: GICD_IPRIORITYR59,
    #[doc = "0x4f0 - GICD interrupt priority register 60"]
    pub gicd_ipriorityr60: GICD_IPRIORITYR60,
    #[doc = "0x4f4 - GICD interrupt priority register 61"]
    pub gicd_ipriorityr61: GICD_IPRIORITYR61,
    #[doc = "0x4f8 - GICD interrupt priority register 62"]
    pub gicd_ipriorityr62: GICD_IPRIORITYR62,
    #[doc = "0x4fc - GICD interrupt priority register 63"]
    pub gicd_ipriorityr63: GICD_IPRIORITYR63,
    #[doc = "0x500 - GICD interrupt priority register 64"]
    pub gicd_ipriorityr64: GICD_IPRIORITYR64,
    #[doc = "0x504 - GICD interrupt priority register 65"]
    pub gicd_ipriorityr65: GICD_IPRIORITYR65,
    #[doc = "0x508 - GICD interrupt priority register 66"]
    pub gicd_ipriorityr66: GICD_IPRIORITYR66,
    #[doc = "0x50c - GICD interrupt priority register 67"]
    pub gicd_ipriorityr67: GICD_IPRIORITYR67,
    #[doc = "0x510 - GICD interrupt priority register 68"]
    pub gicd_ipriorityr68: GICD_IPRIORITYR68,
    #[doc = "0x514 - GICD interrupt priority register 69"]
    pub gicd_ipriorityr69: GICD_IPRIORITYR69,
    #[doc = "0x518 - GICD interrupt priority register 70"]
    pub gicd_ipriorityr70: GICD_IPRIORITYR70,
    #[doc = "0x51c - GICD interrupt priority register 71"]
    pub gicd_ipriorityr71: GICD_IPRIORITYR71,
    _reserved138: [u8; 736usize],
    #[doc = "0x800 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    pub gicd_itargetsr0: GICD_ITARGETSR0,
    #[doc = "0x804 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    pub gicd_itargetsr1: GICD_ITARGETSR1,
    #[doc = "0x808 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    pub gicd_itargetsr2: GICD_ITARGETSR2,
    #[doc = "0x80c - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    pub gicd_itargetsr3: GICD_ITARGETSR3,
    #[doc = "0x810 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    pub gicd_itargetsr4: GICD_ITARGETSR4,
    #[doc = "0x814 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    pub gicd_itargetsr5: GICD_ITARGETSR5,
    #[doc = "0x818 - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    pub gicd_itargetsr6: GICD_ITARGETSR6,
    #[doc = "0x81c - For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
    pub gicd_itargetsr7: GICD_ITARGETSR7,
    #[doc = "0x820 - GICD interrupt processor target register 8"]
    pub gicd_itargetsr8: GICD_ITARGETSR8,
    #[doc = "0x824 - GICD interrupt processor target register 9"]
    pub gicd_itargetsr9: GICD_ITARGETSR9,
    #[doc = "0x828 - GICD interrupt processor target register 10"]
    pub gicd_itargetsr10: GICD_ITARGETSR10,
    #[doc = "0x82c - GICD interrupt processor target register 11"]
    pub gicd_itargetsr11: GICD_ITARGETSR11,
    #[doc = "0x830 - GICD interrupt processor target register 12"]
    pub gicd_itargetsr12: GICD_ITARGETSR12,
    #[doc = "0x834 - GICD interrupt processor target register 13"]
    pub gicd_itargetsr13: GICD_ITARGETSR13,
    #[doc = "0x838 - GICD interrupt processor target register 14"]
    pub gicd_itargetsr14: GICD_ITARGETSR14,
    #[doc = "0x83c - GICD interrupt processor target register 15"]
    pub gicd_itargetsr15: GICD_ITARGETSR15,
    #[doc = "0x840 - GICD interrupt processor target register 16"]
    pub gicd_itargetsr16: GICD_ITARGETSR16,
    #[doc = "0x844 - GICD interrupt processor target register 17"]
    pub gicd_itargetsr17: GICD_ITARGETSR17,
    #[doc = "0x848 - GICD interrupt processor target register 18"]
    pub gicd_itargetsr18: GICD_ITARGETSR18,
    #[doc = "0x84c - GICD interrupt processor target register 19"]
    pub gicd_itargetsr19: GICD_ITARGETSR19,
    #[doc = "0x850 - GICD interrupt processor target register 20"]
    pub gicd_itargetsr20: GICD_ITARGETSR20,
    #[doc = "0x854 - GICD interrupt processor target register 21"]
    pub gicd_itargetsr21: GICD_ITARGETSR21,
    #[doc = "0x858 - GICD interrupt processor target register 22"]
    pub gicd_itargetsr22: GICD_ITARGETSR22,
    #[doc = "0x85c - GICD interrupt processor target register 23"]
    pub gicd_itargetsr23: GICD_ITARGETSR23,
    #[doc = "0x860 - GICD interrupt processor target register 24"]
    pub gicd_itargetsr24: GICD_ITARGETSR24,
    #[doc = "0x864 - GICD interrupt processor target register 25"]
    pub gicd_itargetsr25: GICD_ITARGETSR25,
    #[doc = "0x868 - GICD interrupt processor target register 26"]
    pub gicd_itargetsr26: GICD_ITARGETSR26,
    #[doc = "0x86c - GICD interrupt processor target register 27"]
    pub gicd_itargetsr27: GICD_ITARGETSR27,
    #[doc = "0x870 - GICD interrupt processor target register 28"]
    pub gicd_itargetsr28: GICD_ITARGETSR28,
    #[doc = "0x874 - GICD interrupt processor target register 29"]
    pub gicd_itargetsr29: GICD_ITARGETSR29,
    #[doc = "0x878 - GICD interrupt processor target register 30"]
    pub gicd_itargetsr30: GICD_ITARGETSR30,
    #[doc = "0x87c - GICD interrupt processor target register 31"]
    pub gicd_itargetsr31: GICD_ITARGETSR31,
    #[doc = "0x880 - GICD interrupt processor target register 32"]
    pub gicd_itargetsr32: GICD_ITARGETSR32,
    #[doc = "0x884 - GICD interrupt processor target register 33"]
    pub gicd_itargetsr33: GICD_ITARGETSR33,
    #[doc = "0x888 - GICD interrupt processor target register 34"]
    pub gicd_itargetsr34: GICD_ITARGETSR34,
    #[doc = "0x88c - GICD interrupt processor target register 35"]
    pub gicd_itargetsr35: GICD_ITARGETSR35,
    #[doc = "0x890 - GICD interrupt processor target register 36"]
    pub gicd_itargetsr36: GICD_ITARGETSR36,
    #[doc = "0x894 - GICD interrupt processor target register 37"]
    pub gicd_itargetsr37: GICD_ITARGETSR37,
    #[doc = "0x898 - GICD interrupt processor target register 38"]
    pub gicd_itargetsr38: GICD_ITARGETSR38,
    #[doc = "0x89c - GICD interrupt processor target register 39"]
    pub gicd_itargetsr39: GICD_ITARGETSR39,
    #[doc = "0x8a0 - GICD interrupt processor target register 40"]
    pub gicd_itargetsr40: GICD_ITARGETSR40,
    #[doc = "0x8a4 - GICD interrupt processor target register 41"]
    pub gicd_itargetsr41: GICD_ITARGETSR41,
    #[doc = "0x8a8 - GICD interrupt processor target register 42"]
    pub gicd_itargetsr42: GICD_ITARGETSR42,
    #[doc = "0x8ac - GICD interrupt processor target register 43"]
    pub gicd_itargetsr43: GICD_ITARGETSR43,
    #[doc = "0x8b0 - GICD interrupt processor target register 44"]
    pub gicd_itargetsr44: GICD_ITARGETSR44,
    #[doc = "0x8b4 - GICD interrupt processor target register 45"]
    pub gicd_itargetsr45: GICD_ITARGETSR45,
    #[doc = "0x8b8 - GICD interrupt processor target register 46"]
    pub gicd_itargetsr46: GICD_ITARGETSR46,
    #[doc = "0x8bc - GICD interrupt processor target register 47"]
    pub gicd_itargetsr47: GICD_ITARGETSR47,
    #[doc = "0x8c0 - GICD interrupt processor target register 48"]
    pub gicd_itargetsr48: GICD_ITARGETSR48,
    #[doc = "0x8c4 - GICD interrupt processor target register 49"]
    pub gicd_itargetsr49: GICD_ITARGETSR49,
    #[doc = "0x8c8 - GICD interrupt processor target register 50"]
    pub gicd_itargetsr50: GICD_ITARGETSR50,
    #[doc = "0x8cc - GICD interrupt processor target register 51"]
    pub gicd_itargetsr51: GICD_ITARGETSR51,
    #[doc = "0x8d0 - GICD interrupt processor target register 52"]
    pub gicd_itargetsr52: GICD_ITARGETSR52,
    #[doc = "0x8d4 - GICD interrupt processor target register 53"]
    pub gicd_itargetsr53: GICD_ITARGETSR53,
    #[doc = "0x8d8 - GICD interrupt processor target register 54"]
    pub gicd_itargetsr54: GICD_ITARGETSR54,
    #[doc = "0x8dc - GICD interrupt processor target register 55"]
    pub gicd_itargetsr55: GICD_ITARGETSR55,
    #[doc = "0x8e0 - GICD interrupt processor target register 56"]
    pub gicd_itargetsr56: GICD_ITARGETSR56,
    #[doc = "0x8e4 - GICD interrupt processor target register 57"]
    pub gicd_itargetsr57: GICD_ITARGETSR57,
    #[doc = "0x8e8 - GICD interrupt processor target register 58"]
    pub gicd_itargetsr58: GICD_ITARGETSR58,
    #[doc = "0x8ec - GICD interrupt processor target register 59"]
    pub gicd_itargetsr59: GICD_ITARGETSR59,
    #[doc = "0x8f0 - GICD interrupt processor target register 60"]
    pub gicd_itargetsr60: GICD_ITARGETSR60,
    #[doc = "0x8f4 - GICD interrupt processor target register 61"]
    pub gicd_itargetsr61: GICD_ITARGETSR61,
    #[doc = "0x8f8 - GICD interrupt processor target register 62"]
    pub gicd_itargetsr62: GICD_ITARGETSR62,
    #[doc = "0x8fc - GICD interrupt processor target register 63"]
    pub gicd_itargetsr63: GICD_ITARGETSR63,
    #[doc = "0x900 - GICD interrupt processor target register 64"]
    pub gicd_itargetsr64: GICD_ITARGETSR64,
    #[doc = "0x904 - GICD interrupt processor target register 65"]
    pub gicd_itargetsr65: GICD_ITARGETSR65,
    #[doc = "0x908 - GICD interrupt processor target register 66"]
    pub gicd_itargetsr66: GICD_ITARGETSR66,
    #[doc = "0x90c - GICD interrupt processor target register 67"]
    pub gicd_itargetsr67: GICD_ITARGETSR67,
    #[doc = "0x910 - GICD interrupt processor target register 68"]
    pub gicd_itargetsr68: GICD_ITARGETSR68,
    #[doc = "0x914 - GICD interrupt processor target register 69"]
    pub gicd_itargetsr69: GICD_ITARGETSR69,
    #[doc = "0x918 - GICD interrupt processor target register 70"]
    pub gicd_itargetsr70: GICD_ITARGETSR70,
    #[doc = "0x91c - GICD interrupt processor target register 71"]
    pub gicd_itargetsr71: GICD_ITARGETSR71,
    _reserved210: [u8; 736usize],
    #[doc = "0xc00 - GICD interrupt configuration register"]
    pub gicd_icfgr0: GICD_ICFGR0,
    #[doc = "0xc04 - GICD interrupt configuration register"]
    pub gicd_icfgr1: GICD_ICFGR1,
    #[doc = "0xc08 - GICD interrupt configuration register 2"]
    pub gicd_icfgr2: GICD_ICFGR2,
    #[doc = "0xc0c - GICD interrupt configuration register 3"]
    pub gicd_icfgr3: GICD_ICFGR3,
    #[doc = "0xc10 - GICD interrupt configuration register 4"]
    pub gicd_icfgr4: GICD_ICFGR4,
    #[doc = "0xc14 - GICD interrupt configuration register 5"]
    pub gicd_icfgr5: GICD_ICFGR5,
    #[doc = "0xc18 - GICD interrupt configuration register 6"]
    pub gicd_icfgr6: GICD_ICFGR6,
    #[doc = "0xc1c - GICD interrupt configuration register 7"]
    pub gicd_icfgr7: GICD_ICFGR7,
    #[doc = "0xc20 - GICD interrupt configuration register 8"]
    pub gicd_icfgr8: GICD_ICFGR8,
    #[doc = "0xc24 - GICD interrupt configuration register 9"]
    pub gicd_icfgr9: GICD_ICFGR9,
    #[doc = "0xc28 - GICD interrupt configuration register 10"]
    pub gicd_icfgr10: GICD_ICFGR10,
    #[doc = "0xc2c - GICD interrupt configuration register 11"]
    pub gicd_icfgr11: GICD_ICFGR11,
    #[doc = "0xc30 - GICD interrupt configuration register 12"]
    pub gicd_icfgr12: GICD_ICFGR12,
    #[doc = "0xc34 - GICD interrupt configuration register 13"]
    pub gicd_icfgr13: GICD_ICFGR13,
    #[doc = "0xc38 - GICD interrupt configuration register 14"]
    pub gicd_icfgr14: GICD_ICFGR14,
    #[doc = "0xc3c - GICD interrupt configuration register 15"]
    pub gicd_icfgr15: GICD_ICFGR15,
    #[doc = "0xc40 - GICD interrupt configuration register 16"]
    pub gicd_icfgr16: GICD_ICFGR16,
    #[doc = "0xc44 - GICD interrupt configuration register 17"]
    pub gicd_icfgr17: GICD_ICFGR17,
    _reserved228: [u8; 184usize],
    #[doc = "0xd00 - GICD private peripheral interrupt status register"]
    pub gicd_ppisr: GICD_PPISR,
    _reserved229: [u8; 4usize],
    #[doc = "0xd08 - For interrupts ID = SPI number+32, from SPI \\[x*32+31\\]
to SPI \\[x*32\\]"]
    pub gicd_spisr1: GICD_SPISR1,
    #[doc = "0xd0c - For interrupts ID"]
    pub gicd_spisr2: GICD_SPISR2,
    #[doc = "0xd10 - For interrupts ID"]
    pub gicd_spisr3: GICD_SPISR3,
    #[doc = "0xd14 - For interrupts ID"]
    pub gicd_spisr4: GICD_SPISR4,
    #[doc = "0xd18 - For interrupts ID"]
    pub gicd_spisr5: GICD_SPISR5,
    #[doc = "0xd1c - For interrupts ID"]
    pub gicd_spisr6: GICD_SPISR6,
    #[doc = "0xd20 - For interrupts ID"]
    pub gicd_spisr7: GICD_SPISR7,
    _reserved236: [u8; 476usize],
    #[doc = "0xf00 - GICD software generated interrupt register"]
    pub gicd_sgir: GICD_SGIR,
    _reserved237: [u8; 12usize],
    #[doc = "0xf10 - For SGI x*4 to SGI x*4+3"]
    pub gicd_cpendsgir0: GICD_CPENDSGIR0,
    #[doc = "0xf14 - For SGI x*4 to SGI x*4+3"]
    pub gicd_cpendsgir1: GICD_CPENDSGIR1,
    #[doc = "0xf18 - For SGI x*4 to SGI x*4+3"]
    pub gicd_cpendsgir2: GICD_CPENDSGIR2,
    #[doc = "0xf1c - For SGI x*4 to SGI x*4+3"]
    pub gicd_cpendsgir3: GICD_CPENDSGIR3,
    #[doc = "0xf20 - For SGI x*4 to SGI x*4+3"]
    pub gicd_spendsgir0: GICD_SPENDSGIR0,
    #[doc = "0xf24 - For SGI x*4 to SGI x*4+3"]
    pub gicd_spendsgir1: GICD_SPENDSGIR1,
    #[doc = "0xf28 - For SGI x*4 to SGI x*4+3"]
    pub gicd_spendsgir2: GICD_SPENDSGIR2,
    #[doc = "0xf2c - For SGI x*4 to SGI x*4+3"]
    pub gicd_spendsgir3: GICD_SPENDSGIR3,
    _reserved245: [u8; 160usize],
    #[doc = "0xfd0 - GICD peripheral ID4 register"]
    pub gicd_pidr4: GICD_PIDR4,
    #[doc = "0xfd4 - GICD peripheral ID5 to ID7 register 5"]
    pub gicd_pidr5: GICD_PIDR5,
    #[doc = "0xfd8 - GICD peripheral ID5 to ID7 register 6"]
    pub gicd_pidr6: GICD_PIDR6,
    #[doc = "0xfdc - GICD peripheral ID5 to ID7 register 7"]
    pub gicd_pidr7: GICD_PIDR7,
    #[doc = "0xfe0 - GICD peripheral ID0 register"]
    pub gicd_pidr0: GICD_PIDR0,
    #[doc = "0xfe4 - GICD peripheral ID1 register"]
    pub gicd_pidr1: GICD_PIDR1,
    #[doc = "0xfe8 - GICD peripheral ID2 register"]
    pub gicd_pidr2: GICD_PIDR2,
    #[doc = "0xfec - GICD peripheral ID3 register"]
    pub gicd_pidr3: GICD_PIDR3,
    #[doc = "0xff0 - GICD component ID0 register"]
    pub gicd_cidr0: GICD_CIDR0,
    #[doc = "0xff4 - GICD component ID1 register"]
    pub gicd_cidr1: GICD_CIDR1,
    #[doc = "0xff8 - GICD component ID2 register"]
    pub gicd_cidr2: GICD_CIDR2,
    #[doc = "0xffc - GICD component ID3 register"]
    pub gicd_cidr3: GICD_CIDR3,
}
#[doc = "GICD control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ctlr](gicd_ctlr) module"]
pub type GICD_CTLR = crate::Reg<u32, _GICD_CTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_CTLR;
#[doc = "`read()` method returns [gicd_ctlr::R](gicd_ctlr::R) reader structure"]
impl crate::Readable for GICD_CTLR {}
#[doc = "`write(|w| ..)` method takes [gicd_ctlr::W](gicd_ctlr::W) writer structure"]
impl crate::Writable for GICD_CTLR {}
#[doc = "GICD control register"]
pub mod gicd_ctlr;
#[doc = "GICD interrupt controller type register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_typer](gicd_typer) module"]
pub type GICD_TYPER = crate::Reg<u32, _GICD_TYPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_TYPER;
#[doc = "`read()` method returns [gicd_typer::R](gicd_typer::R) reader structure"]
impl crate::Readable for GICD_TYPER {}
#[doc = "GICD interrupt controller type register"]
pub mod gicd_typer;
#[doc = "GICD implementer identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_iidr](gicd_iidr) module"]
pub type GICD_IIDR = crate::Reg<u32, _GICD_IIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IIDR;
#[doc = "`read()` method returns [gicd_iidr::R](gicd_iidr::R) reader structure"]
impl crate::Readable for GICD_IIDR {}
#[doc = "GICD implementer identification register"]
pub mod gicd_iidr;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr0](gicd_igroupr0) module"]
pub type GICD_IGROUPR0 = crate::Reg<u32, _GICD_IGROUPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR0;
#[doc = "`read()` method returns [gicd_igroupr0::R](gicd_igroupr0::R) reader structure"]
impl crate::Readable for GICD_IGROUPR0 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr0::W](gicd_igroupr0::W) writer structure"]
impl crate::Writable for GICD_IGROUPR0 {}
#[doc = "For interrupts ID"]
pub mod gicd_igroupr0;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr1](gicd_igroupr1) module"]
pub type GICD_IGROUPR1 = crate::Reg<u32, _GICD_IGROUPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR1;
#[doc = "`read()` method returns [gicd_igroupr1::R](gicd_igroupr1::R) reader structure"]
impl crate::Readable for GICD_IGROUPR1 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr1::W](gicd_igroupr1::W) writer structure"]
impl crate::Writable for GICD_IGROUPR1 {}
#[doc = "For interrupts ID"]
pub mod gicd_igroupr1;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr2](gicd_igroupr2) module"]
pub type GICD_IGROUPR2 = crate::Reg<u32, _GICD_IGROUPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR2;
#[doc = "`read()` method returns [gicd_igroupr2::R](gicd_igroupr2::R) reader structure"]
impl crate::Readable for GICD_IGROUPR2 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr2::W](gicd_igroupr2::W) writer structure"]
impl crate::Writable for GICD_IGROUPR2 {}
#[doc = "For interrupts ID"]
pub mod gicd_igroupr2;
#[doc = "For interrupts ID = x*32 to ID = x*32+31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr3](gicd_igroupr3) module"]
pub type GICD_IGROUPR3 = crate::Reg<u32, _GICD_IGROUPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR3;
#[doc = "`read()` method returns [gicd_igroupr3::R](gicd_igroupr3::R) reader structure"]
impl crate::Readable for GICD_IGROUPR3 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr3::W](gicd_igroupr3::W) writer structure"]
impl crate::Writable for GICD_IGROUPR3 {}
#[doc = "For interrupts ID = x*32 to ID = x*32+31"]
pub mod gicd_igroupr3;
#[doc = "For interrupts ID = x*32 to ID = x*32+31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr4](gicd_igroupr4) module"]
pub type GICD_IGROUPR4 = crate::Reg<u32, _GICD_IGROUPR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR4;
#[doc = "`read()` method returns [gicd_igroupr4::R](gicd_igroupr4::R) reader structure"]
impl crate::Readable for GICD_IGROUPR4 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr4::W](gicd_igroupr4::W) writer structure"]
impl crate::Writable for GICD_IGROUPR4 {}
#[doc = "For interrupts ID = x*32 to ID = x*32+31"]
pub mod gicd_igroupr4;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr5](gicd_igroupr5) module"]
pub type GICD_IGROUPR5 = crate::Reg<u32, _GICD_IGROUPR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR5;
#[doc = "`read()` method returns [gicd_igroupr5::R](gicd_igroupr5::R) reader structure"]
impl crate::Readable for GICD_IGROUPR5 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr5::W](gicd_igroupr5::W) writer structure"]
impl crate::Writable for GICD_IGROUPR5 {}
#[doc = "For interrupts ID"]
pub mod gicd_igroupr5;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr6](gicd_igroupr6) module"]
pub type GICD_IGROUPR6 = crate::Reg<u32, _GICD_IGROUPR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR6;
#[doc = "`read()` method returns [gicd_igroupr6::R](gicd_igroupr6::R) reader structure"]
impl crate::Readable for GICD_IGROUPR6 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr6::W](gicd_igroupr6::W) writer structure"]
impl crate::Writable for GICD_IGROUPR6 {}
#[doc = "For interrupts ID"]
pub mod gicd_igroupr6;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr7](gicd_igroupr7) module"]
pub type GICD_IGROUPR7 = crate::Reg<u32, _GICD_IGROUPR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR7;
#[doc = "`read()` method returns [gicd_igroupr7::R](gicd_igroupr7::R) reader structure"]
impl crate::Readable for GICD_IGROUPR7 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr7::W](gicd_igroupr7::W) writer structure"]
impl crate::Writable for GICD_IGROUPR7 {}
#[doc = "For interrupts ID"]
pub mod gicd_igroupr7;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr8](gicd_igroupr8) module"]
pub type GICD_IGROUPR8 = crate::Reg<u32, _GICD_IGROUPR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IGROUPR8;
#[doc = "`read()` method returns [gicd_igroupr8::R](gicd_igroupr8::R) reader structure"]
impl crate::Readable for GICD_IGROUPR8 {}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr8::W](gicd_igroupr8::W) writer structure"]
impl crate::Writable for GICD_IGROUPR8 {}
#[doc = "For interrupts ID"]
pub mod gicd_igroupr8;
#[doc = "For interrupts ID = 0 to ID = 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isenabler0](gicd_isenabler0) module"]
pub type GICD_ISENABLER0 = crate::Reg<u32, _GICD_ISENABLER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISENABLER0;
#[doc = "`read()` method returns [gicd_isenabler0::R](gicd_isenabler0::R) reader structure"]
impl crate::Readable for GICD_ISENABLER0 {}
#[doc = "`write(|w| ..)` method takes [gicd_isenabler0::W](gicd_isenabler0::W) writer structure"]
impl crate::Writable for GICD_ISENABLER0 {}
#[doc = "For interrupts ID = 0 to ID = 31"]
pub mod gicd_isenabler0;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isenabler1](gicd_isenabler1) module"]
pub type GICD_ISENABLER1 = crate::Reg<u32, _GICD_ISENABLER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISENABLER1;
#[doc = "`read()` method returns [gicd_isenabler1::R](gicd_isenabler1::R) reader structure"]
impl crate::Readable for GICD_ISENABLER1 {}
#[doc = "`write(|w| ..)` method takes [gicd_isenabler1::W](gicd_isenabler1::W) writer structure"]
impl crate::Writable for GICD_ISENABLER1 {}
#[doc = "For interrupts ID"]
pub mod gicd_isenabler1;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isenabler2](gicd_isenabler2) module"]
pub type GICD_ISENABLER2 = crate::Reg<u32, _GICD_ISENABLER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISENABLER2;
#[doc = "`read()` method returns [gicd_isenabler2::R](gicd_isenabler2::R) reader structure"]
impl crate::Readable for GICD_ISENABLER2 {}
#[doc = "`write(|w| ..)` method takes [gicd_isenabler2::W](gicd_isenabler2::W) writer structure"]
impl crate::Writable for GICD_ISENABLER2 {}
#[doc = "For interrupts ID"]
pub mod gicd_isenabler2;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isenabler3](gicd_isenabler3) module"]
pub type GICD_ISENABLER3 = crate::Reg<u32, _GICD_ISENABLER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISENABLER3;
#[doc = "`read()` method returns [gicd_isenabler3::R](gicd_isenabler3::R) reader structure"]
impl crate::Readable for GICD_ISENABLER3 {}
#[doc = "`write(|w| ..)` method takes [gicd_isenabler3::W](gicd_isenabler3::W) writer structure"]
impl crate::Writable for GICD_ISENABLER3 {}
#[doc = "For interrupts ID"]
pub mod gicd_isenabler3;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isenabler4](gicd_isenabler4) module"]
pub type GICD_ISENABLER4 = crate::Reg<u32, _GICD_ISENABLER4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISENABLER4;
#[doc = "`read()` method returns [gicd_isenabler4::R](gicd_isenabler4::R) reader structure"]
impl crate::Readable for GICD_ISENABLER4 {}
#[doc = "`write(|w| ..)` method takes [gicd_isenabler4::W](gicd_isenabler4::W) writer structure"]
impl crate::Writable for GICD_ISENABLER4 {}
#[doc = "For interrupts ID"]
pub mod gicd_isenabler4;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isenabler5](gicd_isenabler5) module"]
pub type GICD_ISENABLER5 = crate::Reg<u32, _GICD_ISENABLER5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISENABLER5;
#[doc = "`read()` method returns [gicd_isenabler5::R](gicd_isenabler5::R) reader structure"]
impl crate::Readable for GICD_ISENABLER5 {}
#[doc = "`write(|w| ..)` method takes [gicd_isenabler5::W](gicd_isenabler5::W) writer structure"]
impl crate::Writable for GICD_ISENABLER5 {}
#[doc = "For interrupts ID"]
pub mod gicd_isenabler5;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isenabler6](gicd_isenabler6) module"]
pub type GICD_ISENABLER6 = crate::Reg<u32, _GICD_ISENABLER6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISENABLER6;
#[doc = "`read()` method returns [gicd_isenabler6::R](gicd_isenabler6::R) reader structure"]
impl crate::Readable for GICD_ISENABLER6 {}
#[doc = "`write(|w| ..)` method takes [gicd_isenabler6::W](gicd_isenabler6::W) writer structure"]
impl crate::Writable for GICD_ISENABLER6 {}
#[doc = "For interrupts ID"]
pub mod gicd_isenabler6;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isenabler7](gicd_isenabler7) module"]
pub type GICD_ISENABLER7 = crate::Reg<u32, _GICD_ISENABLER7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISENABLER7;
#[doc = "`read()` method returns [gicd_isenabler7::R](gicd_isenabler7::R) reader structure"]
impl crate::Readable for GICD_ISENABLER7 {}
#[doc = "`write(|w| ..)` method takes [gicd_isenabler7::W](gicd_isenabler7::W) writer structure"]
impl crate::Writable for GICD_ISENABLER7 {}
#[doc = "For interrupts ID"]
pub mod gicd_isenabler7;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isenabler8](gicd_isenabler8) module"]
pub type GICD_ISENABLER8 = crate::Reg<u32, _GICD_ISENABLER8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISENABLER8;
#[doc = "`read()` method returns [gicd_isenabler8::R](gicd_isenabler8::R) reader structure"]
impl crate::Readable for GICD_ISENABLER8 {}
#[doc = "`write(|w| ..)` method takes [gicd_isenabler8::W](gicd_isenabler8::W) writer structure"]
impl crate::Writable for GICD_ISENABLER8 {}
#[doc = "For interrupts ID"]
pub mod gicd_isenabler8;
#[doc = "For interrupts ID = 0 to ID = 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icenabler0](gicd_icenabler0) module"]
pub type GICD_ICENABLER0 = crate::Reg<u32, _GICD_ICENABLER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICENABLER0;
#[doc = "`read()` method returns [gicd_icenabler0::R](gicd_icenabler0::R) reader structure"]
impl crate::Readable for GICD_ICENABLER0 {}
#[doc = "`write(|w| ..)` method takes [gicd_icenabler0::W](gicd_icenabler0::W) writer structure"]
impl crate::Writable for GICD_ICENABLER0 {}
#[doc = "For interrupts ID = 0 to ID = 31"]
pub mod gicd_icenabler0;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icenabler1](gicd_icenabler1) module"]
pub type GICD_ICENABLER1 = crate::Reg<u32, _GICD_ICENABLER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICENABLER1;
#[doc = "`read()` method returns [gicd_icenabler1::R](gicd_icenabler1::R) reader structure"]
impl crate::Readable for GICD_ICENABLER1 {}
#[doc = "`write(|w| ..)` method takes [gicd_icenabler1::W](gicd_icenabler1::W) writer structure"]
impl crate::Writable for GICD_ICENABLER1 {}
#[doc = "For interrupts ID"]
pub mod gicd_icenabler1;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icenabler2](gicd_icenabler2) module"]
pub type GICD_ICENABLER2 = crate::Reg<u32, _GICD_ICENABLER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICENABLER2;
#[doc = "`read()` method returns [gicd_icenabler2::R](gicd_icenabler2::R) reader structure"]
impl crate::Readable for GICD_ICENABLER2 {}
#[doc = "`write(|w| ..)` method takes [gicd_icenabler2::W](gicd_icenabler2::W) writer structure"]
impl crate::Writable for GICD_ICENABLER2 {}
#[doc = "For interrupts ID"]
pub mod gicd_icenabler2;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icenabler3](gicd_icenabler3) module"]
pub type GICD_ICENABLER3 = crate::Reg<u32, _GICD_ICENABLER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICENABLER3;
#[doc = "`read()` method returns [gicd_icenabler3::R](gicd_icenabler3::R) reader structure"]
impl crate::Readable for GICD_ICENABLER3 {}
#[doc = "`write(|w| ..)` method takes [gicd_icenabler3::W](gicd_icenabler3::W) writer structure"]
impl crate::Writable for GICD_ICENABLER3 {}
#[doc = "For interrupts ID"]
pub mod gicd_icenabler3;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icenabler4](gicd_icenabler4) module"]
pub type GICD_ICENABLER4 = crate::Reg<u32, _GICD_ICENABLER4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICENABLER4;
#[doc = "`read()` method returns [gicd_icenabler4::R](gicd_icenabler4::R) reader structure"]
impl crate::Readable for GICD_ICENABLER4 {}
#[doc = "`write(|w| ..)` method takes [gicd_icenabler4::W](gicd_icenabler4::W) writer structure"]
impl crate::Writable for GICD_ICENABLER4 {}
#[doc = "For interrupts ID"]
pub mod gicd_icenabler4;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icenabler5](gicd_icenabler5) module"]
pub type GICD_ICENABLER5 = crate::Reg<u32, _GICD_ICENABLER5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICENABLER5;
#[doc = "`read()` method returns [gicd_icenabler5::R](gicd_icenabler5::R) reader structure"]
impl crate::Readable for GICD_ICENABLER5 {}
#[doc = "`write(|w| ..)` method takes [gicd_icenabler5::W](gicd_icenabler5::W) writer structure"]
impl crate::Writable for GICD_ICENABLER5 {}
#[doc = "For interrupts ID"]
pub mod gicd_icenabler5;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icenabler6](gicd_icenabler6) module"]
pub type GICD_ICENABLER6 = crate::Reg<u32, _GICD_ICENABLER6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICENABLER6;
#[doc = "`read()` method returns [gicd_icenabler6::R](gicd_icenabler6::R) reader structure"]
impl crate::Readable for GICD_ICENABLER6 {}
#[doc = "`write(|w| ..)` method takes [gicd_icenabler6::W](gicd_icenabler6::W) writer structure"]
impl crate::Writable for GICD_ICENABLER6 {}
#[doc = "For interrupts ID"]
pub mod gicd_icenabler6;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icenabler7](gicd_icenabler7) module"]
pub type GICD_ICENABLER7 = crate::Reg<u32, _GICD_ICENABLER7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICENABLER7;
#[doc = "`read()` method returns [gicd_icenabler7::R](gicd_icenabler7::R) reader structure"]
impl crate::Readable for GICD_ICENABLER7 {}
#[doc = "`write(|w| ..)` method takes [gicd_icenabler7::W](gicd_icenabler7::W) writer structure"]
impl crate::Writable for GICD_ICENABLER7 {}
#[doc = "For interrupts ID"]
pub mod gicd_icenabler7;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icenabler8](gicd_icenabler8) module"]
pub type GICD_ICENABLER8 = crate::Reg<u32, _GICD_ICENABLER8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICENABLER8;
#[doc = "`read()` method returns [gicd_icenabler8::R](gicd_icenabler8::R) reader structure"]
impl crate::Readable for GICD_ICENABLER8 {}
#[doc = "`write(|w| ..)` method takes [gicd_icenabler8::W](gicd_icenabler8::W) writer structure"]
impl crate::Writable for GICD_ICENABLER8 {}
#[doc = "For interrupts ID"]
pub mod gicd_icenabler8;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ispendr0](gicd_ispendr0) module"]
pub type GICD_ISPENDR0 = crate::Reg<u32, _GICD_ISPENDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISPENDR0;
#[doc = "`read()` method returns [gicd_ispendr0::R](gicd_ispendr0::R) reader structure"]
impl crate::Readable for GICD_ISPENDR0 {}
#[doc = "`write(|w| ..)` method takes [gicd_ispendr0::W](gicd_ispendr0::W) writer structure"]
impl crate::Writable for GICD_ISPENDR0 {}
#[doc = "For interrupts ID"]
pub mod gicd_ispendr0;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ispendr1](gicd_ispendr1) module"]
pub type GICD_ISPENDR1 = crate::Reg<u32, _GICD_ISPENDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISPENDR1;
#[doc = "`read()` method returns [gicd_ispendr1::R](gicd_ispendr1::R) reader structure"]
impl crate::Readable for GICD_ISPENDR1 {}
#[doc = "`write(|w| ..)` method takes [gicd_ispendr1::W](gicd_ispendr1::W) writer structure"]
impl crate::Writable for GICD_ISPENDR1 {}
#[doc = "For interrupts ID"]
pub mod gicd_ispendr1;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ispendr2](gicd_ispendr2) module"]
pub type GICD_ISPENDR2 = crate::Reg<u32, _GICD_ISPENDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISPENDR2;
#[doc = "`read()` method returns [gicd_ispendr2::R](gicd_ispendr2::R) reader structure"]
impl crate::Readable for GICD_ISPENDR2 {}
#[doc = "`write(|w| ..)` method takes [gicd_ispendr2::W](gicd_ispendr2::W) writer structure"]
impl crate::Writable for GICD_ISPENDR2 {}
#[doc = "For interrupts ID"]
pub mod gicd_ispendr2;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ispendr3](gicd_ispendr3) module"]
pub type GICD_ISPENDR3 = crate::Reg<u32, _GICD_ISPENDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISPENDR3;
#[doc = "`read()` method returns [gicd_ispendr3::R](gicd_ispendr3::R) reader structure"]
impl crate::Readable for GICD_ISPENDR3 {}
#[doc = "`write(|w| ..)` method takes [gicd_ispendr3::W](gicd_ispendr3::W) writer structure"]
impl crate::Writable for GICD_ISPENDR3 {}
#[doc = "For interrupts ID"]
pub mod gicd_ispendr3;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ispendr4](gicd_ispendr4) module"]
pub type GICD_ISPENDR4 = crate::Reg<u32, _GICD_ISPENDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISPENDR4;
#[doc = "`read()` method returns [gicd_ispendr4::R](gicd_ispendr4::R) reader structure"]
impl crate::Readable for GICD_ISPENDR4 {}
#[doc = "`write(|w| ..)` method takes [gicd_ispendr4::W](gicd_ispendr4::W) writer structure"]
impl crate::Writable for GICD_ISPENDR4 {}
#[doc = "For interrupts ID"]
pub mod gicd_ispendr4;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ispendr5](gicd_ispendr5) module"]
pub type GICD_ISPENDR5 = crate::Reg<u32, _GICD_ISPENDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISPENDR5;
#[doc = "`read()` method returns [gicd_ispendr5::R](gicd_ispendr5::R) reader structure"]
impl crate::Readable for GICD_ISPENDR5 {}
#[doc = "`write(|w| ..)` method takes [gicd_ispendr5::W](gicd_ispendr5::W) writer structure"]
impl crate::Writable for GICD_ISPENDR5 {}
#[doc = "For interrupts ID"]
pub mod gicd_ispendr5;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ispendr6](gicd_ispendr6) module"]
pub type GICD_ISPENDR6 = crate::Reg<u32, _GICD_ISPENDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISPENDR6;
#[doc = "`read()` method returns [gicd_ispendr6::R](gicd_ispendr6::R) reader structure"]
impl crate::Readable for GICD_ISPENDR6 {}
#[doc = "`write(|w| ..)` method takes [gicd_ispendr6::W](gicd_ispendr6::W) writer structure"]
impl crate::Writable for GICD_ISPENDR6 {}
#[doc = "For interrupts ID"]
pub mod gicd_ispendr6;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ispendr7](gicd_ispendr7) module"]
pub type GICD_ISPENDR7 = crate::Reg<u32, _GICD_ISPENDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISPENDR7;
#[doc = "`read()` method returns [gicd_ispendr7::R](gicd_ispendr7::R) reader structure"]
impl crate::Readable for GICD_ISPENDR7 {}
#[doc = "`write(|w| ..)` method takes [gicd_ispendr7::W](gicd_ispendr7::W) writer structure"]
impl crate::Writable for GICD_ISPENDR7 {}
#[doc = "For interrupts ID"]
pub mod gicd_ispendr7;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ispendr8](gicd_ispendr8) module"]
pub type GICD_ISPENDR8 = crate::Reg<u32, _GICD_ISPENDR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISPENDR8;
#[doc = "`read()` method returns [gicd_ispendr8::R](gicd_ispendr8::R) reader structure"]
impl crate::Readable for GICD_ISPENDR8 {}
#[doc = "`write(|w| ..)` method takes [gicd_ispendr8::W](gicd_ispendr8::W) writer structure"]
impl crate::Writable for GICD_ISPENDR8 {}
#[doc = "For interrupts ID"]
pub mod gicd_ispendr8;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icpendr0](gicd_icpendr0) module"]
pub type GICD_ICPENDR0 = crate::Reg<u32, _GICD_ICPENDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICPENDR0;
#[doc = "`read()` method returns [gicd_icpendr0::R](gicd_icpendr0::R) reader structure"]
impl crate::Readable for GICD_ICPENDR0 {}
#[doc = "`write(|w| ..)` method takes [gicd_icpendr0::W](gicd_icpendr0::W) writer structure"]
impl crate::Writable for GICD_ICPENDR0 {}
#[doc = "For interrupts ID"]
pub mod gicd_icpendr0;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icpendr1](gicd_icpendr1) module"]
pub type GICD_ICPENDR1 = crate::Reg<u32, _GICD_ICPENDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICPENDR1;
#[doc = "`read()` method returns [gicd_icpendr1::R](gicd_icpendr1::R) reader structure"]
impl crate::Readable for GICD_ICPENDR1 {}
#[doc = "`write(|w| ..)` method takes [gicd_icpendr1::W](gicd_icpendr1::W) writer structure"]
impl crate::Writable for GICD_ICPENDR1 {}
#[doc = "For interrupts ID"]
pub mod gicd_icpendr1;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icpendr2](gicd_icpendr2) module"]
pub type GICD_ICPENDR2 = crate::Reg<u32, _GICD_ICPENDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICPENDR2;
#[doc = "`read()` method returns [gicd_icpendr2::R](gicd_icpendr2::R) reader structure"]
impl crate::Readable for GICD_ICPENDR2 {}
#[doc = "`write(|w| ..)` method takes [gicd_icpendr2::W](gicd_icpendr2::W) writer structure"]
impl crate::Writable for GICD_ICPENDR2 {}
#[doc = "For interrupts ID"]
pub mod gicd_icpendr2;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icpendr3](gicd_icpendr3) module"]
pub type GICD_ICPENDR3 = crate::Reg<u32, _GICD_ICPENDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICPENDR3;
#[doc = "`read()` method returns [gicd_icpendr3::R](gicd_icpendr3::R) reader structure"]
impl crate::Readable for GICD_ICPENDR3 {}
#[doc = "`write(|w| ..)` method takes [gicd_icpendr3::W](gicd_icpendr3::W) writer structure"]
impl crate::Writable for GICD_ICPENDR3 {}
#[doc = "For interrupts ID"]
pub mod gicd_icpendr3;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icpendr4](gicd_icpendr4) module"]
pub type GICD_ICPENDR4 = crate::Reg<u32, _GICD_ICPENDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICPENDR4;
#[doc = "`read()` method returns [gicd_icpendr4::R](gicd_icpendr4::R) reader structure"]
impl crate::Readable for GICD_ICPENDR4 {}
#[doc = "`write(|w| ..)` method takes [gicd_icpendr4::W](gicd_icpendr4::W) writer structure"]
impl crate::Writable for GICD_ICPENDR4 {}
#[doc = "For interrupts ID"]
pub mod gicd_icpendr4;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icpendr5](gicd_icpendr5) module"]
pub type GICD_ICPENDR5 = crate::Reg<u32, _GICD_ICPENDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICPENDR5;
#[doc = "`read()` method returns [gicd_icpendr5::R](gicd_icpendr5::R) reader structure"]
impl crate::Readable for GICD_ICPENDR5 {}
#[doc = "`write(|w| ..)` method takes [gicd_icpendr5::W](gicd_icpendr5::W) writer structure"]
impl crate::Writable for GICD_ICPENDR5 {}
#[doc = "For interrupts ID"]
pub mod gicd_icpendr5;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icpendr6](gicd_icpendr6) module"]
pub type GICD_ICPENDR6 = crate::Reg<u32, _GICD_ICPENDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICPENDR6;
#[doc = "`read()` method returns [gicd_icpendr6::R](gicd_icpendr6::R) reader structure"]
impl crate::Readable for GICD_ICPENDR6 {}
#[doc = "`write(|w| ..)` method takes [gicd_icpendr6::W](gicd_icpendr6::W) writer structure"]
impl crate::Writable for GICD_ICPENDR6 {}
#[doc = "For interrupts ID"]
pub mod gicd_icpendr6;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icpendr7](gicd_icpendr7) module"]
pub type GICD_ICPENDR7 = crate::Reg<u32, _GICD_ICPENDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICPENDR7;
#[doc = "`read()` method returns [gicd_icpendr7::R](gicd_icpendr7::R) reader structure"]
impl crate::Readable for GICD_ICPENDR7 {}
#[doc = "`write(|w| ..)` method takes [gicd_icpendr7::W](gicd_icpendr7::W) writer structure"]
impl crate::Writable for GICD_ICPENDR7 {}
#[doc = "For interrupts ID"]
pub mod gicd_icpendr7;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icpendr8](gicd_icpendr8) module"]
pub type GICD_ICPENDR8 = crate::Reg<u32, _GICD_ICPENDR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICPENDR8;
#[doc = "`read()` method returns [gicd_icpendr8::R](gicd_icpendr8::R) reader structure"]
impl crate::Readable for GICD_ICPENDR8 {}
#[doc = "`write(|w| ..)` method takes [gicd_icpendr8::W](gicd_icpendr8::W) writer structure"]
impl crate::Writable for GICD_ICPENDR8 {}
#[doc = "For interrupts ID"]
pub mod gicd_icpendr8;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isactiver0](gicd_isactiver0) module"]
pub type GICD_ISACTIVER0 = crate::Reg<u32, _GICD_ISACTIVER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISACTIVER0;
#[doc = "`read()` method returns [gicd_isactiver0::R](gicd_isactiver0::R) reader structure"]
impl crate::Readable for GICD_ISACTIVER0 {}
#[doc = "`write(|w| ..)` method takes [gicd_isactiver0::W](gicd_isactiver0::W) writer structure"]
impl crate::Writable for GICD_ISACTIVER0 {}
#[doc = "For interrupts ID"]
pub mod gicd_isactiver0;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isactiver1](gicd_isactiver1) module"]
pub type GICD_ISACTIVER1 = crate::Reg<u32, _GICD_ISACTIVER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISACTIVER1;
#[doc = "`read()` method returns [gicd_isactiver1::R](gicd_isactiver1::R) reader structure"]
impl crate::Readable for GICD_ISACTIVER1 {}
#[doc = "`write(|w| ..)` method takes [gicd_isactiver1::W](gicd_isactiver1::W) writer structure"]
impl crate::Writable for GICD_ISACTIVER1 {}
#[doc = "For interrupts ID"]
pub mod gicd_isactiver1;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isactiver2](gicd_isactiver2) module"]
pub type GICD_ISACTIVER2 = crate::Reg<u32, _GICD_ISACTIVER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISACTIVER2;
#[doc = "`read()` method returns [gicd_isactiver2::R](gicd_isactiver2::R) reader structure"]
impl crate::Readable for GICD_ISACTIVER2 {}
#[doc = "`write(|w| ..)` method takes [gicd_isactiver2::W](gicd_isactiver2::W) writer structure"]
impl crate::Writable for GICD_ISACTIVER2 {}
#[doc = "For interrupts ID"]
pub mod gicd_isactiver2;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isactiver3](gicd_isactiver3) module"]
pub type GICD_ISACTIVER3 = crate::Reg<u32, _GICD_ISACTIVER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISACTIVER3;
#[doc = "`read()` method returns [gicd_isactiver3::R](gicd_isactiver3::R) reader structure"]
impl crate::Readable for GICD_ISACTIVER3 {}
#[doc = "`write(|w| ..)` method takes [gicd_isactiver3::W](gicd_isactiver3::W) writer structure"]
impl crate::Writable for GICD_ISACTIVER3 {}
#[doc = "For interrupts ID"]
pub mod gicd_isactiver3;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isactiver4](gicd_isactiver4) module"]
pub type GICD_ISACTIVER4 = crate::Reg<u32, _GICD_ISACTIVER4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISACTIVER4;
#[doc = "`read()` method returns [gicd_isactiver4::R](gicd_isactiver4::R) reader structure"]
impl crate::Readable for GICD_ISACTIVER4 {}
#[doc = "`write(|w| ..)` method takes [gicd_isactiver4::W](gicd_isactiver4::W) writer structure"]
impl crate::Writable for GICD_ISACTIVER4 {}
#[doc = "For interrupts ID"]
pub mod gicd_isactiver4;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isactiver5](gicd_isactiver5) module"]
pub type GICD_ISACTIVER5 = crate::Reg<u32, _GICD_ISACTIVER5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISACTIVER5;
#[doc = "`read()` method returns [gicd_isactiver5::R](gicd_isactiver5::R) reader structure"]
impl crate::Readable for GICD_ISACTIVER5 {}
#[doc = "`write(|w| ..)` method takes [gicd_isactiver5::W](gicd_isactiver5::W) writer structure"]
impl crate::Writable for GICD_ISACTIVER5 {}
#[doc = "For interrupts ID"]
pub mod gicd_isactiver5;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isactiver6](gicd_isactiver6) module"]
pub type GICD_ISACTIVER6 = crate::Reg<u32, _GICD_ISACTIVER6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISACTIVER6;
#[doc = "`read()` method returns [gicd_isactiver6::R](gicd_isactiver6::R) reader structure"]
impl crate::Readable for GICD_ISACTIVER6 {}
#[doc = "`write(|w| ..)` method takes [gicd_isactiver6::W](gicd_isactiver6::W) writer structure"]
impl crate::Writable for GICD_ISACTIVER6 {}
#[doc = "For interrupts ID"]
pub mod gicd_isactiver6;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isactiver7](gicd_isactiver7) module"]
pub type GICD_ISACTIVER7 = crate::Reg<u32, _GICD_ISACTIVER7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISACTIVER7;
#[doc = "`read()` method returns [gicd_isactiver7::R](gicd_isactiver7::R) reader structure"]
impl crate::Readable for GICD_ISACTIVER7 {}
#[doc = "`write(|w| ..)` method takes [gicd_isactiver7::W](gicd_isactiver7::W) writer structure"]
impl crate::Writable for GICD_ISACTIVER7 {}
#[doc = "For interrupts ID"]
pub mod gicd_isactiver7;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isactiver8](gicd_isactiver8) module"]
pub type GICD_ISACTIVER8 = crate::Reg<u32, _GICD_ISACTIVER8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ISACTIVER8;
#[doc = "`read()` method returns [gicd_isactiver8::R](gicd_isactiver8::R) reader structure"]
impl crate::Readable for GICD_ISACTIVER8 {}
#[doc = "`write(|w| ..)` method takes [gicd_isactiver8::W](gicd_isactiver8::W) writer structure"]
impl crate::Writable for GICD_ISACTIVER8 {}
#[doc = "For interrupts ID"]
pub mod gicd_isactiver8;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icactiver0](gicd_icactiver0) module"]
pub type GICD_ICACTIVER0 = crate::Reg<u32, _GICD_ICACTIVER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICACTIVER0;
#[doc = "`read()` method returns [gicd_icactiver0::R](gicd_icactiver0::R) reader structure"]
impl crate::Readable for GICD_ICACTIVER0 {}
#[doc = "`write(|w| ..)` method takes [gicd_icactiver0::W](gicd_icactiver0::W) writer structure"]
impl crate::Writable for GICD_ICACTIVER0 {}
#[doc = "For interrupts ID"]
pub mod gicd_icactiver0;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icactiver1](gicd_icactiver1) module"]
pub type GICD_ICACTIVER1 = crate::Reg<u32, _GICD_ICACTIVER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICACTIVER1;
#[doc = "`read()` method returns [gicd_icactiver1::R](gicd_icactiver1::R) reader structure"]
impl crate::Readable for GICD_ICACTIVER1 {}
#[doc = "`write(|w| ..)` method takes [gicd_icactiver1::W](gicd_icactiver1::W) writer structure"]
impl crate::Writable for GICD_ICACTIVER1 {}
#[doc = "For interrupts ID"]
pub mod gicd_icactiver1;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icactiver2](gicd_icactiver2) module"]
pub type GICD_ICACTIVER2 = crate::Reg<u32, _GICD_ICACTIVER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICACTIVER2;
#[doc = "`read()` method returns [gicd_icactiver2::R](gicd_icactiver2::R) reader structure"]
impl crate::Readable for GICD_ICACTIVER2 {}
#[doc = "`write(|w| ..)` method takes [gicd_icactiver2::W](gicd_icactiver2::W) writer structure"]
impl crate::Writable for GICD_ICACTIVER2 {}
#[doc = "For interrupts ID"]
pub mod gicd_icactiver2;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icactiver3](gicd_icactiver3) module"]
pub type GICD_ICACTIVER3 = crate::Reg<u32, _GICD_ICACTIVER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICACTIVER3;
#[doc = "`read()` method returns [gicd_icactiver3::R](gicd_icactiver3::R) reader structure"]
impl crate::Readable for GICD_ICACTIVER3 {}
#[doc = "`write(|w| ..)` method takes [gicd_icactiver3::W](gicd_icactiver3::W) writer structure"]
impl crate::Writable for GICD_ICACTIVER3 {}
#[doc = "For interrupts ID"]
pub mod gicd_icactiver3;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icactiver4](gicd_icactiver4) module"]
pub type GICD_ICACTIVER4 = crate::Reg<u32, _GICD_ICACTIVER4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICACTIVER4;
#[doc = "`read()` method returns [gicd_icactiver4::R](gicd_icactiver4::R) reader structure"]
impl crate::Readable for GICD_ICACTIVER4 {}
#[doc = "`write(|w| ..)` method takes [gicd_icactiver4::W](gicd_icactiver4::W) writer structure"]
impl crate::Writable for GICD_ICACTIVER4 {}
#[doc = "For interrupts ID"]
pub mod gicd_icactiver4;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icactiver5](gicd_icactiver5) module"]
pub type GICD_ICACTIVER5 = crate::Reg<u32, _GICD_ICACTIVER5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICACTIVER5;
#[doc = "`read()` method returns [gicd_icactiver5::R](gicd_icactiver5::R) reader structure"]
impl crate::Readable for GICD_ICACTIVER5 {}
#[doc = "`write(|w| ..)` method takes [gicd_icactiver5::W](gicd_icactiver5::W) writer structure"]
impl crate::Writable for GICD_ICACTIVER5 {}
#[doc = "For interrupts ID"]
pub mod gicd_icactiver5;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icactiver6](gicd_icactiver6) module"]
pub type GICD_ICACTIVER6 = crate::Reg<u32, _GICD_ICACTIVER6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICACTIVER6;
#[doc = "`read()` method returns [gicd_icactiver6::R](gicd_icactiver6::R) reader structure"]
impl crate::Readable for GICD_ICACTIVER6 {}
#[doc = "`write(|w| ..)` method takes [gicd_icactiver6::W](gicd_icactiver6::W) writer structure"]
impl crate::Writable for GICD_ICACTIVER6 {}
#[doc = "For interrupts ID"]
pub mod gicd_icactiver6;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icactiver7](gicd_icactiver7) module"]
pub type GICD_ICACTIVER7 = crate::Reg<u32, _GICD_ICACTIVER7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICACTIVER7;
#[doc = "`read()` method returns [gicd_icactiver7::R](gicd_icactiver7::R) reader structure"]
impl crate::Readable for GICD_ICACTIVER7 {}
#[doc = "`write(|w| ..)` method takes [gicd_icactiver7::W](gicd_icactiver7::W) writer structure"]
impl crate::Writable for GICD_ICACTIVER7 {}
#[doc = "For interrupts ID"]
pub mod gicd_icactiver7;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icactiver8](gicd_icactiver8) module"]
pub type GICD_ICACTIVER8 = crate::Reg<u32, _GICD_ICACTIVER8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICACTIVER8;
#[doc = "`read()` method returns [gicd_icactiver8::R](gicd_icactiver8::R) reader structure"]
impl crate::Readable for GICD_ICACTIVER8 {}
#[doc = "`write(|w| ..)` method takes [gicd_icactiver8::W](gicd_icactiver8::W) writer structure"]
impl crate::Writable for GICD_ICACTIVER8 {}
#[doc = "For interrupts ID"]
pub mod gicd_icactiver8;
#[doc = "GICD interrupt priority register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr0](gicd_ipriorityr0) module"]
pub type GICD_IPRIORITYR0 = crate::Reg<u32, _GICD_IPRIORITYR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR0;
#[doc = "`read()` method returns [gicd_ipriorityr0::R](gicd_ipriorityr0::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR0 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr0::W](gicd_ipriorityr0::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR0 {}
#[doc = "GICD interrupt priority register 0"]
pub mod gicd_ipriorityr0;
#[doc = "GICD interrupt priority register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr1](gicd_ipriorityr1) module"]
pub type GICD_IPRIORITYR1 = crate::Reg<u32, _GICD_IPRIORITYR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR1;
#[doc = "`read()` method returns [gicd_ipriorityr1::R](gicd_ipriorityr1::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR1 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr1::W](gicd_ipriorityr1::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR1 {}
#[doc = "GICD interrupt priority register 1"]
pub mod gicd_ipriorityr1;
#[doc = "GICD interrupt priority register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr2](gicd_ipriorityr2) module"]
pub type GICD_IPRIORITYR2 = crate::Reg<u32, _GICD_IPRIORITYR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR2;
#[doc = "`read()` method returns [gicd_ipriorityr2::R](gicd_ipriorityr2::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR2 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr2::W](gicd_ipriorityr2::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR2 {}
#[doc = "GICD interrupt priority register 2"]
pub mod gicd_ipriorityr2;
#[doc = "GICD interrupt priority register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr3](gicd_ipriorityr3) module"]
pub type GICD_IPRIORITYR3 = crate::Reg<u32, _GICD_IPRIORITYR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR3;
#[doc = "`read()` method returns [gicd_ipriorityr3::R](gicd_ipriorityr3::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR3 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr3::W](gicd_ipriorityr3::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR3 {}
#[doc = "GICD interrupt priority register 3"]
pub mod gicd_ipriorityr3;
#[doc = "GICD interrupt priority register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr4](gicd_ipriorityr4) module"]
pub type GICD_IPRIORITYR4 = crate::Reg<u32, _GICD_IPRIORITYR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR4;
#[doc = "`read()` method returns [gicd_ipriorityr4::R](gicd_ipriorityr4::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR4 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr4::W](gicd_ipriorityr4::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR4 {}
#[doc = "GICD interrupt priority register 4"]
pub mod gicd_ipriorityr4;
#[doc = "GICD interrupt priority register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr5](gicd_ipriorityr5) module"]
pub type GICD_IPRIORITYR5 = crate::Reg<u32, _GICD_IPRIORITYR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR5;
#[doc = "`read()` method returns [gicd_ipriorityr5::R](gicd_ipriorityr5::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR5 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr5::W](gicd_ipriorityr5::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR5 {}
#[doc = "GICD interrupt priority register 5"]
pub mod gicd_ipriorityr5;
#[doc = "GICD interrupt priority register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr6](gicd_ipriorityr6) module"]
pub type GICD_IPRIORITYR6 = crate::Reg<u32, _GICD_IPRIORITYR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR6;
#[doc = "`read()` method returns [gicd_ipriorityr6::R](gicd_ipriorityr6::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR6 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr6::W](gicd_ipriorityr6::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR6 {}
#[doc = "GICD interrupt priority register 6"]
pub mod gicd_ipriorityr6;
#[doc = "GICD interrupt priority register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr7](gicd_ipriorityr7) module"]
pub type GICD_IPRIORITYR7 = crate::Reg<u32, _GICD_IPRIORITYR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR7;
#[doc = "`read()` method returns [gicd_ipriorityr7::R](gicd_ipriorityr7::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR7 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr7::W](gicd_ipriorityr7::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR7 {}
#[doc = "GICD interrupt priority register 7"]
pub mod gicd_ipriorityr7;
#[doc = "GICD interrupt priority register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr8](gicd_ipriorityr8) module"]
pub type GICD_IPRIORITYR8 = crate::Reg<u32, _GICD_IPRIORITYR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR8;
#[doc = "`read()` method returns [gicd_ipriorityr8::R](gicd_ipriorityr8::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR8 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr8::W](gicd_ipriorityr8::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR8 {}
#[doc = "GICD interrupt priority register 8"]
pub mod gicd_ipriorityr8;
#[doc = "GICD interrupt priority register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr9](gicd_ipriorityr9) module"]
pub type GICD_IPRIORITYR9 = crate::Reg<u32, _GICD_IPRIORITYR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR9;
#[doc = "`read()` method returns [gicd_ipriorityr9::R](gicd_ipriorityr9::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR9 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr9::W](gicd_ipriorityr9::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR9 {}
#[doc = "GICD interrupt priority register 9"]
pub mod gicd_ipriorityr9;
#[doc = "GICD interrupt priority register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr10](gicd_ipriorityr10) module"]
pub type GICD_IPRIORITYR10 = crate::Reg<u32, _GICD_IPRIORITYR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR10;
#[doc = "`read()` method returns [gicd_ipriorityr10::R](gicd_ipriorityr10::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR10 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr10::W](gicd_ipriorityr10::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR10 {}
#[doc = "GICD interrupt priority register 10"]
pub mod gicd_ipriorityr10;
#[doc = "GICD interrupt priority register 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr11](gicd_ipriorityr11) module"]
pub type GICD_IPRIORITYR11 = crate::Reg<u32, _GICD_IPRIORITYR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR11;
#[doc = "`read()` method returns [gicd_ipriorityr11::R](gicd_ipriorityr11::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR11 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr11::W](gicd_ipriorityr11::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR11 {}
#[doc = "GICD interrupt priority register 11"]
pub mod gicd_ipriorityr11;
#[doc = "GICD interrupt priority register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr12](gicd_ipriorityr12) module"]
pub type GICD_IPRIORITYR12 = crate::Reg<u32, _GICD_IPRIORITYR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR12;
#[doc = "`read()` method returns [gicd_ipriorityr12::R](gicd_ipriorityr12::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR12 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr12::W](gicd_ipriorityr12::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR12 {}
#[doc = "GICD interrupt priority register 12"]
pub mod gicd_ipriorityr12;
#[doc = "GICD interrupt priority register 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr13](gicd_ipriorityr13) module"]
pub type GICD_IPRIORITYR13 = crate::Reg<u32, _GICD_IPRIORITYR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR13;
#[doc = "`read()` method returns [gicd_ipriorityr13::R](gicd_ipriorityr13::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR13 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr13::W](gicd_ipriorityr13::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR13 {}
#[doc = "GICD interrupt priority register 13"]
pub mod gicd_ipriorityr13;
#[doc = "GICD interrupt priority register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr14](gicd_ipriorityr14) module"]
pub type GICD_IPRIORITYR14 = crate::Reg<u32, _GICD_IPRIORITYR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR14;
#[doc = "`read()` method returns [gicd_ipriorityr14::R](gicd_ipriorityr14::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR14 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr14::W](gicd_ipriorityr14::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR14 {}
#[doc = "GICD interrupt priority register 14"]
pub mod gicd_ipriorityr14;
#[doc = "GICD interrupt priority register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr15](gicd_ipriorityr15) module"]
pub type GICD_IPRIORITYR15 = crate::Reg<u32, _GICD_IPRIORITYR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR15;
#[doc = "`read()` method returns [gicd_ipriorityr15::R](gicd_ipriorityr15::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR15 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr15::W](gicd_ipriorityr15::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR15 {}
#[doc = "GICD interrupt priority register 15"]
pub mod gicd_ipriorityr15;
#[doc = "GICD interrupt priority register 16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr16](gicd_ipriorityr16) module"]
pub type GICD_IPRIORITYR16 = crate::Reg<u32, _GICD_IPRIORITYR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR16;
#[doc = "`read()` method returns [gicd_ipriorityr16::R](gicd_ipriorityr16::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR16 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr16::W](gicd_ipriorityr16::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR16 {}
#[doc = "GICD interrupt priority register 16"]
pub mod gicd_ipriorityr16;
#[doc = "GICD interrupt priority register 17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr17](gicd_ipriorityr17) module"]
pub type GICD_IPRIORITYR17 = crate::Reg<u32, _GICD_IPRIORITYR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR17;
#[doc = "`read()` method returns [gicd_ipriorityr17::R](gicd_ipriorityr17::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR17 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr17::W](gicd_ipriorityr17::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR17 {}
#[doc = "GICD interrupt priority register 17"]
pub mod gicd_ipriorityr17;
#[doc = "GICD interrupt priority register 18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr18](gicd_ipriorityr18) module"]
pub type GICD_IPRIORITYR18 = crate::Reg<u32, _GICD_IPRIORITYR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR18;
#[doc = "`read()` method returns [gicd_ipriorityr18::R](gicd_ipriorityr18::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR18 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr18::W](gicd_ipriorityr18::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR18 {}
#[doc = "GICD interrupt priority register 18"]
pub mod gicd_ipriorityr18;
#[doc = "GICD interrupt priority register 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr19](gicd_ipriorityr19) module"]
pub type GICD_IPRIORITYR19 = crate::Reg<u32, _GICD_IPRIORITYR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR19;
#[doc = "`read()` method returns [gicd_ipriorityr19::R](gicd_ipriorityr19::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR19 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr19::W](gicd_ipriorityr19::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR19 {}
#[doc = "GICD interrupt priority register 19"]
pub mod gicd_ipriorityr19;
#[doc = "GICD interrupt priority register 20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr20](gicd_ipriorityr20) module"]
pub type GICD_IPRIORITYR20 = crate::Reg<u32, _GICD_IPRIORITYR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR20;
#[doc = "`read()` method returns [gicd_ipriorityr20::R](gicd_ipriorityr20::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR20 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr20::W](gicd_ipriorityr20::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR20 {}
#[doc = "GICD interrupt priority register 20"]
pub mod gicd_ipriorityr20;
#[doc = "GICD interrupt priority register 21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr21](gicd_ipriorityr21) module"]
pub type GICD_IPRIORITYR21 = crate::Reg<u32, _GICD_IPRIORITYR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR21;
#[doc = "`read()` method returns [gicd_ipriorityr21::R](gicd_ipriorityr21::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR21 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr21::W](gicd_ipriorityr21::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR21 {}
#[doc = "GICD interrupt priority register 21"]
pub mod gicd_ipriorityr21;
#[doc = "GICD interrupt priority register 22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr22](gicd_ipriorityr22) module"]
pub type GICD_IPRIORITYR22 = crate::Reg<u32, _GICD_IPRIORITYR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR22;
#[doc = "`read()` method returns [gicd_ipriorityr22::R](gicd_ipriorityr22::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR22 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr22::W](gicd_ipriorityr22::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR22 {}
#[doc = "GICD interrupt priority register 22"]
pub mod gicd_ipriorityr22;
#[doc = "GICD interrupt priority register 23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr23](gicd_ipriorityr23) module"]
pub type GICD_IPRIORITYR23 = crate::Reg<u32, _GICD_IPRIORITYR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR23;
#[doc = "`read()` method returns [gicd_ipriorityr23::R](gicd_ipriorityr23::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR23 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr23::W](gicd_ipriorityr23::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR23 {}
#[doc = "GICD interrupt priority register 23"]
pub mod gicd_ipriorityr23;
#[doc = "GICD interrupt priority register 24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr24](gicd_ipriorityr24) module"]
pub type GICD_IPRIORITYR24 = crate::Reg<u32, _GICD_IPRIORITYR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR24;
#[doc = "`read()` method returns [gicd_ipriorityr24::R](gicd_ipriorityr24::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR24 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr24::W](gicd_ipriorityr24::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR24 {}
#[doc = "GICD interrupt priority register 24"]
pub mod gicd_ipriorityr24;
#[doc = "GICD interrupt priority register 25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr25](gicd_ipriorityr25) module"]
pub type GICD_IPRIORITYR25 = crate::Reg<u32, _GICD_IPRIORITYR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR25;
#[doc = "`read()` method returns [gicd_ipriorityr25::R](gicd_ipriorityr25::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR25 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr25::W](gicd_ipriorityr25::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR25 {}
#[doc = "GICD interrupt priority register 25"]
pub mod gicd_ipriorityr25;
#[doc = "GICD interrupt priority register 26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr26](gicd_ipriorityr26) module"]
pub type GICD_IPRIORITYR26 = crate::Reg<u32, _GICD_IPRIORITYR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR26;
#[doc = "`read()` method returns [gicd_ipriorityr26::R](gicd_ipriorityr26::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR26 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr26::W](gicd_ipriorityr26::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR26 {}
#[doc = "GICD interrupt priority register 26"]
pub mod gicd_ipriorityr26;
#[doc = "GICD interrupt priority register 27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr27](gicd_ipriorityr27) module"]
pub type GICD_IPRIORITYR27 = crate::Reg<u32, _GICD_IPRIORITYR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR27;
#[doc = "`read()` method returns [gicd_ipriorityr27::R](gicd_ipriorityr27::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR27 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr27::W](gicd_ipriorityr27::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR27 {}
#[doc = "GICD interrupt priority register 27"]
pub mod gicd_ipriorityr27;
#[doc = "GICD interrupt priority register 28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr28](gicd_ipriorityr28) module"]
pub type GICD_IPRIORITYR28 = crate::Reg<u32, _GICD_IPRIORITYR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR28;
#[doc = "`read()` method returns [gicd_ipriorityr28::R](gicd_ipriorityr28::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR28 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr28::W](gicd_ipriorityr28::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR28 {}
#[doc = "GICD interrupt priority register 28"]
pub mod gicd_ipriorityr28;
#[doc = "GICD interrupt priority register 29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr29](gicd_ipriorityr29) module"]
pub type GICD_IPRIORITYR29 = crate::Reg<u32, _GICD_IPRIORITYR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR29;
#[doc = "`read()` method returns [gicd_ipriorityr29::R](gicd_ipriorityr29::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR29 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr29::W](gicd_ipriorityr29::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR29 {}
#[doc = "GICD interrupt priority register 29"]
pub mod gicd_ipriorityr29;
#[doc = "GICD interrupt priority register 30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr30](gicd_ipriorityr30) module"]
pub type GICD_IPRIORITYR30 = crate::Reg<u32, _GICD_IPRIORITYR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR30;
#[doc = "`read()` method returns [gicd_ipriorityr30::R](gicd_ipriorityr30::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR30 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr30::W](gicd_ipriorityr30::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR30 {}
#[doc = "GICD interrupt priority register 30"]
pub mod gicd_ipriorityr30;
#[doc = "GICD interrupt priority register 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr31](gicd_ipriorityr31) module"]
pub type GICD_IPRIORITYR31 = crate::Reg<u32, _GICD_IPRIORITYR31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR31;
#[doc = "`read()` method returns [gicd_ipriorityr31::R](gicd_ipriorityr31::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR31 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr31::W](gicd_ipriorityr31::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR31 {}
#[doc = "GICD interrupt priority register 31"]
pub mod gicd_ipriorityr31;
#[doc = "GICD interrupt priority register 32\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr32](gicd_ipriorityr32) module"]
pub type GICD_IPRIORITYR32 = crate::Reg<u32, _GICD_IPRIORITYR32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR32;
#[doc = "`read()` method returns [gicd_ipriorityr32::R](gicd_ipriorityr32::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR32 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr32::W](gicd_ipriorityr32::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR32 {}
#[doc = "GICD interrupt priority register 32"]
pub mod gicd_ipriorityr32;
#[doc = "GICD interrupt priority register 33\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr33](gicd_ipriorityr33) module"]
pub type GICD_IPRIORITYR33 = crate::Reg<u32, _GICD_IPRIORITYR33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR33;
#[doc = "`read()` method returns [gicd_ipriorityr33::R](gicd_ipriorityr33::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR33 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr33::W](gicd_ipriorityr33::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR33 {}
#[doc = "GICD interrupt priority register 33"]
pub mod gicd_ipriorityr33;
#[doc = "GICD interrupt priority register 34\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr34](gicd_ipriorityr34) module"]
pub type GICD_IPRIORITYR34 = crate::Reg<u32, _GICD_IPRIORITYR34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR34;
#[doc = "`read()` method returns [gicd_ipriorityr34::R](gicd_ipriorityr34::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR34 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr34::W](gicd_ipriorityr34::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR34 {}
#[doc = "GICD interrupt priority register 34"]
pub mod gicd_ipriorityr34;
#[doc = "GICD interrupt priority register 35\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr35](gicd_ipriorityr35) module"]
pub type GICD_IPRIORITYR35 = crate::Reg<u32, _GICD_IPRIORITYR35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR35;
#[doc = "`read()` method returns [gicd_ipriorityr35::R](gicd_ipriorityr35::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR35 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr35::W](gicd_ipriorityr35::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR35 {}
#[doc = "GICD interrupt priority register 35"]
pub mod gicd_ipriorityr35;
#[doc = "GICD interrupt priority register 36\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr36](gicd_ipriorityr36) module"]
pub type GICD_IPRIORITYR36 = crate::Reg<u32, _GICD_IPRIORITYR36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR36;
#[doc = "`read()` method returns [gicd_ipriorityr36::R](gicd_ipriorityr36::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR36 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr36::W](gicd_ipriorityr36::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR36 {}
#[doc = "GICD interrupt priority register 36"]
pub mod gicd_ipriorityr36;
#[doc = "GICD interrupt priority register 37\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr37](gicd_ipriorityr37) module"]
pub type GICD_IPRIORITYR37 = crate::Reg<u32, _GICD_IPRIORITYR37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR37;
#[doc = "`read()` method returns [gicd_ipriorityr37::R](gicd_ipriorityr37::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR37 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr37::W](gicd_ipriorityr37::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR37 {}
#[doc = "GICD interrupt priority register 37"]
pub mod gicd_ipriorityr37;
#[doc = "GICD interrupt priority register 38\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr38](gicd_ipriorityr38) module"]
pub type GICD_IPRIORITYR38 = crate::Reg<u32, _GICD_IPRIORITYR38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR38;
#[doc = "`read()` method returns [gicd_ipriorityr38::R](gicd_ipriorityr38::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR38 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr38::W](gicd_ipriorityr38::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR38 {}
#[doc = "GICD interrupt priority register 38"]
pub mod gicd_ipriorityr38;
#[doc = "GICD interrupt priority register 39\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr39](gicd_ipriorityr39) module"]
pub type GICD_IPRIORITYR39 = crate::Reg<u32, _GICD_IPRIORITYR39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR39;
#[doc = "`read()` method returns [gicd_ipriorityr39::R](gicd_ipriorityr39::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR39 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr39::W](gicd_ipriorityr39::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR39 {}
#[doc = "GICD interrupt priority register 39"]
pub mod gicd_ipriorityr39;
#[doc = "GICD interrupt priority register 40\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr40](gicd_ipriorityr40) module"]
pub type GICD_IPRIORITYR40 = crate::Reg<u32, _GICD_IPRIORITYR40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR40;
#[doc = "`read()` method returns [gicd_ipriorityr40::R](gicd_ipriorityr40::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR40 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr40::W](gicd_ipriorityr40::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR40 {}
#[doc = "GICD interrupt priority register 40"]
pub mod gicd_ipriorityr40;
#[doc = "GICD interrupt priority register 41\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr41](gicd_ipriorityr41) module"]
pub type GICD_IPRIORITYR41 = crate::Reg<u32, _GICD_IPRIORITYR41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR41;
#[doc = "`read()` method returns [gicd_ipriorityr41::R](gicd_ipriorityr41::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR41 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr41::W](gicd_ipriorityr41::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR41 {}
#[doc = "GICD interrupt priority register 41"]
pub mod gicd_ipriorityr41;
#[doc = "GICD interrupt priority register 42\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr42](gicd_ipriorityr42) module"]
pub type GICD_IPRIORITYR42 = crate::Reg<u32, _GICD_IPRIORITYR42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR42;
#[doc = "`read()` method returns [gicd_ipriorityr42::R](gicd_ipriorityr42::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR42 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr42::W](gicd_ipriorityr42::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR42 {}
#[doc = "GICD interrupt priority register 42"]
pub mod gicd_ipriorityr42;
#[doc = "GICD interrupt priority register 43\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr43](gicd_ipriorityr43) module"]
pub type GICD_IPRIORITYR43 = crate::Reg<u32, _GICD_IPRIORITYR43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR43;
#[doc = "`read()` method returns [gicd_ipriorityr43::R](gicd_ipriorityr43::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR43 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr43::W](gicd_ipriorityr43::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR43 {}
#[doc = "GICD interrupt priority register 43"]
pub mod gicd_ipriorityr43;
#[doc = "GICD interrupt priority register 44\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr44](gicd_ipriorityr44) module"]
pub type GICD_IPRIORITYR44 = crate::Reg<u32, _GICD_IPRIORITYR44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR44;
#[doc = "`read()` method returns [gicd_ipriorityr44::R](gicd_ipriorityr44::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR44 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr44::W](gicd_ipriorityr44::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR44 {}
#[doc = "GICD interrupt priority register 44"]
pub mod gicd_ipriorityr44;
#[doc = "GICD interrupt priority register 45\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr45](gicd_ipriorityr45) module"]
pub type GICD_IPRIORITYR45 = crate::Reg<u32, _GICD_IPRIORITYR45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR45;
#[doc = "`read()` method returns [gicd_ipriorityr45::R](gicd_ipriorityr45::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR45 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr45::W](gicd_ipriorityr45::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR45 {}
#[doc = "GICD interrupt priority register 45"]
pub mod gicd_ipriorityr45;
#[doc = "GICD interrupt priority register 46\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr46](gicd_ipriorityr46) module"]
pub type GICD_IPRIORITYR46 = crate::Reg<u32, _GICD_IPRIORITYR46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR46;
#[doc = "`read()` method returns [gicd_ipriorityr46::R](gicd_ipriorityr46::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR46 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr46::W](gicd_ipriorityr46::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR46 {}
#[doc = "GICD interrupt priority register 46"]
pub mod gicd_ipriorityr46;
#[doc = "GICD interrupt priority register 47\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr47](gicd_ipriorityr47) module"]
pub type GICD_IPRIORITYR47 = crate::Reg<u32, _GICD_IPRIORITYR47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR47;
#[doc = "`read()` method returns [gicd_ipriorityr47::R](gicd_ipriorityr47::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR47 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr47::W](gicd_ipriorityr47::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR47 {}
#[doc = "GICD interrupt priority register 47"]
pub mod gicd_ipriorityr47;
#[doc = "GICD interrupt priority register 48\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr48](gicd_ipriorityr48) module"]
pub type GICD_IPRIORITYR48 = crate::Reg<u32, _GICD_IPRIORITYR48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR48;
#[doc = "`read()` method returns [gicd_ipriorityr48::R](gicd_ipriorityr48::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR48 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr48::W](gicd_ipriorityr48::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR48 {}
#[doc = "GICD interrupt priority register 48"]
pub mod gicd_ipriorityr48;
#[doc = "GICD interrupt priority register 49\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr49](gicd_ipriorityr49) module"]
pub type GICD_IPRIORITYR49 = crate::Reg<u32, _GICD_IPRIORITYR49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR49;
#[doc = "`read()` method returns [gicd_ipriorityr49::R](gicd_ipriorityr49::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR49 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr49::W](gicd_ipriorityr49::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR49 {}
#[doc = "GICD interrupt priority register 49"]
pub mod gicd_ipriorityr49;
#[doc = "GICD interrupt priority register 50\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr50](gicd_ipriorityr50) module"]
pub type GICD_IPRIORITYR50 = crate::Reg<u32, _GICD_IPRIORITYR50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR50;
#[doc = "`read()` method returns [gicd_ipriorityr50::R](gicd_ipriorityr50::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR50 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr50::W](gicd_ipriorityr50::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR50 {}
#[doc = "GICD interrupt priority register 50"]
pub mod gicd_ipriorityr50;
#[doc = "GICD interrupt priority register 51\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr51](gicd_ipriorityr51) module"]
pub type GICD_IPRIORITYR51 = crate::Reg<u32, _GICD_IPRIORITYR51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR51;
#[doc = "`read()` method returns [gicd_ipriorityr51::R](gicd_ipriorityr51::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR51 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr51::W](gicd_ipriorityr51::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR51 {}
#[doc = "GICD interrupt priority register 51"]
pub mod gicd_ipriorityr51;
#[doc = "GICD interrupt priority register 52\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr52](gicd_ipriorityr52) module"]
pub type GICD_IPRIORITYR52 = crate::Reg<u32, _GICD_IPRIORITYR52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR52;
#[doc = "`read()` method returns [gicd_ipriorityr52::R](gicd_ipriorityr52::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR52 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr52::W](gicd_ipriorityr52::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR52 {}
#[doc = "GICD interrupt priority register 52"]
pub mod gicd_ipriorityr52;
#[doc = "GICD interrupt priority register 53\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr53](gicd_ipriorityr53) module"]
pub type GICD_IPRIORITYR53 = crate::Reg<u32, _GICD_IPRIORITYR53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR53;
#[doc = "`read()` method returns [gicd_ipriorityr53::R](gicd_ipriorityr53::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR53 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr53::W](gicd_ipriorityr53::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR53 {}
#[doc = "GICD interrupt priority register 53"]
pub mod gicd_ipriorityr53;
#[doc = "GICD interrupt priority register 54\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr54](gicd_ipriorityr54) module"]
pub type GICD_IPRIORITYR54 = crate::Reg<u32, _GICD_IPRIORITYR54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR54;
#[doc = "`read()` method returns [gicd_ipriorityr54::R](gicd_ipriorityr54::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR54 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr54::W](gicd_ipriorityr54::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR54 {}
#[doc = "GICD interrupt priority register 54"]
pub mod gicd_ipriorityr54;
#[doc = "GICD interrupt priority register 55\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr55](gicd_ipriorityr55) module"]
pub type GICD_IPRIORITYR55 = crate::Reg<u32, _GICD_IPRIORITYR55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR55;
#[doc = "`read()` method returns [gicd_ipriorityr55::R](gicd_ipriorityr55::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR55 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr55::W](gicd_ipriorityr55::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR55 {}
#[doc = "GICD interrupt priority register 55"]
pub mod gicd_ipriorityr55;
#[doc = "GICD interrupt priority register 56\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr56](gicd_ipriorityr56) module"]
pub type GICD_IPRIORITYR56 = crate::Reg<u32, _GICD_IPRIORITYR56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR56;
#[doc = "`read()` method returns [gicd_ipriorityr56::R](gicd_ipriorityr56::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR56 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr56::W](gicd_ipriorityr56::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR56 {}
#[doc = "GICD interrupt priority register 56"]
pub mod gicd_ipriorityr56;
#[doc = "GICD interrupt priority register 57\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr57](gicd_ipriorityr57) module"]
pub type GICD_IPRIORITYR57 = crate::Reg<u32, _GICD_IPRIORITYR57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR57;
#[doc = "`read()` method returns [gicd_ipriorityr57::R](gicd_ipriorityr57::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR57 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr57::W](gicd_ipriorityr57::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR57 {}
#[doc = "GICD interrupt priority register 57"]
pub mod gicd_ipriorityr57;
#[doc = "GICD interrupt priority register 58\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr58](gicd_ipriorityr58) module"]
pub type GICD_IPRIORITYR58 = crate::Reg<u32, _GICD_IPRIORITYR58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR58;
#[doc = "`read()` method returns [gicd_ipriorityr58::R](gicd_ipriorityr58::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR58 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr58::W](gicd_ipriorityr58::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR58 {}
#[doc = "GICD interrupt priority register 58"]
pub mod gicd_ipriorityr58;
#[doc = "GICD interrupt priority register 59\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr59](gicd_ipriorityr59) module"]
pub type GICD_IPRIORITYR59 = crate::Reg<u32, _GICD_IPRIORITYR59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR59;
#[doc = "`read()` method returns [gicd_ipriorityr59::R](gicd_ipriorityr59::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR59 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr59::W](gicd_ipriorityr59::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR59 {}
#[doc = "GICD interrupt priority register 59"]
pub mod gicd_ipriorityr59;
#[doc = "GICD interrupt priority register 60\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr60](gicd_ipriorityr60) module"]
pub type GICD_IPRIORITYR60 = crate::Reg<u32, _GICD_IPRIORITYR60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR60;
#[doc = "`read()` method returns [gicd_ipriorityr60::R](gicd_ipriorityr60::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR60 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr60::W](gicd_ipriorityr60::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR60 {}
#[doc = "GICD interrupt priority register 60"]
pub mod gicd_ipriorityr60;
#[doc = "GICD interrupt priority register 61\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr61](gicd_ipriorityr61) module"]
pub type GICD_IPRIORITYR61 = crate::Reg<u32, _GICD_IPRIORITYR61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR61;
#[doc = "`read()` method returns [gicd_ipriorityr61::R](gicd_ipriorityr61::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR61 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr61::W](gicd_ipriorityr61::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR61 {}
#[doc = "GICD interrupt priority register 61"]
pub mod gicd_ipriorityr61;
#[doc = "GICD interrupt priority register 62\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr62](gicd_ipriorityr62) module"]
pub type GICD_IPRIORITYR62 = crate::Reg<u32, _GICD_IPRIORITYR62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR62;
#[doc = "`read()` method returns [gicd_ipriorityr62::R](gicd_ipriorityr62::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR62 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr62::W](gicd_ipriorityr62::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR62 {}
#[doc = "GICD interrupt priority register 62"]
pub mod gicd_ipriorityr62;
#[doc = "GICD interrupt priority register 63\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr63](gicd_ipriorityr63) module"]
pub type GICD_IPRIORITYR63 = crate::Reg<u32, _GICD_IPRIORITYR63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR63;
#[doc = "`read()` method returns [gicd_ipriorityr63::R](gicd_ipriorityr63::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR63 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr63::W](gicd_ipriorityr63::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR63 {}
#[doc = "GICD interrupt priority register 63"]
pub mod gicd_ipriorityr63;
#[doc = "GICD interrupt priority register 64\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr64](gicd_ipriorityr64) module"]
pub type GICD_IPRIORITYR64 = crate::Reg<u32, _GICD_IPRIORITYR64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR64;
#[doc = "`read()` method returns [gicd_ipriorityr64::R](gicd_ipriorityr64::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR64 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr64::W](gicd_ipriorityr64::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR64 {}
#[doc = "GICD interrupt priority register 64"]
pub mod gicd_ipriorityr64;
#[doc = "GICD interrupt priority register 65\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr65](gicd_ipriorityr65) module"]
pub type GICD_IPRIORITYR65 = crate::Reg<u32, _GICD_IPRIORITYR65>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR65;
#[doc = "`read()` method returns [gicd_ipriorityr65::R](gicd_ipriorityr65::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR65 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr65::W](gicd_ipriorityr65::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR65 {}
#[doc = "GICD interrupt priority register 65"]
pub mod gicd_ipriorityr65;
#[doc = "GICD interrupt priority register 66\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr66](gicd_ipriorityr66) module"]
pub type GICD_IPRIORITYR66 = crate::Reg<u32, _GICD_IPRIORITYR66>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR66;
#[doc = "`read()` method returns [gicd_ipriorityr66::R](gicd_ipriorityr66::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR66 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr66::W](gicd_ipriorityr66::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR66 {}
#[doc = "GICD interrupt priority register 66"]
pub mod gicd_ipriorityr66;
#[doc = "GICD interrupt priority register 67\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr67](gicd_ipriorityr67) module"]
pub type GICD_IPRIORITYR67 = crate::Reg<u32, _GICD_IPRIORITYR67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR67;
#[doc = "`read()` method returns [gicd_ipriorityr67::R](gicd_ipriorityr67::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR67 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr67::W](gicd_ipriorityr67::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR67 {}
#[doc = "GICD interrupt priority register 67"]
pub mod gicd_ipriorityr67;
#[doc = "GICD interrupt priority register 68\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr68](gicd_ipriorityr68) module"]
pub type GICD_IPRIORITYR68 = crate::Reg<u32, _GICD_IPRIORITYR68>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR68;
#[doc = "`read()` method returns [gicd_ipriorityr68::R](gicd_ipriorityr68::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR68 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr68::W](gicd_ipriorityr68::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR68 {}
#[doc = "GICD interrupt priority register 68"]
pub mod gicd_ipriorityr68;
#[doc = "GICD interrupt priority register 69\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr69](gicd_ipriorityr69) module"]
pub type GICD_IPRIORITYR69 = crate::Reg<u32, _GICD_IPRIORITYR69>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR69;
#[doc = "`read()` method returns [gicd_ipriorityr69::R](gicd_ipriorityr69::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR69 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr69::W](gicd_ipriorityr69::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR69 {}
#[doc = "GICD interrupt priority register 69"]
pub mod gicd_ipriorityr69;
#[doc = "GICD interrupt priority register 70\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr70](gicd_ipriorityr70) module"]
pub type GICD_IPRIORITYR70 = crate::Reg<u32, _GICD_IPRIORITYR70>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR70;
#[doc = "`read()` method returns [gicd_ipriorityr70::R](gicd_ipriorityr70::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR70 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr70::W](gicd_ipriorityr70::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR70 {}
#[doc = "GICD interrupt priority register 70"]
pub mod gicd_ipriorityr70;
#[doc = "GICD interrupt priority register 71\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr71](gicd_ipriorityr71) module"]
pub type GICD_IPRIORITYR71 = crate::Reg<u32, _GICD_IPRIORITYR71>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_IPRIORITYR71;
#[doc = "`read()` method returns [gicd_ipriorityr71::R](gicd_ipriorityr71::R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR71 {}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr71::W](gicd_ipriorityr71::W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR71 {}
#[doc = "GICD interrupt priority register 71"]
pub mod gicd_ipriorityr71;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr0](gicd_itargetsr0) module"]
pub type GICD_ITARGETSR0 = crate::Reg<u32, _GICD_ITARGETSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR0;
#[doc = "`read()` method returns [gicd_itargetsr0::R](gicd_itargetsr0::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR0 {}
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr0;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr1](gicd_itargetsr1) module"]
pub type GICD_ITARGETSR1 = crate::Reg<u32, _GICD_ITARGETSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR1;
#[doc = "`read()` method returns [gicd_itargetsr1::R](gicd_itargetsr1::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR1 {}
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr1;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr2](gicd_itargetsr2) module"]
pub type GICD_ITARGETSR2 = crate::Reg<u32, _GICD_ITARGETSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR2;
#[doc = "`read()` method returns [gicd_itargetsr2::R](gicd_itargetsr2::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR2 {}
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr2;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr3](gicd_itargetsr3) module"]
pub type GICD_ITARGETSR3 = crate::Reg<u32, _GICD_ITARGETSR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR3;
#[doc = "`read()` method returns [gicd_itargetsr3::R](gicd_itargetsr3::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR3 {}
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr3;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr4](gicd_itargetsr4) module"]
pub type GICD_ITARGETSR4 = crate::Reg<u32, _GICD_ITARGETSR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR4;
#[doc = "`read()` method returns [gicd_itargetsr4::R](gicd_itargetsr4::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR4 {}
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr4;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr5](gicd_itargetsr5) module"]
pub type GICD_ITARGETSR5 = crate::Reg<u32, _GICD_ITARGETSR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR5;
#[doc = "`read()` method returns [gicd_itargetsr5::R](gicd_itargetsr5::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR5 {}
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr5;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr6](gicd_itargetsr6) module"]
pub type GICD_ITARGETSR6 = crate::Reg<u32, _GICD_ITARGETSR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR6;
#[doc = "`read()` method returns [gicd_itargetsr6::R](gicd_itargetsr6::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR6 {}
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr6;
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr7](gicd_itargetsr7) module"]
pub type GICD_ITARGETSR7 = crate::Reg<u32, _GICD_ITARGETSR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR7;
#[doc = "`read()` method returns [gicd_itargetsr7::R](gicd_itargetsr7::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR7 {}
#[doc = "For existing SGIs and PPIs, read of CPU targets field returns the number of the processor performing the read."]
pub mod gicd_itargetsr7;
#[doc = "GICD interrupt processor target register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr8](gicd_itargetsr8) module"]
pub type GICD_ITARGETSR8 = crate::Reg<u32, _GICD_ITARGETSR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR8;
#[doc = "`read()` method returns [gicd_itargetsr8::R](gicd_itargetsr8::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR8 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr8::W](gicd_itargetsr8::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR8 {}
#[doc = "GICD interrupt processor target register 8"]
pub mod gicd_itargetsr8;
#[doc = "GICD interrupt processor target register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr9](gicd_itargetsr9) module"]
pub type GICD_ITARGETSR9 = crate::Reg<u32, _GICD_ITARGETSR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR9;
#[doc = "`read()` method returns [gicd_itargetsr9::R](gicd_itargetsr9::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR9 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr9::W](gicd_itargetsr9::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR9 {}
#[doc = "GICD interrupt processor target register 9"]
pub mod gicd_itargetsr9;
#[doc = "GICD interrupt processor target register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr10](gicd_itargetsr10) module"]
pub type GICD_ITARGETSR10 = crate::Reg<u32, _GICD_ITARGETSR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR10;
#[doc = "`read()` method returns [gicd_itargetsr10::R](gicd_itargetsr10::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR10 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr10::W](gicd_itargetsr10::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR10 {}
#[doc = "GICD interrupt processor target register 10"]
pub mod gicd_itargetsr10;
#[doc = "GICD interrupt processor target register 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr11](gicd_itargetsr11) module"]
pub type GICD_ITARGETSR11 = crate::Reg<u32, _GICD_ITARGETSR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR11;
#[doc = "`read()` method returns [gicd_itargetsr11::R](gicd_itargetsr11::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR11 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr11::W](gicd_itargetsr11::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR11 {}
#[doc = "GICD interrupt processor target register 11"]
pub mod gicd_itargetsr11;
#[doc = "GICD interrupt processor target register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr12](gicd_itargetsr12) module"]
pub type GICD_ITARGETSR12 = crate::Reg<u32, _GICD_ITARGETSR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR12;
#[doc = "`read()` method returns [gicd_itargetsr12::R](gicd_itargetsr12::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR12 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr12::W](gicd_itargetsr12::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR12 {}
#[doc = "GICD interrupt processor target register 12"]
pub mod gicd_itargetsr12;
#[doc = "GICD interrupt processor target register 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr13](gicd_itargetsr13) module"]
pub type GICD_ITARGETSR13 = crate::Reg<u32, _GICD_ITARGETSR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR13;
#[doc = "`read()` method returns [gicd_itargetsr13::R](gicd_itargetsr13::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR13 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr13::W](gicd_itargetsr13::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR13 {}
#[doc = "GICD interrupt processor target register 13"]
pub mod gicd_itargetsr13;
#[doc = "GICD interrupt processor target register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr14](gicd_itargetsr14) module"]
pub type GICD_ITARGETSR14 = crate::Reg<u32, _GICD_ITARGETSR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR14;
#[doc = "`read()` method returns [gicd_itargetsr14::R](gicd_itargetsr14::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR14 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr14::W](gicd_itargetsr14::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR14 {}
#[doc = "GICD interrupt processor target register 14"]
pub mod gicd_itargetsr14;
#[doc = "GICD interrupt processor target register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr15](gicd_itargetsr15) module"]
pub type GICD_ITARGETSR15 = crate::Reg<u32, _GICD_ITARGETSR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR15;
#[doc = "`read()` method returns [gicd_itargetsr15::R](gicd_itargetsr15::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR15 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr15::W](gicd_itargetsr15::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR15 {}
#[doc = "GICD interrupt processor target register 15"]
pub mod gicd_itargetsr15;
#[doc = "GICD interrupt processor target register 16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr16](gicd_itargetsr16) module"]
pub type GICD_ITARGETSR16 = crate::Reg<u32, _GICD_ITARGETSR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR16;
#[doc = "`read()` method returns [gicd_itargetsr16::R](gicd_itargetsr16::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR16 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr16::W](gicd_itargetsr16::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR16 {}
#[doc = "GICD interrupt processor target register 16"]
pub mod gicd_itargetsr16;
#[doc = "GICD interrupt processor target register 17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr17](gicd_itargetsr17) module"]
pub type GICD_ITARGETSR17 = crate::Reg<u32, _GICD_ITARGETSR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR17;
#[doc = "`read()` method returns [gicd_itargetsr17::R](gicd_itargetsr17::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR17 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr17::W](gicd_itargetsr17::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR17 {}
#[doc = "GICD interrupt processor target register 17"]
pub mod gicd_itargetsr17;
#[doc = "GICD interrupt processor target register 18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr18](gicd_itargetsr18) module"]
pub type GICD_ITARGETSR18 = crate::Reg<u32, _GICD_ITARGETSR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR18;
#[doc = "`read()` method returns [gicd_itargetsr18::R](gicd_itargetsr18::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR18 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr18::W](gicd_itargetsr18::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR18 {}
#[doc = "GICD interrupt processor target register 18"]
pub mod gicd_itargetsr18;
#[doc = "GICD interrupt processor target register 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr19](gicd_itargetsr19) module"]
pub type GICD_ITARGETSR19 = crate::Reg<u32, _GICD_ITARGETSR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR19;
#[doc = "`read()` method returns [gicd_itargetsr19::R](gicd_itargetsr19::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR19 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr19::W](gicd_itargetsr19::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR19 {}
#[doc = "GICD interrupt processor target register 19"]
pub mod gicd_itargetsr19;
#[doc = "GICD interrupt processor target register 20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr20](gicd_itargetsr20) module"]
pub type GICD_ITARGETSR20 = crate::Reg<u32, _GICD_ITARGETSR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR20;
#[doc = "`read()` method returns [gicd_itargetsr20::R](gicd_itargetsr20::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR20 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr20::W](gicd_itargetsr20::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR20 {}
#[doc = "GICD interrupt processor target register 20"]
pub mod gicd_itargetsr20;
#[doc = "GICD interrupt processor target register 21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr21](gicd_itargetsr21) module"]
pub type GICD_ITARGETSR21 = crate::Reg<u32, _GICD_ITARGETSR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR21;
#[doc = "`read()` method returns [gicd_itargetsr21::R](gicd_itargetsr21::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR21 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr21::W](gicd_itargetsr21::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR21 {}
#[doc = "GICD interrupt processor target register 21"]
pub mod gicd_itargetsr21;
#[doc = "GICD interrupt processor target register 22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr22](gicd_itargetsr22) module"]
pub type GICD_ITARGETSR22 = crate::Reg<u32, _GICD_ITARGETSR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR22;
#[doc = "`read()` method returns [gicd_itargetsr22::R](gicd_itargetsr22::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR22 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr22::W](gicd_itargetsr22::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR22 {}
#[doc = "GICD interrupt processor target register 22"]
pub mod gicd_itargetsr22;
#[doc = "GICD interrupt processor target register 23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr23](gicd_itargetsr23) module"]
pub type GICD_ITARGETSR23 = crate::Reg<u32, _GICD_ITARGETSR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR23;
#[doc = "`read()` method returns [gicd_itargetsr23::R](gicd_itargetsr23::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR23 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr23::W](gicd_itargetsr23::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR23 {}
#[doc = "GICD interrupt processor target register 23"]
pub mod gicd_itargetsr23;
#[doc = "GICD interrupt processor target register 24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr24](gicd_itargetsr24) module"]
pub type GICD_ITARGETSR24 = crate::Reg<u32, _GICD_ITARGETSR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR24;
#[doc = "`read()` method returns [gicd_itargetsr24::R](gicd_itargetsr24::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR24 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr24::W](gicd_itargetsr24::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR24 {}
#[doc = "GICD interrupt processor target register 24"]
pub mod gicd_itargetsr24;
#[doc = "GICD interrupt processor target register 25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr25](gicd_itargetsr25) module"]
pub type GICD_ITARGETSR25 = crate::Reg<u32, _GICD_ITARGETSR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR25;
#[doc = "`read()` method returns [gicd_itargetsr25::R](gicd_itargetsr25::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR25 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr25::W](gicd_itargetsr25::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR25 {}
#[doc = "GICD interrupt processor target register 25"]
pub mod gicd_itargetsr25;
#[doc = "GICD interrupt processor target register 26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr26](gicd_itargetsr26) module"]
pub type GICD_ITARGETSR26 = crate::Reg<u32, _GICD_ITARGETSR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR26;
#[doc = "`read()` method returns [gicd_itargetsr26::R](gicd_itargetsr26::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR26 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr26::W](gicd_itargetsr26::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR26 {}
#[doc = "GICD interrupt processor target register 26"]
pub mod gicd_itargetsr26;
#[doc = "GICD interrupt processor target register 27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr27](gicd_itargetsr27) module"]
pub type GICD_ITARGETSR27 = crate::Reg<u32, _GICD_ITARGETSR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR27;
#[doc = "`read()` method returns [gicd_itargetsr27::R](gicd_itargetsr27::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR27 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr27::W](gicd_itargetsr27::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR27 {}
#[doc = "GICD interrupt processor target register 27"]
pub mod gicd_itargetsr27;
#[doc = "GICD interrupt processor target register 28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr28](gicd_itargetsr28) module"]
pub type GICD_ITARGETSR28 = crate::Reg<u32, _GICD_ITARGETSR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR28;
#[doc = "`read()` method returns [gicd_itargetsr28::R](gicd_itargetsr28::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR28 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr28::W](gicd_itargetsr28::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR28 {}
#[doc = "GICD interrupt processor target register 28"]
pub mod gicd_itargetsr28;
#[doc = "GICD interrupt processor target register 29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr29](gicd_itargetsr29) module"]
pub type GICD_ITARGETSR29 = crate::Reg<u32, _GICD_ITARGETSR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR29;
#[doc = "`read()` method returns [gicd_itargetsr29::R](gicd_itargetsr29::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR29 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr29::W](gicd_itargetsr29::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR29 {}
#[doc = "GICD interrupt processor target register 29"]
pub mod gicd_itargetsr29;
#[doc = "GICD interrupt processor target register 30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr30](gicd_itargetsr30) module"]
pub type GICD_ITARGETSR30 = crate::Reg<u32, _GICD_ITARGETSR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR30;
#[doc = "`read()` method returns [gicd_itargetsr30::R](gicd_itargetsr30::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR30 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr30::W](gicd_itargetsr30::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR30 {}
#[doc = "GICD interrupt processor target register 30"]
pub mod gicd_itargetsr30;
#[doc = "GICD interrupt processor target register 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr31](gicd_itargetsr31) module"]
pub type GICD_ITARGETSR31 = crate::Reg<u32, _GICD_ITARGETSR31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR31;
#[doc = "`read()` method returns [gicd_itargetsr31::R](gicd_itargetsr31::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR31 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr31::W](gicd_itargetsr31::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR31 {}
#[doc = "GICD interrupt processor target register 31"]
pub mod gicd_itargetsr31;
#[doc = "GICD interrupt processor target register 32\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr32](gicd_itargetsr32) module"]
pub type GICD_ITARGETSR32 = crate::Reg<u32, _GICD_ITARGETSR32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR32;
#[doc = "`read()` method returns [gicd_itargetsr32::R](gicd_itargetsr32::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR32 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr32::W](gicd_itargetsr32::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR32 {}
#[doc = "GICD interrupt processor target register 32"]
pub mod gicd_itargetsr32;
#[doc = "GICD interrupt processor target register 33\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr33](gicd_itargetsr33) module"]
pub type GICD_ITARGETSR33 = crate::Reg<u32, _GICD_ITARGETSR33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR33;
#[doc = "`read()` method returns [gicd_itargetsr33::R](gicd_itargetsr33::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR33 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr33::W](gicd_itargetsr33::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR33 {}
#[doc = "GICD interrupt processor target register 33"]
pub mod gicd_itargetsr33;
#[doc = "GICD interrupt processor target register 34\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr34](gicd_itargetsr34) module"]
pub type GICD_ITARGETSR34 = crate::Reg<u32, _GICD_ITARGETSR34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR34;
#[doc = "`read()` method returns [gicd_itargetsr34::R](gicd_itargetsr34::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR34 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr34::W](gicd_itargetsr34::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR34 {}
#[doc = "GICD interrupt processor target register 34"]
pub mod gicd_itargetsr34;
#[doc = "GICD interrupt processor target register 35\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr35](gicd_itargetsr35) module"]
pub type GICD_ITARGETSR35 = crate::Reg<u32, _GICD_ITARGETSR35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR35;
#[doc = "`read()` method returns [gicd_itargetsr35::R](gicd_itargetsr35::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR35 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr35::W](gicd_itargetsr35::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR35 {}
#[doc = "GICD interrupt processor target register 35"]
pub mod gicd_itargetsr35;
#[doc = "GICD interrupt processor target register 36\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr36](gicd_itargetsr36) module"]
pub type GICD_ITARGETSR36 = crate::Reg<u32, _GICD_ITARGETSR36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR36;
#[doc = "`read()` method returns [gicd_itargetsr36::R](gicd_itargetsr36::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR36 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr36::W](gicd_itargetsr36::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR36 {}
#[doc = "GICD interrupt processor target register 36"]
pub mod gicd_itargetsr36;
#[doc = "GICD interrupt processor target register 37\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr37](gicd_itargetsr37) module"]
pub type GICD_ITARGETSR37 = crate::Reg<u32, _GICD_ITARGETSR37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR37;
#[doc = "`read()` method returns [gicd_itargetsr37::R](gicd_itargetsr37::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR37 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr37::W](gicd_itargetsr37::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR37 {}
#[doc = "GICD interrupt processor target register 37"]
pub mod gicd_itargetsr37;
#[doc = "GICD interrupt processor target register 38\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr38](gicd_itargetsr38) module"]
pub type GICD_ITARGETSR38 = crate::Reg<u32, _GICD_ITARGETSR38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR38;
#[doc = "`read()` method returns [gicd_itargetsr38::R](gicd_itargetsr38::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR38 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr38::W](gicd_itargetsr38::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR38 {}
#[doc = "GICD interrupt processor target register 38"]
pub mod gicd_itargetsr38;
#[doc = "GICD interrupt processor target register 39\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr39](gicd_itargetsr39) module"]
pub type GICD_ITARGETSR39 = crate::Reg<u32, _GICD_ITARGETSR39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR39;
#[doc = "`read()` method returns [gicd_itargetsr39::R](gicd_itargetsr39::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR39 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr39::W](gicd_itargetsr39::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR39 {}
#[doc = "GICD interrupt processor target register 39"]
pub mod gicd_itargetsr39;
#[doc = "GICD interrupt processor target register 40\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr40](gicd_itargetsr40) module"]
pub type GICD_ITARGETSR40 = crate::Reg<u32, _GICD_ITARGETSR40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR40;
#[doc = "`read()` method returns [gicd_itargetsr40::R](gicd_itargetsr40::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR40 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr40::W](gicd_itargetsr40::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR40 {}
#[doc = "GICD interrupt processor target register 40"]
pub mod gicd_itargetsr40;
#[doc = "GICD interrupt processor target register 41\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr41](gicd_itargetsr41) module"]
pub type GICD_ITARGETSR41 = crate::Reg<u32, _GICD_ITARGETSR41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR41;
#[doc = "`read()` method returns [gicd_itargetsr41::R](gicd_itargetsr41::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR41 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr41::W](gicd_itargetsr41::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR41 {}
#[doc = "GICD interrupt processor target register 41"]
pub mod gicd_itargetsr41;
#[doc = "GICD interrupt processor target register 42\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr42](gicd_itargetsr42) module"]
pub type GICD_ITARGETSR42 = crate::Reg<u32, _GICD_ITARGETSR42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR42;
#[doc = "`read()` method returns [gicd_itargetsr42::R](gicd_itargetsr42::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR42 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr42::W](gicd_itargetsr42::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR42 {}
#[doc = "GICD interrupt processor target register 42"]
pub mod gicd_itargetsr42;
#[doc = "GICD interrupt processor target register 43\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr43](gicd_itargetsr43) module"]
pub type GICD_ITARGETSR43 = crate::Reg<u32, _GICD_ITARGETSR43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR43;
#[doc = "`read()` method returns [gicd_itargetsr43::R](gicd_itargetsr43::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR43 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr43::W](gicd_itargetsr43::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR43 {}
#[doc = "GICD interrupt processor target register 43"]
pub mod gicd_itargetsr43;
#[doc = "GICD interrupt processor target register 44\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr44](gicd_itargetsr44) module"]
pub type GICD_ITARGETSR44 = crate::Reg<u32, _GICD_ITARGETSR44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR44;
#[doc = "`read()` method returns [gicd_itargetsr44::R](gicd_itargetsr44::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR44 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr44::W](gicd_itargetsr44::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR44 {}
#[doc = "GICD interrupt processor target register 44"]
pub mod gicd_itargetsr44;
#[doc = "GICD interrupt processor target register 45\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr45](gicd_itargetsr45) module"]
pub type GICD_ITARGETSR45 = crate::Reg<u32, _GICD_ITARGETSR45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR45;
#[doc = "`read()` method returns [gicd_itargetsr45::R](gicd_itargetsr45::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR45 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr45::W](gicd_itargetsr45::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR45 {}
#[doc = "GICD interrupt processor target register 45"]
pub mod gicd_itargetsr45;
#[doc = "GICD interrupt processor target register 46\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr46](gicd_itargetsr46) module"]
pub type GICD_ITARGETSR46 = crate::Reg<u32, _GICD_ITARGETSR46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR46;
#[doc = "`read()` method returns [gicd_itargetsr46::R](gicd_itargetsr46::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR46 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr46::W](gicd_itargetsr46::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR46 {}
#[doc = "GICD interrupt processor target register 46"]
pub mod gicd_itargetsr46;
#[doc = "GICD interrupt processor target register 47\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr47](gicd_itargetsr47) module"]
pub type GICD_ITARGETSR47 = crate::Reg<u32, _GICD_ITARGETSR47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR47;
#[doc = "`read()` method returns [gicd_itargetsr47::R](gicd_itargetsr47::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR47 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr47::W](gicd_itargetsr47::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR47 {}
#[doc = "GICD interrupt processor target register 47"]
pub mod gicd_itargetsr47;
#[doc = "GICD interrupt processor target register 48\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr48](gicd_itargetsr48) module"]
pub type GICD_ITARGETSR48 = crate::Reg<u32, _GICD_ITARGETSR48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR48;
#[doc = "`read()` method returns [gicd_itargetsr48::R](gicd_itargetsr48::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR48 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr48::W](gicd_itargetsr48::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR48 {}
#[doc = "GICD interrupt processor target register 48"]
pub mod gicd_itargetsr48;
#[doc = "GICD interrupt processor target register 49\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr49](gicd_itargetsr49) module"]
pub type GICD_ITARGETSR49 = crate::Reg<u32, _GICD_ITARGETSR49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR49;
#[doc = "`read()` method returns [gicd_itargetsr49::R](gicd_itargetsr49::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR49 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr49::W](gicd_itargetsr49::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR49 {}
#[doc = "GICD interrupt processor target register 49"]
pub mod gicd_itargetsr49;
#[doc = "GICD interrupt processor target register 50\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr50](gicd_itargetsr50) module"]
pub type GICD_ITARGETSR50 = crate::Reg<u32, _GICD_ITARGETSR50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR50;
#[doc = "`read()` method returns [gicd_itargetsr50::R](gicd_itargetsr50::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR50 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr50::W](gicd_itargetsr50::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR50 {}
#[doc = "GICD interrupt processor target register 50"]
pub mod gicd_itargetsr50;
#[doc = "GICD interrupt processor target register 51\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr51](gicd_itargetsr51) module"]
pub type GICD_ITARGETSR51 = crate::Reg<u32, _GICD_ITARGETSR51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR51;
#[doc = "`read()` method returns [gicd_itargetsr51::R](gicd_itargetsr51::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR51 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr51::W](gicd_itargetsr51::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR51 {}
#[doc = "GICD interrupt processor target register 51"]
pub mod gicd_itargetsr51;
#[doc = "GICD interrupt processor target register 52\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr52](gicd_itargetsr52) module"]
pub type GICD_ITARGETSR52 = crate::Reg<u32, _GICD_ITARGETSR52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR52;
#[doc = "`read()` method returns [gicd_itargetsr52::R](gicd_itargetsr52::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR52 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr52::W](gicd_itargetsr52::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR52 {}
#[doc = "GICD interrupt processor target register 52"]
pub mod gicd_itargetsr52;
#[doc = "GICD interrupt processor target register 53\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr53](gicd_itargetsr53) module"]
pub type GICD_ITARGETSR53 = crate::Reg<u32, _GICD_ITARGETSR53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR53;
#[doc = "`read()` method returns [gicd_itargetsr53::R](gicd_itargetsr53::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR53 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr53::W](gicd_itargetsr53::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR53 {}
#[doc = "GICD interrupt processor target register 53"]
pub mod gicd_itargetsr53;
#[doc = "GICD interrupt processor target register 54\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr54](gicd_itargetsr54) module"]
pub type GICD_ITARGETSR54 = crate::Reg<u32, _GICD_ITARGETSR54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR54;
#[doc = "`read()` method returns [gicd_itargetsr54::R](gicd_itargetsr54::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR54 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr54::W](gicd_itargetsr54::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR54 {}
#[doc = "GICD interrupt processor target register 54"]
pub mod gicd_itargetsr54;
#[doc = "GICD interrupt processor target register 55\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr55](gicd_itargetsr55) module"]
pub type GICD_ITARGETSR55 = crate::Reg<u32, _GICD_ITARGETSR55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR55;
#[doc = "`read()` method returns [gicd_itargetsr55::R](gicd_itargetsr55::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR55 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr55::W](gicd_itargetsr55::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR55 {}
#[doc = "GICD interrupt processor target register 55"]
pub mod gicd_itargetsr55;
#[doc = "GICD interrupt processor target register 56\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr56](gicd_itargetsr56) module"]
pub type GICD_ITARGETSR56 = crate::Reg<u32, _GICD_ITARGETSR56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR56;
#[doc = "`read()` method returns [gicd_itargetsr56::R](gicd_itargetsr56::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR56 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr56::W](gicd_itargetsr56::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR56 {}
#[doc = "GICD interrupt processor target register 56"]
pub mod gicd_itargetsr56;
#[doc = "GICD interrupt processor target register 57\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr57](gicd_itargetsr57) module"]
pub type GICD_ITARGETSR57 = crate::Reg<u32, _GICD_ITARGETSR57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR57;
#[doc = "`read()` method returns [gicd_itargetsr57::R](gicd_itargetsr57::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR57 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr57::W](gicd_itargetsr57::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR57 {}
#[doc = "GICD interrupt processor target register 57"]
pub mod gicd_itargetsr57;
#[doc = "GICD interrupt processor target register 58\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr58](gicd_itargetsr58) module"]
pub type GICD_ITARGETSR58 = crate::Reg<u32, _GICD_ITARGETSR58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR58;
#[doc = "`read()` method returns [gicd_itargetsr58::R](gicd_itargetsr58::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR58 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr58::W](gicd_itargetsr58::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR58 {}
#[doc = "GICD interrupt processor target register 58"]
pub mod gicd_itargetsr58;
#[doc = "GICD interrupt processor target register 59\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr59](gicd_itargetsr59) module"]
pub type GICD_ITARGETSR59 = crate::Reg<u32, _GICD_ITARGETSR59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR59;
#[doc = "`read()` method returns [gicd_itargetsr59::R](gicd_itargetsr59::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR59 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr59::W](gicd_itargetsr59::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR59 {}
#[doc = "GICD interrupt processor target register 59"]
pub mod gicd_itargetsr59;
#[doc = "GICD interrupt processor target register 60\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr60](gicd_itargetsr60) module"]
pub type GICD_ITARGETSR60 = crate::Reg<u32, _GICD_ITARGETSR60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR60;
#[doc = "`read()` method returns [gicd_itargetsr60::R](gicd_itargetsr60::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR60 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr60::W](gicd_itargetsr60::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR60 {}
#[doc = "GICD interrupt processor target register 60"]
pub mod gicd_itargetsr60;
#[doc = "GICD interrupt processor target register 61\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr61](gicd_itargetsr61) module"]
pub type GICD_ITARGETSR61 = crate::Reg<u32, _GICD_ITARGETSR61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR61;
#[doc = "`read()` method returns [gicd_itargetsr61::R](gicd_itargetsr61::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR61 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr61::W](gicd_itargetsr61::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR61 {}
#[doc = "GICD interrupt processor target register 61"]
pub mod gicd_itargetsr61;
#[doc = "GICD interrupt processor target register 62\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr62](gicd_itargetsr62) module"]
pub type GICD_ITARGETSR62 = crate::Reg<u32, _GICD_ITARGETSR62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR62;
#[doc = "`read()` method returns [gicd_itargetsr62::R](gicd_itargetsr62::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR62 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr62::W](gicd_itargetsr62::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR62 {}
#[doc = "GICD interrupt processor target register 62"]
pub mod gicd_itargetsr62;
#[doc = "GICD interrupt processor target register 63\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr63](gicd_itargetsr63) module"]
pub type GICD_ITARGETSR63 = crate::Reg<u32, _GICD_ITARGETSR63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR63;
#[doc = "`read()` method returns [gicd_itargetsr63::R](gicd_itargetsr63::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR63 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr63::W](gicd_itargetsr63::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR63 {}
#[doc = "GICD interrupt processor target register 63"]
pub mod gicd_itargetsr63;
#[doc = "GICD interrupt processor target register 64\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr64](gicd_itargetsr64) module"]
pub type GICD_ITARGETSR64 = crate::Reg<u32, _GICD_ITARGETSR64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR64;
#[doc = "`read()` method returns [gicd_itargetsr64::R](gicd_itargetsr64::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR64 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr64::W](gicd_itargetsr64::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR64 {}
#[doc = "GICD interrupt processor target register 64"]
pub mod gicd_itargetsr64;
#[doc = "GICD interrupt processor target register 65\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr65](gicd_itargetsr65) module"]
pub type GICD_ITARGETSR65 = crate::Reg<u32, _GICD_ITARGETSR65>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR65;
#[doc = "`read()` method returns [gicd_itargetsr65::R](gicd_itargetsr65::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR65 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr65::W](gicd_itargetsr65::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR65 {}
#[doc = "GICD interrupt processor target register 65"]
pub mod gicd_itargetsr65;
#[doc = "GICD interrupt processor target register 66\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr66](gicd_itargetsr66) module"]
pub type GICD_ITARGETSR66 = crate::Reg<u32, _GICD_ITARGETSR66>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR66;
#[doc = "`read()` method returns [gicd_itargetsr66::R](gicd_itargetsr66::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR66 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr66::W](gicd_itargetsr66::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR66 {}
#[doc = "GICD interrupt processor target register 66"]
pub mod gicd_itargetsr66;
#[doc = "GICD interrupt processor target register 67\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr67](gicd_itargetsr67) module"]
pub type GICD_ITARGETSR67 = crate::Reg<u32, _GICD_ITARGETSR67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR67;
#[doc = "`read()` method returns [gicd_itargetsr67::R](gicd_itargetsr67::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR67 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr67::W](gicd_itargetsr67::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR67 {}
#[doc = "GICD interrupt processor target register 67"]
pub mod gicd_itargetsr67;
#[doc = "GICD interrupt processor target register 68\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr68](gicd_itargetsr68) module"]
pub type GICD_ITARGETSR68 = crate::Reg<u32, _GICD_ITARGETSR68>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR68;
#[doc = "`read()` method returns [gicd_itargetsr68::R](gicd_itargetsr68::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR68 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr68::W](gicd_itargetsr68::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR68 {}
#[doc = "GICD interrupt processor target register 68"]
pub mod gicd_itargetsr68;
#[doc = "GICD interrupt processor target register 69\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr69](gicd_itargetsr69) module"]
pub type GICD_ITARGETSR69 = crate::Reg<u32, _GICD_ITARGETSR69>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR69;
#[doc = "`read()` method returns [gicd_itargetsr69::R](gicd_itargetsr69::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR69 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr69::W](gicd_itargetsr69::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR69 {}
#[doc = "GICD interrupt processor target register 69"]
pub mod gicd_itargetsr69;
#[doc = "GICD interrupt processor target register 70\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr70](gicd_itargetsr70) module"]
pub type GICD_ITARGETSR70 = crate::Reg<u32, _GICD_ITARGETSR70>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR70;
#[doc = "`read()` method returns [gicd_itargetsr70::R](gicd_itargetsr70::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR70 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr70::W](gicd_itargetsr70::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR70 {}
#[doc = "GICD interrupt processor target register 70"]
pub mod gicd_itargetsr70;
#[doc = "GICD interrupt processor target register 71\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr71](gicd_itargetsr71) module"]
pub type GICD_ITARGETSR71 = crate::Reg<u32, _GICD_ITARGETSR71>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ITARGETSR71;
#[doc = "`read()` method returns [gicd_itargetsr71::R](gicd_itargetsr71::R) reader structure"]
impl crate::Readable for GICD_ITARGETSR71 {}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr71::W](gicd_itargetsr71::W) writer structure"]
impl crate::Writable for GICD_ITARGETSR71 {}
#[doc = "GICD interrupt processor target register 71"]
pub mod gicd_itargetsr71;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr0](gicd_icfgr0) module"]
pub type GICD_ICFGR0 = crate::Reg<u32, _GICD_ICFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR0;
#[doc = "`read()` method returns [gicd_icfgr0::R](gicd_icfgr0::R) reader structure"]
impl crate::Readable for GICD_ICFGR0 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr0::W](gicd_icfgr0::W) writer structure"]
impl crate::Writable for GICD_ICFGR0 {}
#[doc = "GICD interrupt configuration register"]
pub mod gicd_icfgr0;
#[doc = "GICD interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr1](gicd_icfgr1) module"]
pub type GICD_ICFGR1 = crate::Reg<u32, _GICD_ICFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR1;
#[doc = "`read()` method returns [gicd_icfgr1::R](gicd_icfgr1::R) reader structure"]
impl crate::Readable for GICD_ICFGR1 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr1::W](gicd_icfgr1::W) writer structure"]
impl crate::Writable for GICD_ICFGR1 {}
#[doc = "GICD interrupt configuration register"]
pub mod gicd_icfgr1;
#[doc = "GICD interrupt configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr2](gicd_icfgr2) module"]
pub type GICD_ICFGR2 = crate::Reg<u32, _GICD_ICFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR2;
#[doc = "`read()` method returns [gicd_icfgr2::R](gicd_icfgr2::R) reader structure"]
impl crate::Readable for GICD_ICFGR2 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr2::W](gicd_icfgr2::W) writer structure"]
impl crate::Writable for GICD_ICFGR2 {}
#[doc = "GICD interrupt configuration register 2"]
pub mod gicd_icfgr2;
#[doc = "GICD interrupt configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr3](gicd_icfgr3) module"]
pub type GICD_ICFGR3 = crate::Reg<u32, _GICD_ICFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR3;
#[doc = "`read()` method returns [gicd_icfgr3::R](gicd_icfgr3::R) reader structure"]
impl crate::Readable for GICD_ICFGR3 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr3::W](gicd_icfgr3::W) writer structure"]
impl crate::Writable for GICD_ICFGR3 {}
#[doc = "GICD interrupt configuration register 3"]
pub mod gicd_icfgr3;
#[doc = "GICD interrupt configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr4](gicd_icfgr4) module"]
pub type GICD_ICFGR4 = crate::Reg<u32, _GICD_ICFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR4;
#[doc = "`read()` method returns [gicd_icfgr4::R](gicd_icfgr4::R) reader structure"]
impl crate::Readable for GICD_ICFGR4 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr4::W](gicd_icfgr4::W) writer structure"]
impl crate::Writable for GICD_ICFGR4 {}
#[doc = "GICD interrupt configuration register 4"]
pub mod gicd_icfgr4;
#[doc = "GICD interrupt configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr5](gicd_icfgr5) module"]
pub type GICD_ICFGR5 = crate::Reg<u32, _GICD_ICFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR5;
#[doc = "`read()` method returns [gicd_icfgr5::R](gicd_icfgr5::R) reader structure"]
impl crate::Readable for GICD_ICFGR5 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr5::W](gicd_icfgr5::W) writer structure"]
impl crate::Writable for GICD_ICFGR5 {}
#[doc = "GICD interrupt configuration register 5"]
pub mod gicd_icfgr5;
#[doc = "GICD interrupt configuration register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr6](gicd_icfgr6) module"]
pub type GICD_ICFGR6 = crate::Reg<u32, _GICD_ICFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR6;
#[doc = "`read()` method returns [gicd_icfgr6::R](gicd_icfgr6::R) reader structure"]
impl crate::Readable for GICD_ICFGR6 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr6::W](gicd_icfgr6::W) writer structure"]
impl crate::Writable for GICD_ICFGR6 {}
#[doc = "GICD interrupt configuration register 6"]
pub mod gicd_icfgr6;
#[doc = "GICD interrupt configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr7](gicd_icfgr7) module"]
pub type GICD_ICFGR7 = crate::Reg<u32, _GICD_ICFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR7;
#[doc = "`read()` method returns [gicd_icfgr7::R](gicd_icfgr7::R) reader structure"]
impl crate::Readable for GICD_ICFGR7 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr7::W](gicd_icfgr7::W) writer structure"]
impl crate::Writable for GICD_ICFGR7 {}
#[doc = "GICD interrupt configuration register 7"]
pub mod gicd_icfgr7;
#[doc = "GICD interrupt configuration register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr8](gicd_icfgr8) module"]
pub type GICD_ICFGR8 = crate::Reg<u32, _GICD_ICFGR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR8;
#[doc = "`read()` method returns [gicd_icfgr8::R](gicd_icfgr8::R) reader structure"]
impl crate::Readable for GICD_ICFGR8 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr8::W](gicd_icfgr8::W) writer structure"]
impl crate::Writable for GICD_ICFGR8 {}
#[doc = "GICD interrupt configuration register 8"]
pub mod gicd_icfgr8;
#[doc = "GICD interrupt configuration register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr9](gicd_icfgr9) module"]
pub type GICD_ICFGR9 = crate::Reg<u32, _GICD_ICFGR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR9;
#[doc = "`read()` method returns [gicd_icfgr9::R](gicd_icfgr9::R) reader structure"]
impl crate::Readable for GICD_ICFGR9 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr9::W](gicd_icfgr9::W) writer structure"]
impl crate::Writable for GICD_ICFGR9 {}
#[doc = "GICD interrupt configuration register 9"]
pub mod gicd_icfgr9;
#[doc = "GICD interrupt configuration register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr10](gicd_icfgr10) module"]
pub type GICD_ICFGR10 = crate::Reg<u32, _GICD_ICFGR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR10;
#[doc = "`read()` method returns [gicd_icfgr10::R](gicd_icfgr10::R) reader structure"]
impl crate::Readable for GICD_ICFGR10 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr10::W](gicd_icfgr10::W) writer structure"]
impl crate::Writable for GICD_ICFGR10 {}
#[doc = "GICD interrupt configuration register 10"]
pub mod gicd_icfgr10;
#[doc = "GICD interrupt configuration register 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr11](gicd_icfgr11) module"]
pub type GICD_ICFGR11 = crate::Reg<u32, _GICD_ICFGR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR11;
#[doc = "`read()` method returns [gicd_icfgr11::R](gicd_icfgr11::R) reader structure"]
impl crate::Readable for GICD_ICFGR11 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr11::W](gicd_icfgr11::W) writer structure"]
impl crate::Writable for GICD_ICFGR11 {}
#[doc = "GICD interrupt configuration register 11"]
pub mod gicd_icfgr11;
#[doc = "GICD interrupt configuration register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr12](gicd_icfgr12) module"]
pub type GICD_ICFGR12 = crate::Reg<u32, _GICD_ICFGR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR12;
#[doc = "`read()` method returns [gicd_icfgr12::R](gicd_icfgr12::R) reader structure"]
impl crate::Readable for GICD_ICFGR12 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr12::W](gicd_icfgr12::W) writer structure"]
impl crate::Writable for GICD_ICFGR12 {}
#[doc = "GICD interrupt configuration register 12"]
pub mod gicd_icfgr12;
#[doc = "GICD interrupt configuration register 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr13](gicd_icfgr13) module"]
pub type GICD_ICFGR13 = crate::Reg<u32, _GICD_ICFGR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR13;
#[doc = "`read()` method returns [gicd_icfgr13::R](gicd_icfgr13::R) reader structure"]
impl crate::Readable for GICD_ICFGR13 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr13::W](gicd_icfgr13::W) writer structure"]
impl crate::Writable for GICD_ICFGR13 {}
#[doc = "GICD interrupt configuration register 13"]
pub mod gicd_icfgr13;
#[doc = "GICD interrupt configuration register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr14](gicd_icfgr14) module"]
pub type GICD_ICFGR14 = crate::Reg<u32, _GICD_ICFGR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR14;
#[doc = "`read()` method returns [gicd_icfgr14::R](gicd_icfgr14::R) reader structure"]
impl crate::Readable for GICD_ICFGR14 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr14::W](gicd_icfgr14::W) writer structure"]
impl crate::Writable for GICD_ICFGR14 {}
#[doc = "GICD interrupt configuration register 14"]
pub mod gicd_icfgr14;
#[doc = "GICD interrupt configuration register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr15](gicd_icfgr15) module"]
pub type GICD_ICFGR15 = crate::Reg<u32, _GICD_ICFGR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR15;
#[doc = "`read()` method returns [gicd_icfgr15::R](gicd_icfgr15::R) reader structure"]
impl crate::Readable for GICD_ICFGR15 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr15::W](gicd_icfgr15::W) writer structure"]
impl crate::Writable for GICD_ICFGR15 {}
#[doc = "GICD interrupt configuration register 15"]
pub mod gicd_icfgr15;
#[doc = "GICD interrupt configuration register 16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr16](gicd_icfgr16) module"]
pub type GICD_ICFGR16 = crate::Reg<u32, _GICD_ICFGR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR16;
#[doc = "`read()` method returns [gicd_icfgr16::R](gicd_icfgr16::R) reader structure"]
impl crate::Readable for GICD_ICFGR16 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr16::W](gicd_icfgr16::W) writer structure"]
impl crate::Writable for GICD_ICFGR16 {}
#[doc = "GICD interrupt configuration register 16"]
pub mod gicd_icfgr16;
#[doc = "GICD interrupt configuration register 17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icfgr17](gicd_icfgr17) module"]
pub type GICD_ICFGR17 = crate::Reg<u32, _GICD_ICFGR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_ICFGR17;
#[doc = "`read()` method returns [gicd_icfgr17::R](gicd_icfgr17::R) reader structure"]
impl crate::Readable for GICD_ICFGR17 {}
#[doc = "`write(|w| ..)` method takes [gicd_icfgr17::W](gicd_icfgr17::W) writer structure"]
impl crate::Writable for GICD_ICFGR17 {}
#[doc = "GICD interrupt configuration register 17"]
pub mod gicd_icfgr17;
#[doc = "GICD private peripheral interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ppisr](gicd_ppisr) module"]
pub type GICD_PPISR = crate::Reg<u32, _GICD_PPISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_PPISR;
#[doc = "`read()` method returns [gicd_ppisr::R](gicd_ppisr::R) reader structure"]
impl crate::Readable for GICD_PPISR {}
#[doc = "GICD private peripheral interrupt status register"]
pub mod gicd_ppisr;
#[doc = "For interrupts ID = SPI number+32, from SPI \\[x*32+31\\]
to SPI \\[x*32\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spisr1](gicd_spisr1) module"]
pub type GICD_SPISR1 = crate::Reg<u32, _GICD_SPISR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_SPISR1;
#[doc = "`read()` method returns [gicd_spisr1::R](gicd_spisr1::R) reader structure"]
impl crate::Readable for GICD_SPISR1 {}
#[doc = "For interrupts ID = SPI number+32, from SPI \\[x*32+31\\]
to SPI \\[x*32\\]"]
pub mod gicd_spisr1;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spisr2](gicd_spisr2) module"]
pub type GICD_SPISR2 = crate::Reg<u32, _GICD_SPISR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_SPISR2;
#[doc = "`read()` method returns [gicd_spisr2::R](gicd_spisr2::R) reader structure"]
impl crate::Readable for GICD_SPISR2 {}
#[doc = "For interrupts ID"]
pub mod gicd_spisr2;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spisr3](gicd_spisr3) module"]
pub type GICD_SPISR3 = crate::Reg<u32, _GICD_SPISR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_SPISR3;
#[doc = "`read()` method returns [gicd_spisr3::R](gicd_spisr3::R) reader structure"]
impl crate::Readable for GICD_SPISR3 {}
#[doc = "For interrupts ID"]
pub mod gicd_spisr3;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spisr4](gicd_spisr4) module"]
pub type GICD_SPISR4 = crate::Reg<u32, _GICD_SPISR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_SPISR4;
#[doc = "`read()` method returns [gicd_spisr4::R](gicd_spisr4::R) reader structure"]
impl crate::Readable for GICD_SPISR4 {}
#[doc = "For interrupts ID"]
pub mod gicd_spisr4;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spisr5](gicd_spisr5) module"]
pub type GICD_SPISR5 = crate::Reg<u32, _GICD_SPISR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_SPISR5;
#[doc = "`read()` method returns [gicd_spisr5::R](gicd_spisr5::R) reader structure"]
impl crate::Readable for GICD_SPISR5 {}
#[doc = "For interrupts ID"]
pub mod gicd_spisr5;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spisr6](gicd_spisr6) module"]
pub type GICD_SPISR6 = crate::Reg<u32, _GICD_SPISR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_SPISR6;
#[doc = "`read()` method returns [gicd_spisr6::R](gicd_spisr6::R) reader structure"]
impl crate::Readable for GICD_SPISR6 {}
#[doc = "For interrupts ID"]
pub mod gicd_spisr6;
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spisr7](gicd_spisr7) module"]
pub type GICD_SPISR7 = crate::Reg<u32, _GICD_SPISR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_SPISR7;
#[doc = "`read()` method returns [gicd_spisr7::R](gicd_spisr7::R) reader structure"]
impl crate::Readable for GICD_SPISR7 {}
#[doc = "For interrupts ID"]
pub mod gicd_spisr7;
#[doc = "GICD software generated interrupt register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_sgir](gicd_sgir) module"]
pub type GICD_SGIR = crate::Reg<u32, _GICD_SGIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_SGIR;
#[doc = "`write(|w| ..)` method takes [gicd_sgir::W](gicd_sgir::W) writer structure"]
impl crate::Writable for GICD_SGIR {}
#[doc = "GICD software generated interrupt register"]
pub mod gicd_sgir;
#[doc = "For SGI x*4 to SGI x*4+3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_cpendsgir0](gicd_cpendsgir0) module"]
pub type GICD_CPENDSGIR0 = crate::Reg<u32, _GICD_CPENDSGIR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_CPENDSGIR0;
#[doc = "`read()` method returns [gicd_cpendsgir0::R](gicd_cpendsgir0::R) reader structure"]
impl crate::Readable for GICD_CPENDSGIR0 {}
#[doc = "`write(|w| ..)` method takes [gicd_cpendsgir0::W](gicd_cpendsgir0::W) writer structure"]
impl crate::Writable for GICD_CPENDSGIR0 {}
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_cpendsgir0;
#[doc = "For SGI x*4 to SGI x*4+3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_cpendsgir1](gicd_cpendsgir1) module"]
pub type GICD_CPENDSGIR1 = crate::Reg<u32, _GICD_CPENDSGIR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_CPENDSGIR1;
#[doc = "`read()` method returns [gicd_cpendsgir1::R](gicd_cpendsgir1::R) reader structure"]
impl crate::Readable for GICD_CPENDSGIR1 {}
#[doc = "`write(|w| ..)` method takes [gicd_cpendsgir1::W](gicd_cpendsgir1::W) writer structure"]
impl crate::Writable for GICD_CPENDSGIR1 {}
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_cpendsgir1;
#[doc = "For SGI x*4 to SGI x*4+3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_cpendsgir2](gicd_cpendsgir2) module"]
pub type GICD_CPENDSGIR2 = crate::Reg<u32, _GICD_CPENDSGIR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_CPENDSGIR2;
#[doc = "`read()` method returns [gicd_cpendsgir2::R](gicd_cpendsgir2::R) reader structure"]
impl crate::Readable for GICD_CPENDSGIR2 {}
#[doc = "`write(|w| ..)` method takes [gicd_cpendsgir2::W](gicd_cpendsgir2::W) writer structure"]
impl crate::Writable for GICD_CPENDSGIR2 {}
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_cpendsgir2;
#[doc = "For SGI x*4 to SGI x*4+3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_cpendsgir3](gicd_cpendsgir3) module"]
pub type GICD_CPENDSGIR3 = crate::Reg<u32, _GICD_CPENDSGIR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_CPENDSGIR3;
#[doc = "`read()` method returns [gicd_cpendsgir3::R](gicd_cpendsgir3::R) reader structure"]
impl crate::Readable for GICD_CPENDSGIR3 {}
#[doc = "`write(|w| ..)` method takes [gicd_cpendsgir3::W](gicd_cpendsgir3::W) writer structure"]
impl crate::Writable for GICD_CPENDSGIR3 {}
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_cpendsgir3;
#[doc = "For SGI x*4 to SGI x*4+3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spendsgir0](gicd_spendsgir0) module"]
pub type GICD_SPENDSGIR0 = crate::Reg<u32, _GICD_SPENDSGIR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_SPENDSGIR0;
#[doc = "`read()` method returns [gicd_spendsgir0::R](gicd_spendsgir0::R) reader structure"]
impl crate::Readable for GICD_SPENDSGIR0 {}
#[doc = "`write(|w| ..)` method takes [gicd_spendsgir0::W](gicd_spendsgir0::W) writer structure"]
impl crate::Writable for GICD_SPENDSGIR0 {}
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_spendsgir0;
#[doc = "For SGI x*4 to SGI x*4+3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spendsgir1](gicd_spendsgir1) module"]
pub type GICD_SPENDSGIR1 = crate::Reg<u32, _GICD_SPENDSGIR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_SPENDSGIR1;
#[doc = "`read()` method returns [gicd_spendsgir1::R](gicd_spendsgir1::R) reader structure"]
impl crate::Readable for GICD_SPENDSGIR1 {}
#[doc = "`write(|w| ..)` method takes [gicd_spendsgir1::W](gicd_spendsgir1::W) writer structure"]
impl crate::Writable for GICD_SPENDSGIR1 {}
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_spendsgir1;
#[doc = "For SGI x*4 to SGI x*4+3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spendsgir2](gicd_spendsgir2) module"]
pub type GICD_SPENDSGIR2 = crate::Reg<u32, _GICD_SPENDSGIR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_SPENDSGIR2;
#[doc = "`read()` method returns [gicd_spendsgir2::R](gicd_spendsgir2::R) reader structure"]
impl crate::Readable for GICD_SPENDSGIR2 {}
#[doc = "`write(|w| ..)` method takes [gicd_spendsgir2::W](gicd_spendsgir2::W) writer structure"]
impl crate::Writable for GICD_SPENDSGIR2 {}
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_spendsgir2;
#[doc = "For SGI x*4 to SGI x*4+3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spendsgir3](gicd_spendsgir3) module"]
pub type GICD_SPENDSGIR3 = crate::Reg<u32, _GICD_SPENDSGIR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_SPENDSGIR3;
#[doc = "`read()` method returns [gicd_spendsgir3::R](gicd_spendsgir3::R) reader structure"]
impl crate::Readable for GICD_SPENDSGIR3 {}
#[doc = "`write(|w| ..)` method takes [gicd_spendsgir3::W](gicd_spendsgir3::W) writer structure"]
impl crate::Writable for GICD_SPENDSGIR3 {}
#[doc = "For SGI x*4 to SGI x*4+3"]
pub mod gicd_spendsgir3;
#[doc = "GICD peripheral ID4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr4](gicd_pidr4) module"]
pub type GICD_PIDR4 = crate::Reg<u32, _GICD_PIDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_PIDR4;
#[doc = "`read()` method returns [gicd_pidr4::R](gicd_pidr4::R) reader structure"]
impl crate::Readable for GICD_PIDR4 {}
#[doc = "GICD peripheral ID4 register"]
pub mod gicd_pidr4;
#[doc = "GICD peripheral ID5 to ID7 register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr5](gicd_pidr5) module"]
pub type GICD_PIDR5 = crate::Reg<u32, _GICD_PIDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_PIDR5;
#[doc = "`read()` method returns [gicd_pidr5::R](gicd_pidr5::R) reader structure"]
impl crate::Readable for GICD_PIDR5 {}
#[doc = "GICD peripheral ID5 to ID7 register 5"]
pub mod gicd_pidr5;
#[doc = "GICD peripheral ID5 to ID7 register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr6](gicd_pidr6) module"]
pub type GICD_PIDR6 = crate::Reg<u32, _GICD_PIDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_PIDR6;
#[doc = "`read()` method returns [gicd_pidr6::R](gicd_pidr6::R) reader structure"]
impl crate::Readable for GICD_PIDR6 {}
#[doc = "GICD peripheral ID5 to ID7 register 6"]
pub mod gicd_pidr6;
#[doc = "GICD peripheral ID5 to ID7 register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr7](gicd_pidr7) module"]
pub type GICD_PIDR7 = crate::Reg<u32, _GICD_PIDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_PIDR7;
#[doc = "`read()` method returns [gicd_pidr7::R](gicd_pidr7::R) reader structure"]
impl crate::Readable for GICD_PIDR7 {}
#[doc = "GICD peripheral ID5 to ID7 register 7"]
pub mod gicd_pidr7;
#[doc = "GICD peripheral ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr0](gicd_pidr0) module"]
pub type GICD_PIDR0 = crate::Reg<u32, _GICD_PIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_PIDR0;
#[doc = "`read()` method returns [gicd_pidr0::R](gicd_pidr0::R) reader structure"]
impl crate::Readable for GICD_PIDR0 {}
#[doc = "GICD peripheral ID0 register"]
pub mod gicd_pidr0;
#[doc = "GICD peripheral ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr1](gicd_pidr1) module"]
pub type GICD_PIDR1 = crate::Reg<u32, _GICD_PIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_PIDR1;
#[doc = "`read()` method returns [gicd_pidr1::R](gicd_pidr1::R) reader structure"]
impl crate::Readable for GICD_PIDR1 {}
#[doc = "GICD peripheral ID1 register"]
pub mod gicd_pidr1;
#[doc = "GICD peripheral ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr2](gicd_pidr2) module"]
pub type GICD_PIDR2 = crate::Reg<u32, _GICD_PIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_PIDR2;
#[doc = "`read()` method returns [gicd_pidr2::R](gicd_pidr2::R) reader structure"]
impl crate::Readable for GICD_PIDR2 {}
#[doc = "GICD peripheral ID2 register"]
pub mod gicd_pidr2;
#[doc = "GICD peripheral ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr3](gicd_pidr3) module"]
pub type GICD_PIDR3 = crate::Reg<u32, _GICD_PIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_PIDR3;
#[doc = "`read()` method returns [gicd_pidr3::R](gicd_pidr3::R) reader structure"]
impl crate::Readable for GICD_PIDR3 {}
#[doc = "GICD peripheral ID3 register"]
pub mod gicd_pidr3;
#[doc = "GICD component ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_cidr0](gicd_cidr0) module"]
pub type GICD_CIDR0 = crate::Reg<u32, _GICD_CIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_CIDR0;
#[doc = "`read()` method returns [gicd_cidr0::R](gicd_cidr0::R) reader structure"]
impl crate::Readable for GICD_CIDR0 {}
#[doc = "GICD component ID0 register"]
pub mod gicd_cidr0;
#[doc = "GICD component ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_cidr1](gicd_cidr1) module"]
pub type GICD_CIDR1 = crate::Reg<u32, _GICD_CIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_CIDR1;
#[doc = "`read()` method returns [gicd_cidr1::R](gicd_cidr1::R) reader structure"]
impl crate::Readable for GICD_CIDR1 {}
#[doc = "GICD component ID1 register"]
pub mod gicd_cidr1;
#[doc = "GICD component ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_cidr2](gicd_cidr2) module"]
pub type GICD_CIDR2 = crate::Reg<u32, _GICD_CIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_CIDR2;
#[doc = "`read()` method returns [gicd_cidr2::R](gicd_cidr2::R) reader structure"]
impl crate::Readable for GICD_CIDR2 {}
#[doc = "GICD component ID2 register"]
pub mod gicd_cidr2;
#[doc = "GICD component ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_cidr3](gicd_cidr3) module"]
pub type GICD_CIDR3 = crate::Reg<u32, _GICD_CIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICD_CIDR3;
#[doc = "`read()` method returns [gicd_cidr3::R](gicd_cidr3::R) reader structure"]
impl crate::Readable for GICD_CIDR3 {}
#[doc = "GICD component ID3 register"]
pub mod gicd_cidr3;
