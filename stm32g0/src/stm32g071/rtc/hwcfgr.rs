#[doc = "Register `HWCFGR` reader"]
pub struct R(crate::R<HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWCFGR` writer"]
pub struct W(crate::W<HWCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR_SPEC>;
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
impl From<crate::W<HWCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALARMB` reader - ALARMB"]
pub struct ALARMB_R(crate::FieldReader<u8, u8>);
impl ALARMB_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALARMB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALARMB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALARMB` writer - ALARMB"]
pub struct ALARMB_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARMB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `WAKEUP` reader - WAKEUP"]
pub struct WAKEUP_R(crate::FieldReader<u8, u8>);
impl WAKEUP_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP` writer - WAKEUP"]
pub struct WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `SMOOTH_CALIB` reader - SMOOTH_CALIB"]
pub struct SMOOTH_CALIB_R(crate::FieldReader<u8, u8>);
impl SMOOTH_CALIB_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMOOTH_CALIB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMOOTH_CALIB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMOOTH_CALIB` writer - SMOOTH_CALIB"]
pub struct SMOOTH_CALIB_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOOTH_CALIB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TIMESTAMP` reader - TIMESTAMP"]
pub struct TIMESTAMP_R(crate::FieldReader<u8, u8>);
impl TIMESTAMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TIMESTAMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMESTAMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMESTAMP` writer - TIMESTAMP"]
pub struct TIMESTAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMESTAMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `OPTIONREG_OUT` reader - OPTIONREG_OUT"]
pub struct OPTIONREG_OUT_R(crate::FieldReader<u8, u8>);
impl OPTIONREG_OUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        OPTIONREG_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPTIONREG_OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPTIONREG_OUT` writer - OPTIONREG_OUT"]
pub struct OPTIONREG_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTIONREG_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `TRUST_ZONE` reader - TRUST_ZONE"]
pub struct TRUST_ZONE_R(crate::FieldReader<u8, u8>);
impl TRUST_ZONE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRUST_ZONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRUST_ZONE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRUST_ZONE` writer - TRUST_ZONE"]
pub struct TRUST_ZONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRUST_ZONE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ALARMB"]
    #[inline(always)]
    pub fn alarmb(&self) -> ALARMB_R {
        ALARMB_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - WAKEUP"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SMOOTH_CALIB"]
    #[inline(always)]
    pub fn smooth_calib(&self) -> SMOOTH_CALIB_R {
        SMOOTH_CALIB_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - TIMESTAMP"]
    #[inline(always)]
    pub fn timestamp(&self) -> TIMESTAMP_R {
        TIMESTAMP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - OPTIONREG_OUT"]
    #[inline(always)]
    pub fn optionreg_out(&self) -> OPTIONREG_OUT_R {
        OPTIONREG_OUT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - TRUST_ZONE"]
    #[inline(always)]
    pub fn trust_zone(&self) -> TRUST_ZONE_R {
        TRUST_ZONE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ALARMB"]
    #[inline(always)]
    pub fn alarmb(&mut self) -> ALARMB_W {
        ALARMB_W { w: self }
    }
    #[doc = "Bits 4:7 - WAKEUP"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WAKEUP_W {
        WAKEUP_W { w: self }
    }
    #[doc = "Bits 8:11 - SMOOTH_CALIB"]
    #[inline(always)]
    pub fn smooth_calib(&mut self) -> SMOOTH_CALIB_W {
        SMOOTH_CALIB_W { w: self }
    }
    #[doc = "Bits 12:15 - TIMESTAMP"]
    #[inline(always)]
    pub fn timestamp(&mut self) -> TIMESTAMP_W {
        TIMESTAMP_W { w: self }
    }
    #[doc = "Bits 16:23 - OPTIONREG_OUT"]
    #[inline(always)]
    pub fn optionreg_out(&mut self) -> OPTIONREG_OUT_W {
        OPTIONREG_OUT_W { w: self }
    }
    #[doc = "Bits 24:27 - TRUST_ZONE"]
    #[inline(always)]
    pub fn trust_zone(&mut self) -> TRUST_ZONE_W {
        TRUST_ZONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr](index.html) module"]
pub struct HWCFGR_SPEC;
impl crate::RegisterSpec for HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr::R](R) reader structure"]
impl crate::Readable for HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwcfgr::W](W) writer structure"]
impl crate::Writable for HWCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HWCFGR to value 0"]
impl crate::Resettable for HWCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
