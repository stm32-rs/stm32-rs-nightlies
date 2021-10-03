#[doc = "Register `JPEG_CFR` writer"]
pub struct W(crate::W<JPEG_CFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JPEG_CFR_SPEC>;
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
impl From<crate::W<JPEG_CFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JPEG_CFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEOCF` writer - Clear End of Conversion Flag"]
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
#[doc = "Field `CHPDF` writer - Clear Header Parsing Done Flag"]
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
impl W {
    #[doc = "Bit 5 - Clear End of Conversion Flag"]
    #[inline(always)]
    pub fn ceocf(&mut self) -> CEOCF_W {
        CEOCF_W { w: self }
    }
    #[doc = "Bit 6 - Clear Header Parsing Done Flag"]
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
#[doc = "JPEG clear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_cfr](index.html) module"]
pub struct JPEG_CFR_SPEC;
impl crate::RegisterSpec for JPEG_CFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [jpeg_cfr::W](W) writer structure"]
impl crate::Writable for JPEG_CFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JPEG_CFR to value 0"]
impl crate::Resettable for JPEG_CFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
