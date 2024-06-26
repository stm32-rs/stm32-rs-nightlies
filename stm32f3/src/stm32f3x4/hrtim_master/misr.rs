///Register `MISR` reader
pub type R = crate::R<MISRrs>;
/**Master Compare 1 Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCMP1 {
    ///0: No master compare interrupt occurred
    NoEvent = 0,
    ///1: Master compare interrupt occurred
    Event = 1,
}
impl From<MCMP1> for bool {
    #[inline(always)]
    fn from(variant: MCMP1) -> Self {
        variant as u8 != 0
    }
}
///Field `MCMP1` reader - Master Compare 1 Interrupt Flag
pub type MCMP1_R = crate::BitReader<MCMP1>;
impl MCMP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MCMP1 {
        match self.bits {
            false => MCMP1::NoEvent,
            true => MCMP1::Event,
        }
    }
    ///No master compare interrupt occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == MCMP1::NoEvent
    }
    ///Master compare interrupt occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == MCMP1::Event
    }
}
///Field `MCMP2` reader - Master Compare 2 Interrupt Flag
pub use MCMP1_R as MCMP2_R;
///Field `MCMP3` reader - Master Compare 3 Interrupt Flag
pub use MCMP1_R as MCMP3_R;
///Field `MCMP4` reader - Master Compare 4 Interrupt Flag
pub use MCMP1_R as MCMP4_R;
/**Master Repetition Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MREP {
    ///0: No master repetition interrupt occurred
    NoEvent = 0,
    ///1: Master repetition interrupt occurred
    Event = 1,
}
impl From<MREP> for bool {
    #[inline(always)]
    fn from(variant: MREP) -> Self {
        variant as u8 != 0
    }
}
///Field `MREP` reader - Master Repetition Interrupt Flag
pub type MREP_R = crate::BitReader<MREP>;
impl MREP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MREP {
        match self.bits {
            false => MREP::NoEvent,
            true => MREP::Event,
        }
    }
    ///No master repetition interrupt occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == MREP::NoEvent
    }
    ///Master repetition interrupt occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == MREP::Event
    }
}
/**Sync Input Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC {
    ///0: No sync input interrupt occurred
    NoEvent = 0,
    ///1: Sync input interrupt occurred
    Event = 1,
}
impl From<SYNC> for bool {
    #[inline(always)]
    fn from(variant: SYNC) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNC` reader - Sync Input Interrupt Flag
pub type SYNC_R = crate::BitReader<SYNC>;
impl SYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYNC {
        match self.bits {
            false => SYNC::NoEvent,
            true => SYNC::Event,
        }
    }
    ///No sync input interrupt occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == SYNC::NoEvent
    }
    ///Sync input interrupt occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == SYNC::Event
    }
}
/**Master Update Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUPD {
    ///0: No master update interrupt occurred
    NoEvent = 0,
    ///1: Master update interrupt occurred
    Event = 1,
}
impl From<MUPD> for bool {
    #[inline(always)]
    fn from(variant: MUPD) -> Self {
        variant as u8 != 0
    }
}
///Field `MUPD` reader - Master Update Interrupt Flag
pub type MUPD_R = crate::BitReader<MUPD>;
impl MUPD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MUPD {
        match self.bits {
            false => MUPD::NoEvent,
            true => MUPD::Event,
        }
    }
    ///No master update interrupt occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == MUPD::NoEvent
    }
    ///Master update interrupt occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == MUPD::Event
    }
}
impl R {
    ///Bit 0 - Master Compare 1 Interrupt Flag
    #[inline(always)]
    pub fn mcmp1(&self) -> MCMP1_R {
        MCMP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Master Compare 2 Interrupt Flag
    #[inline(always)]
    pub fn mcmp2(&self) -> MCMP2_R {
        MCMP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Master Compare 3 Interrupt Flag
    #[inline(always)]
    pub fn mcmp3(&self) -> MCMP3_R {
        MCMP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Master Compare 4 Interrupt Flag
    #[inline(always)]
    pub fn mcmp4(&self) -> MCMP4_R {
        MCMP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Master Repetition Interrupt Flag
    #[inline(always)]
    pub fn mrep(&self) -> MREP_R {
        MREP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Sync Input Interrupt Flag
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Master Update Interrupt Flag
    #[inline(always)]
    pub fn mupd(&self) -> MUPD_R {
        MUPD_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR")
            .field("mupd", &self.mupd())
            .field("sync", &self.sync())
            .field("mrep", &self.mrep())
            .field("mcmp1", &self.mcmp1())
            .field("mcmp4", &self.mcmp4())
            .field("mcmp3", &self.mcmp3())
            .field("mcmp2", &self.mcmp2())
            .finish()
    }
}
/**Master Timer Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_Master:MISR)*/
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
///`read()` method returns [`misr::R`](R) reader structure
impl crate::Readable for MISRrs {}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISRrs {
    const RESET_VALUE: u32 = 0;
}
