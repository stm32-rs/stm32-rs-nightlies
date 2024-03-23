#[doc = "Register `AHB2LPENR` reader"]
pub type R = crate::R<AHB2LPENRrs>;
#[doc = "Register `AHB2LPENR` writer"]
pub type W = crate::W<AHB2LPENRrs>;
#[doc = "Camera interface enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMILPEN {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DisabledInSleep = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    EnabledInSleep = 1,
}
impl From<DCMILPEN> for bool {
    #[inline(always)]
    fn from(variant: DCMILPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMILPEN` reader - Camera interface enable during Sleep mode"]
pub type DCMILPEN_R = crate::BitReader<DCMILPEN>;
impl DCMILPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCMILPEN {
        match self.bits {
            false => DCMILPEN::DisabledInSleep,
            true => DCMILPEN::EnabledInSleep,
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == DCMILPEN::DisabledInSleep
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == DCMILPEN::EnabledInSleep
    }
}
#[doc = "Field `DCMILPEN` writer - Camera interface enable during Sleep mode"]
pub type DCMILPEN_W<'a, REG> = crate::BitWriter<'a, REG, DCMILPEN>;
impl<'a, REG> DCMILPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(DCMILPEN::DisabledInSleep)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(DCMILPEN::EnabledInSleep)
    }
}
#[doc = "Field `JPEGLPEN` reader - JPEG module enabled during Sleep mode"]
pub use DCMILPEN_R as JPEGLPEN_R;
#[doc = "Field `CRYPLPEN` reader - Cryptography modules clock enable during Sleep mode"]
pub use DCMILPEN_R as CRYPLPEN_R;
#[doc = "Field `HASHLPEN` reader - Hash modules clock enable during Sleep mode"]
pub use DCMILPEN_R as HASHLPEN_R;
#[doc = "Field `RNGLPEN` reader - Random number generator clock enable during Sleep mode"]
pub use DCMILPEN_R as RNGLPEN_R;
#[doc = "Field `OTGFSLPEN` reader - USB OTG FS clock enable during Sleep mode"]
pub use DCMILPEN_R as OTGFSLPEN_R;
#[doc = "Field `JPEGLPEN` writer - JPEG module enabled during Sleep mode"]
pub use DCMILPEN_W as JPEGLPEN_W;
#[doc = "Field `CRYPLPEN` writer - Cryptography modules clock enable during Sleep mode"]
pub use DCMILPEN_W as CRYPLPEN_W;
#[doc = "Field `HASHLPEN` writer - Hash modules clock enable during Sleep mode"]
pub use DCMILPEN_W as HASHLPEN_W;
#[doc = "Field `RNGLPEN` writer - Random number generator clock enable during Sleep mode"]
pub use DCMILPEN_W as RNGLPEN_W;
#[doc = "Field `OTGFSLPEN` writer - USB OTG FS clock enable during Sleep mode"]
pub use DCMILPEN_W as OTGFSLPEN_W;
impl R {
    #[doc = "Bit 0 - Camera interface enable during Sleep mode"]
    #[inline(always)]
    pub fn dcmilpen(&self) -> DCMILPEN_R {
        DCMILPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - JPEG module enabled during Sleep mode"]
    #[inline(always)]
    pub fn jpeglpen(&self) -> JPEGLPEN_R {
        JPEGLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Cryptography modules clock enable during Sleep mode"]
    #[inline(always)]
    pub fn cryplpen(&self) -> CRYPLPEN_R {
        CRYPLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hash modules clock enable during Sleep mode"]
    #[inline(always)]
    pub fn hashlpen(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 0 - Camera interface enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dcmilpen(&mut self) -> DCMILPEN_W<AHB2LPENRrs> {
        DCMILPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - JPEG module enabled during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn jpeglpen(&mut self) -> JPEGLPEN_W<AHB2LPENRrs> {
        JPEGLPEN_W::new(self, 1)
    }
    #[doc = "Bit 4 - Cryptography modules clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn cryplpen(&mut self) -> CRYPLPEN_W<AHB2LPENRrs> {
        CRYPLPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Hash modules clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn hashlpen(&mut self) -> HASHLPEN_W<AHB2LPENRrs> {
        HASHLPEN_W::new(self, 5)
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
