#[doc = "Register `STGENC_CNTCR` reader"]
pub struct R(crate::R<STGENC_CNTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_CNTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_CNTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_CNTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STGENC_CNTCR` writer"]
pub struct W(crate::W<STGENC_CNTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STGENC_CNTCR_SPEC>;
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
impl From<crate::W<STGENC_CNTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STGENC_CNTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - EN"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `HLTDBG` reader - HLTDBG"]
pub struct HLTDBG_R(crate::FieldReader<bool, bool>);
impl HLTDBG_R {
    pub(crate) fn new(bits: bool) -> Self {
        HLTDBG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HLTDBG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HLTDBG` writer - HLTDBG"]
pub struct HLTDBG_W<'a> {
    w: &'a mut W,
}
impl<'a> HLTDBG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HLTDBG"]
    #[inline(always)]
    pub fn hltdbg(&self) -> HLTDBG_R {
        HLTDBG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - HLTDBG"]
    #[inline(always)]
    pub fn hltdbg(&mut self) -> HLTDBG_W {
        HLTDBG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "STGENC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cntcr](index.html) module"]
pub struct STGENC_CNTCR_SPEC;
impl crate::RegisterSpec for STGENC_CNTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenc_cntcr::R](R) reader structure"]
impl crate::Readable for STGENC_CNTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stgenc_cntcr::W](W) writer structure"]
impl crate::Writable for STGENC_CNTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STGENC_CNTCR to value 0"]
impl crate::Resettable for STGENC_CNTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
