#[doc = "Register `DIFSEL` reader"]
pub struct R(crate::R<DIFSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIFSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIFSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIFSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIFSEL` writer"]
pub struct W(crate::W<DIFSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIFSEL_SPEC>;
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
impl From<crate::W<DIFSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIFSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIFSEL_1_15` reader - Differential mode for channels 15 to 1"]
pub struct DIFSEL_1_15_R(crate::FieldReader<u16, u16>);
impl DIFSEL_1_15_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIFSEL_1_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFSEL_1_15_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFSEL_1_15` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_1_15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_1_15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 1)) | ((value as u32 & 0x7fff) << 1);
        self.w
    }
}
#[doc = "Field `DIFSEL_16_18` reader - Differential mode for channels 18 to 16"]
pub struct DIFSEL_16_18_R(crate::FieldReader<u8, u8>);
impl DIFSEL_16_18_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIFSEL_16_18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFSEL_16_18_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 1:15 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_1_15(&self) -> DIFSEL_1_15_R {
        DIFSEL_1_15_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bits 16:18 - Differential mode for channels 18 to 16"]
    #[inline(always)]
    pub fn difsel_16_18(&self) -> DIFSEL_16_18_R {
        DIFSEL_16_18_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 1:15 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_1_15(&mut self) -> DIFSEL_1_15_W {
        DIFSEL_1_15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Differential Mode Selection Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [difsel](index.html) module"]
pub struct DIFSEL_SPEC;
impl crate::RegisterSpec for DIFSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [difsel::R](R) reader structure"]
impl crate::Readable for DIFSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [difsel::W](W) writer structure"]
impl crate::Writable for DIFSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIFSEL to value 0"]
impl crate::Resettable for DIFSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
