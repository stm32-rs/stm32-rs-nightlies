#[doc = "Register `AHB1LPENR` reader"]
pub type R = crate::R<AHB1LPENRrs>;
#[doc = "Register `AHB1LPENR` writer"]
pub type W = crate::W<AHB1LPENRrs>;
#[doc = "GPDMA1 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPDMA1LPEN {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<GPDMA1LPEN> for bool {
    #[inline(always)]
    fn from(variant: GPDMA1LPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPDMA1LPEN` reader - GPDMA1 clock enable during sleep mode Set and reset by software."]
pub type GPDMA1LPEN_R = crate::BitReader<GPDMA1LPEN>;
impl GPDMA1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPDMA1LPEN {
        match self.bits {
            false => GPDMA1LPEN::Disabled,
            true => GPDMA1LPEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPDMA1LPEN::Disabled
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPDMA1LPEN::Enabled
    }
}
#[doc = "Field `GPDMA1LPEN` writer - GPDMA1 clock enable during sleep mode Set and reset by software."]
pub type GPDMA1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, GPDMA1LPEN>;
impl<'a, REG> GPDMA1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1LPEN::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1LPEN::Enabled)
    }
}
#[doc = "Field `GPDMA2LPEN` reader - GPDMA2 clock enable during sleep mode Set and reset by software."]
pub use GPDMA1LPEN_R as GPDMA2LPEN_R;
#[doc = "Field `FLITFLPEN` reader - Flash interface (FLITF) clock enable during sleep mode Set and reset by software."]
pub use GPDMA1LPEN_R as FLITFLPEN_R;
#[doc = "Field `CRCLPEN` reader - CRC clock enable during sleep mode Set and reset by software."]
pub use GPDMA1LPEN_R as CRCLPEN_R;
#[doc = "Field `RAMCFGLPEN` reader - RAMCFG clock enable during sleep mode Set and reset by software."]
pub use GPDMA1LPEN_R as RAMCFGLPEN_R;
#[doc = "Field `GTZC1LPEN` reader - GTZC1 clock enable during sleep mode"]
pub use GPDMA1LPEN_R as GTZC1LPEN_R;
#[doc = "Field `BKPRAMLPEN` reader - BKPRAM clock enable during sleep mode Set and reset by software"]
pub use GPDMA1LPEN_R as BKPRAMLPEN_R;
#[doc = "Field `ICACHELPEN` reader - ICACHE clock enable during sleep mode Set and reset by software"]
pub use GPDMA1LPEN_R as ICACHELPEN_R;
#[doc = "Field `SRAM1LPEN` reader - SRAM1 clock enable during sleep mode Set and reset by software"]
pub use GPDMA1LPEN_R as SRAM1LPEN_R;
#[doc = "Field `GPDMA2LPEN` writer - GPDMA2 clock enable during sleep mode Set and reset by software."]
pub use GPDMA1LPEN_W as GPDMA2LPEN_W;
#[doc = "Field `FLITFLPEN` writer - Flash interface (FLITF) clock enable during sleep mode Set and reset by software."]
pub use GPDMA1LPEN_W as FLITFLPEN_W;
#[doc = "Field `CRCLPEN` writer - CRC clock enable during sleep mode Set and reset by software."]
pub use GPDMA1LPEN_W as CRCLPEN_W;
#[doc = "Field `RAMCFGLPEN` writer - RAMCFG clock enable during sleep mode Set and reset by software."]
pub use GPDMA1LPEN_W as RAMCFGLPEN_W;
#[doc = "Field `GTZC1LPEN` writer - GTZC1 clock enable during sleep mode"]
pub use GPDMA1LPEN_W as GTZC1LPEN_W;
#[doc = "Field `BKPRAMLPEN` writer - BKPRAM clock enable during sleep mode Set and reset by software"]
pub use GPDMA1LPEN_W as BKPRAMLPEN_W;
#[doc = "Field `ICACHELPEN` writer - ICACHE clock enable during sleep mode Set and reset by software"]
pub use GPDMA1LPEN_W as ICACHELPEN_W;
#[doc = "Field `SRAM1LPEN` writer - SRAM1 clock enable during sleep mode Set and reset by software"]
pub use GPDMA1LPEN_W as SRAM1LPEN_W;
impl R {
    #[doc = "Bit 0 - GPDMA1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpdma1lpen(&self) -> GPDMA1LPEN_R {
        GPDMA1LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPDMA2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpdma2lpen(&self) -> GPDMA2LPEN_R {
        GPDMA2LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash interface (FLITF) clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn flitflpen(&self) -> FLITFLPEN_R {
        FLITFLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - RAMCFG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn ramcfglpen(&self) -> RAMCFGLPEN_R {
        RAMCFGLPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - GTZC1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn gtzc1lpen(&self) -> GTZC1LPEN_R {
        GTZC1LPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - BKPRAM clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    pub fn bkpramlpen(&self) -> BKPRAMLPEN_R {
        BKPRAMLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ICACHE clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    pub fn icachelpen(&self) -> ICACHELPEN_R {
        ICACHELPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM1 clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPDMA1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpdma1lpen(&mut self) -> GPDMA1LPEN_W<AHB1LPENRrs> {
        GPDMA1LPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPDMA2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpdma2lpen(&mut self) -> GPDMA2LPEN_W<AHB1LPENRrs> {
        GPDMA2LPEN_W::new(self, 1)
    }
    #[doc = "Bit 8 - Flash interface (FLITF) clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn flitflpen(&mut self) -> FLITFLPEN_W<AHB1LPENRrs> {
        FLITFLPEN_W::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn crclpen(&mut self) -> CRCLPEN_W<AHB1LPENRrs> {
        CRCLPEN_W::new(self, 12)
    }
    #[doc = "Bit 17 - RAMCFG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ramcfglpen(&mut self) -> RAMCFGLPEN_W<AHB1LPENRrs> {
        RAMCFGLPEN_W::new(self, 17)
    }
    #[doc = "Bit 24 - GTZC1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gtzc1lpen(&mut self) -> GTZC1LPEN_W<AHB1LPENRrs> {
        GTZC1LPEN_W::new(self, 24)
    }
    #[doc = "Bit 28 - BKPRAM clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn bkpramlpen(&mut self) -> BKPRAMLPEN_W<AHB1LPENRrs> {
        BKPRAMLPEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - ICACHE clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn icachelpen(&mut self) -> ICACHELPEN_W<AHB1LPENRrs> {
        ICACHELPEN_W::new(self, 29)
    }
    #[doc = "Bit 31 - SRAM1 clock enable during sleep mode Set and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W<AHB1LPENRrs> {
        SRAM1LPEN_W::new(self, 31)
    }
}
#[doc = "RCC AHB1 sleep clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1LPENRrs;
impl crate::RegisterSpec for AHB1LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1lpenr::R`](R) reader structure"]
impl crate::Readable for AHB1LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb1lpenr::W`](W) writer structure"]
impl crate::Writable for AHB1LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1LPENR to value 0xf13a_d103"]
impl crate::Resettable for AHB1LPENRrs {
    const RESET_VALUE: u32 = 0xf13a_d103;
}
