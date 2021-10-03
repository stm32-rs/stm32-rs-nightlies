#[doc = "Register `IPCC_C1MR` reader"]
pub struct R(crate::R<IPCC_C1MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCC_C1MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCC_C1MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCC_C1MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPCC_C1MR` writer"]
pub struct W(crate::W<IPCC_C1MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPCC_C1MR_SPEC>;
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
impl From<crate::W<IPCC_C1MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPCC_C1MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHxOM` reader - CHxOM"]
pub struct CHXOM_R(crate::FieldReader<u8, u8>);
impl CHXOM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHXOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHXOM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHxOM` writer - CHxOM"]
pub struct CHXOM_W<'a> {
    w: &'a mut W,
}
impl<'a> CHXOM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `CHxFM` reader - CHxFM"]
pub struct CHXFM_R(crate::FieldReader<u8, u8>);
impl CHXFM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHXFM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHXFM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHxFM` writer - CHxFM"]
pub struct CHXFM_W<'a> {
    w: &'a mut W,
}
impl<'a> CHXFM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - CHxOM"]
    #[inline(always)]
    pub fn chx_om(&self) -> CHXOM_R {
        CHXOM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - CHxFM"]
    #[inline(always)]
    pub fn chx_fm(&self) -> CHXFM_R {
        CHXFM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - CHxOM"]
    #[inline(always)]
    pub fn chx_om(&mut self) -> CHXOM_W {
        CHXOM_W { w: self }
    }
    #[doc = "Bits 16:21 - CHxFM"]
    #[inline(always)]
    pub fn chx_fm(&mut self) -> CHXFM_W {
        CHXFM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPCC Processor 1 mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_c1mr](index.html) module"]
pub struct IPCC_C1MR_SPEC;
impl crate::RegisterSpec for IPCC_C1MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipcc_c1mr::R](R) reader structure"]
impl crate::Readable for IPCC_C1MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipcc_c1mr::W](W) writer structure"]
impl crate::Writable for IPCC_C1MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPCC_C1MR to value 0xffff_ffff"]
impl crate::Resettable for IPCC_C1MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
