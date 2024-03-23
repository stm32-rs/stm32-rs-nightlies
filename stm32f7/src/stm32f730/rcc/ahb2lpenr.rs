#[doc = "Register `AHB2LPENR` reader"]
pub type R = crate::R<AHB2LPENRrs>;
#[doc = "Register `AHB2LPENR` writer"]
pub type W = crate::W<AHB2LPENRrs>;
#[doc = "AES module clock enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESLPEN {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DisabledInSleep = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    EnabledInSleep = 1,
}
impl From<AESLPEN> for bool {
    #[inline(always)]
    fn from(variant: AESLPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESLPEN` reader - AES module clock enable during Sleep mode"]
pub type AESLPEN_R = crate::BitReader<AESLPEN>;
impl AESLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AESLPEN {
        match self.bits {
            false => AESLPEN::DisabledInSleep,
            true => AESLPEN::EnabledInSleep,
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == AESLPEN::DisabledInSleep
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == AESLPEN::EnabledInSleep
    }
}
#[doc = "Field `AESLPEN` writer - AES module clock enable during Sleep mode"]
pub type AESLPEN_W<'a, REG> = crate::BitWriter<'a, REG, AESLPEN>;
impl<'a, REG> AESLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(AESLPEN::DisabledInSleep)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(AESLPEN::EnabledInSleep)
    }
}
#[doc = "Field `RNGLPEN` reader - Random number generator clock enable during Sleep mode"]
pub use AESLPEN_R as RNGLPEN_R;
#[doc = "Field `OTGFSLPEN` reader - USB OTG FS clock enable during Sleep mode"]
pub use AESLPEN_R as OTGFSLPEN_R;
#[doc = "Field `RNGLPEN` writer - Random number generator clock enable during Sleep mode"]
pub use AESLPEN_W as RNGLPEN_W;
#[doc = "Field `OTGFSLPEN` writer - USB OTG FS clock enable during Sleep mode"]
pub use AESLPEN_W as OTGFSLPEN_W;
impl R {
    #[doc = "Bit 4 - AES module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn aeslpen(&self) -> AESLPEN_R {
        AESLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Random number generator clock enable during Sleep mode"]
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB OTG FS clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otgfslpen(&self) -> OTGFSLPEN_R {
        OTGFSLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - AES module clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn aeslpen(&mut self) -> AESLPEN_W<AHB2LPENRrs> {
        AESLPEN_W::new(self, 4)
    }
    #[doc = "Bit 6 - Random number generator clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn rnglpen(&mut self) -> RNGLPEN_W<AHB2LPENRrs> {
        RNGLPEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - USB OTG FS clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn otgfslpen(&mut self) -> OTGFSLPEN_W<AHB2LPENRrs> {
        OTGFSLPEN_W::new(self, 7)
    }
}
#[doc = "AHB2 peripheral clock enable in low power mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets AHB2LPENR to value 0xf1"]
impl crate::Resettable for AHB2LPENRrs {
    const RESET_VALUE: u32 = 0xf1;
}
