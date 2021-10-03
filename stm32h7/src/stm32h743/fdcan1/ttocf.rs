#[doc = "Register `TTOCF` reader"]
pub struct R(crate::R<TTOCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTOCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTOCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTOCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTOCF` writer"]
pub struct W(crate::W<TTOCF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTOCF_SPEC>;
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
impl From<crate::W<TTOCF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTOCF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OM` reader - Operation Mode"]
pub struct OM_R(crate::FieldReader<u8, u8>);
impl OM_R {
    pub(crate) fn new(bits: u8) -> Self {
        OM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OM` writer - Operation Mode"]
pub struct OM_W<'a> {
    w: &'a mut W,
}
impl<'a> OM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `GEN` reader - Gap Enable"]
pub struct GEN_R(crate::FieldReader<bool, bool>);
impl GEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN` writer - Gap Enable"]
pub struct GEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `TM` reader - Time Master"]
pub struct TM_R(crate::FieldReader<bool, bool>);
impl TM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TM` writer - Time Master"]
pub struct TM_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_W<'a> {
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
#[doc = "Field `LDSDL` reader - LD of Synchronization Deviation Limit"]
pub struct LDSDL_R(crate::FieldReader<u8, u8>);
impl LDSDL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LDSDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDSDL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDSDL` writer - LD of Synchronization Deviation Limit"]
pub struct LDSDL_W<'a> {
    w: &'a mut W,
}
impl<'a> LDSDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `IRTO` reader - Initial Reference Trigger Offset"]
pub struct IRTO_R(crate::FieldReader<u8, u8>);
impl IRTO_R {
    pub(crate) fn new(bits: u8) -> Self {
        IRTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRTO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRTO` writer - Initial Reference Trigger Offset"]
pub struct IRTO_W<'a> {
    w: &'a mut W,
}
impl<'a> IRTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `EECS` reader - Enable External Clock Synchronization"]
pub struct EECS_R(crate::FieldReader<bool, bool>);
impl EECS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EECS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EECS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EECS` writer - Enable External Clock Synchronization"]
pub struct EECS_W<'a> {
    w: &'a mut W,
}
impl<'a> EECS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `AWL` reader - Application Watchdog Limit"]
pub struct AWL_R(crate::FieldReader<u8, u8>);
impl AWL_R {
    pub(crate) fn new(bits: u8) -> Self {
        AWL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWL` writer - Application Watchdog Limit"]
pub struct AWL_W<'a> {
    w: &'a mut W,
}
impl<'a> AWL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `EGTF` reader - Enable Global Time Filtering"]
pub struct EGTF_R(crate::FieldReader<bool, bool>);
impl EGTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        EGTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EGTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EGTF` writer - Enable Global Time Filtering"]
pub struct EGTF_W<'a> {
    w: &'a mut W,
}
impl<'a> EGTF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `ECC` reader - Enable Clock Calibration"]
pub struct ECC_R(crate::FieldReader<bool, bool>);
impl ECC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECC` writer - Enable Clock Calibration"]
pub struct ECC_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `EVTP` reader - Event Trigger Polarity"]
pub struct EVTP_R(crate::FieldReader<bool, bool>);
impl EVTP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EVTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVTP` writer - Event Trigger Polarity"]
pub struct EVTP_W<'a> {
    w: &'a mut W,
}
impl<'a> EVTP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Operation Mode"]
    #[inline(always)]
    pub fn om(&self) -> OM_R {
        OM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Gap Enable"]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Time Master"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - LD of Synchronization Deviation Limit"]
    #[inline(always)]
    pub fn ldsdl(&self) -> LDSDL_R {
        LDSDL_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:14 - Initial Reference Trigger Offset"]
    #[inline(always)]
    pub fn irto(&self) -> IRTO_R {
        IRTO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Enable External Clock Synchronization"]
    #[inline(always)]
    pub fn eecs(&self) -> EECS_R {
        EECS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Application Watchdog Limit"]
    #[inline(always)]
    pub fn awl(&self) -> AWL_R {
        AWL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Enable Global Time Filtering"]
    #[inline(always)]
    pub fn egtf(&self) -> EGTF_R {
        EGTF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable Clock Calibration"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Event Trigger Polarity"]
    #[inline(always)]
    pub fn evtp(&self) -> EVTP_R {
        EVTP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operation Mode"]
    #[inline(always)]
    pub fn om(&mut self) -> OM_W {
        OM_W { w: self }
    }
    #[doc = "Bit 3 - Gap Enable"]
    #[inline(always)]
    pub fn gen(&mut self) -> GEN_W {
        GEN_W { w: self }
    }
    #[doc = "Bit 4 - Time Master"]
    #[inline(always)]
    pub fn tm(&mut self) -> TM_W {
        TM_W { w: self }
    }
    #[doc = "Bits 5:7 - LD of Synchronization Deviation Limit"]
    #[inline(always)]
    pub fn ldsdl(&mut self) -> LDSDL_W {
        LDSDL_W { w: self }
    }
    #[doc = "Bits 8:14 - Initial Reference Trigger Offset"]
    #[inline(always)]
    pub fn irto(&mut self) -> IRTO_W {
        IRTO_W { w: self }
    }
    #[doc = "Bit 15 - Enable External Clock Synchronization"]
    #[inline(always)]
    pub fn eecs(&mut self) -> EECS_W {
        EECS_W { w: self }
    }
    #[doc = "Bits 16:23 - Application Watchdog Limit"]
    #[inline(always)]
    pub fn awl(&mut self) -> AWL_W {
        AWL_W { w: self }
    }
    #[doc = "Bit 24 - Enable Global Time Filtering"]
    #[inline(always)]
    pub fn egtf(&mut self) -> EGTF_W {
        EGTF_W { w: self }
    }
    #[doc = "Bit 25 - Enable Clock Calibration"]
    #[inline(always)]
    pub fn ecc(&mut self) -> ECC_W {
        ECC_W { w: self }
    }
    #[doc = "Bit 26 - Event Trigger Polarity"]
    #[inline(always)]
    pub fn evtp(&mut self) -> EVTP_W {
        EVTP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Operation Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttocf](index.html) module"]
pub struct TTOCF_SPEC;
impl crate::RegisterSpec for TTOCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttocf::R](R) reader structure"]
impl crate::Readable for TTOCF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttocf::W](W) writer structure"]
impl crate::Writable for TTOCF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TTOCF to value 0"]
impl crate::Resettable for TTOCF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
