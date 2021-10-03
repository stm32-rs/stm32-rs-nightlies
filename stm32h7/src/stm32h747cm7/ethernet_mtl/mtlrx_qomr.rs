#[doc = "Register `MTLRxQOMR` reader"]
pub struct R(crate::R<MTLRXQOMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLRXQOMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLRXQOMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLRXQOMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTLRxQOMR` writer"]
pub struct W(crate::W<MTLRXQOMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTLRXQOMR_SPEC>;
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
impl From<crate::W<MTLRXQOMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTLRXQOMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RQS` reader - Receive Queue Size"]
pub struct RQS_R(crate::FieldReader<u8, u8>);
impl RQS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RQS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RQS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RQS` writer - Receive Queue Size"]
pub struct RQS_W<'a> {
    w: &'a mut W,
}
impl<'a> RQS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `RFD` reader - Threshold for Deactivating Flow Control"]
pub struct RFD_R(crate::FieldReader<u8, u8>);
impl RFD_R {
    pub(crate) fn new(bits: u8) -> Self {
        RFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFD` writer - Threshold for Deactivating Flow Control"]
pub struct RFD_W<'a> {
    w: &'a mut W,
}
impl<'a> RFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | ((value as u32 & 0x07) << 14);
        self.w
    }
}
#[doc = "Field `RFA` reader - Threshold for Activating Flow Control"]
pub struct RFA_R(crate::FieldReader<u8, u8>);
impl RFA_R {
    pub(crate) fn new(bits: u8) -> Self {
        RFA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFA` writer - Threshold for Activating Flow Control"]
pub struct RFA_W<'a> {
    w: &'a mut W,
}
impl<'a> RFA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `EHFC` reader - Enable Hardware Flow Control"]
pub struct EHFC_R(crate::FieldReader<bool, bool>);
impl EHFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        EHFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EHFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EHFC` writer - Enable Hardware Flow Control"]
pub struct EHFC_W<'a> {
    w: &'a mut W,
}
impl<'a> EHFC_W<'a> {
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
#[doc = "Field `DIS_TCP_EF` reader - Disable Dropping of TCP"]
pub struct DIS_TCP_EF_R(crate::FieldReader<bool, bool>);
impl DIS_TCP_EF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_TCP_EF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_TCP_EF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_TCP_EF` writer - Disable Dropping of TCP"]
pub struct DIS_TCP_EF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_TCP_EF_W<'a> {
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
#[doc = "Field `RSF` reader - Receive Queue Store and Forward"]
pub struct RSF_R(crate::FieldReader<bool, bool>);
impl RSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSF` writer - Receive Queue Store and Forward"]
pub struct RSF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSF_W<'a> {
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
#[doc = "Field `FEP` reader - Forward Error Packets"]
pub struct FEP_R(crate::FieldReader<bool, bool>);
impl FEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEP` writer - Forward Error Packets"]
pub struct FEP_W<'a> {
    w: &'a mut W,
}
impl<'a> FEP_W<'a> {
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
#[doc = "Field `FUP` reader - Forward Undersized Good Packets"]
pub struct FUP_R(crate::FieldReader<bool, bool>);
impl FUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUP` writer - Forward Undersized Good Packets"]
pub struct FUP_W<'a> {
    w: &'a mut W,
}
impl<'a> FUP_W<'a> {
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
#[doc = "Field `RTC` reader - Receive Queue Threshold Control"]
pub struct RTC_R(crate::FieldReader<u8, u8>);
impl RTC_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC` writer - Receive Queue Threshold Control"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:22 - Receive Queue Size"]
    #[inline(always)]
    pub fn rqs(&self) -> RQS_R {
        RQS_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 14:16 - Threshold for Deactivating Flow Control"]
    #[inline(always)]
    pub fn rfd(&self) -> RFD_R {
        RFD_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Threshold for Activating Flow Control"]
    #[inline(always)]
    pub fn rfa(&self) -> RFA_R {
        RFA_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Enable Hardware Flow Control"]
    #[inline(always)]
    pub fn ehfc(&self) -> EHFC_R {
        EHFC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Disable Dropping of TCP"]
    #[inline(always)]
    pub fn dis_tcp_ef(&self) -> DIS_TCP_EF_R {
        DIS_TCP_EF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive Queue Store and Forward"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Forward Error Packets"]
    #[inline(always)]
    pub fn fep(&self) -> FEP_R {
        FEP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Forward Undersized Good Packets"]
    #[inline(always)]
    pub fn fup(&self) -> FUP_R {
        FUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - Receive Queue Threshold Control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 20:22 - Receive Queue Size"]
    #[inline(always)]
    pub fn rqs(&mut self) -> RQS_W {
        RQS_W { w: self }
    }
    #[doc = "Bits 14:16 - Threshold for Deactivating Flow Control"]
    #[inline(always)]
    pub fn rfd(&mut self) -> RFD_W {
        RFD_W { w: self }
    }
    #[doc = "Bits 8:10 - Threshold for Activating Flow Control"]
    #[inline(always)]
    pub fn rfa(&mut self) -> RFA_W {
        RFA_W { w: self }
    }
    #[doc = "Bit 7 - Enable Hardware Flow Control"]
    #[inline(always)]
    pub fn ehfc(&mut self) -> EHFC_W {
        EHFC_W { w: self }
    }
    #[doc = "Bit 6 - Disable Dropping of TCP"]
    #[inline(always)]
    pub fn dis_tcp_ef(&mut self) -> DIS_TCP_EF_W {
        DIS_TCP_EF_W { w: self }
    }
    #[doc = "Bit 5 - Receive Queue Store and Forward"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W {
        RSF_W { w: self }
    }
    #[doc = "Bit 4 - Forward Error Packets"]
    #[inline(always)]
    pub fn fep(&mut self) -> FEP_W {
        FEP_W { w: self }
    }
    #[doc = "Bit 3 - Forward Undersized Good Packets"]
    #[inline(always)]
    pub fn fup(&mut self) -> FUP_W {
        FUP_W { w: self }
    }
    #[doc = "Bits 0:1 - Receive Queue Threshold Control"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx queue operating mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlrx_qomr](index.html) module"]
pub struct MTLRXQOMR_SPEC;
impl crate::RegisterSpec for MTLRXQOMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtlrx_qomr::R](R) reader structure"]
impl crate::Readable for MTLRXQOMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtlrx_qomr::W](W) writer structure"]
impl crate::Writable for MTLRXQOMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTLRxQOMR to value 0x0070_0000"]
impl crate::Resettable for MTLRXQOMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0070_0000
    }
}
