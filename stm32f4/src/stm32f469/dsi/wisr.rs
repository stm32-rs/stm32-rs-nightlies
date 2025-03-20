///Register `WISR` reader
pub type R = crate::R<WISRrs>;
/**Tearing effect interrupt flag This bit is set when a tearing effect event occurs.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF {
    ///0: No tearing effect event occurred
    B0x0 = 0,
    ///1: Tearing effect event occurred
    B0x1 = 1,
}
impl From<TEIF> for bool {
    #[inline(always)]
    fn from(variant: TEIF) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIF` reader - Tearing effect interrupt flag This bit is set when a tearing effect event occurs.
pub type TEIF_R = crate::BitReader<TEIF>;
impl TEIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEIF {
        match self.bits {
            false => TEIF::B0x0,
            true => TEIF::B0x1,
        }
    }
    ///No tearing effect event occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF::B0x0
    }
    ///Tearing effect event occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF::B0x1
    }
}
/**End of refresh interrupt flag This bit is set when the transfer of a frame in adapted command mode is finished.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERIF {
    ///0: No end of refresh event occurred
    B0x0 = 0,
    ///1: End of refresh event occurred
    B0x1 = 1,
}
impl From<ERIF> for bool {
    #[inline(always)]
    fn from(variant: ERIF) -> Self {
        variant as u8 != 0
    }
}
///Field `ERIF` reader - End of refresh interrupt flag This bit is set when the transfer of a frame in adapted command mode is finished.
pub type ERIF_R = crate::BitReader<ERIF>;
impl ERIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERIF {
        match self.bits {
            false => ERIF::B0x0,
            true => ERIF::B0x1,
        }
    }
    ///No end of refresh event occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ERIF::B0x0
    }
    ///End of refresh event occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ERIF::B0x1
    }
}
/**Busy flag This bit is set when the transfer of a frame in adapted command mode is ongoing.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY {
    ///0: No transfer on going
    B0x0 = 0,
    ///1: Transfer on going
    B0x1 = 1,
}
impl From<BUSY> for bool {
    #[inline(always)]
    fn from(variant: BUSY) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSY` reader - Busy flag This bit is set when the transfer of a frame in adapted command mode is ongoing.
pub type BUSY_R = crate::BitReader<BUSY>;
impl BUSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSY {
        match self.bits {
            false => BUSY::B0x0,
            true => BUSY::B0x1,
        }
    }
    ///No transfer on going
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BUSY::B0x0
    }
    ///Transfer on going
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BUSY::B0x1
    }
}
/**PLL lock status This bit is set when the PLL is locked and cleared when it is unlocked.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLLS {
    ///0: PLL is unlocked.
    B0x0 = 0,
    ///1: PLL is locked.
    B0x1 = 1,
}
impl From<PLLLS> for bool {
    #[inline(always)]
    fn from(variant: PLLLS) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLLS` reader - PLL lock status This bit is set when the PLL is locked and cleared when it is unlocked.
pub type PLLLS_R = crate::BitReader<PLLLS>;
impl PLLLS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLLS {
        match self.bits {
            false => PLLLS::B0x0,
            true => PLLLS::B0x1,
        }
    }
    ///PLL is unlocked.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLLS::B0x0
    }
    ///PLL is locked.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLLS::B0x1
    }
}
/**PLL lock interrupt flag This bit is set when the PLL becomes locked.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLLIF {
    ///0: No PLL lock event occurred
    B0x0 = 0,
    ///1: PLL lock event occurred
    B0x1 = 1,
}
impl From<PLLLIF> for bool {
    #[inline(always)]
    fn from(variant: PLLLIF) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLLIF` reader - PLL lock interrupt flag This bit is set when the PLL becomes locked.
pub type PLLLIF_R = crate::BitReader<PLLLIF>;
impl PLLLIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLLIF {
        match self.bits {
            false => PLLLIF::B0x0,
            true => PLLLIF::B0x1,
        }
    }
    ///No PLL lock event occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLLIF::B0x0
    }
    ///PLL lock event occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLLIF::B0x1
    }
}
/**PLL unlock interrupt flag This bit is set when the PLL becomes unlocked.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLUIF {
    ///0: No PLL unlock event occurred
    B0x0 = 0,
    ///1: PLL unlock event occurred
    B0x1 = 1,
}
impl From<PLLUIF> for bool {
    #[inline(always)]
    fn from(variant: PLLUIF) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLUIF` reader - PLL unlock interrupt flag This bit is set when the PLL becomes unlocked.
pub type PLLUIF_R = crate::BitReader<PLLUIF>;
impl PLLUIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLUIF {
        match self.bits {
            false => PLLUIF::B0x0,
            true => PLLUIF::B0x1,
        }
    }
    ///No PLL unlock event occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLUIF::B0x0
    }
    ///PLL unlock event occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLUIF::B0x1
    }
}
/**Regulator ready status This bit gives the status of the regulator.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRS {
    ///0: Regulator is not ready.
    B0x0 = 0,
    ///1: Regulator is ready.
    B0x1 = 1,
}
impl From<RRS> for bool {
    #[inline(always)]
    fn from(variant: RRS) -> Self {
        variant as u8 != 0
    }
}
///Field `RRS` reader - Regulator ready status This bit gives the status of the regulator.
pub type RRS_R = crate::BitReader<RRS>;
impl RRS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RRS {
        match self.bits {
            false => RRS::B0x0,
            true => RRS::B0x1,
        }
    }
    ///Regulator is not ready.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RRS::B0x0
    }
    ///Regulator is ready.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RRS::B0x1
    }
}
/**Regulator ready interrupt flag This bit is set when the regulator becomes ready.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRIF {
    ///0: No regulator ready event occurred
    B0x0 = 0,
    ///1: Regulator ready event occurred
    B0x1 = 1,
}
impl From<RRIF> for bool {
    #[inline(always)]
    fn from(variant: RRIF) -> Self {
        variant as u8 != 0
    }
}
///Field `RRIF` reader - Regulator ready interrupt flag This bit is set when the regulator becomes ready.
pub type RRIF_R = crate::BitReader<RRIF>;
impl RRIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RRIF {
        match self.bits {
            false => RRIF::B0x0,
            true => RRIF::B0x1,
        }
    }
    ///No regulator ready event occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RRIF::B0x0
    }
    ///Regulator ready event occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RRIF::B0x1
    }
}
impl R {
    ///Bit 0 - Tearing effect interrupt flag This bit is set when a tearing effect event occurs.
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of refresh interrupt flag This bit is set when the transfer of a frame in adapted command mode is finished.
    #[inline(always)]
    pub fn erif(&self) -> ERIF_R {
        ERIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Busy flag This bit is set when the transfer of a frame in adapted command mode is ongoing.
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - PLL lock status This bit is set when the PLL is locked and cleared when it is unlocked.
    #[inline(always)]
    pub fn pllls(&self) -> PLLLS_R {
        PLLLS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLL lock interrupt flag This bit is set when the PLL becomes locked.
    #[inline(always)]
    pub fn plllif(&self) -> PLLLIF_R {
        PLLLIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PLL unlock interrupt flag This bit is set when the PLL becomes unlocked.
    #[inline(always)]
    pub fn plluif(&self) -> PLLUIF_R {
        PLLUIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Regulator ready status This bit gives the status of the regulator.
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Regulator ready interrupt flag This bit is set when the regulator becomes ready.
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WISR")
            .field("teif", &self.teif())
            .field("erif", &self.erif())
            .field("busy", &self.busy())
            .field("pllls", &self.pllls())
            .field("plllif", &self.plllif())
            .field("plluif", &self.plluif())
            .field("rrs", &self.rrs())
            .field("rrif", &self.rrif())
            .finish()
    }
}
/**DSI Wrapper interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`wisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:WISR)*/
pub struct WISRrs;
impl crate::RegisterSpec for WISRrs {
    type Ux = u32;
}
///`read()` method returns [`wisr::R`](R) reader structure
impl crate::Readable for WISRrs {}
///`reset()` method sets WISR to value 0
impl crate::Resettable for WISRrs {}
