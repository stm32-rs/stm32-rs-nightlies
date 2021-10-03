#[doc = "Register `CSICFGR` reader"]
pub struct R(crate::R<CSICFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSICFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSICFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSICFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSICFGR` writer"]
pub struct W(crate::W<CSICFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSICFGR_SPEC>;
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
impl From<crate::W<CSICFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSICFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSITRIM` reader - CSI clock trimming"]
pub struct CSITRIM_R(crate::FieldReader<u8, u8>);
impl CSITRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSITRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSITRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSITRIM` writer - CSI clock trimming"]
pub struct CSITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CSITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `CSICAL` reader - CSI clock calibration"]
pub struct CSICAL_R(crate::FieldReader<u16, u16>);
impl CSICAL_R {
    pub(crate) fn new(bits: u16) -> Self {
        CSICAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSICAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSICAL` writer - CSI clock calibration"]
pub struct CSICAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSICAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29 - CSI clock trimming"]
    #[inline(always)]
    pub fn csitrim(&self) -> CSITRIM_R {
        CSITRIM_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 0:8 - CSI clock calibration"]
    #[inline(always)]
    pub fn csical(&self) -> CSICAL_R {
        CSICAL_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:29 - CSI clock trimming"]
    #[inline(always)]
    pub fn csitrim(&mut self) -> CSITRIM_W {
        CSITRIM_W { w: self }
    }
    #[doc = "Bits 0:8 - CSI clock calibration"]
    #[inline(always)]
    pub fn csical(&mut self) -> CSICAL_W {
        CSICAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC CSI configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csicfgr](index.html) module"]
pub struct CSICFGR_SPEC;
impl crate::RegisterSpec for CSICFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csicfgr::R](R) reader structure"]
impl crate::Readable for CSICFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csicfgr::W](W) writer structure"]
impl crate::Writable for CSICFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSICFGR to value 0"]
impl crate::Resettable for CSICFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
