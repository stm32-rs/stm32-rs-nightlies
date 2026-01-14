///Register `ITLINE2` reader
pub type R = crate::R<ITLINE2rs>;
/**TAMP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<TAMP> for bool {
    #[inline(always)]
    fn from(variant: TAMP) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP` reader - TAMP
pub type TAMP_R = crate::BitReader<TAMP>;
impl TAMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMP {
        match self.bits {
            false => TAMP::NotInterrupted,
            true => TAMP::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == TAMP::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == TAMP::Interrupted
    }
}
///Field `RTC` reader - RTC
pub use TAMP_R as RTC_R;
impl R {
    ///Bit 0 - TAMP
    #[inline(always)]
    pub fn tamp(&self) -> TAMP_R {
        TAMP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RTC
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE2")
            .field("tamp", &self.tamp())
            .field("rtc", &self.rtc())
            .finish()
    }
}
/**interrupt line 2 status register

You can [`read`](crate::Reg::read) this register and get [`itline2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#SYSCFG:ITLINE2)*/
pub struct ITLINE2rs;
impl crate::RegisterSpec for ITLINE2rs {
    type Ux = u32;
}
///`read()` method returns [`itline2::R`](R) reader structure
impl crate::Readable for ITLINE2rs {}
///`reset()` method sets ITLINE2 to value 0
impl crate::Resettable for ITLINE2rs {}
