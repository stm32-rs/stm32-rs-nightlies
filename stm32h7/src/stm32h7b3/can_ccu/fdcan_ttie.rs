#[doc = "Register `FDCAN_TTIE` reader"]
pub struct R(crate::R<FDCAN_TTIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TTIE` writer"]
pub struct W(crate::W<FDCAN_TTIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTIE_SPEC>;
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
impl From<crate::W<FDCAN_TTIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBCE` reader - Start of Basic Cycle Interrupt Enable"]
pub struct SBCE_R(crate::FieldReader<bool, bool>);
impl SBCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBCE` writer - Start of Basic Cycle Interrupt Enable"]
pub struct SBCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SBCE_W<'a> {
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
#[doc = "Field `SMCE` reader - Start of Matrix Cycle Interrupt Enable"]
pub struct SMCE_R(crate::FieldReader<bool, bool>);
impl SMCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMCE` writer - Start of Matrix Cycle Interrupt Enable"]
pub struct SMCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMCE_W<'a> {
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
#[doc = "Field `CSME` reader - Change of Synchronization Mode Interrupt Enable"]
pub struct CSME_R(crate::FieldReader<bool, bool>);
impl CSME_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSME` writer - Change of Synchronization Mode Interrupt Enable"]
pub struct CSME_W<'a> {
    w: &'a mut W,
}
impl<'a> CSME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SOGE` reader - Start of Gap Interrupt Enable"]
pub struct SOGE_R(crate::FieldReader<bool, bool>);
impl SOGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOGE` writer - Start of Gap Interrupt Enable"]
pub struct SOGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOGE_W<'a> {
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
#[doc = "Field `RTMIE` reader - Register Time Mark Interrupt Enable"]
pub struct RTMIE_R(crate::FieldReader<bool, bool>);
impl RTMIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTMIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTMIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTMIE` writer - Register Time Mark Interrupt Enable"]
pub struct RTMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTMIE_W<'a> {
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
#[doc = "Field `TTMIE` reader - Trigger Time Mark Event Internal Interrupt Enable"]
pub struct TTMIE_R(crate::FieldReader<bool, bool>);
impl TTMIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TTMIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTMIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTMIE` writer - Trigger Time Mark Event Internal Interrupt Enable"]
pub struct TTMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TTMIE_W<'a> {
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
#[doc = "Field `SWEE` reader - Stop Watch Event Interrupt Enable"]
pub struct SWEE_R(crate::FieldReader<bool, bool>);
impl SWEE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWEE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWEE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWEE` writer - Stop Watch Event Interrupt Enable"]
pub struct SWEE_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEE_W<'a> {
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
#[doc = "Field `GTWE` reader - Global Time Wrap Interrupt Enable"]
pub struct GTWE_R(crate::FieldReader<bool, bool>);
impl GTWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GTWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTWE` writer - Global Time Wrap Interrupt Enable"]
pub struct GTWE_W<'a> {
    w: &'a mut W,
}
impl<'a> GTWE_W<'a> {
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
#[doc = "Field `GTDE` reader - Global Time Discontinuity Interrupt Enable"]
pub struct GTDE_R(crate::FieldReader<bool, bool>);
impl GTDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GTDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTDE` writer - Global Time Discontinuity Interrupt Enable"]
pub struct GTDE_W<'a> {
    w: &'a mut W,
}
impl<'a> GTDE_W<'a> {
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
#[doc = "Field `GTEE` reader - Global Time Error Interrupt Enable"]
pub struct GTEE_R(crate::FieldReader<bool, bool>);
impl GTEE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GTEE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTEE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTEE` writer - Global Time Error Interrupt Enable"]
pub struct GTEE_W<'a> {
    w: &'a mut W,
}
impl<'a> GTEE_W<'a> {
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
#[doc = "Field `TXUE` reader - Tx Count Underflow Interrupt Enable"]
pub struct TXUE_R(crate::FieldReader<bool, bool>);
impl TXUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUE` writer - Tx Count Underflow Interrupt Enable"]
pub struct TXUE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUE_W<'a> {
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
#[doc = "Field `TXOE` reader - Tx Count Overflow Interrupt Enable"]
pub struct TXOE_R(crate::FieldReader<bool, bool>);
impl TXOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOE` writer - Tx Count Overflow Interrupt Enable"]
pub struct TXOE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOE_W<'a> {
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
#[doc = "Field `SE1E` reader - Scheduling Error 1 Interrupt Enable"]
pub struct SE1E_R(crate::FieldReader<bool, bool>);
impl SE1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE1E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE1E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SE1E` writer - Scheduling Error 1 Interrupt Enable"]
pub struct SE1E_W<'a> {
    w: &'a mut W,
}
impl<'a> SE1E_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `SE2E` reader - Scheduling Error 2 Interrupt Enable"]
pub struct SE2E_R(crate::FieldReader<bool, bool>);
impl SE2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE2E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE2E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SE2E` writer - Scheduling Error 2 Interrupt Enable"]
pub struct SE2E_W<'a> {
    w: &'a mut W,
}
impl<'a> SE2E_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `ELCE` reader - Change Error Level Interrupt Enable"]
pub struct ELCE_R(crate::FieldReader<bool, bool>);
impl ELCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ELCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ELCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ELCE` writer - Change Error Level Interrupt Enable"]
pub struct ELCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ELCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `IWTGE` reader - Initialization Watch Trigger Interrupt Enable"]
pub struct IWTGE_R(crate::FieldReader<bool, bool>);
impl IWTGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWTGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWTGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWTGE` writer - Initialization Watch Trigger Interrupt Enable"]
pub struct IWTGE_W<'a> {
    w: &'a mut W,
}
impl<'a> IWTGE_W<'a> {
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
#[doc = "Field `WTE` reader - Watch Trigger Interrupt Enable"]
pub struct WTE_R(crate::FieldReader<bool, bool>);
impl WTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WTE` writer - Watch Trigger Interrupt Enable"]
pub struct WTE_W<'a> {
    w: &'a mut W,
}
impl<'a> WTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `AWE` reader - Application Watchdog Interrupt Enable"]
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
#[doc = "Field `AWE` writer - Application Watchdog Interrupt Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CERE` reader - Configuration Error Interrupt Enable"]
pub struct CERE_R(crate::FieldReader<bool, bool>);
impl CERE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CERE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CERE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CERE` writer - Configuration Error Interrupt Enable"]
pub struct CERE_W<'a> {
    w: &'a mut W,
}
impl<'a> CERE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Start of Basic Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn sbce(&self) -> SBCE_R {
        SBCE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn smce(&self) -> SMCE_R {
        SMCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode Interrupt Enable"]
    #[inline(always)]
    pub fn csme(&self) -> CSME_R {
        CSME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of Gap Interrupt Enable"]
    #[inline(always)]
    pub fn soge(&self) -> SOGE_R {
        SOGE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Enable"]
    #[inline(always)]
    pub fn rtmie(&self) -> RTMIE_R {
        RTMIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Interrupt Enable"]
    #[inline(always)]
    pub fn ttmie(&self) -> TTMIE_R {
        TTMIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stop Watch Event Interrupt Enable"]
    #[inline(always)]
    pub fn swee(&self) -> SWEE_R {
        SWEE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Global Time Wrap Interrupt Enable"]
    #[inline(always)]
    pub fn gtwe(&self) -> GTWE_R {
        GTWE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Global Time Discontinuity Interrupt Enable"]
    #[inline(always)]
    pub fn gtde(&self) -> GTDE_R {
        GTDE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Global Time Error Interrupt Enable"]
    #[inline(always)]
    pub fn gtee(&self) -> GTEE_R {
        GTEE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Tx Count Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn txue(&self) -> TXUE_R {
        TXUE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx Count Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txoe(&self) -> TXOE_R {
        TXOE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Scheduling Error 1 Interrupt Enable"]
    #[inline(always)]
    pub fn se1e(&self) -> SE1E_R {
        SE1E_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Scheduling Error 2 Interrupt Enable"]
    #[inline(always)]
    pub fn se2e(&self) -> SE2E_R {
        SE2E_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Change Error Level Interrupt Enable"]
    #[inline(always)]
    pub fn elce(&self) -> ELCE_R {
        ELCE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn iwtge(&self) -> IWTGE_R {
        IWTGE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn wte(&self) -> WTE_R {
        WTE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Application Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn awe(&self) -> AWE_R {
        AWE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Configuration Error Interrupt Enable"]
    #[inline(always)]
    pub fn cere(&self) -> CERE_R {
        CERE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of Basic Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn sbce(&mut self) -> SBCE_W {
        SBCE_W { w: self }
    }
    #[doc = "Bit 1 - Start of Matrix Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn smce(&mut self) -> SMCE_W {
        SMCE_W { w: self }
    }
    #[doc = "Bit 2 - Change of Synchronization Mode Interrupt Enable"]
    #[inline(always)]
    pub fn csme(&mut self) -> CSME_W {
        CSME_W { w: self }
    }
    #[doc = "Bit 3 - Start of Gap Interrupt Enable"]
    #[inline(always)]
    pub fn soge(&mut self) -> SOGE_W {
        SOGE_W { w: self }
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Enable"]
    #[inline(always)]
    pub fn rtmie(&mut self) -> RTMIE_W {
        RTMIE_W { w: self }
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Interrupt Enable"]
    #[inline(always)]
    pub fn ttmie(&mut self) -> TTMIE_W {
        TTMIE_W { w: self }
    }
    #[doc = "Bit 6 - Stop Watch Event Interrupt Enable"]
    #[inline(always)]
    pub fn swee(&mut self) -> SWEE_W {
        SWEE_W { w: self }
    }
    #[doc = "Bit 7 - Global Time Wrap Interrupt Enable"]
    #[inline(always)]
    pub fn gtwe(&mut self) -> GTWE_W {
        GTWE_W { w: self }
    }
    #[doc = "Bit 8 - Global Time Discontinuity Interrupt Enable"]
    #[inline(always)]
    pub fn gtde(&mut self) -> GTDE_W {
        GTDE_W { w: self }
    }
    #[doc = "Bit 9 - Global Time Error Interrupt Enable"]
    #[inline(always)]
    pub fn gtee(&mut self) -> GTEE_W {
        GTEE_W { w: self }
    }
    #[doc = "Bit 10 - Tx Count Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn txue(&mut self) -> TXUE_W {
        TXUE_W { w: self }
    }
    #[doc = "Bit 11 - Tx Count Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txoe(&mut self) -> TXOE_W {
        TXOE_W { w: self }
    }
    #[doc = "Bit 12 - Scheduling Error 1 Interrupt Enable"]
    #[inline(always)]
    pub fn se1e(&mut self) -> SE1E_W {
        SE1E_W { w: self }
    }
    #[doc = "Bit 13 - Scheduling Error 2 Interrupt Enable"]
    #[inline(always)]
    pub fn se2e(&mut self) -> SE2E_W {
        SE2E_W { w: self }
    }
    #[doc = "Bit 14 - Change Error Level Interrupt Enable"]
    #[inline(always)]
    pub fn elce(&mut self) -> ELCE_W {
        ELCE_W { w: self }
    }
    #[doc = "Bit 15 - Initialization Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn iwtge(&mut self) -> IWTGE_W {
        IWTGE_W { w: self }
    }
    #[doc = "Bit 16 - Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn wte(&mut self) -> WTE_W {
        WTE_W { w: self }
    }
    #[doc = "Bit 17 - Application Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn awe(&mut self) -> AWE_W {
        AWE_W { w: self }
    }
    #[doc = "Bit 18 - Configuration Error Interrupt Enable"]
    #[inline(always)]
    pub fn cere(&mut self) -> CERE_W {
        CERE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttie](index.html) module"]
pub struct FDCAN_TTIE_SPEC;
impl crate::RegisterSpec for FDCAN_TTIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttie::R](R) reader structure"]
impl crate::Readable for FDCAN_TTIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ttie::W](W) writer structure"]
impl crate::Writable for FDCAN_TTIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_TTIE to value 0"]
impl crate::Resettable for FDCAN_TTIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
