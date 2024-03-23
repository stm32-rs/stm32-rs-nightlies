#[doc = "Register `AHB4LPENR` reader"]
pub type R = crate::R<AHB4LPENRrs>;
#[doc = "Register `AHB4LPENR` writer"]
pub type W = crate::W<AHB4LPENRrs>;
#[doc = "SDMMC1 and SDMMC1 delay peripheral clock enable during sleep mode Set and reset by software\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1LPEN {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<SDMMC1LPEN> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1LPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMMC1LPEN` reader - SDMMC1 and SDMMC1 delay peripheral clock enable during sleep mode Set and reset by software"]
pub type SDMMC1LPEN_R = crate::BitReader<SDMMC1LPEN>;
impl SDMMC1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDMMC1LPEN {
        match self.bits {
            false => SDMMC1LPEN::Disabled,
            true => SDMMC1LPEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDMMC1LPEN::Disabled
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDMMC1LPEN::Enabled
    }
}
#[doc = "Field `SDMMC1LPEN` writer - SDMMC1 and SDMMC1 delay peripheral clock enable during sleep mode Set and reset by software"]
pub type SDMMC1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, SDMMC1LPEN>;
impl<'a, REG> SDMMC1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1LPEN::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDMMC1LPEN::Enabled)
    }
}
#[doc = "Field `FMCLPEN` reader - FMC clock enable during sleep mode Set and reset by software."]
pub use SDMMC1LPEN_R as FMCLPEN_R;
#[doc = "Field `OCTOSPI1LPEN` reader - OCTOSPI1 clock enable during sleep mode Set and reset by software."]
pub use SDMMC1LPEN_R as OCTOSPI1LPEN_R;
#[doc = "Field `FMCLPEN` writer - FMC clock enable during sleep mode Set and reset by software."]
pub use SDMMC1LPEN_W as FMCLPEN_W;
#[doc = "Field `OCTOSPI1LPEN` writer - OCTOSPI1 clock enable during sleep mode Set and reset by software."]
pub use SDMMC1LPEN_W as OCTOSPI1LPEN_W;
impl R {
    #[doc = "Bit 11 - SDMMC1 and SDMMC1 delay peripheral clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - FMC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - OCTOSPI1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn octospi1lpen(&self) -> OCTOSPI1LPEN_R {
        OCTOSPI1LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - SDMMC1 and SDMMC1 delay peripheral clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W<AHB4LPENRrs> {
        SDMMC1LPEN_W::new(self, 11)
    }
    #[doc = "Bit 16 - FMC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmclpen(&mut self) -> FMCLPEN_W<AHB4LPENRrs> {
        FMCLPEN_W::new(self, 16)
    }
    #[doc = "Bit 20 - OCTOSPI1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn octospi1lpen(&mut self) -> OCTOSPI1LPEN_W<AHB4LPENRrs> {
        OCTOSPI1LPEN_W::new(self, 20)
    }
}
#[doc = "RCC AHB4 sleep clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB4LPENRrs;
impl crate::RegisterSpec for AHB4LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb4lpenr::R`](R) reader structure"]
impl crate::Readable for AHB4LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb4lpenr::W`](W) writer structure"]
impl crate::Writable for AHB4LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB4LPENR to value 0x0011_1880"]
impl crate::Resettable for AHB4LPENRrs {
    const RESET_VALUE: u32 = 0x0011_1880;
}
