#[doc = "Register `DFSDM_FLT5EXMIN` reader"]
pub struct R(crate::R<DFSDM_FLT5EXMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT5EXMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT5EXMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT5EXMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSDM_FLT5EXMIN` writer"]
pub struct W(crate::W<DFSDM_FLT5EXMIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM_FLT5EXMIN_SPEC>;
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
impl From<crate::W<DFSDM_FLT5EXMIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM_FLT5EXMIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXMINCH` reader - EXMINCH"]
pub struct EXMINCH_R(crate::FieldReader<u8, u8>);
impl EXMINCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXMINCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXMINCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXMIN` reader - EXMIN"]
pub struct EXMIN_R(crate::FieldReader<u32, u32>);
impl EXMIN_R {
    pub(crate) fn new(bits: u32) -> Self {
        EXMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXMIN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXMIN` writer - EXMIN"]
pub struct EXMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - EXMINCH"]
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:31 - EXMIN"]
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 8:31 - EXMIN"]
    #[inline(always)]
    pub fn exmin(&mut self) -> EXMIN_W {
        EXMIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM filter 5 extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt5exmin](index.html) module"]
pub struct DFSDM_FLT5EXMIN_SPEC;
impl crate::RegisterSpec for DFSDM_FLT5EXMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_flt5exmin::R](R) reader structure"]
impl crate::Readable for DFSDM_FLT5EXMIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt5exmin::W](W) writer structure"]
impl crate::Writable for DFSDM_FLT5EXMIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFSDM_FLT5EXMIN to value 0x7fff_ff00"]
impl crate::Resettable for DFSDM_FLT5EXMIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7fff_ff00
    }
}
