///Register `AHB4LPENR` reader
pub type R = crate::R<AHB4LPENRrs>;
///Register `AHB4LPENR` writer
pub type W = crate::W<AHB4LPENRrs>;
/**SDMMC1 and SDMMC1 delay peripheral clock enable during sleep mode Set and reset by software

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1LPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<SDMMC1LPEN> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1LPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SDMMC1LPEN` reader - SDMMC1 and SDMMC1 delay peripheral clock enable during sleep mode Set and reset by software
pub type SDMMC1LPEN_R = crate::BitReader<SDMMC1LPEN>;
impl SDMMC1LPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDMMC1LPEN {
        match self.bits {
            false => SDMMC1LPEN::Disabled,
            true => SDMMC1LPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDMMC1LPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDMMC1LPEN::Enabled
    }
}
///Field `SDMMC1LPEN` writer - SDMMC1 and SDMMC1 delay peripheral clock enable during sleep mode Set and reset by software
pub type SDMMC1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, SDMMC1LPEN>;
impl<'a, REG> SDMMC1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1LPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1LPEN::Enabled)
    }
}
///Field `SDMMC2LPEN` reader - SDMMC2 and SDMMC2 delay peripheral clock enable during sleep mode Set and reset by software.
pub use SDMMC1LPEN_R as SDMMC2LPEN_R;
///Field `FMCLPEN` reader - FMC clock enable during sleep mode Set and reset by software.
pub use SDMMC1LPEN_R as FMCLPEN_R;
///Field `OCTOSPI1LPEN` reader - OCTOSPI1 clock enable during sleep mode Set and reset by software.
pub use SDMMC1LPEN_R as OCTOSPI1LPEN_R;
///Field `SDMMC2LPEN` writer - SDMMC2 and SDMMC2 delay peripheral clock enable during sleep mode Set and reset by software.
pub use SDMMC1LPEN_W as SDMMC2LPEN_W;
///Field `FMCLPEN` writer - FMC clock enable during sleep mode Set and reset by software.
pub use SDMMC1LPEN_W as FMCLPEN_W;
///Field `OCTOSPI1LPEN` writer - OCTOSPI1 clock enable during sleep mode Set and reset by software.
pub use SDMMC1LPEN_W as OCTOSPI1LPEN_W;
impl R {
    ///Bit 11 - SDMMC1 and SDMMC1 delay peripheral clock enable during sleep mode Set and reset by software
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SDMMC2 and SDMMC2 delay peripheral clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - FMC clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - OCTOSPI1 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn octospi1lpen(&self) -> OCTOSPI1LPEN_R {
        OCTOSPI1LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB4LPENR")
            .field("sdmmc1lpen", &self.sdmmc1lpen())
            .field("sdmmc2lpen", &self.sdmmc2lpen())
            .field("fmclpen", &self.fmclpen())
            .field("octospi1lpen", &self.octospi1lpen())
            .finish()
    }
}
impl W {
    ///Bit 11 - SDMMC1 and SDMMC1 delay peripheral clock enable during sleep mode Set and reset by software
    #[inline(always)]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W<'_, AHB4LPENRrs> {
        SDMMC1LPEN_W::new(self, 11)
    }
    ///Bit 12 - SDMMC2 and SDMMC2 delay peripheral clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W<'_, AHB4LPENRrs> {
        SDMMC2LPEN_W::new(self, 12)
    }
    ///Bit 16 - FMC clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn fmclpen(&mut self) -> FMCLPEN_W<'_, AHB4LPENRrs> {
        FMCLPEN_W::new(self, 16)
    }
    ///Bit 20 - OCTOSPI1 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn octospi1lpen(&mut self) -> OCTOSPI1LPEN_W<'_, AHB4LPENRrs> {
        OCTOSPI1LPEN_W::new(self, 20)
    }
}
/**RCC AHB4 sleep clock register

You can [`read`](crate::Reg::read) this register and get [`ahb4lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#RCC:AHB4LPENR)*/
pub struct AHB4LPENRrs;
impl crate::RegisterSpec for AHB4LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb4lpenr::R`](R) reader structure
impl crate::Readable for AHB4LPENRrs {}
///`write(|w| ..)` method takes [`ahb4lpenr::W`](W) writer structure
impl crate::Writable for AHB4LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4LPENR to value 0x0011_1880
impl crate::Resettable for AHB4LPENRrs {
    const RESET_VALUE: u32 = 0x0011_1880;
}
