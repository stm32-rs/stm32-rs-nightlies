#[doc = "Register `FDCAN_TTILS` reader"]
pub struct R(crate::R<FDCAN_TTILS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTILS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTILS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTILS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TTILS` writer"]
pub struct W(crate::W<FDCAN_TTILS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTILS_SPEC>;
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
impl From<crate::W<FDCAN_TTILS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTILS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBCL` reader - Start of Basic Cycle Interrupt Line"]
pub struct SBCL_R(crate::FieldReader<bool, bool>);
impl SBCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBCL` writer - Start of Basic Cycle Interrupt Line"]
pub struct SBCL_W<'a> {
    w: &'a mut W,
}
impl<'a> SBCL_W<'a> {
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
#[doc = "Field `SMCL` reader - Start of Matrix Cycle Interrupt Line"]
pub struct SMCL_R(crate::FieldReader<bool, bool>);
impl SMCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMCL` writer - Start of Matrix Cycle Interrupt Line"]
pub struct SMCL_W<'a> {
    w: &'a mut W,
}
impl<'a> SMCL_W<'a> {
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
#[doc = "Field `CSML` reader - Change of Synchronization Mode Interrupt Line"]
pub struct CSML_R(crate::FieldReader<bool, bool>);
impl CSML_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSML_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSML_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSML` writer - Change of Synchronization Mode Interrupt Line"]
pub struct CSML_W<'a> {
    w: &'a mut W,
}
impl<'a> CSML_W<'a> {
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
#[doc = "Field `SOGL` reader - Start of Gap Interrupt Line"]
pub struct SOGL_R(crate::FieldReader<bool, bool>);
impl SOGL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOGL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOGL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOGL` writer - Start of Gap Interrupt Line"]
pub struct SOGL_W<'a> {
    w: &'a mut W,
}
impl<'a> SOGL_W<'a> {
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
#[doc = "Field `RTMIL` reader - Register Time Mark Interrupt Line"]
pub struct RTMIL_R(crate::FieldReader<bool, bool>);
impl RTMIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTMIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTMIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTMIL` writer - Register Time Mark Interrupt Line"]
pub struct RTMIL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTMIL_W<'a> {
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
#[doc = "Field `TTMIL` reader - Trigger Time Mark Event Internal Interrupt Line"]
pub struct TTMIL_R(crate::FieldReader<bool, bool>);
impl TTMIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TTMIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTMIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTMIL` writer - Trigger Time Mark Event Internal Interrupt Line"]
pub struct TTMIL_W<'a> {
    w: &'a mut W,
}
impl<'a> TTMIL_W<'a> {
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
#[doc = "Field `SWEL` reader - Stop Watch Event Interrupt Line"]
pub struct SWEL_R(crate::FieldReader<bool, bool>);
impl SWEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWEL` writer - Stop Watch Event Interrupt Line"]
pub struct SWEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEL_W<'a> {
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
#[doc = "Field `GTWL` reader - Global Time Wrap Interrupt Line"]
pub struct GTWL_R(crate::FieldReader<bool, bool>);
impl GTWL_R {
    pub(crate) fn new(bits: bool) -> Self {
        GTWL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTWL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTWL` writer - Global Time Wrap Interrupt Line"]
pub struct GTWL_W<'a> {
    w: &'a mut W,
}
impl<'a> GTWL_W<'a> {
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
#[doc = "Field `GTDL` reader - Global Time Discontinuity Interrupt Line"]
pub struct GTDL_R(crate::FieldReader<bool, bool>);
impl GTDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        GTDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTDL` writer - Global Time Discontinuity Interrupt Line"]
pub struct GTDL_W<'a> {
    w: &'a mut W,
}
impl<'a> GTDL_W<'a> {
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
#[doc = "Field `GTEL` reader - Global Time Error Interrupt Line"]
pub struct GTEL_R(crate::FieldReader<bool, bool>);
impl GTEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        GTEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTEL` writer - Global Time Error Interrupt Line"]
pub struct GTEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GTEL_W<'a> {
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
#[doc = "Field `TXUL` reader - Tx Count Underflow Interrupt Line"]
pub struct TXUL_R(crate::FieldReader<bool, bool>);
impl TXUL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUL` writer - Tx Count Underflow Interrupt Line"]
pub struct TXUL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUL_W<'a> {
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
#[doc = "Field `TXOL` reader - Tx Count Overflow Interrupt Line"]
pub struct TXOL_R(crate::FieldReader<bool, bool>);
impl TXOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOL` writer - Tx Count Overflow Interrupt Line"]
pub struct TXOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOL_W<'a> {
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
#[doc = "Field `SE1L` reader - Scheduling Error 1 Interrupt Line"]
pub struct SE1L_R(crate::FieldReader<bool, bool>);
impl SE1L_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE1L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE1L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SE1L` writer - Scheduling Error 1 Interrupt Line"]
pub struct SE1L_W<'a> {
    w: &'a mut W,
}
impl<'a> SE1L_W<'a> {
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
#[doc = "Field `SE2L` reader - Scheduling Error 2 Interrupt Line"]
pub struct SE2L_R(crate::FieldReader<bool, bool>);
impl SE2L_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE2L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE2L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SE2L` writer - Scheduling Error 2 Interrupt Line"]
pub struct SE2L_W<'a> {
    w: &'a mut W,
}
impl<'a> SE2L_W<'a> {
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
#[doc = "Field `ELCL` reader - Change Error Level Interrupt Line"]
pub struct ELCL_R(crate::FieldReader<bool, bool>);
impl ELCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ELCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ELCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ELCL` writer - Change Error Level Interrupt Line"]
pub struct ELCL_W<'a> {
    w: &'a mut W,
}
impl<'a> ELCL_W<'a> {
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
#[doc = "Field `IWTGL` reader - Initialization Watch Trigger Interrupt Line"]
pub struct IWTGL_R(crate::FieldReader<bool, bool>);
impl IWTGL_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWTGL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWTGL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWTGL` writer - Initialization Watch Trigger Interrupt Line"]
pub struct IWTGL_W<'a> {
    w: &'a mut W,
}
impl<'a> IWTGL_W<'a> {
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
#[doc = "Field `WTL` reader - Watch Trigger Interrupt Line"]
pub struct WTL_R(crate::FieldReader<bool, bool>);
impl WTL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WTL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WTL` writer - Watch Trigger Interrupt Line"]
pub struct WTL_W<'a> {
    w: &'a mut W,
}
impl<'a> WTL_W<'a> {
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
#[doc = "Field `AWL` reader - Application Watchdog Interrupt Line"]
pub struct AWL_R(crate::FieldReader<bool, bool>);
impl AWL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWL` writer - Application Watchdog Interrupt Line"]
pub struct AWL_W<'a> {
    w: &'a mut W,
}
impl<'a> AWL_W<'a> {
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
#[doc = "Field `CERL` reader - Configuration Error Interrupt Line"]
pub struct CERL_R(crate::FieldReader<bool, bool>);
impl CERL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CERL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CERL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CERL` writer - Configuration Error Interrupt Line"]
pub struct CERL_W<'a> {
    w: &'a mut W,
}
impl<'a> CERL_W<'a> {
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
    #[doc = "Bit 0 - Start of Basic Cycle Interrupt Line"]
    #[inline(always)]
    pub fn sbcl(&self) -> SBCL_R {
        SBCL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle Interrupt Line"]
    #[inline(always)]
    pub fn smcl(&self) -> SMCL_R {
        SMCL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode Interrupt Line"]
    #[inline(always)]
    pub fn csml(&self) -> CSML_R {
        CSML_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of Gap Interrupt Line"]
    #[inline(always)]
    pub fn sogl(&self) -> SOGL_R {
        SOGL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Line"]
    #[inline(always)]
    pub fn rtmil(&self) -> RTMIL_R {
        RTMIL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Interrupt Line"]
    #[inline(always)]
    pub fn ttmil(&self) -> TTMIL_R {
        TTMIL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stop Watch Event Interrupt Line"]
    #[inline(always)]
    pub fn swel(&self) -> SWEL_R {
        SWEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Global Time Wrap Interrupt Line"]
    #[inline(always)]
    pub fn gtwl(&self) -> GTWL_R {
        GTWL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Global Time Discontinuity Interrupt Line"]
    #[inline(always)]
    pub fn gtdl(&self) -> GTDL_R {
        GTDL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Global Time Error Interrupt Line"]
    #[inline(always)]
    pub fn gtel(&self) -> GTEL_R {
        GTEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Tx Count Underflow Interrupt Line"]
    #[inline(always)]
    pub fn txul(&self) -> TXUL_R {
        TXUL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx Count Overflow Interrupt Line"]
    #[inline(always)]
    pub fn txol(&self) -> TXOL_R {
        TXOL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Scheduling Error 1 Interrupt Line"]
    #[inline(always)]
    pub fn se1l(&self) -> SE1L_R {
        SE1L_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Scheduling Error 2 Interrupt Line"]
    #[inline(always)]
    pub fn se2l(&self) -> SE2L_R {
        SE2L_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Change Error Level Interrupt Line"]
    #[inline(always)]
    pub fn elcl(&self) -> ELCL_R {
        ELCL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger Interrupt Line"]
    #[inline(always)]
    pub fn iwtgl(&self) -> IWTGL_R {
        IWTGL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Watch Trigger Interrupt Line"]
    #[inline(always)]
    pub fn wtl(&self) -> WTL_R {
        WTL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Application Watchdog Interrupt Line"]
    #[inline(always)]
    pub fn awl(&self) -> AWL_R {
        AWL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Configuration Error Interrupt Line"]
    #[inline(always)]
    pub fn cerl(&self) -> CERL_R {
        CERL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of Basic Cycle Interrupt Line"]
    #[inline(always)]
    pub fn sbcl(&mut self) -> SBCL_W {
        SBCL_W { w: self }
    }
    #[doc = "Bit 1 - Start of Matrix Cycle Interrupt Line"]
    #[inline(always)]
    pub fn smcl(&mut self) -> SMCL_W {
        SMCL_W { w: self }
    }
    #[doc = "Bit 2 - Change of Synchronization Mode Interrupt Line"]
    #[inline(always)]
    pub fn csml(&mut self) -> CSML_W {
        CSML_W { w: self }
    }
    #[doc = "Bit 3 - Start of Gap Interrupt Line"]
    #[inline(always)]
    pub fn sogl(&mut self) -> SOGL_W {
        SOGL_W { w: self }
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Line"]
    #[inline(always)]
    pub fn rtmil(&mut self) -> RTMIL_W {
        RTMIL_W { w: self }
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Interrupt Line"]
    #[inline(always)]
    pub fn ttmil(&mut self) -> TTMIL_W {
        TTMIL_W { w: self }
    }
    #[doc = "Bit 6 - Stop Watch Event Interrupt Line"]
    #[inline(always)]
    pub fn swel(&mut self) -> SWEL_W {
        SWEL_W { w: self }
    }
    #[doc = "Bit 7 - Global Time Wrap Interrupt Line"]
    #[inline(always)]
    pub fn gtwl(&mut self) -> GTWL_W {
        GTWL_W { w: self }
    }
    #[doc = "Bit 8 - Global Time Discontinuity Interrupt Line"]
    #[inline(always)]
    pub fn gtdl(&mut self) -> GTDL_W {
        GTDL_W { w: self }
    }
    #[doc = "Bit 9 - Global Time Error Interrupt Line"]
    #[inline(always)]
    pub fn gtel(&mut self) -> GTEL_W {
        GTEL_W { w: self }
    }
    #[doc = "Bit 10 - Tx Count Underflow Interrupt Line"]
    #[inline(always)]
    pub fn txul(&mut self) -> TXUL_W {
        TXUL_W { w: self }
    }
    #[doc = "Bit 11 - Tx Count Overflow Interrupt Line"]
    #[inline(always)]
    pub fn txol(&mut self) -> TXOL_W {
        TXOL_W { w: self }
    }
    #[doc = "Bit 12 - Scheduling Error 1 Interrupt Line"]
    #[inline(always)]
    pub fn se1l(&mut self) -> SE1L_W {
        SE1L_W { w: self }
    }
    #[doc = "Bit 13 - Scheduling Error 2 Interrupt Line"]
    #[inline(always)]
    pub fn se2l(&mut self) -> SE2L_W {
        SE2L_W { w: self }
    }
    #[doc = "Bit 14 - Change Error Level Interrupt Line"]
    #[inline(always)]
    pub fn elcl(&mut self) -> ELCL_W {
        ELCL_W { w: self }
    }
    #[doc = "Bit 15 - Initialization Watch Trigger Interrupt Line"]
    #[inline(always)]
    pub fn iwtgl(&mut self) -> IWTGL_W {
        IWTGL_W { w: self }
    }
    #[doc = "Bit 16 - Watch Trigger Interrupt Line"]
    #[inline(always)]
    pub fn wtl(&mut self) -> WTL_W {
        WTL_W { w: self }
    }
    #[doc = "Bit 17 - Application Watchdog Interrupt Line"]
    #[inline(always)]
    pub fn awl(&mut self) -> AWL_W {
        AWL_W { w: self }
    }
    #[doc = "Bit 18 - Configuration Error Interrupt Line"]
    #[inline(always)]
    pub fn cerl(&mut self) -> CERL_W {
        CERL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Interrupt Line Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttils](index.html) module"]
pub struct FDCAN_TTILS_SPEC;
impl crate::RegisterSpec for FDCAN_TTILS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttils::R](R) reader structure"]
impl crate::Readable for FDCAN_TTILS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ttils::W](W) writer structure"]
impl crate::Writable for FDCAN_TTILS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_TTILS to value 0"]
impl crate::Resettable for FDCAN_TTILS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
