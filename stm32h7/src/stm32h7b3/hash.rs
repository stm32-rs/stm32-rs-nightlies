#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - data input register"]
    pub din: crate::Reg<din::DIN_SPEC>,
    #[doc = "0x08 - start register"]
    pub str: crate::Reg<str::STR_SPEC>,
    #[doc = "0x0c - digest registers"]
    pub hr0: crate::Reg<hr0::HR0_SPEC>,
    #[doc = "0x10 - digest registers"]
    pub hr1: crate::Reg<hr1::HR1_SPEC>,
    #[doc = "0x14 - digest registers"]
    pub hr2: crate::Reg<hr2::HR2_SPEC>,
    #[doc = "0x18 - digest registers"]
    pub hr3: crate::Reg<hr3::HR3_SPEC>,
    #[doc = "0x1c - digest registers"]
    pub hr4: crate::Reg<hr4::HR4_SPEC>,
    #[doc = "0x20 - interrupt enable register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x24 - status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    _reserved10: [u8; 0xd0],
    #[doc = "0xf8 - context swap registers"]
    pub csr0: crate::Reg<csr0::CSR0_SPEC>,
    #[doc = "0xfc - context swap registers"]
    pub csr1: crate::Reg<csr1::CSR1_SPEC>,
    #[doc = "0x100 - context swap registers"]
    pub csr2: crate::Reg<csr2::CSR2_SPEC>,
    #[doc = "0x104 - context swap registers"]
    pub csr3: crate::Reg<csr3::CSR3_SPEC>,
    #[doc = "0x108 - context swap registers"]
    pub csr4: crate::Reg<csr4::CSR4_SPEC>,
    #[doc = "0x10c - context swap registers"]
    pub csr5: crate::Reg<csr5::CSR5_SPEC>,
    #[doc = "0x110 - context swap registers"]
    pub csr6: crate::Reg<csr6::CSR6_SPEC>,
    #[doc = "0x114 - context swap registers"]
    pub csr7: crate::Reg<csr7::CSR7_SPEC>,
    #[doc = "0x118 - context swap registers"]
    pub csr8: crate::Reg<csr8::CSR8_SPEC>,
    #[doc = "0x11c - context swap registers"]
    pub csr9: crate::Reg<csr9::CSR9_SPEC>,
    #[doc = "0x120 - context swap registers"]
    pub csr10: crate::Reg<csr10::CSR10_SPEC>,
    #[doc = "0x124 - context swap registers"]
    pub csr11: crate::Reg<csr11::CSR11_SPEC>,
    #[doc = "0x128 - context swap registers"]
    pub csr12: crate::Reg<csr12::CSR12_SPEC>,
    #[doc = "0x12c - context swap registers"]
    pub csr13: crate::Reg<csr13::CSR13_SPEC>,
    #[doc = "0x130 - context swap registers"]
    pub csr14: crate::Reg<csr14::CSR14_SPEC>,
    #[doc = "0x134 - context swap registers"]
    pub csr15: crate::Reg<csr15::CSR15_SPEC>,
    #[doc = "0x138 - context swap registers"]
    pub csr16: crate::Reg<csr16::CSR16_SPEC>,
    #[doc = "0x13c - context swap registers"]
    pub csr17: crate::Reg<csr17::CSR17_SPEC>,
    #[doc = "0x140 - context swap registers"]
    pub csr18: crate::Reg<csr18::CSR18_SPEC>,
    #[doc = "0x144 - context swap registers"]
    pub csr19: crate::Reg<csr19::CSR19_SPEC>,
    #[doc = "0x148 - context swap registers"]
    pub csr20: crate::Reg<csr20::CSR20_SPEC>,
    #[doc = "0x14c - context swap registers"]
    pub csr21: crate::Reg<csr21::CSR21_SPEC>,
    #[doc = "0x150 - context swap registers"]
    pub csr22: crate::Reg<csr22::CSR22_SPEC>,
    #[doc = "0x154 - context swap registers"]
    pub csr23: crate::Reg<csr23::CSR23_SPEC>,
    #[doc = "0x158 - context swap registers"]
    pub csr24: crate::Reg<csr24::CSR24_SPEC>,
    #[doc = "0x15c - context swap registers"]
    pub csr25: crate::Reg<csr25::CSR25_SPEC>,
    #[doc = "0x160 - context swap registers"]
    pub csr26: crate::Reg<csr26::CSR26_SPEC>,
    #[doc = "0x164 - context swap registers"]
    pub csr27: crate::Reg<csr27::CSR27_SPEC>,
    #[doc = "0x168 - context swap registers"]
    pub csr28: crate::Reg<csr28::CSR28_SPEC>,
    #[doc = "0x16c - context swap registers"]
    pub csr29: crate::Reg<csr29::CSR29_SPEC>,
    #[doc = "0x170 - context swap registers"]
    pub csr30: crate::Reg<csr30::CSR30_SPEC>,
    #[doc = "0x174 - context swap registers"]
    pub csr31: crate::Reg<csr31::CSR31_SPEC>,
    #[doc = "0x178 - context swap registers"]
    pub csr32: crate::Reg<csr32::CSR32_SPEC>,
    #[doc = "0x17c - context swap registers"]
    pub csr33: crate::Reg<csr33::CSR33_SPEC>,
    #[doc = "0x180 - context swap registers"]
    pub csr34: crate::Reg<csr34::CSR34_SPEC>,
    #[doc = "0x184 - context swap registers"]
    pub csr35: crate::Reg<csr35::CSR35_SPEC>,
    #[doc = "0x188 - context swap registers"]
    pub csr36: crate::Reg<csr36::CSR36_SPEC>,
    #[doc = "0x18c - context swap registers"]
    pub csr37: crate::Reg<csr37::CSR37_SPEC>,
    #[doc = "0x190 - context swap registers"]
    pub csr38: crate::Reg<csr38::CSR38_SPEC>,
    #[doc = "0x194 - context swap registers"]
    pub csr39: crate::Reg<csr39::CSR39_SPEC>,
    #[doc = "0x198 - context swap registers"]
    pub csr40: crate::Reg<csr40::CSR40_SPEC>,
    #[doc = "0x19c - context swap registers"]
    pub csr41: crate::Reg<csr41::CSR41_SPEC>,
    #[doc = "0x1a0 - context swap registers"]
    pub csr42: crate::Reg<csr42::CSR42_SPEC>,
    #[doc = "0x1a4 - context swap registers"]
    pub csr43: crate::Reg<csr43::CSR43_SPEC>,
    #[doc = "0x1a8 - context swap registers"]
    pub csr44: crate::Reg<csr44::CSR44_SPEC>,
    #[doc = "0x1ac - context swap registers"]
    pub csr45: crate::Reg<csr45::CSR45_SPEC>,
    #[doc = "0x1b0 - context swap registers"]
    pub csr46: crate::Reg<csr46::CSR46_SPEC>,
    #[doc = "0x1b4 - context swap registers"]
    pub csr47: crate::Reg<csr47::CSR47_SPEC>,
    #[doc = "0x1b8 - context swap registers"]
    pub csr48: crate::Reg<csr48::CSR48_SPEC>,
    #[doc = "0x1bc - context swap registers"]
    pub csr49: crate::Reg<csr49::CSR49_SPEC>,
    #[doc = "0x1c0 - context swap registers"]
    pub csr50: crate::Reg<csr50::CSR50_SPEC>,
    #[doc = "0x1c4 - context swap registers"]
    pub csr51: crate::Reg<csr51::CSR51_SPEC>,
    #[doc = "0x1c8 - context swap registers"]
    pub csr52: crate::Reg<csr52::CSR52_SPEC>,
    #[doc = "0x1cc - context swap registers"]
    pub csr53: crate::Reg<csr53::CSR53_SPEC>,
    _reserved64: [u8; 0x0140],
    #[doc = "0x310 - HASH digest register"]
    pub hash_hr0: crate::Reg<hash_hr0::HASH_HR0_SPEC>,
    #[doc = "0x314 - read-only"]
    pub hash_hr1: crate::Reg<hash_hr1::HASH_HR1_SPEC>,
    #[doc = "0x318 - read-only"]
    pub hash_hr2: crate::Reg<hash_hr2::HASH_HR2_SPEC>,
    #[doc = "0x31c - read-only"]
    pub hash_hr3: crate::Reg<hash_hr3::HASH_HR3_SPEC>,
    #[doc = "0x320 - read-only"]
    pub hash_hr4: crate::Reg<hash_hr4::HASH_HR4_SPEC>,
    #[doc = "0x324 - read-only"]
    pub hash_hr5: crate::Reg<hash_hr5::HASH_HR5_SPEC>,
    #[doc = "0x328 - read-only"]
    pub hash_hr6: crate::Reg<hash_hr6::HASH_HR6_SPEC>,
    #[doc = "0x32c - read-only"]
    pub hash_hr7: crate::Reg<hash_hr7::HASH_HR7_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "DIN register accessor: an alias for `Reg<DIN_SPEC>`"]
