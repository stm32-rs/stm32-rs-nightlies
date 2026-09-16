///Register `RSR` reader
pub type R = crate::R<RSRrs>;
///Register `RSR` writer
pub type W = crate::W<RSRrs>;
/**remove reset flag Set and reset by software to reset the value of the reset flags.

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
///Field `RMVF` reader - remove reset flag Set and reset by software to reset the value of the reset flags.
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
///Field `RMVF` writer - remove reset flag Set and reset by software to reset the value of the reset flags.
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
/**CPU domain power-switch reset flag Reset by software by writing the RMVF bit. Set by hardware when a the CPU domain exits from DStop or after of power-on reset. Set also when the CPU domain exists DStop2 but only when a pad reset has occurred during DStop2 (PINRST bit also set by hardware)

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDRSTFR {
    ///0: No reset occurred for block
    NoResetOccurred = 0,
    ///1: Reset occurred for block
    ResetOccurred = 1,
}
impl From<CDRSTFR> for bool {
    #[inline(always)]
    fn from(variant: CDRSTFR) -> Self {
        variant as u8 != 0
    }
}
///Field `CDRSTF` reader - CPU domain power-switch reset flag Reset by software by writing the RMVF bit. Set by hardware when a the CPU domain exits from DStop or after of power-on reset. Set also when the CPU domain exists DStop2 but only when a pad reset has occurred during DStop2 (PINRST bit also set by hardware)
pub type CDRSTF_R = crate::BitReader<CDRSTFR>;
impl CDRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CDRSTFR {
        match self.bits {
            false => CDRSTFR::NoResetOccurred,
            true => CDRSTFR::ResetOccurred,
        }
    }
    ///No reset occurred for block
    #[inline(always)]
    pub fn is_no_reset_occurred(&self) -> bool {
        *self == CDRSTFR::NoResetOccurred
    }
    ///Reset occurred for block
    #[inline(always)]
    pub fn is_reset_occurred(&self) -> bool {
        *self == CDRSTFR::ResetOccurred
    }
}
///Field `BORRSTF` reader - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst).
pub use CDRSTF_R as BORRSTF_R;
///Field `PINRSTF` reader - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs.
pub use CDRSTF_R as PINRSTF_R;
///Field `PORRSTF` reader - POR/PDR reset flag Reset by software by writing the RMVF bit. Set by hardware when a POR/PDR reset occurs.
pub use CDRSTF_R as PORRSTF_R;
///Field `SFTRSTF` reader - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M7.
pub use CDRSTF_R as SFTRSTF_R;
///Field `IWDGRSTF` reader - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs.
pub use CDRSTF_R as IWDGRSTF_R;
///Field `WWDGRSTF` reader - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs.
pub use CDRSTF_R as WWDGRSTF_R;
///Field `LPWRRSTF` reader - reset due to illegal CD DStop or CD DStop2 or CPU CStop flag Reset by software by writing the RMVF bit. Set by hardware when the CPU domain goes erroneously in DStop or DStop2, or when the CPU goes erroneously in CStop.
pub use CDRSTF_R as LPWRRSTF_R;
impl R {
    ///Bit 16 - remove reset flag Set and reset by software to reset the value of the reset flags.
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 19 - CPU domain power-switch reset flag Reset by software by writing the RMVF bit. Set by hardware when a the CPU domain exits from DStop or after of power-on reset. Set also when the CPU domain exists DStop2 but only when a pad reset has occurred during DStop2 (PINRST bit also set by hardware)
    #[inline(always)]
    pub fn cdrstf(&self) -> CDRSTF_R {
        CDRSTF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst).
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs.
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - POR/PDR reset flag Reset by software by writing the RMVF bit. Set by hardware when a POR/PDR reset occurs.
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M7.
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs.
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs.
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - reset due to illegal CD DStop or CD DStop2 or CPU CStop flag Reset by software by writing the RMVF bit. Set by hardware when the CPU domain goes erroneously in DStop or DStop2, or when the CPU goes erroneously in CStop.
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSR")
            .field("rmvf", &self.rmvf())
            .field("cdrstf", &self.cdrstf())
            .field("borrstf", &self.borrstf())
            .field("pinrstf", &self.pinrstf())
            .field("porrstf", &self.porrstf())
            .field("sftrstf", &self.sftrstf())
            .field("iwdgrstf", &self.iwdgrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("lpwrrstf", &self.lpwrrstf())
            .finish()
    }
}
impl W {
    ///Bit 16 - remove reset flag Set and reset by software to reset the value of the reset flags.
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<'_, RSRrs> {
        RMVF_W::new(self, 16)
    }
}
/**RCC reset status register

You can [`read`](crate::Reg::read) this register and get [`rsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#RCC:RSR)*/
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
///`reset()` method sets RSR to value 0x00e8_0000
impl crate::Resettable for RSRrs {
    const RESET_VALUE: u32 = 0x00e8_0000;
}
