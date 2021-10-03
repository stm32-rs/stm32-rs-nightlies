#[doc = "Register `TTIR` reader"]
pub struct R(crate::R<TTIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTIR` writer"]
pub struct W(crate::W<TTIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTIR_SPEC>;
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
impl From<crate::W<TTIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBC` reader - Start of Basic Cycle"]
pub struct SBC_R(crate::FieldReader<bool, bool>);
impl SBC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBC` writer - Start of Basic Cycle"]
pub struct SBC_W<'a> {
    w: &'a mut W,
}
impl<'a> SBC_W<'a> {
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
#[doc = "Field `SMC` reader - Start of Matrix Cycle"]
pub struct SMC_R(crate::FieldReader<bool, bool>);
impl SMC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMC` writer - Start of Matrix Cycle"]
pub struct SMC_W<'a> {
    w: &'a mut W,
}
impl<'a> SMC_W<'a> {
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
#[doc = "Field `CSM` reader - Change of Synchronization Mode"]
pub struct CSM_R(crate::FieldReader<bool, bool>);
impl CSM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSM` writer - Change of Synchronization Mode"]
pub struct CSM_W<'a> {
    w: &'a mut W,
}
impl<'a> CSM_W<'a> {
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
#[doc = "Field `SOG` reader - Start of Gap"]
pub struct SOG_R(crate::FieldReader<bool, bool>);
impl SOG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOG` writer - Start of Gap"]
pub struct SOG_W<'a> {
    w: &'a mut W,
}
impl<'a> SOG_W<'a> {
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
#[doc = "Field `RTMI` reader - Register Time Mark Interrupt."]
pub struct RTMI_R(crate::FieldReader<bool, bool>);
impl RTMI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTMI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTMI` writer - Register Time Mark Interrupt."]
pub struct RTMI_W<'a> {
    w: &'a mut W,
}
impl<'a> RTMI_W<'a> {
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
#[doc = "Field `TTMI` reader - Trigger Time Mark Event Internal"]
pub struct TTMI_R(crate::FieldReader<bool, bool>);
impl TTMI_R {
    pub(crate) fn new(bits: bool) -> Self {
        TTMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTMI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTMI` writer - Trigger Time Mark Event Internal"]
pub struct TTMI_W<'a> {
    w: &'a mut W,
}
impl<'a> TTMI_W<'a> {
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
#[doc = "Field `SWE` reader - Stop Watch Event"]
pub struct SWE_R(crate::FieldReader<bool, bool>);
impl SWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWE` writer - Stop Watch Event"]
pub struct SWE_W<'a> {
    w: &'a mut W,
}
impl<'a> SWE_W<'a> {
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
#[doc = "Field `GTW` reader - Global Time Wrap"]
pub struct GTW_R(crate::FieldReader<bool, bool>);
impl GTW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GTW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTW` writer - Global Time Wrap"]
pub struct GTW_W<'a> {
    w: &'a mut W,
}
impl<'a> GTW_W<'a> {
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
#[doc = "Field `GTD` reader - Global Time Discontinuity"]
pub struct GTD_R(crate::FieldReader<bool, bool>);
impl GTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GTD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTD` writer - Global Time Discontinuity"]
pub struct GTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GTD_W<'a> {
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
#[doc = "Field `GTE` reader - Global Time Error"]
pub struct GTE_R(crate::FieldReader<bool, bool>);
impl GTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTE` writer - Global Time Error"]
pub struct GTE_W<'a> {
    w: &'a mut W,
}
impl<'a> GTE_W<'a> {
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
#[doc = "Field `TXU` reader - Tx Count Underflow"]
pub struct TXU_R(crate::FieldReader<bool, bool>);
impl TXU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXU` writer - Tx Count Underflow"]
pub struct TXU_W<'a> {
    w: &'a mut W,
}
impl<'a> TXU_W<'a> {
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
#[doc = "Field `TXO` reader - Tx Count Overflow"]
pub struct TXO_R(crate::FieldReader<bool, bool>);
impl TXO_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXO` writer - Tx Count Overflow"]
pub struct TXO_W<'a> {
    w: &'a mut W,
}
impl<'a> TXO_W<'a> {
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
#[doc = "Field `SE1` reader - Scheduling Error 1"]
pub struct SE1_R(crate::FieldReader<bool, bool>);
impl SE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SE1` writer - Scheduling Error 1"]
pub struct SE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SE1_W<'a> {
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
#[doc = "Field `SE2` reader - Scheduling Error 2"]
pub struct SE2_R(crate::FieldReader<bool, bool>);
impl SE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SE2` writer - Scheduling Error 2"]
pub struct SE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SE2_W<'a> {
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
#[doc = "Field `ELC` reader - Error Level Changed."]
pub struct ELC_R(crate::FieldReader<bool, bool>);
impl ELC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ELC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ELC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ELC` writer - Error Level Changed."]
pub struct ELC_W<'a> {
    w: &'a mut W,
}
impl<'a> ELC_W<'a> {
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
#[doc = "Field `IWTG` reader - Initialization Watch Trigger"]
pub struct IWTG_R(crate::FieldReader<bool, bool>);
impl IWTG_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWTG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWTG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWTG` writer - Initialization Watch Trigger"]
pub struct IWTG_W<'a> {
    w: &'a mut W,
}
impl<'a> IWTG_W<'a> {
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
#[doc = "Field `WT` reader - Watch Trigger"]
pub struct WT_R(crate::FieldReader<bool, bool>);
impl WT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WT` writer - Watch Trigger"]
pub struct WT_W<'a> {
    w: &'a mut W,
}
impl<'a> WT_W<'a> {
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
#[doc = "Field `AW` reader - Application Watchdog"]
pub struct AW_R(crate::FieldReader<bool, bool>);
impl AW_R {
    pub(crate) fn new(bits: bool) -> Self {
        AW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AW` writer - Application Watchdog"]
pub struct AW_W<'a> {
    w: &'a mut W,
}
impl<'a> AW_W<'a> {
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
#[doc = "Field `CER` reader - Configuration Error"]
pub struct CER_R(crate::FieldReader<bool, bool>);
impl CER_R {
    pub(crate) fn new(bits: bool) -> Self {
        CER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CER` writer - Configuration Error"]
pub struct CER_W<'a> {
    w: &'a mut W,
}
impl<'a> CER_W<'a> {
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
    #[doc = "Bit 0 - Start of Basic Cycle"]
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle"]
    #[inline(always)]
    pub fn smc(&self) -> SMC_R {
        SMC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode"]
    #[inline(always)]
    pub fn csm(&self) -> CSM_R {
        CSM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of Gap"]
    #[inline(always)]
    pub fn sog(&self) -> SOG_R {
        SOG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt."]
    #[inline(always)]
    pub fn rtmi(&self) -> RTMI_R {
        RTMI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal"]
    #[inline(always)]
    pub fn ttmi(&self) -> TTMI_R {
        TTMI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stop Watch Event"]
    #[inline(always)]
    pub fn swe(&self) -> SWE_R {
        SWE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Global Time Wrap"]
    #[inline(always)]
    pub fn gtw(&self) -> GTW_R {
        GTW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Global Time Discontinuity"]
    #[inline(always)]
    pub fn gtd(&self) -> GTD_R {
        GTD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Global Time Error"]
    #[inline(always)]
    pub fn gte(&self) -> GTE_R {
        GTE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Tx Count Underflow"]
    #[inline(always)]
    pub fn txu(&self) -> TXU_R {
        TXU_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx Count Overflow"]
    #[inline(always)]
    pub fn txo(&self) -> TXO_R {
        TXO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Scheduling Error 1"]
    #[inline(always)]
    pub fn se1(&self) -> SE1_R {
        SE1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Scheduling Error 2"]
    #[inline(always)]
    pub fn se2(&self) -> SE2_R {
        SE2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Error Level Changed."]
    #[inline(always)]
    pub fn elc(&self) -> ELC_R {
        ELC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger"]
    #[inline(always)]
    pub fn iwtg(&self) -> IWTG_R {
        IWTG_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Watch Trigger"]
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Application Watchdog"]
    #[inline(always)]
    pub fn aw(&self) -> AW_R {
        AW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Configuration Error"]
    #[inline(always)]
    pub fn cer(&self) -> CER_R {
        CER_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of Basic Cycle"]
    #[inline(always)]
    pub fn sbc(&mut self) -> SBC_W {
        SBC_W { w: self }
    }
    #[doc = "Bit 1 - Start of Matrix Cycle"]
    #[inline(always)]
    pub fn smc(&mut self) -> SMC_W {
        SMC_W { w: self }
    }
    #[doc = "Bit 2 - Change of Synchronization Mode"]
    #[inline(always)]
    pub fn csm(&mut self) -> CSM_W {
        CSM_W { w: self }
    }
    #[doc = "Bit 3 - Start of Gap"]
    #[inline(always)]
    pub fn sog(&mut self) -> SOG_W {
        SOG_W { w: self }
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt."]
    #[inline(always)]
    pub fn rtmi(&mut self) -> RTMI_W {
        RTMI_W { w: self }
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal"]
    #[inline(always)]
    pub fn ttmi(&mut self) -> TTMI_W {
        TTMI_W { w: self }
    }
    #[doc = "Bit 6 - Stop Watch Event"]
    #[inline(always)]
    pub fn swe(&mut self) -> SWE_W {
        SWE_W { w: self }
    }
    #[doc = "Bit 7 - Global Time Wrap"]
    #[inline(always)]
    pub fn gtw(&mut self) -> GTW_W {
        GTW_W { w: self }
    }
    #[doc = "Bit 8 - Global Time Discontinuity"]
    #[inline(always)]
    pub fn gtd(&mut self) -> GTD_W {
        GTD_W { w: self }
    }
    #[doc = "Bit 9 - Global Time Error"]
    #[inline(always)]
    pub fn gte(&mut self) -> GTE_W {
        GTE_W { w: self }
    }
    #[doc = "Bit 10 - Tx Count Underflow"]
    #[inline(always)]
    pub fn txu(&mut self) -> TXU_W {
        TXU_W { w: self }
    }
    #[doc = "Bit 11 - Tx Count Overflow"]
    #[inline(always)]
    pub fn txo(&mut self) -> TXO_W {
        TXO_W { w: self }
    }
    #[doc = "Bit 12 - Scheduling Error 1"]
    #[inline(always)]
    pub fn se1(&mut self) -> SE1_W {
        SE1_W { w: self }
    }
    #[doc = "Bit 13 - Scheduling Error 2"]
    #[inline(always)]
    pub fn se2(&mut self) -> SE2_W {
        SE2_W { w: self }
    }
    #[doc = "Bit 14 - Error Level Changed."]
    #[inline(always)]
    pub fn elc(&mut self) -> ELC_W {
        ELC_W { w: self }
    }
    #[doc = "Bit 15 - Initialization Watch Trigger"]
    #[inline(always)]
    pub fn iwtg(&mut self) -> IWTG_W {
        IWTG_W { w: self }
    }
    #[doc = "Bit 16 - Watch Trigger"]
    #[inline(always)]
    pub fn wt(&mut self) -> WT_W {
        WT_W { w: self }
    }
    #[doc = "Bit 17 - Application Watchdog"]
    #[inline(always)]
    pub fn aw(&mut self) -> AW_W {
        AW_W { w: self }
    }
    #[doc = "Bit 18 - Configuration Error"]
    #[inline(always)]
    pub fn cer(&mut self) -> CER_W {
        CER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttir](index.html) module"]
pub struct TTIR_SPEC;
impl crate::RegisterSpec for TTIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttir::R](R) reader structure"]
impl crate::Readable for TTIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttir::W](W) writer structure"]
impl crate::Writable for TTIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TTIR to value 0"]
impl crate::Resettable for TTIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
