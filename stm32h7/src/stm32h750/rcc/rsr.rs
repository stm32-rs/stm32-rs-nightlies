///Register `RSR` reader
pub type R = crate::R<RSRrs>;
///Register `RSR` writer
pub type W = crate::W<RSRrs>;
/**Remove reset flag

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
///Field `RMVF` reader - Remove reset flag
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
///Field `RMVF` writer - Remove reset flag
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
/**CPU reset flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPURSTFR {
    ///0: No reset occurred for block
    NoResetOccurred = 0,
    ///1: Reset occurred for block
    ResetOccurred = 1,
}
impl From<CPURSTFR> for bool {
    #[inline(always)]
    fn from(variant: CPURSTFR) -> Self {
        variant as u8 != 0
    }
}
///Field `CPURSTF` reader - CPU reset flag
pub type CPURSTF_R = crate::BitReader<CPURSTFR>;
impl CPURSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CPURSTFR {
        match self.bits {
            false => CPURSTFR::NoResetOccurred,
            true => CPURSTFR::ResetOccurred,
        }
    }
    ///No reset occurred for block
    #[inline(always)]
    pub fn is_no_reset_occurred(&self) -> bool {
        *self == CPURSTFR::NoResetOccurred
    }
    ///Reset occurred for block
    #[inline(always)]
    pub fn is_reset_occurred(&self) -> bool {
        *self == CPURSTFR::ResetOccurred
    }
}
///Field `CPURSTF` writer - CPU reset flag
pub type CPURSTF_W<'a, REG> = crate::BitWriter<'a, REG, CPURSTFR>;
impl<'a, REG> CPURSTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No reset occurred for block
    #[inline(always)]
    pub fn no_reset_occurred(self) -> &'a mut crate::W<REG> {
        self.variant(CPURSTFR::NoResetOccurred)
    }
    ///Reset occurred for block
    #[inline(always)]
    pub fn reset_occurred(self) -> &'a mut crate::W<REG> {
        self.variant(CPURSTFR::ResetOccurred)
    }
}
///Field `D1RSTF` reader - D1 domain power switch reset flag
pub use CPURSTF_R as D1RSTF_R;
///Field `D2RSTF` reader - D2 domain power switch reset flag
pub use CPURSTF_R as D2RSTF_R;
///Field `BORRSTF` reader - BOR reset flag
pub use CPURSTF_R as BORRSTF_R;
///Field `PINRSTF` reader - Pin reset flag (NRST)
pub use CPURSTF_R as PINRSTF_R;
///Field `PORRSTF` reader - POR/PDR reset flag
pub use CPURSTF_R as PORRSTF_R;
///Field `SFTRSTF` reader - System reset from CPU reset flag
pub use CPURSTF_R as SFTRSTF_R;
///Field `IWDG1RSTF` reader - Independent Watchdog reset flag
pub use CPURSTF_R as IWDG1RSTF_R;
///Field `WWDG1RSTF` reader - Window Watchdog reset flag
pub use CPURSTF_R as WWDG1RSTF_R;
///Field `LPWRRSTF` reader - Reset due to illegal D1 DStandby or CPU CStop flag
pub use CPURSTF_R as LPWRRSTF_R;
///Field `D1RSTF` writer - D1 domain power switch reset flag
pub use CPURSTF_W as D1RSTF_W;
///Field `D2RSTF` writer - D2 domain power switch reset flag
pub use CPURSTF_W as D2RSTF_W;
///Field `BORRSTF` writer - BOR reset flag
pub use CPURSTF_W as BORRSTF_W;
///Field `PINRSTF` writer - Pin reset flag (NRST)
pub use CPURSTF_W as PINRSTF_W;
///Field `PORRSTF` writer - POR/PDR reset flag
pub use CPURSTF_W as PORRSTF_W;
///Field `SFTRSTF` writer - System reset from CPU reset flag
pub use CPURSTF_W as SFTRSTF_W;
///Field `IWDG1RSTF` writer - Independent Watchdog reset flag
pub use CPURSTF_W as IWDG1RSTF_W;
///Field `WWDG1RSTF` writer - Window Watchdog reset flag
pub use CPURSTF_W as WWDG1RSTF_W;
///Field `LPWRRSTF` writer - Reset due to illegal D1 DStandby or CPU CStop flag
pub use CPURSTF_W as LPWRRSTF_W;
impl R {
    ///Bit 16 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CPU reset flag
    #[inline(always)]
    pub fn cpurstf(&self) -> CPURSTF_R {
        CPURSTF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - D1 domain power switch reset flag
    #[inline(always)]
    pub fn d1rstf(&self) -> D1RSTF_R {
        D1RSTF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - D2 domain power switch reset flag
    #[inline(always)]
    pub fn d2rstf(&self) -> D2RSTF_R {
        D2RSTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - BOR reset flag
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Pin reset flag (NRST)
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - POR/PDR reset flag
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - System reset from CPU reset flag
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - Independent Watchdog reset flag
    #[inline(always)]
    pub fn iwdg1rstf(&self) -> IWDG1RSTF_R {
        IWDG1RSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Window Watchdog reset flag
    #[inline(always)]
    pub fn wwdg1rstf(&self) -> WWDG1RSTF_R {
        WWDG1RSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Reset due to illegal D1 DStandby or CPU CStop flag
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSR")
            .field("rmvf", &self.rmvf())
            .field("cpurstf", &self.cpurstf())
            .field("d1rstf", &self.d1rstf())
            .field("d2rstf", &self.d2rstf())
            .field("borrstf", &self.borrstf())
            .field("pinrstf", &self.pinrstf())
            .field("porrstf", &self.porrstf())
            .field("sftrstf", &self.sftrstf())
            .field("iwdg1rstf", &self.iwdg1rstf())
            .field("wwdg1rstf", &self.wwdg1rstf())
            .field("lpwrrstf", &self.lpwrrstf())
            .finish()
    }
}
impl W {
    ///Bit 16 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<'_, RSRrs> {
        RMVF_W::new(self, 16)
    }
    ///Bit 17 - CPU reset flag
    #[inline(always)]
    pub fn cpurstf(&mut self) -> CPURSTF_W<'_, RSRrs> {
        CPURSTF_W::new(self, 17)
    }
    ///Bit 19 - D1 domain power switch reset flag
    #[inline(always)]
    pub fn d1rstf(&mut self) -> D1RSTF_W<'_, RSRrs> {
        D1RSTF_W::new(self, 19)
    }
    ///Bit 20 - D2 domain power switch reset flag
    #[inline(always)]
    pub fn d2rstf(&mut self) -> D2RSTF_W<'_, RSRrs> {
        D2RSTF_W::new(self, 20)
    }
    ///Bit 21 - BOR reset flag
    #[inline(always)]
    pub fn borrstf(&mut self) -> BORRSTF_W<'_, RSRrs> {
        BORRSTF_W::new(self, 21)
    }
    ///Bit 22 - Pin reset flag (NRST)
    #[inline(always)]
    pub fn pinrstf(&mut self) -> PINRSTF_W<'_, RSRrs> {
        PINRSTF_W::new(self, 22)
    }
    ///Bit 23 - POR/PDR reset flag
    #[inline(always)]
    pub fn porrstf(&mut self) -> PORRSTF_W<'_, RSRrs> {
        PORRSTF_W::new(self, 23)
    }
    ///Bit 24 - System reset from CPU reset flag
    #[inline(always)]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<'_, RSRrs> {
        SFTRSTF_W::new(self, 24)
    }
    ///Bit 26 - Independent Watchdog reset flag
    #[inline(always)]
    pub fn iwdg1rstf(&mut self) -> IWDG1RSTF_W<'_, RSRrs> {
        IWDG1RSTF_W::new(self, 26)
    }
    ///Bit 28 - Window Watchdog reset flag
    #[inline(always)]
    pub fn wwdg1rstf(&mut self) -> WWDG1RSTF_W<'_, RSRrs> {
        WWDG1RSTF_W::new(self, 28)
    }
    ///Bit 30 - Reset due to illegal D1 DStandby or CPU CStop flag
    #[inline(always)]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W<'_, RSRrs> {
        LPWRRSTF_W::new(self, 30)
    }
}
/**RCC Reset Status Register

You can [`read`](crate::Reg::read) this register and get [`rsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RCC:RSR)*/
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
///`reset()` method sets RSR to value 0
impl crate::Resettable for RSRrs {}
