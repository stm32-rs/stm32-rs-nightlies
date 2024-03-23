#[doc = "Register `AHB2LPENR` reader"]
pub type R = crate::R<AHB2LPENRrs>;
#[doc = "Register `AHB2LPENR` writer"]
pub type W = crate::W<AHB2LPENRrs>;
#[doc = "digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMI_PSSILPEN {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<DCMI_PSSILPEN> for bool {
    #[inline(always)]
    fn from(variant: DCMI_PSSILPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMI_PSSILPEN` reader - digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software."]
pub type DCMI_PSSILPEN_R = crate::BitReader<DCMI_PSSILPEN>;
impl DCMI_PSSILPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCMI_PSSILPEN {
        match self.bits {
            false => DCMI_PSSILPEN::Disabled,
            true => DCMI_PSSILPEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMI_PSSILPEN::Disabled
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMI_PSSILPEN::Enabled
    }
}
#[doc = "Field `DCMI_PSSILPEN` writer - digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software."]
pub type DCMI_PSSILPEN_W<'a, REG> = crate::BitWriter<'a, REG, DCMI_PSSILPEN>;
impl<'a, REG> DCMI_PSSILPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMI_PSSILPEN::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMI_PSSILPEN::Enabled)
    }
}
#[doc = "Field `CRYPTLPEN` reader - CRYPT peripheral clock enable during CSleep mode Set and reset by software."]
pub use DCMI_PSSILPEN_R as CRYPTLPEN_R;
#[doc = "Field `HASHLPEN` reader - HASH peripheral clock enable during CSleep mode Set and reset by software."]
pub use DCMI_PSSILPEN_R as HASHLPEN_R;
#[doc = "Field `RNGLPEN` reader - RNG peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock."]
pub use DCMI_PSSILPEN_R as RNGLPEN_R;
#[doc = "Field `SDMMC2LPEN` reader - SDMMC2 and SDMMC2 delay clock enable during CSleep mode Set and reset by software."]
pub use DCMI_PSSILPEN_R as SDMMC2LPEN_R;
#[doc = "Field `DFSDMDMALPEN` reader - DFSDMDMA clock enable during CSleep mode Set and reset by software."]
pub use DCMI_PSSILPEN_R as DFSDMDMALPEN_R;
#[doc = "Field `AHBSRAM1LPEN` reader - AHBSRAM1 clock enable during CSleep mode Set and reset by software."]
pub use DCMI_PSSILPEN_R as AHBSRAM1LPEN_R;
#[doc = "Field `AHBSRAM2LPEN` reader - AHBSRAM2 clock enable during CSleep mode Set and reset by software."]
pub use DCMI_PSSILPEN_R as AHBSRAM2LPEN_R;
#[doc = "Field `CRYPTLPEN` writer - CRYPT peripheral clock enable during CSleep mode Set and reset by software."]
pub use DCMI_PSSILPEN_W as CRYPTLPEN_W;
#[doc = "Field `HASHLPEN` writer - HASH peripheral clock enable during CSleep mode Set and reset by software."]
pub use DCMI_PSSILPEN_W as HASHLPEN_W;
#[doc = "Field `RNGLPEN` writer - RNG peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock."]
pub use DCMI_PSSILPEN_W as RNGLPEN_W;
#[doc = "Field `SDMMC2LPEN` writer - SDMMC2 and SDMMC2 delay clock enable during CSleep mode Set and reset by software."]
pub use DCMI_PSSILPEN_W as SDMMC2LPEN_W;
#[doc = "Field `DFSDMDMALPEN` writer - DFSDMDMA clock enable during CSleep mode Set and reset by software."]
pub use DCMI_PSSILPEN_W as DFSDMDMALPEN_W;
#[doc = "Field `AHBSRAM1LPEN` writer - AHBSRAM1 clock enable during CSleep mode Set and reset by software."]
pub use DCMI_PSSILPEN_W as AHBSRAM1LPEN_W;
#[doc = "Field `AHBSRAM2LPEN` writer - AHBSRAM2 clock enable during CSleep mode Set and reset by software."]
pub use DCMI_PSSILPEN_W as AHBSRAM2LPEN_W;
impl R {
    #[doc = "Bit 0 - digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software."]
    #[inline(always)]
    pub fn dcmi_pssilpen(&self) -> DCMI_PSSILPEN_R {
        DCMI_PSSILPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn cryptlpen(&self) -> CRYPTLPEN_R {
        CRYPTLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn hashlpen(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock."]
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - DFSDMDMA clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn dfsdmdmalpen(&self) -> DFSDMDMALPEN_R {
        DFSDMDMALPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 29 - AHBSRAM1 clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn ahbsram1lpen(&self) -> AHBSRAM1LPEN_R {
        AHBSRAM1LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - AHBSRAM2 clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn ahbsram2lpen(&self) -> AHBSRAM2LPEN_R {
        AHBSRAM2LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dcmi_pssilpen(&mut self) -> DCMI_PSSILPEN_W<AHB2LPENRrs> {
        DCMI_PSSILPEN_W::new(self, 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn cryptlpen(&mut self) -> CRYPTLPEN_W<AHB2LPENRrs> {
        CRYPTLPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hashlpen(&mut self) -> HASHLPEN_W<AHB2LPENRrs> {
        HASHLPEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - RNG peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock."]
    #[inline(always)]
    #[must_use]
    pub fn rnglpen(&mut self) -> RNGLPEN_W<AHB2LPENRrs> {
        RNGLPEN_W::new(self, 6)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W<AHB2LPENRrs> {
        SDMMC2LPEN_W::new(self, 9)
    }
    #[doc = "Bit 11 - DFSDMDMA clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dfsdmdmalpen(&mut self) -> DFSDMDMALPEN_W<AHB2LPENRrs> {
        DFSDMDMALPEN_W::new(self, 11)
    }
    #[doc = "Bit 29 - AHBSRAM1 clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ahbsram1lpen(&mut self) -> AHBSRAM1LPEN_W<AHB2LPENRrs> {
        AHBSRAM1LPEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - AHBSRAM2 clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ahbsram2lpen(&mut self) -> AHBSRAM2LPEN_W<AHB2LPENRrs> {
        AHBSRAM2LPEN_W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2LPENRrs;
impl crate::RegisterSpec for AHB2LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2lpenr::R`](R) reader structure"]
impl crate::Readable for AHB2LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb2lpenr::W`](W) writer structure"]
impl crate::Writable for AHB2LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2LPENR to value 0x6000_0a71"]
impl crate::Resettable for AHB2LPENRrs {
    const RESET_VALUE: u32 = 0x6000_0a71;
}
