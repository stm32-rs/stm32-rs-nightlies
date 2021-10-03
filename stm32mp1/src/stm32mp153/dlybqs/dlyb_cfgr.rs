#[doc = "Register `DLYB_CFGR` reader"]
pub struct R(crate::R<DLYB_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLYB_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLYB_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLYB_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLYB_CFGR` writer"]
pub struct W(crate::W<DLYB_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLYB_CFGR_SPEC>;
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
impl From<crate::W<DLYB_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLYB_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - SEL"]
pub struct SEL_R(crate::FieldReader<u8, u8>);
impl SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - SEL"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `UNIT` reader - UNIT"]
pub struct UNIT_R(crate::FieldReader<u8, u8>);
impl UNIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        UNIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNIT` writer - UNIT"]
pub struct UNIT_W<'a> {
    w: &'a mut W,
}
impl<'a> UNIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `LNG` reader - LNG"]
pub struct LNG_R(crate::FieldReader<u16, u16>);
impl LNG_R {
    pub(crate) fn new(bits: u16) -> Self {
        LNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNGF` reader - LNGF"]
pub struct LNGF_R(crate::FieldReader<bool, bool>);
impl LNGF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LNGF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNGF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - SEL"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - UNIT"]
    #[inline(always)]
    pub fn unit(&self) -> UNIT_R {
        UNIT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:27 - LNG"]
    #[inline(always)]
    pub fn lng(&self) -> LNG_R {
        LNG_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - LNGF"]
    #[inline(always)]
    pub fn lngf(&self) -> LNGF_R {
        LNGF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - SEL"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Bits 8:14 - UNIT"]
    #[inline(always)]
    pub fn unit(&mut self) -> UNIT_W {
        UNIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DLYB configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlyb_cfgr](index.html) module"]
pub struct DLYB_CFGR_SPEC;
impl crate::RegisterSpec for DLYB_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlyb_cfgr::R](R) reader structure"]
impl crate::Readable for DLYB_CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlyb_cfgr::W](W) writer structure"]
impl crate::Writable for DLYB_CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLYB_CFGR to value 0"]
impl crate::Resettable for DLYB_CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
