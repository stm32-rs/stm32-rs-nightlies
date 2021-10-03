#[doc = "Register `CH1CFGR1` reader"]
pub struct R(crate::R<CH1CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1CFGR1` writer"]
pub struct W(crate::W<CH1CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1CFGR1_SPEC>;
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
impl From<crate::W<CH1CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATPACK` reader - DATPACK"]
pub struct DATPACK_R(crate::FieldReader<u8, u8>);
impl DATPACK_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATPACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATPACK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATPACK` writer - DATPACK"]
pub struct DATPACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DATPACK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `DATMPX` reader - DATMPX"]
pub struct DATMPX_R(crate::FieldReader<u8, u8>);
impl DATMPX_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATMPX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATMPX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMPX` writer - DATMPX"]
pub struct DATMPX_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMPX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `CHINSEL` reader - CHINSEL"]
pub struct CHINSEL_R(crate::FieldReader<bool, bool>);
impl CHINSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHINSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHINSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHINSEL` writer - CHINSEL"]
pub struct CHINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHINSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CHEN` reader - CHEN"]
pub struct CHEN_R(crate::FieldReader<bool, bool>);
impl CHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN` writer - CHEN"]
pub struct CHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CKABEN` reader - CKABEN"]
pub struct CKABEN_R(crate::FieldReader<bool, bool>);
impl CKABEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKABEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKABEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKABEN` writer - CKABEN"]
pub struct CKABEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKABEN_W<'a> {
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
#[doc = "Field `SCDEN` reader - SCDEN"]
pub struct SCDEN_R(crate::FieldReader<bool, bool>);
impl SCDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCDEN` writer - SCDEN"]
pub struct SCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDEN_W<'a> {
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
#[doc = "Field `SPICKSEL` reader - SPICKSEL"]
pub struct SPICKSEL_R(crate::FieldReader<u8, u8>);
impl SPICKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPICKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPICKSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPICKSEL` writer - SPICKSEL"]
pub struct SPICKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPICKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `SITP` reader - SITP"]
pub struct SITP_R(crate::FieldReader<u8, u8>);
impl SITP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SITP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SITP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SITP` writer - SITP"]
pub struct SITP_W<'a> {
    w: &'a mut W,
}
impl<'a> SITP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:15 - DATPACK"]
    #[inline(always)]
    pub fn datpack(&self) -> DATPACK_R {
        DATPACK_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - DATMPX"]
    #[inline(always)]
    pub fn datmpx(&self) -> DATMPX_R {
        DATMPX_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 8 - CHINSEL"]
    #[inline(always)]
    pub fn chinsel(&self) -> CHINSEL_R {
        CHINSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CHEN"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CKABEN"]
    #[inline(always)]
    pub fn ckaben(&self) -> CKABEN_R {
        CKABEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SCDEN"]
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - SPICKSEL"]
    #[inline(always)]
    pub fn spicksel(&self) -> SPICKSEL_R {
        SPICKSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - SITP"]
    #[inline(always)]
    pub fn sitp(&self) -> SITP_R {
        SITP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 14:15 - DATPACK"]
    #[inline(always)]
    pub fn datpack(&mut self) -> DATPACK_W {
        DATPACK_W { w: self }
    }
    #[doc = "Bits 12:13 - DATMPX"]
    #[inline(always)]
    pub fn datmpx(&mut self) -> DATMPX_W {
        DATMPX_W { w: self }
    }
    #[doc = "Bit 8 - CHINSEL"]
    #[inline(always)]
    pub fn chinsel(&mut self) -> CHINSEL_W {
        CHINSEL_W { w: self }
    }
    #[doc = "Bit 7 - CHEN"]
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W {
        CHEN_W { w: self }
    }
    #[doc = "Bit 6 - CKABEN"]
    #[inline(always)]
    pub fn ckaben(&mut self) -> CKABEN_W {
        CKABEN_W { w: self }
    }
    #[doc = "Bit 5 - SCDEN"]
    #[inline(always)]
    pub fn scden(&mut self) -> SCDEN_W {
        SCDEN_W { w: self }
    }
    #[doc = "Bits 2:3 - SPICKSEL"]
    #[inline(always)]
    pub fn spicksel(&mut self) -> SPICKSEL_W {
        SPICKSEL_W { w: self }
    }
    #[doc = "Bits 0:1 - SITP"]
    #[inline(always)]
    pub fn sitp(&mut self) -> SITP_W {
        SITP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH1CFGR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1cfgr1](index.html) module"]
pub struct CH1CFGR1_SPEC;
impl crate::RegisterSpec for CH1CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1cfgr1::R](R) reader structure"]
impl crate::Readable for CH1CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1cfgr1::W](W) writer structure"]
impl crate::Writable for CH1CFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH1CFGR1 to value 0"]
impl crate::Resettable for CH1CFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
