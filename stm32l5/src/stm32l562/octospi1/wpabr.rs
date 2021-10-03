#[doc = "Register `WPABR` reader"]
pub struct R(crate::R<WPABR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPABR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPABR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPABR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPABR` writer"]
pub struct W(crate::W<WPABR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPABR_SPEC>;
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
impl From<crate::W<WPABR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPABR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LM` reader - Latency mode"]
pub struct LM_R(crate::FieldReader<bool, bool>);
impl LM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LM` writer - Latency mode"]
pub struct LM_W<'a> {
    w: &'a mut W,
}
impl<'a> LM_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `WZL` reader - Write zero latency"]
pub struct WZL_R(crate::FieldReader<bool, bool>);
impl WZL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WZL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WZL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WZL` writer - Write zero latency"]
pub struct WZL_W<'a> {
    w: &'a mut W,
}
impl<'a> WZL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TACC` reader - Access time"]
pub struct TACC_R(crate::FieldReader<u8, u8>);
impl TACC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TACC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TACC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TACC` writer - Access time"]
pub struct TACC_W<'a> {
    w: &'a mut W,
}
impl<'a> TACC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TRWR` reader - Read write recovery time"]
pub struct TRWR_R(crate::FieldReader<u8, u8>);
impl TRWR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRWR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRWR` writer - Read write recovery time"]
pub struct TRWR_W<'a> {
    w: &'a mut W,
}
impl<'a> TRWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Latency mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write zero latency"]
    #[inline(always)]
    pub fn wzl(&self) -> WZL_R {
        WZL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Access time"]
    #[inline(always)]
    pub fn tacc(&self) -> TACC_R {
        TACC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Read write recovery time"]
    #[inline(always)]
    pub fn trwr(&self) -> TRWR_R {
        TRWR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Latency mode"]
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W {
        LM_W { w: self }
    }
    #[doc = "Bit 1 - Write zero latency"]
    #[inline(always)]
    pub fn wzl(&mut self) -> WZL_W {
        WZL_W { w: self }
    }
    #[doc = "Bits 8:15 - Access time"]
    #[inline(always)]
    pub fn tacc(&mut self) -> TACC_W {
        TACC_W { w: self }
    }
    #[doc = "Bits 16:23 - Read write recovery time"]
    #[inline(always)]
    pub fn trwr(&mut self) -> TRWR_W {
        TRWR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "write alternate bytes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpabr](index.html) module"]
pub struct WPABR_SPEC;
impl crate::RegisterSpec for WPABR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpabr::R](R) reader structure"]
impl crate::Readable for WPABR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpabr::W](W) writer structure"]
impl crate::Writable for WPABR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPABR to value 0"]
impl crate::Resettable for WPABR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
