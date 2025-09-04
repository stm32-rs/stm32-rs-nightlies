#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    m0_fn_mod2: M0_FN_MOD2,
    _reserved1: [u8; 0xd8],
    m0_read_qos: M0_READ_QOS,
    m0_fn_mod: M0_FN_MOD,
    m0_write_qos: M0_WRITE_QOS,
    _reserved4: [u8; 0x0f18],
    m1_fn_mod2: M1_FN_MOD2,
    _reserved5: [u8; 0xd8],
    m1_read_qos: M1_READ_QOS,
    m1_write_qos: M1_WRITE_QOS,
    m1_fn_mod: M1_FN_MOD,
    _reserved8: [u8; 0x0ee8],
    periph_id_4: PERIPH_ID_4,
    periph_id_5: PERIPH_ID_5,
    periph_id_6: PERIPH_ID_6,
    periph_id_7: PERIPH_ID_7,
    periph_id_0: PERIPH_ID_0,
    periph_id_1: PERIPH_ID_1,
    periph_id_2: PERIPH_ID_2,
    periph_id_3: PERIPH_ID_3,
    comp_id_0: COMP_ID_0,
    comp_id_1: COMP_ID_1,
    comp_id_2: COMP_ID_2,
    comp_id_3: COMP_ID_3,
    m2_fn_mod2: M2_FN_MOD2,
    _reserved21: [u8; 0xd8],
    m2_read_qos: M2_READ_QOS,
    m2_write_qos: M2_WRITE_QOS,
    m2_fn_mod: M2_FN_MOD,
    _reserved24: [u8; 0x0f18],
    m5_fn_mod2: M5_FN_MOD2,
    _reserved25: [u8; 0xd8],
    m5_read_qos: M5_READ_QOS,
    m5_write_qos: M5_WRITE_QOS,
    m5_fn_mod: M5_FN_MOD,
    _reserved28: [u8; 0x0ff4],
    m3_read_qos: M3_READ_QOS,
    m3_write_qos: M3_WRITE_QOS,
    m3_fn_mod: M3_FN_MOD,
    _reserved31: [u8; 0x0ff4],
    m7_read_qos: M7_READ_QOS,
    m7_write_qos: M7_WRITE_QOS,
    m7_fn_mod: M7_FN_MOD,
    _reserved34: [u8; 0x0ff4],
    m8_read_qos: M8_READ_QOS,
    m8_write_qos: M8_WRITE_QOS,
    m8_fn_mod: M8_FN_MOD,
    _reserved37: [u8; 0x1f18],
    m4_fn_mod2: M4_FN_MOD2,
    _reserved38: [u8; 0xd8],
    m4_read_qos: M4_READ_QOS,
    m4_write_qos: M4_WRITE_QOS,
    m4_fn_mod: M4_FN_MOD,
    _reserved41: [u8; 0x0ff4],
    m9_read_qos: M9_READ_QOS,
    m9_write_qos: M9_WRITE_QOS,
    m9_fn_mod: M9_FN_MOD,
    _reserved44: [u8; 0x0ff4],
    m10_read_qos: M10_READ_QOS,
    m10_write_qos: M10_WRITE_QOS,
    m10_fn_mod: M10_FN_MOD,
    _reserved47: [u8; 0x0f18],
    m6_fn_mod2: M6_FN_MOD2,
    _reserved48: [u8; 0xd8],
    m6_read_qos: M6_READ_QOS,
    m6_write_qos: M6_WRITE_QOS,
    m6_fn_mod: M6_FN_MOD,
    _reserved51: [u8; 0x0003_6f40],
    m0_fn_mod_ahb: M0_FN_MOD_AHB,
    _reserved52: [u8; 0x0ffc],
    m1_fn_mod_ahb: M1_FN_MOD_AHB,
    _reserved53: [u8; 0x0ffc],
    m2_fn_mod_ahb: M2_FN_MOD_AHB,
    _reserved54: [u8; 0x0ffc],
    m5_fn_mod_ahb: M5_FN_MOD_AHB,
    _reserved55: [u8; 0x5000],
    fn_mod_lb: FN_MOD_LB,
    _reserved56: [u8; 0x2ff8],
    m6_fn_mod_ahb: M6_FN_MOD_AHB,
}
impl RegisterBlock {
    ///0x00 - AXIMC master 0 packing functionality register
    #[inline(always)]
    pub const fn m0_fn_mod2(&self) -> &M0_FN_MOD2 {
        &self.m0_fn_mod2
    }
    ///0xdc - AXIMC master 0 read priority register
    #[inline(always)]
    pub const fn m0_read_qos(&self) -> &M0_READ_QOS {
        &self.m0_read_qos
    }
    ///0xe0 - AXIMC master 0 issuing capability override functionality register
    #[inline(always)]
    pub const fn m0_fn_mod(&self) -> &M0_FN_MOD {
        &self.m0_fn_mod
    }
    ///0xe4 - AXIMC master 0 write priority register
    #[inline(always)]
    pub const fn m0_write_qos(&self) -> &M0_WRITE_QOS {
        &self.m0_write_qos
    }
    ///0x1000 - AXIMC master 1 packing functionality register
    #[inline(always)]
    pub const fn m1_fn_mod2(&self) -> &M1_FN_MOD2 {
        &self.m1_fn_mod2
    }
    ///0x10dc - AXIMC master 1 read priority register
    #[inline(always)]
    pub const fn m1_read_qos(&self) -> &M1_READ_QOS {
        &self.m1_read_qos
    }
    ///0x10e0 - AXIMC master 1 write priority register
    #[inline(always)]
    pub const fn m1_write_qos(&self) -> &M1_WRITE_QOS {
        &self.m1_write_qos
    }
    ///0x10e4 - AXIMC master 1 issuing capability override functionality register
    #[inline(always)]
    pub const fn m1_fn_mod(&self) -> &M1_FN_MOD {
        &self.m1_fn_mod
    }
    ///0x1fd0 - AXIMC peripheral ID4 register
    #[inline(always)]
    pub const fn periph_id_4(&self) -> &PERIPH_ID_4 {
        &self.periph_id_4
    }
    ///0x1fd4 - AXIMC peripheral ID5 register
    #[inline(always)]
    pub const fn periph_id_5(&self) -> &PERIPH_ID_5 {
        &self.periph_id_5
    }
    ///0x1fd8 - AXIMC peripheral ID6 register
    #[inline(always)]
    pub const fn periph_id_6(&self) -> &PERIPH_ID_6 {
        &self.periph_id_6
    }
    ///0x1fdc - AXIMC peripheral ID7 register
    #[inline(always)]
    pub const fn periph_id_7(&self) -> &PERIPH_ID_7 {
        &self.periph_id_7
    }
    ///0x1fe0 - AXIMC peripheral ID0 register
    #[inline(always)]
    pub const fn periph_id_0(&self) -> &PERIPH_ID_0 {
        &self.periph_id_0
    }
    ///0x1fe4 - AXIMC peripheral ID1 register
    #[inline(always)]
    pub const fn periph_id_1(&self) -> &PERIPH_ID_1 {
        &self.periph_id_1
    }
    ///0x1fe8 - AXIMC peripheral ID2 register
    #[inline(always)]
    pub const fn periph_id_2(&self) -> &PERIPH_ID_2 {
        &self.periph_id_2
    }
    ///0x1fec - AXIMC peripheral ID3 register
    #[inline(always)]
    pub const fn periph_id_3(&self) -> &PERIPH_ID_3 {
        &self.periph_id_3
    }
    ///0x1ff0 - AXIMC component ID0 register
    #[inline(always)]
    pub const fn comp_id_0(&self) -> &COMP_ID_0 {
        &self.comp_id_0
    }
    ///0x1ff4 - AXIMC component ID1 register
    #[inline(always)]
    pub const fn comp_id_1(&self) -> &COMP_ID_1 {
        &self.comp_id_1
    }
    ///0x1ff8 - AXIMC component ID2 register
    #[inline(always)]
    pub const fn comp_id_2(&self) -> &COMP_ID_2 {
        &self.comp_id_2
    }
    ///0x1ffc - AXIMC component ID3 register
    #[inline(always)]
    pub const fn comp_id_3(&self) -> &COMP_ID_3 {
        &self.comp_id_3
    }
    ///0x2000 - AXIMC master 2 packing functionality register
    #[inline(always)]
    pub const fn m2_fn_mod2(&self) -> &M2_FN_MOD2 {
        &self.m2_fn_mod2
    }
    ///0x20dc - AXIMC master 2 read priority register
    #[inline(always)]
    pub const fn m2_read_qos(&self) -> &M2_READ_QOS {
        &self.m2_read_qos
    }
    ///0x20e0 - AXIMC master 2 write priority register
    #[inline(always)]
    pub const fn m2_write_qos(&self) -> &M2_WRITE_QOS {
        &self.m2_write_qos
    }
    ///0x20e4 - AXIMC master 2 issuing capability override functionality register
    #[inline(always)]
    pub const fn m2_fn_mod(&self) -> &M2_FN_MOD {
        &self.m2_fn_mod
    }
    ///0x3000 - AXIMC master 5 packing functionality register
    #[inline(always)]
    pub const fn m5_fn_mod2(&self) -> &M5_FN_MOD2 {
        &self.m5_fn_mod2
    }
    ///0x30dc - AXIMC master 5 read priority register
    #[inline(always)]
    pub const fn m5_read_qos(&self) -> &M5_READ_QOS {
        &self.m5_read_qos
    }
    ///0x30e0 - AXIMC master 5 write priority register
    #[inline(always)]
    pub const fn m5_write_qos(&self) -> &M5_WRITE_QOS {
        &self.m5_write_qos
    }
    ///0x30e4 - AXIMC master 5 issuing capability override functionality register
    #[inline(always)]
    pub const fn m5_fn_mod(&self) -> &M5_FN_MOD {
        &self.m5_fn_mod
    }
    ///0x40dc - AXIMC master 3 read priority register
    #[inline(always)]
    pub const fn m3_read_qos(&self) -> &M3_READ_QOS {
        &self.m3_read_qos
    }
    ///0x40e0 - AXIMC master 3 write priority register
    #[inline(always)]
    pub const fn m3_write_qos(&self) -> &M3_WRITE_QOS {
        &self.m3_write_qos
    }
    ///0x40e4 - AXIMC master 3 packing functionality register
    #[inline(always)]
    pub const fn m3_fn_mod(&self) -> &M3_FN_MOD {
        &self.m3_fn_mod
    }
    ///0x50dc - AXIMC master 7 read priority register
    #[inline(always)]
    pub const fn m7_read_qos(&self) -> &M7_READ_QOS {
        &self.m7_read_qos
    }
    ///0x50e0 - AXIMC master 7 write priority register
    #[inline(always)]
    pub const fn m7_write_qos(&self) -> &M7_WRITE_QOS {
        &self.m7_write_qos
    }
    ///0x50e4 - AXIMC master 7 issuing capability override functionality register
    #[inline(always)]
    pub const fn m7_fn_mod(&self) -> &M7_FN_MOD {
        &self.m7_fn_mod
    }
    ///0x60dc - AXIMC master 8 read priority register
    #[inline(always)]
    pub const fn m8_read_qos(&self) -> &M8_READ_QOS {
        &self.m8_read_qos
    }
    ///0x60e0 - AXIMC master 8 write priority register
    #[inline(always)]
    pub const fn m8_write_qos(&self) -> &M8_WRITE_QOS {
        &self.m8_write_qos
    }
    ///0x60e4 - AXIMC master 8 issuing capability override functionality register
    #[inline(always)]
    pub const fn m8_fn_mod(&self) -> &M8_FN_MOD {
        &self.m8_fn_mod
    }
    ///0x8000 - AXIMC master 4 packing functionality register
    #[inline(always)]
    pub const fn m4_fn_mod2(&self) -> &M4_FN_MOD2 {
        &self.m4_fn_mod2
    }
    ///0x80dc - AXIMC master 4 read priority register
    #[inline(always)]
    pub const fn m4_read_qos(&self) -> &M4_READ_QOS {
        &self.m4_read_qos
    }
    ///0x80e0 - AXIMC master 4 write priority register
    #[inline(always)]
    pub const fn m4_write_qos(&self) -> &M4_WRITE_QOS {
        &self.m4_write_qos
    }
    ///0x80e4 - AXIMC master 4 packing functionality register
    #[inline(always)]
    pub const fn m4_fn_mod(&self) -> &M4_FN_MOD {
        &self.m4_fn_mod
    }
    ///0x90dc - AXIMC master 9 read priority register
    #[inline(always)]
    pub const fn m9_read_qos(&self) -> &M9_READ_QOS {
        &self.m9_read_qos
    }
    ///0x90e0 - AXIMC master 9 write priority register
    #[inline(always)]
    pub const fn m9_write_qos(&self) -> &M9_WRITE_QOS {
        &self.m9_write_qos
    }
    ///0x90e4 - AXIMC master 9 issuing capability override functionality register
    #[inline(always)]
    pub const fn m9_fn_mod(&self) -> &M9_FN_MOD {
        &self.m9_fn_mod
    }
    ///0xa0dc - AXIMC master 10 read priority register
    #[inline(always)]
    pub const fn m10_read_qos(&self) -> &M10_READ_QOS {
        &self.m10_read_qos
    }
    ///0xa0e0 - AXIMC master 10 write priority register
    #[inline(always)]
    pub const fn m10_write_qos(&self) -> &M10_WRITE_QOS {
        &self.m10_write_qos
    }
    ///0xa0e4 - AXIMC master 10 issuing capability override functionality register
    #[inline(always)]
    pub const fn m10_fn_mod(&self) -> &M10_FN_MOD {
        &self.m10_fn_mod
    }
    ///0xb000 - AXIMC master 6 packing functionality register
    #[inline(always)]
    pub const fn m6_fn_mod2(&self) -> &M6_FN_MOD2 {
        &self.m6_fn_mod2
    }
    ///0xb0dc - AXIMC master 6 read priority register
    #[inline(always)]
    pub const fn m6_read_qos(&self) -> &M6_READ_QOS {
        &self.m6_read_qos
    }
    ///0xb0e0 - AXIMC master 6 write priority register
    #[inline(always)]
    pub const fn m6_write_qos(&self) -> &M6_WRITE_QOS {
        &self.m6_write_qos
    }
    ///0xb0e4 - AXIMC master 6 issuing capability override functionality register
    #[inline(always)]
    pub const fn m6_fn_mod(&self) -> &M6_FN_MOD {
        &self.m6_fn_mod
    }
    ///0x42028 - AXIMC master 0 AHB conversion override functionality register
    #[inline(always)]
    pub const fn m0_fn_mod_ahb(&self) -> &M0_FN_MOD_AHB {
        &self.m0_fn_mod_ahb
    }
    ///0x43028 - AXIMC master 1 AHB conversion override functionality register
    #[inline(always)]
    pub const fn m1_fn_mod_ahb(&self) -> &M1_FN_MOD_AHB {
        &self.m1_fn_mod_ahb
    }
    ///0x44028 - AXIMC master 2 AHB conversion override functionality register
    #[inline(always)]
    pub const fn m2_fn_mod_ahb(&self) -> &M2_FN_MOD_AHB {
        &self.m2_fn_mod_ahb
    }
    ///0x45028 - AXIMC master 5 AHB conversion override functionality register
    #[inline(always)]
    pub const fn m5_fn_mod_ahb(&self) -> &M5_FN_MOD_AHB {
        &self.m5_fn_mod_ahb
    }
    ///0x4a02c - AXIMC long burst capability inhibition register
    #[inline(always)]
    pub const fn fn_mod_lb(&self) -> &FN_MOD_LB {
        &self.fn_mod_lb
    }
    ///0x4d028 - AXIMC master 6 AHB conversion override functionality register
    #[inline(always)]
    pub const fn m6_fn_mod_ahb(&self) -> &M6_FN_MOD_AHB {
        &self.m6_fn_mod_ahb
    }
}
/**M0_FN_MOD2 (rw) register accessor: AXIMC master 0 packing functionality register

You can [`read`](crate::Reg::read) this register and get [`m0_fn_mod2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0_fn_mod2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M0_FN_MOD2)

For information about available fields see [`mod@m0_fn_mod2`] module*/
pub type M0_FN_MOD2 = crate::Reg<m0_fn_mod2::M0_FN_MOD2rs>;
///AXIMC master 0 packing functionality register
pub mod m0_fn_mod2;
/**M0_READ_QOS (rw) register accessor: AXIMC master 0 read priority register

You can [`read`](crate::Reg::read) this register and get [`m0_read_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0_read_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M0_READ_QOS)

For information about available fields see [`mod@m0_read_qos`] module*/
pub type M0_READ_QOS = crate::Reg<m0_read_qos::M0_READ_QOSrs>;
///AXIMC master 0 read priority register
pub mod m0_read_qos;
/**M0_WRITE_QOS (rw) register accessor: AXIMC master 0 write priority register

You can [`read`](crate::Reg::read) this register and get [`m0_write_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0_write_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M0_WRITE_QOS)

For information about available fields see [`mod@m0_write_qos`] module*/
pub type M0_WRITE_QOS = crate::Reg<m0_write_qos::M0_WRITE_QOSrs>;
///AXIMC master 0 write priority register
pub mod m0_write_qos;
/**M0_FN_MOD (rw) register accessor: AXIMC master 0 issuing capability override functionality register

You can [`read`](crate::Reg::read) this register and get [`m0_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M0_FN_MOD)

For information about available fields see [`mod@m0_fn_mod`] module*/
pub type M0_FN_MOD = crate::Reg<m0_fn_mod::M0_FN_MODrs>;
///AXIMC master 0 issuing capability override functionality register
pub mod m0_fn_mod;
/**M1_FN_MOD2 (rw) register accessor: AXIMC master 1 packing functionality register

You can [`read`](crate::Reg::read) this register and get [`m1_fn_mod2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1_fn_mod2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M1_FN_MOD2)

For information about available fields see [`mod@m1_fn_mod2`] module*/
pub type M1_FN_MOD2 = crate::Reg<m1_fn_mod2::M1_FN_MOD2rs>;
///AXIMC master 1 packing functionality register
pub mod m1_fn_mod2;
/**M1_READ_QOS (rw) register accessor: AXIMC master 1 read priority register

You can [`read`](crate::Reg::read) this register and get [`m1_read_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1_read_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M1_READ_QOS)

For information about available fields see [`mod@m1_read_qos`] module*/
pub type M1_READ_QOS = crate::Reg<m1_read_qos::M1_READ_QOSrs>;
///AXIMC master 1 read priority register
pub mod m1_read_qos;
/**M1_WRITE_QOS (rw) register accessor: AXIMC master 1 write priority register

You can [`read`](crate::Reg::read) this register and get [`m1_write_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1_write_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M1_WRITE_QOS)

For information about available fields see [`mod@m1_write_qos`] module*/
pub type M1_WRITE_QOS = crate::Reg<m1_write_qos::M1_WRITE_QOSrs>;
///AXIMC master 1 write priority register
pub mod m1_write_qos;
/**M1_FN_MOD (rw) register accessor: AXIMC master 1 issuing capability override functionality register

You can [`read`](crate::Reg::read) this register and get [`m1_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M1_FN_MOD)

For information about available fields see [`mod@m1_fn_mod`] module*/
pub type M1_FN_MOD = crate::Reg<m1_fn_mod::M1_FN_MODrs>;
///AXIMC master 1 issuing capability override functionality register
pub mod m1_fn_mod;
/**M2_FN_MOD2 (rw) register accessor: AXIMC master 2 packing functionality register

You can [`read`](crate::Reg::read) this register and get [`m2_fn_mod2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2_fn_mod2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M2_FN_MOD2)

For information about available fields see [`mod@m2_fn_mod2`] module*/
pub type M2_FN_MOD2 = crate::Reg<m2_fn_mod2::M2_FN_MOD2rs>;
///AXIMC master 2 packing functionality register
pub mod m2_fn_mod2;
/**M2_READ_QOS (rw) register accessor: AXIMC master 2 read priority register

You can [`read`](crate::Reg::read) this register and get [`m2_read_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2_read_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M2_READ_QOS)

For information about available fields see [`mod@m2_read_qos`] module*/
pub type M2_READ_QOS = crate::Reg<m2_read_qos::M2_READ_QOSrs>;
///AXIMC master 2 read priority register
pub mod m2_read_qos;
/**M2_WRITE_QOS (rw) register accessor: AXIMC master 2 write priority register

You can [`read`](crate::Reg::read) this register and get [`m2_write_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2_write_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M2_WRITE_QOS)

For information about available fields see [`mod@m2_write_qos`] module*/
pub type M2_WRITE_QOS = crate::Reg<m2_write_qos::M2_WRITE_QOSrs>;
///AXIMC master 2 write priority register
pub mod m2_write_qos;
/**M2_FN_MOD (rw) register accessor: AXIMC master 2 issuing capability override functionality register

You can [`read`](crate::Reg::read) this register and get [`m2_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M2_FN_MOD)

For information about available fields see [`mod@m2_fn_mod`] module*/
pub type M2_FN_MOD = crate::Reg<m2_fn_mod::M2_FN_MODrs>;
///AXIMC master 2 issuing capability override functionality register
pub mod m2_fn_mod;
/**M5_FN_MOD2 (rw) register accessor: AXIMC master 5 packing functionality register

You can [`read`](crate::Reg::read) this register and get [`m5_fn_mod2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5_fn_mod2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M5_FN_MOD2)

For information about available fields see [`mod@m5_fn_mod2`] module*/
pub type M5_FN_MOD2 = crate::Reg<m5_fn_mod2::M5_FN_MOD2rs>;
///AXIMC master 5 packing functionality register
pub mod m5_fn_mod2;
/**M5_READ_QOS (rw) register accessor: AXIMC master 5 read priority register

You can [`read`](crate::Reg::read) this register and get [`m5_read_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5_read_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M5_READ_QOS)

For information about available fields see [`mod@m5_read_qos`] module*/
pub type M5_READ_QOS = crate::Reg<m5_read_qos::M5_READ_QOSrs>;
///AXIMC master 5 read priority register
pub mod m5_read_qos;
/**M5_WRITE_QOS (rw) register accessor: AXIMC master 5 write priority register

You can [`read`](crate::Reg::read) this register and get [`m5_write_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5_write_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M5_WRITE_QOS)

For information about available fields see [`mod@m5_write_qos`] module*/
pub type M5_WRITE_QOS = crate::Reg<m5_write_qos::M5_WRITE_QOSrs>;
///AXIMC master 5 write priority register
pub mod m5_write_qos;
/**M5_FN_MOD (rw) register accessor: AXIMC master 5 issuing capability override functionality register

You can [`read`](crate::Reg::read) this register and get [`m5_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M5_FN_MOD)

For information about available fields see [`mod@m5_fn_mod`] module*/
pub type M5_FN_MOD = crate::Reg<m5_fn_mod::M5_FN_MODrs>;
///AXIMC master 5 issuing capability override functionality register
pub mod m5_fn_mod;
/**M3_READ_QOS (rw) register accessor: AXIMC master 3 read priority register

You can [`read`](crate::Reg::read) this register and get [`m3_read_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3_read_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M3_READ_QOS)

For information about available fields see [`mod@m3_read_qos`] module*/
pub type M3_READ_QOS = crate::Reg<m3_read_qos::M3_READ_QOSrs>;
///AXIMC master 3 read priority register
pub mod m3_read_qos;
/**M3_WRITE_QOS (rw) register accessor: AXIMC master 3 write priority register

You can [`read`](crate::Reg::read) this register and get [`m3_write_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3_write_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M3_WRITE_QOS)

For information about available fields see [`mod@m3_write_qos`] module*/
pub type M3_WRITE_QOS = crate::Reg<m3_write_qos::M3_WRITE_QOSrs>;
///AXIMC master 3 write priority register
pub mod m3_write_qos;
/**M3_FN_MOD (rw) register accessor: AXIMC master 3 packing functionality register

You can [`read`](crate::Reg::read) this register and get [`m3_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M3_FN_MOD)

For information about available fields see [`mod@m3_fn_mod`] module*/
pub type M3_FN_MOD = crate::Reg<m3_fn_mod::M3_FN_MODrs>;
///AXIMC master 3 packing functionality register
pub mod m3_fn_mod;
/**M7_READ_QOS (rw) register accessor: AXIMC master 7 read priority register

You can [`read`](crate::Reg::read) this register and get [`m7_read_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m7_read_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M7_READ_QOS)

For information about available fields see [`mod@m7_read_qos`] module*/
pub type M7_READ_QOS = crate::Reg<m7_read_qos::M7_READ_QOSrs>;
///AXIMC master 7 read priority register
pub mod m7_read_qos;
/**M7_WRITE_QOS (rw) register accessor: AXIMC master 7 write priority register

You can [`read`](crate::Reg::read) this register and get [`m7_write_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m7_write_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M7_WRITE_QOS)

For information about available fields see [`mod@m7_write_qos`] module*/
pub type M7_WRITE_QOS = crate::Reg<m7_write_qos::M7_WRITE_QOSrs>;
///AXIMC master 7 write priority register
pub mod m7_write_qos;
/**M7_FN_MOD (rw) register accessor: AXIMC master 7 issuing capability override functionality register

You can [`read`](crate::Reg::read) this register and get [`m7_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m7_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M7_FN_MOD)

For information about available fields see [`mod@m7_fn_mod`] module*/
pub type M7_FN_MOD = crate::Reg<m7_fn_mod::M7_FN_MODrs>;
///AXIMC master 7 issuing capability override functionality register
pub mod m7_fn_mod;
/**M8_READ_QOS (rw) register accessor: AXIMC master 8 read priority register

You can [`read`](crate::Reg::read) this register and get [`m8_read_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m8_read_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M8_READ_QOS)

For information about available fields see [`mod@m8_read_qos`] module*/
pub type M8_READ_QOS = crate::Reg<m8_read_qos::M8_READ_QOSrs>;
///AXIMC master 8 read priority register
pub mod m8_read_qos;
/**M8_WRITE_QOS (rw) register accessor: AXIMC master 8 write priority register

You can [`read`](crate::Reg::read) this register and get [`m8_write_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m8_write_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M8_WRITE_QOS)

For information about available fields see [`mod@m8_write_qos`] module*/
pub type M8_WRITE_QOS = crate::Reg<m8_write_qos::M8_WRITE_QOSrs>;
///AXIMC master 8 write priority register
pub mod m8_write_qos;
/**M8_FN_MOD (rw) register accessor: AXIMC master 8 issuing capability override functionality register

You can [`read`](crate::Reg::read) this register and get [`m8_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m8_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M8_FN_MOD)

For information about available fields see [`mod@m8_fn_mod`] module*/
pub type M8_FN_MOD = crate::Reg<m8_fn_mod::M8_FN_MODrs>;
///AXIMC master 8 issuing capability override functionality register
pub mod m8_fn_mod;
/**M4_FN_MOD2 (rw) register accessor: AXIMC master 4 packing functionality register

You can [`read`](crate::Reg::read) this register and get [`m4_fn_mod2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4_fn_mod2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M4_FN_MOD2)

For information about available fields see [`mod@m4_fn_mod2`] module*/
pub type M4_FN_MOD2 = crate::Reg<m4_fn_mod2::M4_FN_MOD2rs>;
///AXIMC master 4 packing functionality register
pub mod m4_fn_mod2;
/**M4_READ_QOS (rw) register accessor: AXIMC master 4 read priority register

You can [`read`](crate::Reg::read) this register and get [`m4_read_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4_read_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M4_READ_QOS)

For information about available fields see [`mod@m4_read_qos`] module*/
pub type M4_READ_QOS = crate::Reg<m4_read_qos::M4_READ_QOSrs>;
///AXIMC master 4 read priority register
pub mod m4_read_qos;
/**M4_WRITE_QOS (rw) register accessor: AXIMC master 4 write priority register

You can [`read`](crate::Reg::read) this register and get [`m4_write_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4_write_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M4_WRITE_QOS)

For information about available fields see [`mod@m4_write_qos`] module*/
pub type M4_WRITE_QOS = crate::Reg<m4_write_qos::M4_WRITE_QOSrs>;
///AXIMC master 4 write priority register
pub mod m4_write_qos;
/**M4_FN_MOD (rw) register accessor: AXIMC master 4 packing functionality register

You can [`read`](crate::Reg::read) this register and get [`m4_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M4_FN_MOD)

For information about available fields see [`mod@m4_fn_mod`] module*/
pub type M4_FN_MOD = crate::Reg<m4_fn_mod::M4_FN_MODrs>;
///AXIMC master 4 packing functionality register
pub mod m4_fn_mod;
/**M9_READ_QOS (rw) register accessor: AXIMC master 9 read priority register

You can [`read`](crate::Reg::read) this register and get [`m9_read_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m9_read_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M9_READ_QOS)

For information about available fields see [`mod@m9_read_qos`] module*/
pub type M9_READ_QOS = crate::Reg<m9_read_qos::M9_READ_QOSrs>;
///AXIMC master 9 read priority register
pub mod m9_read_qos;
/**M9_WRITE_QOS (rw) register accessor: AXIMC master 9 write priority register

You can [`read`](crate::Reg::read) this register and get [`m9_write_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m9_write_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M9_WRITE_QOS)

For information about available fields see [`mod@m9_write_qos`] module*/
pub type M9_WRITE_QOS = crate::Reg<m9_write_qos::M9_WRITE_QOSrs>;
///AXIMC master 9 write priority register
pub mod m9_write_qos;
/**M9_FN_MOD (rw) register accessor: AXIMC master 9 issuing capability override functionality register

You can [`read`](crate::Reg::read) this register and get [`m9_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m9_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M9_FN_MOD)

For information about available fields see [`mod@m9_fn_mod`] module*/
pub type M9_FN_MOD = crate::Reg<m9_fn_mod::M9_FN_MODrs>;
///AXIMC master 9 issuing capability override functionality register
pub mod m9_fn_mod;
/**M10_READ_QOS (rw) register accessor: AXIMC master 10 read priority register

You can [`read`](crate::Reg::read) this register and get [`m10_read_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m10_read_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M10_READ_QOS)

For information about available fields see [`mod@m10_read_qos`] module*/
pub type M10_READ_QOS = crate::Reg<m10_read_qos::M10_READ_QOSrs>;
///AXIMC master 10 read priority register
pub mod m10_read_qos;
/**M10_WRITE_QOS (rw) register accessor: AXIMC master 10 write priority register

You can [`read`](crate::Reg::read) this register and get [`m10_write_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m10_write_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M10_WRITE_QOS)

For information about available fields see [`mod@m10_write_qos`] module*/
pub type M10_WRITE_QOS = crate::Reg<m10_write_qos::M10_WRITE_QOSrs>;
///AXIMC master 10 write priority register
pub mod m10_write_qos;
/**M10_FN_MOD (rw) register accessor: AXIMC master 10 issuing capability override functionality register

You can [`read`](crate::Reg::read) this register and get [`m10_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m10_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M10_FN_MOD)

For information about available fields see [`mod@m10_fn_mod`] module*/
pub type M10_FN_MOD = crate::Reg<m10_fn_mod::M10_FN_MODrs>;
///AXIMC master 10 issuing capability override functionality register
pub mod m10_fn_mod;
/**M6_FN_MOD2 (rw) register accessor: AXIMC master 6 packing functionality register

You can [`read`](crate::Reg::read) this register and get [`m6_fn_mod2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m6_fn_mod2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M6_FN_MOD2)

For information about available fields see [`mod@m6_fn_mod2`] module*/
pub type M6_FN_MOD2 = crate::Reg<m6_fn_mod2::M6_FN_MOD2rs>;
///AXIMC master 6 packing functionality register
pub mod m6_fn_mod2;
/**M6_READ_QOS (rw) register accessor: AXIMC master 6 read priority register

You can [`read`](crate::Reg::read) this register and get [`m6_read_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m6_read_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M6_READ_QOS)

For information about available fields see [`mod@m6_read_qos`] module*/
pub type M6_READ_QOS = crate::Reg<m6_read_qos::M6_READ_QOSrs>;
///AXIMC master 6 read priority register
pub mod m6_read_qos;
/**M6_WRITE_QOS (rw) register accessor: AXIMC master 6 write priority register

You can [`read`](crate::Reg::read) this register and get [`m6_write_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m6_write_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M6_WRITE_QOS)

For information about available fields see [`mod@m6_write_qos`] module*/
pub type M6_WRITE_QOS = crate::Reg<m6_write_qos::M6_WRITE_QOSrs>;
///AXIMC master 6 write priority register
pub mod m6_write_qos;
/**M6_FN_MOD (rw) register accessor: AXIMC master 6 issuing capability override functionality register

You can [`read`](crate::Reg::read) this register and get [`m6_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m6_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M6_FN_MOD)

For information about available fields see [`mod@m6_fn_mod`] module*/
pub type M6_FN_MOD = crate::Reg<m6_fn_mod::M6_FN_MODrs>;
///AXIMC master 6 issuing capability override functionality register
pub mod m6_fn_mod;
/**PERIPH_ID_4 (r) register accessor: AXIMC peripheral ID4 register

You can [`read`](crate::Reg::read) this register and get [`periph_id_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:PERIPH_ID_4)

For information about available fields see [`mod@periph_id_4`] module*/
pub type PERIPH_ID_4 = crate::Reg<periph_id_4::PERIPH_ID_4rs>;
///AXIMC peripheral ID4 register
pub mod periph_id_4;
/**PERIPH_ID_5 (r) register accessor: AXIMC peripheral ID5 register

You can [`read`](crate::Reg::read) this register and get [`periph_id_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:PERIPH_ID_5)

For information about available fields see [`mod@periph_id_5`] module*/
pub type PERIPH_ID_5 = crate::Reg<periph_id_5::PERIPH_ID_5rs>;
///AXIMC peripheral ID5 register
pub mod periph_id_5;
/**PERIPH_ID_6 (r) register accessor: AXIMC peripheral ID6 register

You can [`read`](crate::Reg::read) this register and get [`periph_id_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:PERIPH_ID_6)

For information about available fields see [`mod@periph_id_6`] module*/
pub type PERIPH_ID_6 = crate::Reg<periph_id_6::PERIPH_ID_6rs>;
///AXIMC peripheral ID6 register
pub mod periph_id_6;
/**PERIPH_ID_7 (r) register accessor: AXIMC peripheral ID7 register

You can [`read`](crate::Reg::read) this register and get [`periph_id_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:PERIPH_ID_7)

For information about available fields see [`mod@periph_id_7`] module*/
pub type PERIPH_ID_7 = crate::Reg<periph_id_7::PERIPH_ID_7rs>;
///AXIMC peripheral ID7 register
pub mod periph_id_7;
/**PERIPH_ID_0 (r) register accessor: AXIMC peripheral ID0 register

You can [`read`](crate::Reg::read) this register and get [`periph_id_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:PERIPH_ID_0)

For information about available fields see [`mod@periph_id_0`] module*/
pub type PERIPH_ID_0 = crate::Reg<periph_id_0::PERIPH_ID_0rs>;
///AXIMC peripheral ID0 register
pub mod periph_id_0;
/**PERIPH_ID_1 (r) register accessor: AXIMC peripheral ID1 register

You can [`read`](crate::Reg::read) this register and get [`periph_id_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:PERIPH_ID_1)

For information about available fields see [`mod@periph_id_1`] module*/
pub type PERIPH_ID_1 = crate::Reg<periph_id_1::PERIPH_ID_1rs>;
///AXIMC peripheral ID1 register
pub mod periph_id_1;
/**PERIPH_ID_2 (r) register accessor: AXIMC peripheral ID2 register

You can [`read`](crate::Reg::read) this register and get [`periph_id_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:PERIPH_ID_2)

For information about available fields see [`mod@periph_id_2`] module*/
pub type PERIPH_ID_2 = crate::Reg<periph_id_2::PERIPH_ID_2rs>;
///AXIMC peripheral ID2 register
pub mod periph_id_2;
/**PERIPH_ID_3 (r) register accessor: AXIMC peripheral ID3 register

You can [`read`](crate::Reg::read) this register and get [`periph_id_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:PERIPH_ID_3)

For information about available fields see [`mod@periph_id_3`] module*/
pub type PERIPH_ID_3 = crate::Reg<periph_id_3::PERIPH_ID_3rs>;
///AXIMC peripheral ID3 register
pub mod periph_id_3;
/**COMP_ID_0 (r) register accessor: AXIMC component ID0 register

You can [`read`](crate::Reg::read) this register and get [`comp_id_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:COMP_ID_0)

For information about available fields see [`mod@comp_id_0`] module*/
pub type COMP_ID_0 = crate::Reg<comp_id_0::COMP_ID_0rs>;
///AXIMC component ID0 register
pub mod comp_id_0;
/**COMP_ID_1 (r) register accessor: AXIMC component ID1 register

You can [`read`](crate::Reg::read) this register and get [`comp_id_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:COMP_ID_1)

For information about available fields see [`mod@comp_id_1`] module*/
pub type COMP_ID_1 = crate::Reg<comp_id_1::COMP_ID_1rs>;
///AXIMC component ID1 register
pub mod comp_id_1;
/**COMP_ID_2 (r) register accessor: AXIMC component ID2 register

You can [`read`](crate::Reg::read) this register and get [`comp_id_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:COMP_ID_2)

For information about available fields see [`mod@comp_id_2`] module*/
pub type COMP_ID_2 = crate::Reg<comp_id_2::COMP_ID_2rs>;
///AXIMC component ID2 register
pub mod comp_id_2;
/**COMP_ID_3 (r) register accessor: AXIMC component ID3 register

You can [`read`](crate::Reg::read) this register and get [`comp_id_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:COMP_ID_3)

For information about available fields see [`mod@comp_id_3`] module*/
pub type COMP_ID_3 = crate::Reg<comp_id_3::COMP_ID_3rs>;
///AXIMC component ID3 register
pub mod comp_id_3;
/**M0_FN_MOD_AHB (rw) register accessor: AXIMC master 0 AHB conversion override functionality register

You can [`read`](crate::Reg::read) this register and get [`m0_fn_mod_ahb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0_fn_mod_ahb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M0_FN_MOD_AHB)

For information about available fields see [`mod@m0_fn_mod_ahb`] module*/
pub type M0_FN_MOD_AHB = crate::Reg<m0_fn_mod_ahb::M0_FN_MOD_AHBrs>;
///AXIMC master 0 AHB conversion override functionality register
pub mod m0_fn_mod_ahb;
/**M1_FN_MOD_AHB (rw) register accessor: AXIMC master 1 AHB conversion override functionality register

You can [`read`](crate::Reg::read) this register and get [`m1_fn_mod_ahb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1_fn_mod_ahb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M1_FN_MOD_AHB)

For information about available fields see [`mod@m1_fn_mod_ahb`] module*/
pub type M1_FN_MOD_AHB = crate::Reg<m1_fn_mod_ahb::M1_FN_MOD_AHBrs>;
///AXIMC master 1 AHB conversion override functionality register
pub mod m1_fn_mod_ahb;
/**M2_FN_MOD_AHB (rw) register accessor: AXIMC master 2 AHB conversion override functionality register

You can [`read`](crate::Reg::read) this register and get [`m2_fn_mod_ahb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2_fn_mod_ahb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M2_FN_MOD_AHB)

For information about available fields see [`mod@m2_fn_mod_ahb`] module*/
pub type M2_FN_MOD_AHB = crate::Reg<m2_fn_mod_ahb::M2_FN_MOD_AHBrs>;
///AXIMC master 2 AHB conversion override functionality register
pub mod m2_fn_mod_ahb;
/**M5_FN_MOD_AHB (rw) register accessor: AXIMC master 5 AHB conversion override functionality register

You can [`read`](crate::Reg::read) this register and get [`m5_fn_mod_ahb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5_fn_mod_ahb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M5_FN_MOD_AHB)

For information about available fields see [`mod@m5_fn_mod_ahb`] module*/
pub type M5_FN_MOD_AHB = crate::Reg<m5_fn_mod_ahb::M5_FN_MOD_AHBrs>;
///AXIMC master 5 AHB conversion override functionality register
pub mod m5_fn_mod_ahb;
/**M6_FN_MOD_AHB (rw) register accessor: AXIMC master 6 AHB conversion override functionality register

You can [`read`](crate::Reg::read) this register and get [`m6_fn_mod_ahb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m6_fn_mod_ahb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:M6_FN_MOD_AHB)

For information about available fields see [`mod@m6_fn_mod_ahb`] module*/
pub type M6_FN_MOD_AHB = crate::Reg<m6_fn_mod_ahb::M6_FN_MOD_AHBrs>;
///AXIMC master 6 AHB conversion override functionality register
pub mod m6_fn_mod_ahb;
/**FN_MOD_LB (rw) register accessor: AXIMC long burst capability inhibition register

You can [`read`](crate::Reg::read) this register and get [`fn_mod_lb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fn_mod_lb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#AXIMC_Mx:FN_MOD_LB)

For information about available fields see [`mod@fn_mod_lb`] module*/
pub type FN_MOD_LB = crate::Reg<fn_mod_lb::FN_MOD_LBrs>;
///AXIMC long burst capability inhibition register
pub mod fn_mod_lb;
