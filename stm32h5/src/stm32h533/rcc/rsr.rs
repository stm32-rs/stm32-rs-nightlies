///Register `RSR` reader
pub type R = crate::R<RSRrs>;
///Register `RSR` writer
pub type W = crate::W<RSRrs>;
/**remove reset flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVF {
    ///0: Reset not activated
    NotActivated = 0,
    ///1: Reset the reset status flags
    Reset = 1,
}
impl From<RMVF> for bool {
    #[inline(always)]
    fn from(variant: RMVF) -> Self {
        variant as u8 != 0
    }
}
///Field `RMVF` reader - remove reset flag
pub type RMVF_R = crate::BitReader<RMVF>;
impl RMVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RMVF {
        match self.bits {
            false => RMVF::NotActivated,
            true => RMVF::Reset,
        }
    }
    ///Reset not activated
    #[inline(always)]
    pub fn is_not_activated(&self) -> bool {
        *self == RMVF::NotActivated
    }
    ///Reset the reset status flags
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RMVF::Reset
    }
}
///Field `RMVF` writer - remove reset flag
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG, RMVF>;
impl<'a, REG> RMVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset not activated
    #[inline(always)]
    pub fn not_activated(self) -> &'a mut crate::W<REG> {
        self.variant(RMVF::NotActivated)
    }
    ///Reset the reset status flags
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RMVF::Reset)
    }
}
/**pin reset flag (NRST)

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINRSTFR {
    ///0: No reset occurred for block
    NoResetOccurred = 0,
    ///1: Reset occurred for block
    ResetOccurred = 1,
}
impl From<PINRSTFR> for bool {
    #[inline(always)]
    fn from(variant: PINRSTFR) -> Self {
        variant as u8 != 0
    }
}
///Field `PINRSTF` reader - pin reset flag (NRST)
pub type PINRSTF_R = crate::BitReader<PINRSTFR>;
impl PINRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PINRSTFR {
        match self.bits {
            false => PINRSTFR::NoResetOccurred,
            true => PINRSTFR::ResetOccurred,
        }
    }
    ///No reset occurred for block
    #[inline(always)]
    pub fn is_no_reset_occurred(&self) -> bool {
        *self == PINRSTFR::NoResetOccurred
    }
    ///Reset occurred for block
    #[inline(always)]
    pub fn is_reset_occurred(&self) -> bool {
        *self == PINRSTFR::ResetOccurred
    }
}
///Field `PINRSTF` writer - pin reset flag (NRST)
pub type PINRSTF_W<'a, REG> = crate::BitWriter<'a, REG, PINRSTFR>;
impl<'a, REG> PINRSTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No reset occurred for block
    #[inline(always)]
    pub fn no_reset_occurred(self) -> &'a mut crate::W<REG> {
        self.variant(PINRSTFR::NoResetOccurred)
    }
    ///Reset occurred for block
    #[inline(always)]
    pub fn reset_occurred(self) -> &'a mut crate::W<REG> {
        self.variant(PINRSTFR::ResetOccurred)
    }
}
///Field `BORRSTF` reader - BOR reset flag
pub use PINRSTF_R as BORRSTF_R;
///Field `SFTRSTF` reader - system reset from CPU reset flag
pub use PINRSTF_R as SFTRSTF_R;
///Field `IWDGRSTF` reader - independent watchdog reset flag
pub use PINRSTF_R as IWDGRSTF_R;
///Field `WWDGRSTF` reader - window watchdog reset flag
pub use PINRSTF_R as WWDGRSTF_R;
///Field `LPWRRSTF` reader - Low-power reset flag
pub use PINRSTF_R as LPWRRSTF_R;
///Field `BORRSTF` writer - BOR reset flag
pub use PINRSTF_W as BORRSTF_W;
///Field `SFTRSTF` writer - system reset from CPU reset flag
pub use PINRSTF_W as SFTRSTF_W;
///Field `IWDGRSTF` writer - independent watchdog reset flag
pub use PINRSTF_W as IWDGRSTF_W;
///Field `WWDGRSTF` writer - window watchdog reset flag
pub use PINRSTF_W as WWDGRSTF_W;
///Field `LPWRRSTF` writer - Low-power reset flag
pub use PINRSTF_W as LPWRRSTF_W;
impl R {
    ///Bit 23 - remove reset flag
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 26 - pin reset flag (NRST)
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - BOR reset flag
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - system reset from CPU reset flag
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - independent watchdog reset flag
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSR")
            .field("rmvf", &self.rmvf())
            .field("pinrstf", &self.pinrstf())
            .field("borrstf", &self.borrstf())
            .field("sftrstf", &self.sftrstf())
            .field("iwdgrstf", &self.iwdgrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("lpwrrstf", &self.lpwrrstf())
            .finish()
    }
}
impl W {
    ///Bit 23 - remove reset flag
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<'_, RSRrs> {
        RMVF_W::new(self, 23)
    }
    ///Bit 26 - pin reset flag (NRST)
    #[inline(always)]
    pub fn pinrstf(&mut self) -> PINRSTF_W<'_, RSRrs> {
        PINRSTF_W::new(self, 26)
    }
    ///Bit 27 - BOR reset flag
    #[inline(always)]
    pub fn borrstf(&mut self) -> BORRSTF_W<'_, RSRrs> {
        BORRSTF_W::new(self, 27)
    }
    ///Bit 28 - system reset from CPU reset flag
    #[inline(always)]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<'_, RSRrs> {
        SFTRSTF_W::new(self, 28)
    }
    ///Bit 29 - independent watchdog reset flag
    #[inline(always)]
    pub fn iwdgrstf(&mut self) -> IWDGRSTF_W<'_, RSRrs> {
        IWDGRSTF_W::new(self, 29)
    }
    ///Bit 30 - window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W<'_, RSRrs> {
        WWDGRSTF_W::new(self, 30)
    }
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W<'_, RSRrs> {
        LPWRRSTF_W::new(self, 31)
    }
}
/**RCC reset status register

You can [`read`](crate::Reg::read) this register and get [`rsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#RCC:RSR)*/
pub struct RSRrs;
impl crate::RegisterSpec for RSRrs {
    type Ux = u32;
}
///`read()` method returns [`rsr::R`](R) reader structure
impl crate::Readable for RSRrs {}
///`write(|w| ..)` method takes [`rsr::W`](W) writer structure
impl crate::Writable for RSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RSR to value 0x0c00_0000
impl crate::Resettable for RSRrs {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
