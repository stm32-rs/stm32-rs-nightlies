#[doc = "Register `AHB2LPENR` reader"]
pub type R = crate::R<AHB2LPENRrs>;
#[doc = "Register `AHB2LPENR` writer"]
pub type W = crate::W<AHB2LPENRrs>;
#[doc = "DCMI peripheral clock enable during csleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMILPEN {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<DCMILPEN> for bool {
    #[inline(always)]
    fn from(variant: DCMILPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMILPEN` reader - DCMI peripheral clock enable during csleep mode"]
pub type DCMILPEN_R = crate::BitReader<DCMILPEN>;
impl DCMILPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCMILPEN {
        match self.bits {
            false => DCMILPEN::Disabled,
            true => DCMILPEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMILPEN::Disabled
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMILPEN::Enabled
    }
}
#[doc = "Field `DCMILPEN` writer - DCMI peripheral clock enable during csleep mode"]
pub type DCMILPEN_W<'a, REG> = crate::BitWriter<'a, REG, DCMILPEN>;
impl<'a, REG> DCMILPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMILPEN::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMILPEN::Enabled)
    }
}
#[doc = "Field `CRYPTLPEN` reader - CRYPT peripheral clock enable during CSleep mode"]
pub use DCMILPEN_R as CRYPTLPEN_R;
#[doc = "Field `HASHLPEN` reader - HASH peripheral clock enable during CSleep mode"]
pub use DCMILPEN_R as HASHLPEN_R;
#[doc = "Field `RNGLPEN` reader - RNG peripheral clock enable during CSleep mode"]
pub use DCMILPEN_R as RNGLPEN_R;
#[doc = "Field `SDMMC2LPEN` reader - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
pub use DCMILPEN_R as SDMMC2LPEN_R;
#[doc = "Field `FMACLPEN` reader - FMAC enable during CSleep Mode"]
pub use DCMILPEN_R as FMACLPEN_R;
#[doc = "Field `CORDICLPEN` reader - CORDIC enable during CSleep Mode"]
pub use DCMILPEN_R as CORDICLPEN_R;
#[doc = "Field `SRAM1LPEN` reader - SRAM1 Clock Enable During CSleep Mode"]
pub use DCMILPEN_R as SRAM1LPEN_R;
#[doc = "Field `SRAM2LPEN` reader - SRAM2 Clock Enable During CSleep Mode"]
pub use DCMILPEN_R as SRAM2LPEN_R;
#[doc = "Field `CRYPTLPEN` writer - CRYPT peripheral clock enable during CSleep mode"]
pub use DCMILPEN_W as CRYPTLPEN_W;
#[doc = "Field `HASHLPEN` writer - HASH peripheral clock enable during CSleep mode"]
pub use DCMILPEN_W as HASHLPEN_W;
#[doc = "Field `RNGLPEN` writer - RNG peripheral clock enable during CSleep mode"]
pub use DCMILPEN_W as RNGLPEN_W;
#[doc = "Field `SDMMC2LPEN` writer - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
pub use DCMILPEN_W as SDMMC2LPEN_W;
#[doc = "Field `FMACLPEN` writer - FMAC enable during CSleep Mode"]
pub use DCMILPEN_W as FMACLPEN_W;
#[doc = "Field `CORDICLPEN` writer - CORDIC enable during CSleep Mode"]
pub use DCMILPEN_W as CORDICLPEN_W;
#[doc = "Field `SRAM1LPEN` writer - SRAM1 Clock Enable During CSleep Mode"]
pub use DCMILPEN_W as SRAM1LPEN_W;
#[doc = "Field `SRAM2LPEN` writer - SRAM2 Clock Enable During CSleep Mode"]
pub use DCMILPEN_W as SRAM2LPEN_W;
impl R {
    #[doc = "Bit 0 - DCMI peripheral clock enable during csleep mode"]
    #[inline(always)]
    pub fn dcmilpen(&self) -> DCMILPEN_R {
        DCMILPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn cryptlpen(&self) -> CRYPTLPEN_R {
        CRYPTLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn hashlpen(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - FMAC enable during CSleep Mode"]
    #[inline(always)]
    pub fn fmaclpen(&self) -> FMACLPEN_R {
        FMACLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CORDIC enable during CSleep Mode"]
    #[inline(always)]
    pub fn cordiclpen(&self) -> CORDICLPEN_R {
        CORDICLPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 29 - SRAM1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCMI peripheral clock enable during csleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dcmilpen(&mut self) -> DCMILPEN_W<AHB2LPENRrs> {
        DCMILPEN_W::new(self, 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn cryptlpen(&mut self) -> CRYPTLPEN_W<AHB2LPENRrs> {
        CRYPTLPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn hashlpen(&mut self) -> HASHLPEN_W<AHB2LPENRrs> {
        HASHLPEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - RNG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn rnglpen(&mut self) -> RNGLPEN_W<AHB2LPENRrs> {
        RNGLPEN_W::new(self, 6)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W<AHB2LPENRrs> {
        SDMMC2LPEN_W::new(self, 9)
    }
    #[doc = "Bit 16 - FMAC enable during CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmaclpen(&mut self) -> FMACLPEN_W<AHB2LPENRrs> {
        FMACLPEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - CORDIC enable during CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cordiclpen(&mut self) -> CORDICLPEN_W<AHB2LPENRrs> {
        CORDICLPEN_W::new(self, 17)
    }
    #[doc = "Bit 29 - SRAM1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W<AHB2LPENRrs> {
        SRAM1LPEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - SRAM2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W<AHB2LPENRrs> {
        SRAM2LPEN_W::new(self, 30)
    }
}
#[doc = "RCC AHB2 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets AHB2LPENR to value 0"]
impl crate::Resettable for AHB2LPENRrs {
    const RESET_VALUE: u32 = 0;
}
