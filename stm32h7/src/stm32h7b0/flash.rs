#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    acr: ACR,
    _reserved_1_acr_: [u8; 0x0200],
}
impl RegisterBlock {
    ///0x00 - Access control register
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    ///0x04..0x204 - Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R
    ///
    ///<div class="warning">`n` is the index of cluster in the array. `n == 0` corresponds to `BANK1` cluster.</div>
    #[inline(always)]
    pub const fn bank(&self, n: usize) -> &BANK {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(256 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x04..0x204 - Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R
    #[inline(always)]
    pub fn bank_iter(&self) -> impl Iterator<Item = &BANK> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(256 * n)
                .cast()
        })
    }
    ///0x04..0x104 - Cluster BANK1, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R
    #[inline(always)]
    pub const fn bank1(&self) -> &BANK {
        self.bank(0)
    }
    ///0x104..0x204 - Cluster BANK2, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R
    #[inline(always)]
    pub const fn bank2(&self) -> &BANK {
        self.bank(1)
    }
    ///0x08 - FLASH option key register
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x18 - FLASH option control register
    #[inline(always)]
    pub const fn optcr(&self) -> &OPTCR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x1c - FLASH option status register
    #[inline(always)]
    pub const fn optsr_cur(&self) -> &OPTSR_CUR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - FLASH option status register
    #[inline(always)]
    pub const fn optsr_prg(&self) -> &OPTSR_PRG {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x24 - FLASH option clear control register
    #[inline(always)]
    pub const fn optccr(&self) -> &OPTCCR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    ///0x40 - FLASH register with boot address
    #[inline(always)]
    pub const fn boot_curr(&self) -> &BOOT_CURR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    ///0x44 - FLASH register with boot address
    #[inline(always)]
    pub const fn boot_prgr(&self) -> &BOOT_PRGR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(68).cast() }
    }
    ///0x5c - FLASH CRC data register
    #[inline(always)]
    pub const fn crcdatar(&self) -> &CRCDATAR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(92).cast() }
    }
    ///0x68 - FLASH OTP block lock
    #[inline(always)]
    pub const fn otpbl_cur(&self) -> &OTPBL_CUR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(104).cast() }
    }
    ///0x6c - FLASH OTP block lock
    #[inline(always)]
    pub const fn otpbl_prg(&self) -> &OTPBL_PRG {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    ///0x100 - Access control register
    #[inline(always)]
    pub const fn acr_(&self) -> &ACR_ {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    ///0x108 - FLASH option key register
    #[inline(always)]
    pub const fn optkeyr_(&self) -> &OPTKEYR_ {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(264).cast() }
    }
    ///0x118 - FLASH option control register
    #[inline(always)]
    pub const fn optcr_(&self) -> &OPTCR_ {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(280).cast() }
    }
    ///0x11c - FLASH option status register
    #[inline(always)]
    pub const fn optsr_cur_(&self) -> &OPTSR_CUR_ {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(284).cast() }
    }
    ///0x120 - FLASH option status register
    #[inline(always)]
    pub const fn optsr_prg_(&self) -> &OPTSR_PRG_ {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(288).cast() }
    }
    ///0x124 - FLASH option clear control register
    #[inline(always)]
    pub const fn optccr_(&self) -> &OPTCCR_ {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(292).cast() }
    }
    ///0x140 - FLASH register with boot address
    #[inline(always)]
    pub const fn boot_curr_(&self) -> &BOOT_CURR_ {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    ///0x144 - FLASH register with boot address
    #[inline(always)]
    pub const fn boot_prgr_(&self) -> &BOOT_PRGR_ {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(324).cast() }
    }
}
/**ACR (rw) register accessor: Access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:ACR)

For information about available fields see [`mod@acr`] module*/
pub type ACR = crate::Reg<acr::ACRrs>;
///Access control register
pub mod acr;
///Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R
pub use self::bank::BANK;
///Cluster
///Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R
pub mod bank;
/**OPTKEYR (w) register accessor: FLASH option key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:OPTKEYR)

For information about available fields see [`mod@optkeyr`] module*/
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
///FLASH option key register
pub mod optkeyr;
/**OPTCR (rw) register accessor: FLASH option control register

You can [`read`](crate::Reg::read) this register and get [`optcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:OPTCR)

For information about available fields see [`mod@optcr`] module*/
pub type OPTCR = crate::Reg<optcr::OPTCRrs>;
///FLASH option control register
pub mod optcr;
/**OPTSR_CUR (rw) register accessor: FLASH option status register

You can [`read`](crate::Reg::read) this register and get [`optsr_cur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_cur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:OPTSR_CUR)

For information about available fields see [`mod@optsr_cur`] module*/
pub type OPTSR_CUR = crate::Reg<optsr_cur::OPTSR_CURrs>;
///FLASH option status register
pub mod optsr_cur;
/**OPTSR_PRG (rw) register accessor: FLASH option status register

You can [`read`](crate::Reg::read) this register and get [`optsr_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:OPTSR_PRG)

For information about available fields see [`mod@optsr_prg`] module*/
pub type OPTSR_PRG = crate::Reg<optsr_prg::OPTSR_PRGrs>;
///FLASH option status register
pub mod optsr_prg;
/**OPTCCR (w) register accessor: FLASH option clear control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:OPTCCR)

For information about available fields see [`mod@optccr`] module*/
pub type OPTCCR = crate::Reg<optccr::OPTCCRrs>;
///FLASH option clear control register
pub mod optccr;
/**BOOT_CURR (r) register accessor: FLASH register with boot address

You can [`read`](crate::Reg::read) this register and get [`boot_curr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:BOOT_CURR)

For information about available fields see [`mod@boot_curr`] module*/
pub type BOOT_CURR = crate::Reg<boot_curr::BOOT_CURRrs>;
///FLASH register with boot address
pub mod boot_curr;
/**BOOT_PRGR (r) register accessor: FLASH register with boot address

You can [`read`](crate::Reg::read) this register and get [`boot_prgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:BOOT_PRGR)

For information about available fields see [`mod@boot_prgr`] module*/
pub type BOOT_PRGR = crate::Reg<boot_prgr::BOOT_PRGRrs>;
///FLASH register with boot address
pub mod boot_prgr;
/**CRCDATAR (rw) register accessor: FLASH CRC data register

You can [`read`](crate::Reg::read) this register and get [`crcdatar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdatar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:CRCDATAR)

For information about available fields see [`mod@crcdatar`] module*/
pub type CRCDATAR = crate::Reg<crcdatar::CRCDATARrs>;
///FLASH CRC data register
pub mod crcdatar;
/**OTPBL_CUR (r) register accessor: FLASH OTP block lock

You can [`read`](crate::Reg::read) this register and get [`otpbl_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:OTPBL_CUR)

For information about available fields see [`mod@otpbl_cur`] module*/
pub type OTPBL_CUR = crate::Reg<otpbl_cur::OTPBL_CURrs>;
///FLASH OTP block lock
pub mod otpbl_cur;
/**OTPBL_PRG (rw) register accessor: FLASH OTP block lock

You can [`read`](crate::Reg::read) this register and get [`otpbl_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpbl_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:OTPBL_PRG)

For information about available fields see [`mod@otpbl_prg`] module*/
pub type OTPBL_PRG = crate::Reg<otpbl_prg::OTPBL_PRGrs>;
///FLASH OTP block lock
pub mod otpbl_prg;
/**ACR_ (rw) register accessor: Access control register

You can [`read`](crate::Reg::read) this register and get [`acr_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:ACR_)

For information about available fields see [`mod@acr_`] module*/
pub type ACR_ = crate::Reg<acr_::ACR_rs>;
///Access control register
pub mod acr_;
/**OPTKEYR_ (rw) register accessor: FLASH option key register

You can [`read`](crate::Reg::read) this register and get [`optkeyr_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:OPTKEYR_)

For information about available fields see [`mod@optkeyr_`] module*/
pub type OPTKEYR_ = crate::Reg<optkeyr_::OPTKEYR_rs>;
///FLASH option key register
pub mod optkeyr_;
/**OPTCR_ (rw) register accessor: FLASH option control register

You can [`read`](crate::Reg::read) this register and get [`optcr_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:OPTCR_)

For information about available fields see [`mod@optcr_`] module*/
pub type OPTCR_ = crate::Reg<optcr_::OPTCR_rs>;
///FLASH option control register
pub mod optcr_;
/**OPTSR_CUR_ (rw) register accessor: FLASH option status register

You can [`read`](crate::Reg::read) this register and get [`optsr_cur_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_cur_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:OPTSR_CUR_)

For information about available fields see [`mod@optsr_cur_`] module*/
pub type OPTSR_CUR_ = crate::Reg<optsr_cur_::OPTSR_CUR_rs>;
///FLASH option status register
pub mod optsr_cur_;
/**OPTSR_PRG_ (rw) register accessor: FLASH option status register

You can [`read`](crate::Reg::read) this register and get [`optsr_prg_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_prg_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:OPTSR_PRG_)

For information about available fields see [`mod@optsr_prg_`] module*/
pub type OPTSR_PRG_ = crate::Reg<optsr_prg_::OPTSR_PRG_rs>;
///FLASH option status register
pub mod optsr_prg_;
/**OPTCCR_ (w) register accessor: FLASH option clear control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optccr_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:OPTCCR_)

For information about available fields see [`mod@optccr_`] module*/
pub type OPTCCR_ = crate::Reg<optccr_::OPTCCR_rs>;
///FLASH option clear control register
pub mod optccr_;
/**BOOT_CURR_ (r) register accessor: FLASH register with boot address

You can [`read`](crate::Reg::read) this register and get [`boot_curr_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:BOOT_CURR_)

For information about available fields see [`mod@boot_curr_`] module*/
pub type BOOT_CURR_ = crate::Reg<boot_curr_::BOOT_CURR_rs>;
///FLASH register with boot address
pub mod boot_curr_;
/**BOOT_PRGR_ (r) register accessor: FLASH register with boot address

You can [`read`](crate::Reg::read) this register and get [`boot_prgr_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FLASH:BOOT_PRGR_)

For information about available fields see [`mod@boot_prgr_`] module*/
pub type BOOT_PRGR_ = crate::Reg<boot_prgr_::BOOT_PRGR_rs>;
///FLASH register with boot address
pub mod boot_prgr_;
