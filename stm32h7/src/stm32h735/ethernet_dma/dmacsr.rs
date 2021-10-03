#[doc = "Register `DMACSR` reader"]
pub struct R(crate::R<DMACSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACSR` writer"]
pub struct W(crate::W<DMACSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACSR_SPEC>;
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
impl From<crate::W<DMACSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REB` reader - Rx DMA Error Bits"]
pub struct REB_R(crate::FieldReader<u8, u8>);
impl REB_R {
    pub(crate) fn new(bits: u8) -> Self {
        REB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REB` writer - Rx DMA Error Bits"]
pub struct REB_W<'a> {
    w: &'a mut W,
}
impl<'a> REB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Field `TEB` reader - Tx DMA Error Bits"]
pub struct TEB_R(crate::FieldReader<u8, u8>);
impl TEB_R {
    pub(crate) fn new(bits: u8) -> Self {
        TEB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEB` writer - Tx DMA Error Bits"]
pub struct TEB_W<'a> {
    w: &'a mut W,
}
impl<'a> TEB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `NIS` reader - Normal Interrupt Summary"]
pub struct NIS_R(crate::FieldReader<bool, bool>);
impl NIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        NIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NIS` writer - Normal Interrupt Summary"]
pub struct NIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NIS_W<'a> {
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
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary"]
pub struct AIS_R(crate::FieldReader<bool, bool>);
impl AIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary"]
pub struct AIS_W<'a> {
    w: &'a mut W,
}
impl<'a> AIS_W<'a> {
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
#[doc = "Field `CDE` reader - Context Descriptor Error"]
pub struct CDE_R(crate::FieldReader<bool, bool>);
impl CDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDE` writer - Context Descriptor Error"]
pub struct CDE_W<'a> {
    w: &'a mut W,
}
impl<'a> CDE_W<'a> {
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
#[doc = "Field `FBE` reader - Fatal Bus Error"]
pub struct FBE_R(crate::FieldReader<bool, bool>);
impl FBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBE` writer - Fatal Bus Error"]
pub struct FBE_W<'a> {
    w: &'a mut W,
}
impl<'a> FBE_W<'a> {
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
#[doc = "Field `ERI` reader - Early Receive Interrupt"]
pub struct ERI_R(crate::FieldReader<bool, bool>);
impl ERI_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERI` writer - Early Receive Interrupt"]
pub struct ERI_W<'a> {
    w: &'a mut W,
}
impl<'a> ERI_W<'a> {
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
#[doc = "Field `ETI` reader - Early Transmit Interrupt"]
pub struct ETI_R(crate::FieldReader<bool, bool>);
impl ETI_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETI` writer - Early Transmit Interrupt"]
pub struct ETI_W<'a> {
    w: &'a mut W,
}
impl<'a> ETI_W<'a> {
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
#[doc = "Field `RWT` reader - Receive Watchdog Timeout"]
pub struct RWT_R(crate::FieldReader<bool, bool>);
impl RWT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWT` writer - Receive Watchdog Timeout"]
pub struct RWT_W<'a> {
    w: &'a mut W,
}
impl<'a> RWT_W<'a> {
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
#[doc = "Field `RPS` reader - Receive Process Stopped"]
pub struct RPS_R(crate::FieldReader<bool, bool>);
impl RPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPS` writer - Receive Process Stopped"]
pub struct RPS_W<'a> {
    w: &'a mut W,
}
impl<'a> RPS_W<'a> {
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
#[doc = "Field `RBU` reader - Receive Buffer Unavailable"]
pub struct RBU_R(crate::FieldReader<bool, bool>);
impl RBU_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBU` writer - Receive Buffer Unavailable"]
pub struct RBU_W<'a> {
    w: &'a mut W,
}
impl<'a> RBU_W<'a> {
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
#[doc = "Field `RI` reader - Receive Interrupt"]
pub struct RI_R(crate::FieldReader<bool, bool>);
impl RI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RI` writer - Receive Interrupt"]
pub struct RI_W<'a> {
    w: &'a mut W,
}
impl<'a> RI_W<'a> {
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
#[doc = "Field `TBU` reader - Transmit Buffer Unavailable"]
pub struct TBU_R(crate::FieldReader<bool, bool>);
impl TBU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBU` writer - Transmit Buffer Unavailable"]
pub struct TBU_W<'a> {
    w: &'a mut W,
}
impl<'a> TBU_W<'a> {
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
#[doc = "Field `TPS` reader - Transmit Process Stopped"]
pub struct TPS_R(crate::FieldReader<bool, bool>);
impl TPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPS` writer - Transmit Process Stopped"]
pub struct TPS_W<'a> {
    w: &'a mut W,
}
impl<'a> TPS_W<'a> {
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
#[doc = "Field `TI` reader - Transmit Interrupt"]
pub struct TI_R(crate::FieldReader<bool, bool>);
impl TI_R {
    pub(crate) fn new(bits: bool) -> Self {
        TI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI` writer - Transmit Interrupt"]
pub struct TI_W<'a> {
    w: &'a mut W,
}
impl<'a> TI_W<'a> {
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
impl R {
    #[doc = "Bits 19:21 - Rx DMA Error Bits"]
    #[inline(always)]
    pub fn reb(&self) -> REB_R {
        REB_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Tx DMA Error Bits"]
    #[inline(always)]
    pub fn teb(&self) -> TEB_R {
        TEB_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Context Descriptor Error"]
    #[inline(always)]
    pub fn cde(&self) -> CDE_R {
        CDE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fatal Bus Error"]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn eri(&self) -> ERI_R {
        ERI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn eti(&self) -> ETI_R {
        ETI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn rbu(&self) -> RBU_R {
        RBU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 19:21 - Rx DMA Error Bits"]
    #[inline(always)]
    pub fn reb(&mut self) -> REB_W {
        REB_W { w: self }
    }
    #[doc = "Bits 16:18 - Tx DMA Error Bits"]
    #[inline(always)]
    pub fn teb(&mut self) -> TEB_W {
        TEB_W { w: self }
    }
    #[doc = "Bit 15 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&mut self) -> NIS_W {
        NIS_W { w: self }
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W {
        AIS_W { w: self }
    }
    #[doc = "Bit 13 - Context Descriptor Error"]
    #[inline(always)]
    pub fn cde(&mut self) -> CDE_W {
        CDE_W { w: self }
    }
    #[doc = "Bit 12 - Fatal Bus Error"]
    #[inline(always)]
    pub fn fbe(&mut self) -> FBE_W {
        FBE_W { w: self }
    }
    #[doc = "Bit 11 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn eri(&mut self) -> ERI_W {
        ERI_W { w: self }
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn eti(&mut self) -> ETI_W {
        ETI_W { w: self }
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&mut self) -> RWT_W {
        RWT_W { w: self }
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn rps(&mut self) -> RPS_W {
        RPS_W { w: self }
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn rbu(&mut self) -> RBU_W {
        RBU_W { w: self }
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn ri(&mut self) -> RI_W {
        RI_W { w: self }
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn tbu(&mut self) -> TBU_W {
        TBU_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn tps(&mut self) -> TPS_W {
        TPS_W { w: self }
    }
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn ti(&mut self) -> TI_W {
        TI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacsr](index.html) module"]
pub struct DMACSR_SPEC;
impl crate::RegisterSpec for DMACSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacsr::R](R) reader structure"]
impl crate::Readable for DMACSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacsr::W](W) writer structure"]
impl crate::Writable for DMACSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACSR to value 0"]
impl crate::Resettable for DMACSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
