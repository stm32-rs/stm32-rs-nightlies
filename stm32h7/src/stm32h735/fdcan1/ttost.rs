#[doc = "Register `TTOST` reader"]
pub struct R(crate::R<TTOST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTOST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTOST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTOST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTOST` writer"]
pub struct W(crate::W<TTOST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTOST_SPEC>;
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
impl From<crate::W<TTOST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTOST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EL` reader - Error Level"]
pub struct EL_R(crate::FieldReader<u8, u8>);
impl EL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EL` writer - Error Level"]
pub struct EL_W<'a> {
    w: &'a mut W,
}
impl<'a> EL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `MS` reader - Master State."]
pub struct MS_R(crate::FieldReader<u8, u8>);
impl MS_R {
    pub(crate) fn new(bits: u8) -> Self {
        MS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MS` writer - Master State."]
pub struct MS_W<'a> {
    w: &'a mut W,
}
impl<'a> MS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `SYS` reader - Synchronization State"]
pub struct SYS_R(crate::FieldReader<u8, u8>);
impl SYS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS` writer - Synchronization State"]
pub struct SYS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `QGTP` reader - Quality of Global Time Phase"]
pub struct QGTP_R(crate::FieldReader<bool, bool>);
impl QGTP_R {
    pub(crate) fn new(bits: bool) -> Self {
        QGTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QGTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QGTP` writer - Quality of Global Time Phase"]
pub struct QGTP_W<'a> {
    w: &'a mut W,
}
impl<'a> QGTP_W<'a> {
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
#[doc = "Field `QCS` reader - Quality of Clock Speed"]
pub struct QCS_R(crate::FieldReader<bool, bool>);
impl QCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        QCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QCS` writer - Quality of Clock Speed"]
pub struct QCS_W<'a> {
    w: &'a mut W,
}
impl<'a> QCS_W<'a> {
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
#[doc = "Field `RTO` reader - Reference Trigger Offset"]
pub struct RTO_R(crate::FieldReader<u8, u8>);
impl RTO_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTO` writer - Reference Trigger Offset"]
pub struct RTO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `WGTD` reader - Wait for Global Time Discontinuity"]
pub struct WGTD_R(crate::FieldReader<bool, bool>);
impl WGTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        WGTD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WGTD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WGTD` writer - Wait for Global Time Discontinuity"]
pub struct WGTD_W<'a> {
    w: &'a mut W,
}
impl<'a> WGTD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `GFI` reader - Gap Finished Indicator."]
pub struct GFI_R(crate::FieldReader<bool, bool>);
impl GFI_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFI` writer - Gap Finished Indicator."]
pub struct GFI_W<'a> {
    w: &'a mut W,
}
impl<'a> GFI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `TMP` reader - Time Master Priority"]
pub struct TMP_R(crate::FieldReader<u8, u8>);
impl TMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMP` writer - Time Master Priority"]
pub struct TMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `GSI` reader - Gap Started Indicator."]
pub struct GSI_R(crate::FieldReader<bool, bool>);
impl GSI_R {
    pub(crate) fn new(bits: bool) -> Self {
        GSI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GSI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GSI` writer - Gap Started Indicator."]