pub type DIN = crate::Reg<din::DIN_SPEC>;
#[doc = "data input register"]
pub mod din;
#[doc = "STR register accessor: an alias for `Reg<STR_SPEC>`"]
pub type STR = crate::Reg<str::STR_SPEC>;
#[doc = "start register"]
pub mod str;
#[doc = "HR0 register accessor: an alias for `Reg<HR0_SPEC>`"]
pub type HR0 = crate::Reg<hr0::HR0_SPEC>;
#[doc = "digest registers"]
pub mod hr0;
#[doc = "HR1 register accessor: an alias for `Reg<HR1_SPEC>`"]
pub type HR1 = crate::Reg<hr1::HR1_SPEC>;
#[doc = "digest registers"]
pub mod hr1;
#[doc = "HR2 register accessor: an alias for `Reg<HR2_SPEC>`"]
pub type HR2 = crate::Reg<hr2::HR2_SPEC>;
#[doc = "digest registers"]
pub mod hr2;
#[doc = "HR3 register accessor: an alias for `Reg<HR3_SPEC>`"]
pub type HR3 = crate::Reg<hr3::HR3_SPEC>;
#[doc = "digest registers"]
pub mod hr3;
#[doc = "HR4 register accessor: an alias for `Reg<HR4_SPEC>`"]
pub type HR4 = crate::Reg<hr4::HR4_SPEC>;
#[doc = "digest registers"]
pub mod hr4;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "interrupt enable register"]
pub mod imr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "CSR0 register accessor: an alias for `Reg<CSR0_SPEC>`"]
pub type CSR0 = crate::Reg<csr0::CSR0_SPEC>;
#[doc = "context swap registers"]
pub mod csr0;
#[doc = "CSR1 register accessor: an alias for `Reg<CSR1_SPEC>`"]
pub type CSR1 = crate::Reg<csr1::CSR1_SPEC>;
#[doc = "context swap registers"]
pub mod csr1;
#[doc = "CSR2 register accessor: an alias for `Reg<CSR2_SPEC>`"]
pub type CSR2 = crate::Reg<csr2::CSR2_SPEC>;
#[doc = "context swap registers"]
pub mod csr2;
#[doc = "CSR3 register accessor: an alias for `Reg<CSR3_SPEC>`"]
pub type CSR3 = crate::Reg<csr3::CSR3_SPEC>;
#[doc = "context swap registers"]
pub mod csr3;
#[doc = "CSR4 register accessor: an alias for `Reg<CSR4_SPEC>`"]
pub type CSR4 = crate::Reg<csr4::CSR4_SPEC>;
#[doc = "context swap registers"]
pub mod csr4;
#[doc = "CSR5 register accessor: an alias for `Reg<CSR5_SPEC>`"]
pub type CSR5 = crate::Reg<csr5::CSR5_SPEC>;
#[doc = "context swap registers"]
pub mod csr5;
#[doc = "CSR6 register accessor: an alias for `Reg<CSR6_SPEC>`"]
pub type CSR6 = crate::Reg<csr6::CSR6_SPEC>;
#[doc = "context swap registers"]
pub mod csr6;
#[doc = "CSR7 register accessor: an alias for `Reg<CSR7_SPEC>`"]
pub type CSR7 = crate::Reg<csr7::CSR7_SPEC>;
#[doc = "context swap registers"]
pub mod csr7;
#[doc = "CSR8 register accessor: an alias for `Reg<CSR8_SPEC>`"]
pub type CSR8 = crate::Reg<csr8::CSR8_SPEC>;
#[doc = "context swap registers"]
pub mod csr8;
#[doc = "CSR9 register accessor: an alias for `Reg<CSR9_SPEC>`"]
pub type CSR9 = crate::Reg<csr9::CSR9_SPEC>;
#[doc = "context swap registers"]
pub mod csr9;
#[doc = "CSR10 register accessor: an alias for `Reg<CSR10_SPEC>`"]
pub type CSR10 = crate::Reg<csr10::CSR10_SPEC>;
#[doc = "context swap registers"]
pub mod csr10;
#[doc = "CSR11 register accessor: an alias for `Reg<CSR11_SPEC>`"]
pub type CSR11 = crate::Reg<csr11::CSR11_SPEC>;
#[doc = "context swap registers"]
pub mod csr11;
#[doc = "CSR12 register accessor: an alias for `Reg<CSR12_SPEC>`"]
pub type CSR12 = crate::Reg<csr12::CSR12_SPEC>;
#[doc = "context swap registers"]
pub mod csr12;
#[doc = "CSR13 register accessor: an alias for `Reg<CSR13_SPEC>`"]
pub type CSR13 = crate::Reg<csr13::CSR13_SPEC>;
#[doc = "context swap registers"]
pub mod csr13;
#[doc = "CSR14 register accessor: an alias for `Reg<CSR14_SPEC>`"]
pub type CSR14 = crate::Reg<csr14::CSR14_SPEC>;
#[doc = "context swap registers"]
pub mod csr14;
#[doc = "CSR15 register accessor: an alias for `Reg<CSR15_SPEC>`"]
pub type CSR15 = crate::Reg<csr15::CSR15_SPEC>;
#[doc = "context swap registers"]
pub mod csr15;
#[doc = "CSR16 register accessor: an alias for `Reg<CSR16_SPEC>`"]
pub type CSR16 = crate::Reg<csr16::CSR16_SPEC>;
#[doc = "context swap registers"]
pub mod csr16;
#[doc = "CSR17 register accessor: an alias for `Reg<CSR17_SPEC>`"]
pub type CSR17 = crate::Reg<csr17::CSR17_SPEC>;
#[doc = "context swap registers"]
pub mod csr17;
#[doc = "CSR18 register accessor: an alias for `Reg<CSR18_SPEC>`"]
pub type CSR18 = crate::Reg<csr18::CSR18_SPEC>;
#[doc = "context swap registers"]
pub mod csr18;
#[doc = "CSR19 register accessor: an alias for `Reg<CSR19_SPEC>`"]
pub type CSR19 = crate::Reg<csr19::CSR19_SPEC>;
#[doc = "context swap registers"]
pub mod csr19;
#[doc = "CSR20 register accessor: an alias for `Reg<CSR20_SPEC>`"]
pub type CSR20 = crate::Reg<csr20::CSR20_SPEC>;
#[doc = "context swap registers"]
pub mod csr20;
#[doc = "CSR21 register accessor: an alias for `Reg<CSR21_SPEC>`"]
pub type CSR21 = crate::Reg<csr21::CSR21_SPEC>;
#[doc = "context swap registers"]
pub mod csr21;
#[doc = "CSR22 register accessor: an alias for `Reg<CSR22_SPEC>`"]
pub type CSR22 = crate::Reg<csr22::CSR22_SPEC>;
#[doc = "context swap registers"]
pub mod csr22;
#[doc = "CSR23 register accessor: an alias for `Reg<CSR23_SPEC>`"]
pub type CSR23 = crate::Reg<csr23::CSR23_SPEC>;
#[doc = "context swap registers"]
pub mod csr23;
#[doc = "CSR24 register accessor: an alias for `Reg<CSR24_SPEC>`"]
pub type CSR24 = crate::Reg<csr24::CSR24_SPEC>;
#[doc = "context swap registers"]
pub mod csr24;
#[doc = "CSR25 register accessor: an alias for `Reg<CSR25_SPEC>`"]
pub type CSR25 = crate::Reg<csr25::CSR25_SPEC>;
#[doc = "context swap registers"]
pub mod csr25;
#[doc = "CSR26 register accessor: an alias for `Reg<CSR26_SPEC>`"]
pub type CSR26 = crate::Reg<csr26::CSR26_SPEC>;
#[doc = "context swap registers"]
pub mod csr26;
#[doc = "CSR27 register accessor: an alias for `Reg<CSR27_SPEC>`"]
pub type CSR27 = crate::Reg<csr27::CSR27_SPEC>;
#[doc = "context swap registers"]
pub mod csr27;
#[doc = "CSR28 register accessor: an alias for `Reg<CSR28_SPEC>`"]
pub type CSR28 = crate::Reg<csr28::CSR28_SPEC>;
#[doc = "context swap registers"]
pub mod csr28;
#[doc = "CSR29 register accessor: an alias for `Reg<CSR29_SPEC>`"]
pub type CSR29 = crate::Reg<csr29::CSR29_SPEC>;
#[doc = "context swap registers"]
pub mod csr29;
#[doc = "CSR30 register accessor: an alias for `Reg<CSR30_SPEC>`"]
pub type CSR30 = crate::Reg<csr30::CSR30_SPEC>;
#[doc = "context swap registers"]
pub mod csr30;
#[doc = "CSR31 register accessor: an alias for `Reg<CSR31_SPEC>`"]
pub type CSR31 = crate::Reg<csr31::CSR31_SPEC>;
#[doc = "context swap registers"]
pub mod csr31;
#[doc = "CSR32 register accessor: an alias for `Reg<CSR32_SPEC>`"]
pub type CSR32 = crate::Reg<csr32::CSR32_SPEC>;
#[doc = "context swap registers"]
pub mod csr32;
#[doc = "CSR33 register accessor: an alias for `Reg<CSR33_SPEC>`"]
pub type CSR33 = crate::Reg<csr33::CSR33_SPEC>;
#[doc = "context swap registers"]
pub mod csr33;
#[doc = "CSR34 register accessor: an alias for `Reg<CSR34_SPEC>`"]
pub type CSR34 = crate::Reg<csr34::CSR34_SPEC>;
#[doc = "context swap registers"]
pub mod csr34;
#[doc = "CSR35 register accessor: an alias for `Reg<CSR35_SPEC>`"]
pub type CSR35 = crate::Reg<csr35::CSR35_SPEC>;
#[doc = "context swap registers"]
pub mod csr35;
#[doc = "CSR36 register accessor: an alias for `Reg<CSR36_SPEC>`"]
pub type CSR36 = crate::Reg<csr36::CSR36_SPEC>;
#[doc = "context swap registers"]
pub mod csr36;
#[doc = "CSR37 register accessor: an alias for `Reg<CSR37_SPEC>`"]
pub type CSR37 = crate::Reg<csr37::CSR37_SPEC>;
#[doc = "context swap registers"]
pub mod csr37;
#[doc = "CSR38 register accessor: an alias for `Reg<CSR38_SPEC>`"]
pub type CSR38 = crate::Reg<csr38::CSR38_SPEC>;
#[doc = "context swap registers"]
pub mod csr38;
#[doc = "CSR39 register accessor: an alias for `Reg<CSR39_SPEC>`"]
pub type CSR39 = crate::Reg<csr39::CSR39_SPEC>;
#[doc = "context swap registers"]
pub mod csr39;
#[doc = "CSR40 register accessor: an alias for `Reg<CSR40_SPEC>`"]
pub type CSR40 = crate::Reg<csr40::CSR40_SPEC>;
#[doc = "context swap registers"]
pub mod csr40;
#[doc = "CSR41 register accessor: an alias for `Reg<CSR41_SPEC>`"]
pub type CSR41 = crate::Reg<csr41::CSR41_SPEC>;
#[doc = "context swap registers"]
pub mod csr41;
#[doc = "CSR42 register accessor: an alias for `Reg<CSR42_SPEC>`"]
pub type CSR42 = crate::Reg<csr42::CSR42_SPEC>;
#[doc = "context swap registers"]
pub mod csr42;
#[doc = "CSR43 register accessor: an alias for `Reg<CSR43_SPEC>`"]
pub type CSR43 = crate::Reg<csr43::CSR43_SPEC>;
#[doc = "context swap registers"]
pub mod csr43;
#[doc = "CSR44 register accessor: an alias for `Reg<CSR44_SPEC>`"]
pub type CSR44 = crate::Reg<csr44::CSR44_SPEC>;
#[doc = "context swap registers"]
pub mod csr44;
#[doc = "CSR45 register accessor: an alias for `Reg<CSR45_SPEC>`"]
pub type CSR45 = crate::Reg<csr45::CSR45_SPEC>;
#[doc = "context swap registers"]
pub mod csr45;
#[doc = "CSR46 register accessor: an alias for `Reg<CSR46_SPEC>`"]
pub type CSR46 = crate::Reg<csr46::CSR46_SPEC>;
#[doc = "context swap registers"]
pub mod csr46;
#[doc = "CSR47 register accessor: an alias for `Reg<CSR47_SPEC>`"]
pub type CSR47 = crate::Reg<csr47::CSR47_SPEC>;
#[doc = "context swap registers"]
pub mod csr47;
#[doc = "CSR48 register accessor: an alias for `Reg<CSR48_SPEC>`"]
pub type CSR48 = crate::Reg<csr48::CSR48_SPEC>;
#[doc = "context swap registers"]
pub mod csr48;
#[doc = "CSR49 register accessor: an alias for `Reg<CSR49_SPEC>`"]
pub type CSR49 = crate::Reg<csr49::CSR49_SPEC>;
#[doc = "context swap registers"]
pub mod csr49;
#[doc = "CSR50 register accessor: an alias for `Reg<CSR50_SPEC>`"]
pub type CSR50 = crate::Reg<csr50::CSR50_SPEC>;
#[doc = "context swap registers"]
pub mod csr50;
#[doc = "CSR51 register accessor: an alias for `Reg<CSR51_SPEC>`"]
pub type CSR51 = crate::Reg<csr51::CSR51_SPEC>;
#[doc = "context swap registers"]
pub mod csr51;
#[doc = "CSR52 register accessor: an alias for `Reg<CSR52_SPEC>`"]
pub type CSR52 = crate::Reg<csr52::CSR52_SPEC>;
#[doc = "context swap registers"]
pub mod csr52;
#[doc = "CSR53 register accessor: an alias for `Reg<CSR53_SPEC>`"]
pub type CSR53 = crate::Reg<csr53::CSR53_SPEC>;
#[doc = "context swap registers"]
pub mod csr53;
#[doc = "HASH_HR0 register accessor: an alias for `Reg<HASH_HR0_SPEC>`"]
pub type HASH_HR0 = crate::Reg<hash_hr0::HASH_HR0_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr0;
#[doc = "HASH_HR1 register accessor: an alias for `Reg<HASH_HR1_SPEC>`"]
pub type HASH_HR1 = crate::Reg<hash_hr1::HASH_HR1_SPEC>;
#[doc = "read-only"]
pub mod hash_hr1;
#[doc = "HASH_HR2 register accessor: an alias for `Reg<HASH_HR2_SPEC>`"]
pub type HASH_HR2 = crate::Reg<hash_hr2::HASH_HR2_SPEC>;
#[doc = "read-only"]
pub mod hash_hr2;
#[doc = "HASH_HR3 register accessor: an alias for `Reg<HASH_HR3_SPEC>`"]
pub type HASH_HR3 = crate::Reg<hash_hr3::HASH_HR3_SPEC>;
#[doc = "read-only"]
pub mod hash_hr3;
#[doc = "HASH_HR4 register accessor: an alias for `Reg<HASH_HR4_SPEC>`"]
pub type HASH_HR4 = crate::Reg<hash_hr4::HASH_HR4_SPEC>;
#[doc = "read-only"]
pub mod hash_hr4;
#[doc = "HASH_HR5 register accessor: an alias for `Reg<HASH_HR5_SPEC>`"]
pub type HASH_HR5 = crate::Reg<hash_hr5::HASH_HR5_SPEC>;
#[doc = "read-only"]
pub mod hash_hr5;
#[doc = "HASH_HR6 register accessor: an alias for `Reg<HASH_HR6_SPEC>`"]
pub type HASH_HR6 = crate::Reg<hash_hr6::HASH_HR6_SPEC>;
#[doc = "read-only"]
pub mod hash_hr6;
#[doc = "HASH_HR7 register accessor: an alias for `Reg<HASH_HR7_SPEC>`"]
pub type HASH_HR7 = crate::Reg<hash_hr7::HASH_HR7_SPEC>;
#[doc = "read-only"]
pub mod hash_hr7;
