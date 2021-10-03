#[doc = "Register `HSICFGR` reader"]
pub struct R(crate::R<HSICFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSICFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSICFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSICFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSICFGR` writer"]
pub struct W(crate::W<HSICFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSICFGR_SPEC>;
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
impl From<crate::W<HSICFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSICFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSITRIM` reader - HSI clock trimming"]
pub struct HSITRIM_R(crate::FieldReader<u8, u8>);
impl HSITRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSITRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSITRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSITRIM` writer - HSI clock trimming"]
pub struct HSITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
#[doc = "Field `HSICAL` reader - HSI clock calibration"]
pub struct HSICAL_R(crate::FieldReader<u16, u16>);
impl HSICAL_R {
    pub(crate) fn new(bits: u16) -> Self {
        HSICAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSICAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSICAL` writer - HSI clock calibration"]
pub struct HSICAL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSICAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:30 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bits 0:11 - HSI clock calibration"]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:30 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W {
        HSITRIM_W { w: self }
    }
    #[doc = "Bits 0:11 - HSI clock calibration"]
    #[inline(always)]
    pub fn hsical(&mut self) -> HSICAL_W {
        HSICAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC HSI configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsicfgr](index.html) module"]
pub struct HSICFGR_SPEC;
impl crate::RegisterSpec for HSICFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsicfgr::R](R) reader structure"]
impl crate::Readable for HSICFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsicfgr::W](W) writer structure"]
impl crate::Writable for HSICFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSICFGR to value 0"]
impl crate::Resettable for HSICFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}