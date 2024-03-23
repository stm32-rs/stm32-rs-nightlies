#[doc = "Register `AHB2ENR` reader"]
pub type R = crate::R<AHB2ENRrs>;
#[doc = "Register `AHB2ENR` writer"]
pub type W = crate::W<AHB2ENRrs>;
#[doc = "Camera interface enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMIEN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<DCMIEN> for bool {
    #[inline(always)]
    fn from(variant: DCMIEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMIEN` reader - Camera interface enable"]
pub type DCMIEN_R = crate::BitReader<DCMIEN>;
impl DCMIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCMIEN {
        match self.bits {
            false => DCMIEN::Disabled,
            true => DCMIEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMIEN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMIEN::Enabled
    }
}
#[doc = "Field `DCMIEN` writer - Camera interface enable"]
pub type DCMIEN_W<'a, REG> = crate::BitWriter<'a, REG, DCMIEN>;
impl<'a, REG> DCMIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIEN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIEN::Enabled)
    }
}
#[doc = "Field `CRYPEN` reader - Cryptographic modules clock enable"]
pub use DCMIEN_R as CRYPEN_R;
#[doc = "Field `HASHEN` reader - Hash modules clock enable"]
pub use DCMIEN_R as HASHEN_R;
#[doc = "Field `RNGEN` reader - Random number generator clock enable"]
pub use DCMIEN_R as RNGEN_R;
#[doc = "Field `OTGFSEN` reader - USB OTG FS clock enable"]
pub use DCMIEN_R as OTGFSEN_R;
#[doc = "Field `CRYPEN` writer - Cryptographic modules clock enable"]
pub use DCMIEN_W as CRYPEN_W;
#[doc = "Field `HASHEN` writer - Hash modules clock enable"]
pub use DCMIEN_W as HASHEN_W;
#[doc = "Field `RNGEN` writer - Random number generator clock enable"]
pub use DCMIEN_W as RNGEN_W;
#[doc = "Field `OTGFSEN` writer - USB OTG FS clock enable"]
pub use DCMIEN_W as OTGFSEN_W;
impl R {
    #[doc = "Bit 0 - Camera interface enable"]
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Cryptographic modules clock enable"]
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hash modules clock enable"]
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Random number generator clock enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB OTG FS clock enable"]
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Camera interface enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcmien(&mut self) -> DCMIEN_W<AHB2ENRrs> {
        DCMIEN_W::new(self, 0)
    }
    #[doc = "Bit 4 - Cryptographic modules clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crypen(&mut self) -> CRYPEN_W<AHB2ENRrs> {
        CRYPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Hash modules clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hashen(&mut self) -> HASHEN_W<AHB2ENRrs> {
        HASHEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Random number generator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<AHB2ENRrs> {
        RNGEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - USB OTG FS clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn otgfsen(&mut self) -> OTGFSEN_W<AHB2ENRrs> {
        OTGFSEN_W::new(self, 7)
    }
}
#[doc = "AHB2 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2ENRrs;
impl crate::RegisterSpec for AHB2ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2enr::R`](R) reader structure"]
impl crate::Readable for AHB2ENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure"]
impl crate::Writable for AHB2ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for AHB2ENRrs {
    const RESET_VALUE: u32 = 0;
}
