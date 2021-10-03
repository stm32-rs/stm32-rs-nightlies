#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBPEGEN` reader - Generate Error-Bit on Long Bit Period Error"]
pub struct LBPEGEN_R(crate::FieldReader<bool, bool>);
impl LBPEGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBPEGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LBPEGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBPEGEN` writer - Generate Error-Bit on Long Bit Period Error"]
pub struct LBPEGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LBPEGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `BREGEN` reader - Generate error-bit on bit rising error"]
pub struct BREGEN_R(crate::FieldReader<bool, bool>);
impl BREGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BREGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BREGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BREGEN` writer - Generate error-bit on bit rising error"]
pub struct BREGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BREGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `BRESTP` reader - Rx-stop on bit rising error"]
pub struct BRESTP_R(crate::FieldReader<bool, bool>);
impl BRESTP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRESTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRESTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRESTP` writer - Rx-stop on bit rising error"]
pub struct BRESTP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRESTP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `RXTOL` reader - Rx-Tolerance"]
pub struct RXTOL_R(crate::FieldReader<bool, bool>);
impl RXTOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTOL` writer - Rx-Tolerance"]
pub struct RXTOL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTOL_W<'a> {
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
#[doc = "Field `SFT` reader - Signal Free Time"]
pub struct SFT_R(crate::FieldReader<u8, u8>);
impl SFT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFT` writer - Signal Free Time"]
pub struct SFT_W<'a> {
    w: &'a mut W,
}
impl<'a> SFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `LSTN` reader - Listen mode"]
pub struct LSTN_R(crate::FieldReader<bool, bool>);
impl LSTN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSTN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTN` writer - Listen mode"]
pub struct LSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `OAR` reader - Own Address"]
pub struct OAR_R(crate::FieldReader<u8, u8>);
impl OAR_R {
    pub(crate) fn new(bits: u8) -> Self {
        OAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OAR` writer - Own Address"]
pub struct OAR_W<'a> {
    w: &'a mut W,
}
impl<'a> OAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - Generate Error-Bit on Long Bit Period Error"]
    #[inline(always)]
    pub fn lbpegen(&self) -> LBPEGEN_R {
        LBPEGEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Generate error-bit on bit rising error"]
    #[inline(always)]
    pub fn bregen(&self) -> BREGEN_R {
        BREGEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Rx-stop on bit rising error"]
    #[inline(always)]
    pub fn brestp(&self) -> BRESTP_R {
        BRESTP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Rx-Tolerance"]
    #[inline(always)]
    pub fn rxtol(&self) -> RXTOL_R {
        RXTOL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Signal Free Time"]
    #[inline(always)]
    pub fn sft(&self) -> SFT_R {
        SFT_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - Listen mode"]
    #[inline(always)]
    pub fn lstn(&self) -> LSTN_R {
        LSTN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - Own Address"]
    #[inline(always)]
    pub fn oar(&self) -> OAR_R {
        OAR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 11 - Generate Error-Bit on Long Bit Period Error"]
    #[inline(always)]
    pub fn lbpegen(&mut self) -> LBPEGEN_W {
        LBPEGEN_W { w: self }
    }
    #[doc = "Bit 10 - Generate error-bit on bit rising error"]
    #[inline(always)]
    pub fn bregen(&mut self) -> BREGEN_W {
        BREGEN_W { w: self }
    }
    #[doc = "Bit 9 - Rx-stop on bit rising error"]
    #[inline(always)]
    pub fn brestp(&mut self) -> BRESTP_W {
        BRESTP_W { w: self }
    }
    #[doc = "Bit 8 - Rx-Tolerance"]
    #[inline(always)]
    pub fn rxtol(&mut self) -> RXTOL_W {
        RXTOL_W { w: self }
    }
    #[doc = "Bits 5:7 - Signal Free Time"]
    #[inline(always)]
    pub fn sft(&mut self) -> SFT_W {
        SFT_W { w: self }
    }
    #[doc = "Bit 4 - Listen mode"]
    #[inline(always)]
    pub fn lstn(&mut self) -> LSTN_W {
        LSTN_W { w: self }
    }
    #[doc = "Bits 0:3 - Own Address"]
    #[inline(always)]
    pub fn oar(&mut self) -> OAR_W {
        OAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