pub struct GSI_W<'a> {
    w: &'a mut W,
}
impl<'a> GSI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `WFE` reader - Wait for Event"]
pub struct WFE_R(crate::FieldReader<bool, bool>);
impl WFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WFE` writer - Wait for Event"]
pub struct WFE_W<'a> {
    w: &'a mut W,
}
impl<'a> WFE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `AWE` reader - Application Watchdog Event"]
pub struct AWE_R(crate::FieldReader<bool, bool>);
impl AWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWE` writer - Application Watchdog Event"]
pub struct AWE_W<'a> {
    w: &'a mut W,
}
impl<'a> AWE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `WECS` reader - Wait for External Clock Synchronization"]
pub struct WECS_R(crate::FieldReader<bool, bool>);
impl WECS_R {
    pub(crate) fn new(bits: bool) -> Self {
        WECS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WECS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WECS` writer - Wait for External Clock Synchronization"]
pub struct WECS_W<'a> {
    w: &'a mut W,
}
impl<'a> WECS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `SPL` reader - Schedule Phase Lock"]
pub struct SPL_R(crate::FieldReader<bool, bool>);
impl SPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPL` writer - Schedule Phase Lock"]
pub struct SPL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Error Level"]
    #[inline(always)]
    pub fn el(&self) -> EL_R {
        EL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Master State."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Synchronization State"]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Quality of Global Time Phase"]
    #[inline(always)]
    pub fn qgtp(&self) -> QGTP_R {
        QGTP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Quality of Clock Speed"]
    #[inline(always)]
    pub fn qcs(&self) -> QCS_R {
        QCS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Reference Trigger Offset"]
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 22 - Wait for Global Time Discontinuity"]
    #[inline(always)]
    pub fn wgtd(&self) -> WGTD_R {
        WGTD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Gap Finished Indicator."]
    #[inline(always)]
    pub fn gfi(&self) -> GFI_R {
        GFI_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Time Master Priority"]
    #[inline(always)]
    pub fn tmp(&self) -> TMP_R {
        TMP_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - Gap Started Indicator."]
    #[inline(always)]
    pub fn gsi(&self) -> GSI_R {
        GSI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Wait for Event"]
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Application Watchdog Event"]
    #[inline(always)]
    pub fn awe(&self) -> AWE_R {
        AWE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Wait for External Clock Synchronization"]
    #[inline(always)]
    pub fn wecs(&self) -> WECS_R {
        WECS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Schedule Phase Lock"]
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Error Level"]
    #[inline(always)]
    pub fn el(&mut self) -> EL_W {
        EL_W { w: self }
    }
    #[doc = "Bits 2:3 - Master State."]
    #[inline(always)]
    pub fn ms(&mut self) -> MS_W {
        MS_W { w: self }
    }
    #[doc = "Bits 4:5 - Synchronization State"]
    #[inline(always)]
    pub fn sys(&mut self) -> SYS_W {
        SYS_W { w: self }
    }
    #[doc = "Bit 6 - Quality of Global Time Phase"]
    #[inline(always)]
    pub fn qgtp(&mut self) -> QGTP_W {
        QGTP_W { w: self }
    }
    #[doc = "Bit 7 - Quality of Clock Speed"]
    #[inline(always)]
    pub fn qcs(&mut self) -> QCS_W {
        QCS_W { w: self }
    }
    #[doc = "Bits 8:15 - Reference Trigger Offset"]
    #[inline(always)]
    pub fn rto(&mut self) -> RTO_W {
        RTO_W { w: self }
    }
    #[doc = "Bit 22 - Wait for Global Time Discontinuity"]
    #[inline(always)]
    pub fn wgtd(&mut self) -> WGTD_W {
        WGTD_W { w: self }
    }
    #[doc = "Bit 23 - Gap Finished Indicator."]
    #[inline(always)]
    pub fn gfi(&mut self) -> GFI_W {
        GFI_W { w: self }
    }
    #[doc = "Bits 24:26 - Time Master Priority"]
    #[inline(always)]
    pub fn tmp(&mut self) -> TMP_W {
        TMP_W { w: self }
    }
    #[doc = "Bit 27 - Gap Started Indicator."]
    #[inline(always)]
    pub fn gsi(&mut self) -> GSI_W {
        GSI_W { w: self }
    }
    #[doc = "Bit 28 - Wait for Event"]
    #[inline(always)]
    pub fn wfe(&mut self) -> WFE_W {
        WFE_W { w: self }
    }
    #[doc = "Bit 29 - Application Watchdog Event"]
    #[inline(always)]
    pub fn awe(&mut self) -> AWE_W {
        AWE_W { w: self }
    }
    #[doc = "Bit 30 - Wait for External Clock Synchronization"]
    #[inline(always)]
    pub fn wecs(&mut self) -> WECS_W {
        WECS_W { w: self }
    }
    #[doc = "Bit 31 - Schedule Phase Lock"]
    #[inline(always)]
    pub fn spl(&mut self) -> SPL_W {
        SPL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Operation Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttost](index.html) module"]
pub struct TTOST_SPEC;
impl crate::RegisterSpec for TTOST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttost::R](R) reader structure"]
impl crate::Readable for TTOST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttost::W](W) writer structure"]
impl crate::Writable for TTOST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TTOST to value 0"]
impl crate::Resettable for TTOST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
