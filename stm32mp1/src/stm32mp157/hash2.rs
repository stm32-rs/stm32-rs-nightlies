#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hash_cr: HASH_CR,
    hash_din: HASH_DIN,
    hash_str: HASH_STR,
    hash_hr0: HASH_HR0,
    hash_hr1: HASH_HR1,
    hash_hr2: HASH_HR2,
    hash_hr3: HASH_HR3,
    hash_hr4: HASH_HR4,
    hash_imr: HASH_IMR,
    hash_sr: HASH_SR,
    _reserved10: [u8; 0xd0],
    hash_csr0: HASH_CSR0,
    hash_csr1: HASH_CSR1,
    hash_csr2: HASH_CSR2,
    hash_csr3: HASH_CSR3,
    hash_csr4: HASH_CSR4,
    hash_csr5: HASH_CSR5,
    hash_csr6: HASH_CSR6,
    hash_csr7: HASH_CSR7,
    hash_csr8: HASH_CSR8,
    hash_csr9: HASH_CSR9,
    hash_csr10: HASH_CSR10,
    hash_csr11: HASH_CSR11,
    hash_csr12: HASH_CSR12,
    hash_csr13: HASH_CSR13,
    hash_csr14: HASH_CSR14,
    hash_csr15: HASH_CSR15,
    hash_csr16: HASH_CSR16,
    hash_csr17: HASH_CSR17,
    hash_csr18: HASH_CSR18,
    hash_csr19: HASH_CSR19,
    hash_csr20: HASH_CSR20,
    hash_csr21: HASH_CSR21,
    hash_csr22: HASH_CSR22,
    hash_csr23: HASH_CSR23,
    hash_csr24: HASH_CSR24,
    hash_csr25: HASH_CSR25,
    hash_csr26: HASH_CSR26,
    hash_csr27: HASH_CSR27,
    hash_csr28: HASH_CSR28,
    hash_csr29: HASH_CSR29,
    hash_csr30: HASH_CSR30,
    hash_csr31: HASH_CSR31,
    hash_csr32: HASH_CSR32,
    hash_csr33: HASH_CSR33,
    hash_csr34: HASH_CSR34,
    hash_csr35: HASH_CSR35,
    hash_csr36: HASH_CSR36,
    hash_csr37: HASH_CSR37,
    hash_csr38: HASH_CSR38,
    hash_csr39: HASH_CSR39,
    hash_csr40: HASH_CSR40,
    hash_csr41: HASH_CSR41,
    hash_csr42: HASH_CSR42,
    hash_csr43: HASH_CSR43,
    hash_csr44: HASH_CSR44,
    hash_csr45: HASH_CSR45,
    hash_csr46: HASH_CSR46,
    hash_csr47: HASH_CSR47,
    hash_csr48: HASH_CSR48,
    hash_csr49: HASH_CSR49,
    hash_csr50: HASH_CSR50,
    hash_csr51: HASH_CSR51,
    hash_csr52: HASH_CSR52,
    hash_csr53: HASH_CSR53,
    _reserved64: [u8; 0x0154],
    hash_hr5: HASH_HR5,
    hash_hr6: HASH_HR6,
    hash_hr7: HASH_HR7,
    _reserved67: [u8; 0xc0],
    hash_hwcfgr: HASH_HWCFGR,
    hash_verr: HASH_VERR,
    hash_ipidr: HASH_IPIDR,
    hash_mid: HASH_MID,
}
impl RegisterBlock {
    #[doc = "0x00 - HASH control register"]
    #[inline(always)]
    pub const fn hash_cr(&self) -> &HASH_CR {
        &self.hash_cr
    }
    #[doc = "0x04 - HASH_DIN is the data input register."]
    #[inline(always)]
    pub const fn hash_din(&self) -> &HASH_DIN {
        &self.hash_din
    }
    #[doc = "0x08 - The HASH_STR register has two functions: It is used to define the number of valid bits in the last word of the message entered in the hash processor (that is the number of valid least significant bits in the last data written to the HASH_DIN register) It is used to start the processing of the last block in the message by writing the DCAL bit to 1"]
    #[inline(always)]
    pub const fn hash_str(&self) -> &HASH_STR {
        &self.hash_str
    }
    #[doc = "0x0c - HASH digest register 0"]
    #[inline(always)]
    pub const fn hash_hr0(&self) -> &HASH_HR0 {
        &self.hash_hr0
    }
    #[doc = "0x10 - HASH digest register 1"]
    #[inline(always)]
    pub const fn hash_hr1(&self) -> &HASH_HR1 {
        &self.hash_hr1
    }
    #[doc = "0x14 - HASH digest register 2"]
    #[inline(always)]
    pub const fn hash_hr2(&self) -> &HASH_HR2 {
        &self.hash_hr2
    }
    #[doc = "0x18 - HASH digest register 3"]
    #[inline(always)]
    pub const fn hash_hr3(&self) -> &HASH_HR3 {
        &self.hash_hr3
    }
    #[doc = "0x1c - HASH digest register 4"]
    #[inline(always)]
    pub const fn hash_hr4(&self) -> &HASH_HR4 {
        &self.hash_hr4
    }
    #[doc = "0x20 - HASH interrupt enable register"]
    #[inline(always)]
    pub const fn hash_imr(&self) -> &HASH_IMR {
        &self.hash_imr
    }
    #[doc = "0x24 - HASH status register"]
    #[inline(always)]
    pub const fn hash_sr(&self) -> &HASH_SR {
        &self.hash_sr
    }
    #[doc = "0xf8 - These registers contain the complete internal register states of the hash processor. They are useful when a context swap has to be done because a high-priority task needs to use the hash processor while it is already used by another task. When such an event occurs, the HASH_CSRx registers have to be read and the read values have to be saved in the system memory space. Then the hash processor can be used by the preemptive task, and when the hash computation is complete, the saved context can be read from memory and written back into the HASH_CSRx registers."]
    #[inline(always)]
    pub const fn hash_csr0(&self) -> &HASH_CSR0 {
        &self.hash_csr0
    }
    #[doc = "0xfc - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr1(&self) -> &HASH_CSR1 {
        &self.hash_csr1
    }
    #[doc = "0x100 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr2(&self) -> &HASH_CSR2 {
        &self.hash_csr2
    }
    #[doc = "0x104 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr3(&self) -> &HASH_CSR3 {
        &self.hash_csr3
    }
    #[doc = "0x108 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr4(&self) -> &HASH_CSR4 {
        &self.hash_csr4
    }
    #[doc = "0x10c - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr5(&self) -> &HASH_CSR5 {
        &self.hash_csr5
    }
    #[doc = "0x110 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr6(&self) -> &HASH_CSR6 {
        &self.hash_csr6
    }
    #[doc = "0x114 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr7(&self) -> &HASH_CSR7 {
        &self.hash_csr7
    }
    #[doc = "0x118 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr8(&self) -> &HASH_CSR8 {
        &self.hash_csr8
    }
    #[doc = "0x11c - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr9(&self) -> &HASH_CSR9 {
        &self.hash_csr9
    }
    #[doc = "0x120 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr10(&self) -> &HASH_CSR10 {
        &self.hash_csr10
    }
    #[doc = "0x124 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr11(&self) -> &HASH_CSR11 {
        &self.hash_csr11
    }
    #[doc = "0x128 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr12(&self) -> &HASH_CSR12 {
        &self.hash_csr12
    }
    #[doc = "0x12c - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr13(&self) -> &HASH_CSR13 {
        &self.hash_csr13
    }
    #[doc = "0x130 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr14(&self) -> &HASH_CSR14 {
        &self.hash_csr14
    }
    #[doc = "0x134 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr15(&self) -> &HASH_CSR15 {
        &self.hash_csr15
    }
    #[doc = "0x138 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr16(&self) -> &HASH_CSR16 {
        &self.hash_csr16
    }
    #[doc = "0x13c - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr17(&self) -> &HASH_CSR17 {
        &self.hash_csr17
    }
    #[doc = "0x140 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr18(&self) -> &HASH_CSR18 {
        &self.hash_csr18
    }
    #[doc = "0x144 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr19(&self) -> &HASH_CSR19 {
        &self.hash_csr19
    }
    #[doc = "0x148 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr20(&self) -> &HASH_CSR20 {
        &self.hash_csr20
    }
    #[doc = "0x14c - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr21(&self) -> &HASH_CSR21 {
        &self.hash_csr21
    }
    #[doc = "0x150 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr22(&self) -> &HASH_CSR22 {
        &self.hash_csr22
    }
    #[doc = "0x154 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr23(&self) -> &HASH_CSR23 {
        &self.hash_csr23
    }
    #[doc = "0x158 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr24(&self) -> &HASH_CSR24 {
        &self.hash_csr24
    }
    #[doc = "0x15c - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr25(&self) -> &HASH_CSR25 {
        &self.hash_csr25
    }
    #[doc = "0x160 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr26(&self) -> &HASH_CSR26 {
        &self.hash_csr26
    }
    #[doc = "0x164 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr27(&self) -> &HASH_CSR27 {
        &self.hash_csr27
    }
    #[doc = "0x168 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr28(&self) -> &HASH_CSR28 {
        &self.hash_csr28
    }
    #[doc = "0x16c - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr29(&self) -> &HASH_CSR29 {
        &self.hash_csr29
    }
    #[doc = "0x170 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr30(&self) -> &HASH_CSR30 {
        &self.hash_csr30
    }
    #[doc = "0x174 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr31(&self) -> &HASH_CSR31 {
        &self.hash_csr31
    }
    #[doc = "0x178 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr32(&self) -> &HASH_CSR32 {
        &self.hash_csr32
    }
    #[doc = "0x17c - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr33(&self) -> &HASH_CSR33 {
        &self.hash_csr33
    }
    #[doc = "0x180 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr34(&self) -> &HASH_CSR34 {
        &self.hash_csr34
    }
    #[doc = "0x184 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr35(&self) -> &HASH_CSR35 {
        &self.hash_csr35
    }
    #[doc = "0x188 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr36(&self) -> &HASH_CSR36 {
        &self.hash_csr36
    }
    #[doc = "0x18c - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr37(&self) -> &HASH_CSR37 {
        &self.hash_csr37
    }
    #[doc = "0x190 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr38(&self) -> &HASH_CSR38 {
        &self.hash_csr38
    }
    #[doc = "0x194 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr39(&self) -> &HASH_CSR39 {
        &self.hash_csr39
    }
    #[doc = "0x198 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr40(&self) -> &HASH_CSR40 {
        &self.hash_csr40
    }
    #[doc = "0x19c - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr41(&self) -> &HASH_CSR41 {
        &self.hash_csr41
    }
    #[doc = "0x1a0 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr42(&self) -> &HASH_CSR42 {
        &self.hash_csr42
    }
    #[doc = "0x1a4 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr43(&self) -> &HASH_CSR43 {
        &self.hash_csr43
    }
    #[doc = "0x1a8 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr44(&self) -> &HASH_CSR44 {
        &self.hash_csr44
    }
    #[doc = "0x1ac - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr45(&self) -> &HASH_CSR45 {
        &self.hash_csr45
    }
    #[doc = "0x1b0 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr46(&self) -> &HASH_CSR46 {
        &self.hash_csr46
    }
    #[doc = "0x1b4 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr47(&self) -> &HASH_CSR47 {
        &self.hash_csr47
    }
    #[doc = "0x1b8 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr48(&self) -> &HASH_CSR48 {
        &self.hash_csr48
    }
    #[doc = "0x1bc - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr49(&self) -> &HASH_CSR49 {
        &self.hash_csr49
    }
    #[doc = "0x1c0 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr50(&self) -> &HASH_CSR50 {
        &self.hash_csr50
    }
    #[doc = "0x1c4 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr51(&self) -> &HASH_CSR51 {
        &self.hash_csr51
    }
    #[doc = "0x1c8 - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr52(&self) -> &HASH_CSR52 {
        &self.hash_csr52
    }
    #[doc = "0x1cc - HASH context swap registers"]
    #[inline(always)]
    pub const fn hash_csr53(&self) -> &HASH_CSR53 {
        &self.hash_csr53
    }
    #[doc = "0x324 - HASH digest register 5"]
    #[inline(always)]
    pub const fn hash_hr5(&self) -> &HASH_HR5 {
        &self.hash_hr5
    }
    #[doc = "0x328 - HASH digest register 6"]
    #[inline(always)]
    pub const fn hash_hr6(&self) -> &HASH_HR6 {
        &self.hash_hr6
    }
    #[doc = "0x32c - HASH digest register 7"]
    #[inline(always)]
    pub const fn hash_hr7(&self) -> &HASH_HR7 {
        &self.hash_hr7
    }
    #[doc = "0x3f0 - HASH Hardware Configuration Register"]
    #[inline(always)]
    pub const fn hash_hwcfgr(&self) -> &HASH_HWCFGR {
        &self.hash_hwcfgr
    }
    #[doc = "0x3f4 - HASH Version Register"]
    #[inline(always)]
    pub const fn hash_verr(&self) -> &HASH_VERR {
        &self.hash_verr
    }
    #[doc = "0x3f8 - HASH Identification"]
    #[inline(always)]
    pub const fn hash_ipidr(&self) -> &HASH_IPIDR {
        &self.hash_ipidr
    }
    #[doc = "0x3fc - HASH Hardware Magic ID"]
    #[inline(always)]
    pub const fn hash_mid(&self) -> &HASH_MID {
        &self.hash_mid
    }
}
#[doc = "HASH_CR (rw) register accessor: HASH control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_cr`]
module"]
pub type HASH_CR = crate::Reg<hash_cr::HASH_CRrs>;
#[doc = "HASH control register"]
pub mod hash_cr;
#[doc = "HASH_DIN (rw) register accessor: HASH_DIN is the data input register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_din::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_din::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_din`]
module"]
pub type HASH_DIN = crate::Reg<hash_din::HASH_DINrs>;
#[doc = "HASH_DIN is the data input register."]
pub mod hash_din;
#[doc = "HASH_STR (rw) register accessor: The HASH_STR register has two functions: It is used to define the number of valid bits in the last word of the message entered in the hash processor (that is the number of valid least significant bits in the last data written to the HASH_DIN register) It is used to start the processing of the last block in the message by writing the DCAL bit to 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_str::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_str::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_str`]
module"]
pub type HASH_STR = crate::Reg<hash_str::HASH_STRrs>;
#[doc = "The HASH_STR register has two functions: It is used to define the number of valid bits in the last word of the message entered in the hash processor (that is the number of valid least significant bits in the last data written to the HASH_DIN register) It is used to start the processing of the last block in the message by writing the DCAL bit to 1"]
pub mod hash_str;
#[doc = "HASH_HR0 (r) register accessor: HASH digest register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_hr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_hr0`]
module"]
pub type HASH_HR0 = crate::Reg<hash_hr0::HASH_HR0rs>;
#[doc = "HASH digest register 0"]
pub mod hash_hr0;
#[doc = "HASH_HR1 (r) register accessor: HASH digest register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_hr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_hr1`]
module"]
pub type HASH_HR1 = crate::Reg<hash_hr1::HASH_HR1rs>;
#[doc = "HASH digest register 1"]
pub mod hash_hr1;
#[doc = "HASH_HR2 (r) register accessor: HASH digest register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_hr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_hr2`]
module"]
pub type HASH_HR2 = crate::Reg<hash_hr2::HASH_HR2rs>;
#[doc = "HASH digest register 2"]
pub mod hash_hr2;
#[doc = "HASH_HR3 (r) register accessor: HASH digest register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_hr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_hr3`]
module"]
pub type HASH_HR3 = crate::Reg<hash_hr3::HASH_HR3rs>;
#[doc = "HASH digest register 3"]
pub mod hash_hr3;
#[doc = "HASH_HR4 (r) register accessor: HASH digest register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_hr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_hr4`]
module"]
pub type HASH_HR4 = crate::Reg<hash_hr4::HASH_HR4rs>;
#[doc = "HASH digest register 4"]
pub mod hash_hr4;
#[doc = "HASH_IMR (rw) register accessor: HASH interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_imr`]
module"]
pub type HASH_IMR = crate::Reg<hash_imr::HASH_IMRrs>;
#[doc = "HASH interrupt enable register"]
pub mod hash_imr;
#[doc = "HASH_SR (rw) register accessor: HASH status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_sr`]
module"]
pub type HASH_SR = crate::Reg<hash_sr::HASH_SRrs>;
#[doc = "HASH status register"]
pub mod hash_sr;
#[doc = "HASH_CSR0 (rw) register accessor: These registers contain the complete internal register states of the hash processor. They are useful when a context swap has to be done because a high-priority task needs to use the hash processor while it is already used by another task. When such an event occurs, the HASH_CSRx registers have to be read and the read values have to be saved in the system memory space. Then the hash processor can be used by the preemptive task, and when the hash computation is complete, the saved context can be read from memory and written back into the HASH_CSRx registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr0`]
module"]
pub type HASH_CSR0 = crate::Reg<hash_csr0::HASH_CSR0rs>;
#[doc = "These registers contain the complete internal register states of the hash processor. They are useful when a context swap has to be done because a high-priority task needs to use the hash processor while it is already used by another task. When such an event occurs, the HASH_CSRx registers have to be read and the read values have to be saved in the system memory space. Then the hash processor can be used by the preemptive task, and when the hash computation is complete, the saved context can be read from memory and written back into the HASH_CSRx registers."]
pub mod hash_csr0;
#[doc = "HASH_CSR1 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr1`]
module"]
pub type HASH_CSR1 = crate::Reg<hash_csr1::HASH_CSR1rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr1;
#[doc = "HASH_CSR2 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr2`]
module"]
pub type HASH_CSR2 = crate::Reg<hash_csr2::HASH_CSR2rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr2;
#[doc = "HASH_CSR3 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr3`]
module"]
pub type HASH_CSR3 = crate::Reg<hash_csr3::HASH_CSR3rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr3;
#[doc = "HASH_CSR4 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr4`]
module"]
pub type HASH_CSR4 = crate::Reg<hash_csr4::HASH_CSR4rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr4;
#[doc = "HASH_CSR5 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr5`]
module"]
pub type HASH_CSR5 = crate::Reg<hash_csr5::HASH_CSR5rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr5;
#[doc = "HASH_CSR6 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr6`]
module"]
pub type HASH_CSR6 = crate::Reg<hash_csr6::HASH_CSR6rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr6;
#[doc = "HASH_CSR7 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr7`]
module"]
pub type HASH_CSR7 = crate::Reg<hash_csr7::HASH_CSR7rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr7;
#[doc = "HASH_CSR8 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr8`]
module"]
pub type HASH_CSR8 = crate::Reg<hash_csr8::HASH_CSR8rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr8;
#[doc = "HASH_CSR9 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr9`]
module"]
pub type HASH_CSR9 = crate::Reg<hash_csr9::HASH_CSR9rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr9;
#[doc = "HASH_CSR10 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr10`]
module"]
pub type HASH_CSR10 = crate::Reg<hash_csr10::HASH_CSR10rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr10;
#[doc = "HASH_CSR11 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr11`]
module"]
pub type HASH_CSR11 = crate::Reg<hash_csr11::HASH_CSR11rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr11;
#[doc = "HASH_CSR12 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr12`]
module"]
pub type HASH_CSR12 = crate::Reg<hash_csr12::HASH_CSR12rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr12;
#[doc = "HASH_CSR13 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr13`]
module"]
pub type HASH_CSR13 = crate::Reg<hash_csr13::HASH_CSR13rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr13;
#[doc = "HASH_CSR14 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr14`]
module"]
pub type HASH_CSR14 = crate::Reg<hash_csr14::HASH_CSR14rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr14;
#[doc = "HASH_CSR15 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr15`]
module"]
pub type HASH_CSR15 = crate::Reg<hash_csr15::HASH_CSR15rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr15;
#[doc = "HASH_CSR16 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr16`]
module"]
pub type HASH_CSR16 = crate::Reg<hash_csr16::HASH_CSR16rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr16;
#[doc = "HASH_CSR17 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr17`]
module"]
pub type HASH_CSR17 = crate::Reg<hash_csr17::HASH_CSR17rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr17;
#[doc = "HASH_CSR18 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr18`]
module"]
pub type HASH_CSR18 = crate::Reg<hash_csr18::HASH_CSR18rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr18;
#[doc = "HASH_CSR19 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr19`]
module"]
pub type HASH_CSR19 = crate::Reg<hash_csr19::HASH_CSR19rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr19;
#[doc = "HASH_CSR20 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr20`]
module"]
pub type HASH_CSR20 = crate::Reg<hash_csr20::HASH_CSR20rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr20;
#[doc = "HASH_CSR21 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr21`]
module"]
pub type HASH_CSR21 = crate::Reg<hash_csr21::HASH_CSR21rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr21;
#[doc = "HASH_CSR22 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr22`]
module"]
pub type HASH_CSR22 = crate::Reg<hash_csr22::HASH_CSR22rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr22;
#[doc = "HASH_CSR23 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr23`]
module"]
pub type HASH_CSR23 = crate::Reg<hash_csr23::HASH_CSR23rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr23;
#[doc = "HASH_CSR24 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr24`]
module"]
pub type HASH_CSR24 = crate::Reg<hash_csr24::HASH_CSR24rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr24;
#[doc = "HASH_CSR25 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr25`]
module"]
pub type HASH_CSR25 = crate::Reg<hash_csr25::HASH_CSR25rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr25;
#[doc = "HASH_CSR26 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr26`]
module"]
pub type HASH_CSR26 = crate::Reg<hash_csr26::HASH_CSR26rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr26;
#[doc = "HASH_CSR27 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr27`]
module"]
pub type HASH_CSR27 = crate::Reg<hash_csr27::HASH_CSR27rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr27;
#[doc = "HASH_CSR28 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr28`]
module"]
pub type HASH_CSR28 = crate::Reg<hash_csr28::HASH_CSR28rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr28;
#[doc = "HASH_CSR29 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr29`]
module"]
pub type HASH_CSR29 = crate::Reg<hash_csr29::HASH_CSR29rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr29;
#[doc = "HASH_CSR30 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr30`]
module"]
pub type HASH_CSR30 = crate::Reg<hash_csr30::HASH_CSR30rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr30;
#[doc = "HASH_CSR31 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr31`]
module"]
pub type HASH_CSR31 = crate::Reg<hash_csr31::HASH_CSR31rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr31;
#[doc = "HASH_CSR32 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr32`]
module"]
pub type HASH_CSR32 = crate::Reg<hash_csr32::HASH_CSR32rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr32;
#[doc = "HASH_CSR33 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr33`]
module"]
pub type HASH_CSR33 = crate::Reg<hash_csr33::HASH_CSR33rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr33;
#[doc = "HASH_CSR34 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr34`]
module"]
pub type HASH_CSR34 = crate::Reg<hash_csr34::HASH_CSR34rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr34;
#[doc = "HASH_CSR35 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr35`]
module"]
pub type HASH_CSR35 = crate::Reg<hash_csr35::HASH_CSR35rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr35;
#[doc = "HASH_CSR36 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr36`]
module"]
pub type HASH_CSR36 = crate::Reg<hash_csr36::HASH_CSR36rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr36;
#[doc = "HASH_CSR37 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr37`]
module"]
pub type HASH_CSR37 = crate::Reg<hash_csr37::HASH_CSR37rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr37;
#[doc = "HASH_CSR38 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr38`]
module"]
pub type HASH_CSR38 = crate::Reg<hash_csr38::HASH_CSR38rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr38;
#[doc = "HASH_CSR39 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr39`]
module"]
pub type HASH_CSR39 = crate::Reg<hash_csr39::HASH_CSR39rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr39;
#[doc = "HASH_CSR40 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr40`]
module"]
pub type HASH_CSR40 = crate::Reg<hash_csr40::HASH_CSR40rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr40;
#[doc = "HASH_CSR41 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr41`]
module"]
pub type HASH_CSR41 = crate::Reg<hash_csr41::HASH_CSR41rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr41;
#[doc = "HASH_CSR42 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr42`]
module"]
pub type HASH_CSR42 = crate::Reg<hash_csr42::HASH_CSR42rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr42;
#[doc = "HASH_CSR43 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr43`]
module"]
pub type HASH_CSR43 = crate::Reg<hash_csr43::HASH_CSR43rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr43;
#[doc = "HASH_CSR44 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr44`]
module"]
pub type HASH_CSR44 = crate::Reg<hash_csr44::HASH_CSR44rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr44;
#[doc = "HASH_CSR45 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr45`]
module"]
pub type HASH_CSR45 = crate::Reg<hash_csr45::HASH_CSR45rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr45;
#[doc = "HASH_CSR46 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr46`]
module"]
pub type HASH_CSR46 = crate::Reg<hash_csr46::HASH_CSR46rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr46;
#[doc = "HASH_CSR47 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr47`]
module"]
pub type HASH_CSR47 = crate::Reg<hash_csr47::HASH_CSR47rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr47;
#[doc = "HASH_CSR48 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr48`]
module"]
pub type HASH_CSR48 = crate::Reg<hash_csr48::HASH_CSR48rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr48;
#[doc = "HASH_CSR49 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr49`]
module"]
pub type HASH_CSR49 = crate::Reg<hash_csr49::HASH_CSR49rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr49;
#[doc = "HASH_CSR50 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr50`]
module"]
pub type HASH_CSR50 = crate::Reg<hash_csr50::HASH_CSR50rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr50;
#[doc = "HASH_CSR51 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr51`]
module"]
pub type HASH_CSR51 = crate::Reg<hash_csr51::HASH_CSR51rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr51;
#[doc = "HASH_CSR52 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr52`]
module"]
pub type HASH_CSR52 = crate::Reg<hash_csr52::HASH_CSR52rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr52;
#[doc = "HASH_CSR53 (rw) register accessor: HASH context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_csr53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_csr53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_csr53`]
module"]
pub type HASH_CSR53 = crate::Reg<hash_csr53::HASH_CSR53rs>;
#[doc = "HASH context swap registers"]
pub mod hash_csr53;
#[doc = "HASH_HR5 (r) register accessor: HASH digest register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_hr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_hr5`]
module"]
pub type HASH_HR5 = crate::Reg<hash_hr5::HASH_HR5rs>;
#[doc = "HASH digest register 5"]
pub mod hash_hr5;
#[doc = "HASH_HR6 (r) register accessor: HASH digest register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_hr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_hr6`]
module"]
pub type HASH_HR6 = crate::Reg<hash_hr6::HASH_HR6rs>;
#[doc = "HASH digest register 6"]
pub mod hash_hr6;
#[doc = "HASH_HR7 (r) register accessor: HASH digest register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_hr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_hr7`]
module"]
pub type HASH_HR7 = crate::Reg<hash_hr7::HASH_HR7rs>;
#[doc = "HASH digest register 7"]
pub mod hash_hr7;
#[doc = "HASH_HWCFGR (r) register accessor: HASH Hardware Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_hwcfgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_hwcfgr`]
module"]
pub type HASH_HWCFGR = crate::Reg<hash_hwcfgr::HASH_HWCFGRrs>;
#[doc = "HASH Hardware Configuration Register"]
pub mod hash_hwcfgr;
#[doc = "HASH_VERR (r) register accessor: HASH Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_verr`]
module"]
pub type HASH_VERR = crate::Reg<hash_verr::HASH_VERRrs>;
#[doc = "HASH Version Register"]
pub mod hash_verr;
#[doc = "HASH_IPIDR (r) register accessor: HASH Identification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_ipidr`]
module"]
pub type HASH_IPIDR = crate::Reg<hash_ipidr::HASH_IPIDRrs>;
#[doc = "HASH Identification"]
pub mod hash_ipidr;
#[doc = "HASH_MID (r) register accessor: HASH Hardware Magic ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_mid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_mid`]
module"]
pub type HASH_MID = crate::Reg<hash_mid::HASH_MIDrs>;
#[doc = "HASH Hardware Magic ID"]
pub mod hash_mid;
