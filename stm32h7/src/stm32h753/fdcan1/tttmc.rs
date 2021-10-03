#[doc = "Register `TTTMC` reader"]
pub struct R(crate::R<TTTMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTTMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTTMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTTMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTTMC` writer"]
pub struct W(crate::W<TTTMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTTMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TTTMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTTMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMSA` reader - Trigger Memory Start Address"]
pub struct TMSA_R(crate::FieldReader<u16, u16>);
impl TMSA_R {
    pub(crate) fn new(bits: u16) -> Self {
        TMSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMSA` writer - Trigger Memory Start Address"]
pub struct TMSA_W<'a> {
    w: &'a mut W,
}
impl<'a> TMSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | ((value as u32 & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Field `TME` reader - Trigger Memory Elements"]
pub struct TME_R(crate::FieldReader<u8, u8>);
impl TME_R {
    pub(crate) fn new(bits: u8) -> Self {
        TME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TME` writer - Trigger Memory Elements"]
pub struct TME_W<'a> {
    w: &'a mut W,
}
impl<'a> TME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - Trigger Memory Start Address"]
    #[inline(always)]
    pub fn tmsa(&self) -> TMSA_R {
        TMSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Trigger Memory Elements"]
    #[inline(always)]
    pub fn tme(&self) -> TME_R {
        TME_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Trigger Memory Start Address"]
    #[inline(always)]
    pub fn tmsa(&mut self) -> TMSA_W {
        TMSA_W { w: self }
    }
    #[doc = "Bits 16:22 - Trigger Memory Elements"]
    #[inline(always)]
    pub fn tme(&mut self) -> TME_W {
        TME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Trigger Memory Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tttmc](index.html) module"]
pub struct TTTMC_SPEC;
impl crate::RegisterSpec for TTTMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tttmc::R](R) reader structure"]
impl crate::Readable for TTTMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tttmc::W](W) writer structure"]
impl crate::Writable for TTTMC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TTTMC to value 0"]
impl crate::Resettable for TTTMC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
