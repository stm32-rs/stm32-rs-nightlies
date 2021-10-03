#[doc = "Register `JPEG_CONFR3` reader"]
pub struct R(crate::R<JPEG_CONFR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JPEG_CONFR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JPEG_CONFR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JPEG_CONFR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JPEG_CONFR3` writer"]
pub struct W(crate::W<JPEG_CONFR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JPEG_CONFR3_SPEC>;
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
impl From<crate::W<JPEG_CONFR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JPEG_CONFR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XSIZE` reader - X size"]
pub struct XSIZE_R(crate::FieldReader<u16, u16>);
impl XSIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        XSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XSIZE` writer - X size"]
pub struct XSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> XSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - X size"]
    #[inline(always)]
    pub fn xsize(&self) -> XSIZE_R {
        XSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - X size"]
    #[inline(always)]
    pub fn xsize(&mut self) -> XSIZE_W {
        XSIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG codec configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_confr3](index.html) module"]
pub struct JPEG_CONFR3_SPEC;
impl crate::RegisterSpec for JPEG_CONFR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jpeg_confr3::R](R) reader structure"]
impl crate::Readable for JPEG_CONFR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jpeg_confr3::W](W) writer structure"]
impl crate::Writable for JPEG_CONFR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JPEG_CONFR3 to value 0"]
impl crate::Resettable for JPEG_CONFR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
