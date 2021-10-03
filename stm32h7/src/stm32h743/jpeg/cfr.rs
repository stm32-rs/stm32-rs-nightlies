#[doc = "Register `CFR` reader"]
pub struct R(crate::R<CFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFR` writer"]
pub struct W(crate::W<CFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFR_SPEC>;
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
impl From<crate::W<CFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEOCF` reader - Clear End of Conversion Flag Writing 1 clears the End of Conversion Flag of the JPEG Status Register."]
pub struct CEOCF_R(crate::FieldReader<bool, bool>);
impl CEOCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEOCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEOCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEOCF` writer - Clear End of Conversion Flag Writing 1 clears the End of Conversion Flag of the JPEG Status Register."]
pub struct CEOCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CEOCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CHPDF` reader - Clear Header Parsing Done Flag Writing 1 clears the Header Parsing Done Flag of the JPEG Status Register."]
pub struct CHPDF_R(crate::FieldReader<bool, bool>);
impl CHPDF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHPDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHPDF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHPDF` writer - Clear Header Parsing Done Flag Writing 1 clears the Header Parsing Done Flag of the JPEG Status Register."]
pub struct CHPDF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPDF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Clear End of Conversion Flag Writing 1 clears the End of Conversion Flag of the JPEG Status Register."]
    #[inline(always)]
    pub fn ceocf(&self) -> CEOCF_R {
        CEOCF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Clear Header Parsing Done Flag Writing 1 clears the Header Parsing Done Flag of the JPEG Status Register."]
    #[inline(always)]
    pub fn chpdf(&self) -> CHPDF_R {
        CHPDF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clear End of Conversion Flag Writing 1 clears the End of Conversion Flag of the JPEG Status Register."]
    #[inline(always)]
    pub fn ceocf(&mut self) -> CEOCF_W {
        CEOCF_W { w: self }
    }
    #[doc = "Bit 6 - Clear Header Parsing Done Flag Writing 1 clears the Header Parsing Done Flag of the JPEG Status Register."]
    #[inline(always)]
    pub fn chpdf(&mut self) -> CHPDF_W {
        CHPDF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfr](index.html) module"]
pub struct CFR_SPEC;
impl crate::RegisterSpec for CFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfr::R](R) reader structure"]
impl crate::Readable for CFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfr::W](W) writer structure"]
impl crate::Writable for CFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFR to value 0"]
impl crate::Resettable for CFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
