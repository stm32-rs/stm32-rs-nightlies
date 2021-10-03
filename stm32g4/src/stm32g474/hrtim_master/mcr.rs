#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRSTDMA` reader - Burst DMA Update"]
pub struct BRSTDMA_R(crate::FieldReader<u8, u8>);
impl BRSTDMA_R {
    pub(crate) fn new(bits: u8) -> Self {
        BRSTDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRSTDMA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRSTDMA` writer - Burst DMA Update"]
pub struct BRSTDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> BRSTDMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `MREPU` reader - Master Timer Repetition update"]
pub struct MREPU_R(crate::FieldReader<bool, bool>);
impl MREPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        MREPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MREPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MREPU` writer - Master Timer Repetition update"]
pub struct MREPU_W<'a> {
    w: &'a mut W,
}
impl<'a> MREPU_W<'a> {
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
#[doc = "Field `PREEN` reader - Preload enable"]
pub struct PREEN_R(crate::FieldReader<bool, bool>);
impl PREEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PREEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREEN` writer - Preload enable"]
pub struct PREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREEN_W<'a> {
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
#[doc = "Field `DACSYNC` reader - AC Synchronization"]
pub struct DACSYNC_R(crate::FieldReader<u8, u8>);
impl DACSYNC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DACSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACSYNC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACSYNC` writer - AC Synchronization"]
pub struct DACSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> DACSYNC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "Field `TFCEN` reader - Timer F counter enable"]
pub struct TFCEN_R(crate::FieldReader<bool, bool>);
impl TFCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFCEN` writer - Timer F counter enable"]
pub struct TFCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TFCEN_W<'a> {
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
#[doc = "Field `TECEN` reader - Timer E counter enable"]
pub struct TECEN_R(crate::FieldReader<bool, bool>);
impl TECEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TECEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TECEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TECEN` writer - Timer E counter enable"]
pub struct TECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TECEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `TDCEN` reader - Timer D counter enable"]
pub struct TDCEN_R(crate::FieldReader<bool, bool>);
impl TDCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDCEN` writer - Timer D counter enable"]
pub struct TDCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `TCCEN` reader - Timer C counter enable"]
pub struct TCCEN_R(crate::FieldReader<bool, bool>);
impl TCCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCCEN` writer - Timer C counter enable"]
pub struct TCCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `TBCEN` reader - Timer B counter enable"]
pub struct TBCEN_R(crate::FieldReader<bool, bool>);
impl TBCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBCEN` writer - Timer B counter enable"]
pub struct TBCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCEN_W<'a> {
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
#[doc = "Field `TACEN` reader - Timer A counter enable"]
pub struct TACEN_R(crate::FieldReader<bool, bool>);
impl TACEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TACEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TACEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TACEN` writer - Timer A counter enable"]
pub struct TACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TACEN_W<'a> {
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
#[doc = "Field `MCEN` reader - Master Counter enable"]
pub struct MCEN_R(crate::FieldReader<bool, bool>);
impl MCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCEN` writer - Master Counter enable"]
pub struct MCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEN_W<'a> {
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
#[doc = "Field `SYNC_SRC` reader - Synchronization source"]
pub struct SYNC_SRC_R(crate::FieldReader<u8, u8>);
impl SYNC_SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYNC_SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNC_SRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC_SRC` writer - Synchronization source"]
pub struct SYNC_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `SYNC_OUT` reader - Synchronization output"]
pub struct SYNC_OUT_R(crate::FieldReader<u8, u8>);
impl SYNC_OUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYNC_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNC_OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC_OUT` writer - Synchronization output"]
pub struct SYNC_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `SYNCSTRTM` reader - Synchronization Starts Master"]
pub struct SYNCSTRTM_R(crate::FieldReader<bool, bool>);
impl SYNCSTRTM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNCSTRTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCSTRTM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCSTRTM` writer - Synchronization Starts Master"]
pub struct SYNCSTRTM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCSTRTM_W<'a> {
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
#[doc = "Field `SYNCRSTM` reader - Synchronization Resets Master"]
pub struct SYNCRSTM_R(crate::FieldReader<bool, bool>);
impl SYNCRSTM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNCRSTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCRSTM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCRSTM` writer - Synchronization Resets Master"]
pub struct SYNCRSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCRSTM_W<'a> {
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
#[doc = "Field `SYNC_IN` reader - synchronization input"]
pub struct SYNC_IN_R(crate::FieldReader<u8, u8>);
impl SYNC_IN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYNC_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNC_IN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC_IN` writer - synchronization input"]
pub struct SYNC_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `INTLVD` reader - Interleaved mode"]
pub struct INTLVD_R(crate::FieldReader<u8, u8>);
impl INTLVD_R {
    pub(crate) fn new(bits: u8) -> Self {
        INTLVD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTLVD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTLVD` writer - Interleaved mode"]
pub struct INTLVD_W<'a> {
    w: &'a mut W,
}
impl<'a> INTLVD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `HALF` reader - Half mode enable"]
pub struct HALF_R(crate::FieldReader<bool, bool>);
impl HALF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HALF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALF` writer - Half mode enable"]
pub struct HALF_W<'a> {
    w: &'a mut W,
}
impl<'a> HALF_W<'a> {
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
#[doc = "Field `RETRIG` reader - Master Re-triggerable mode"]
pub struct RETRIG_R(crate::FieldReader<bool, bool>);
impl RETRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        RETRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETRIG` writer - Master Re-triggerable mode"]
pub struct RETRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRIG_W<'a> {
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
#[doc = "Field `CONT` reader - Master Continuous mode"]
pub struct CONT_R(crate::FieldReader<bool, bool>);
impl CONT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONT` writer - Master Continuous mode"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
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
#[doc = "Field `CK_PSC` reader - HRTIM Master Clock prescaler"]
pub struct CK_PSC_R(crate::FieldReader<u8, u8>);
impl CK_PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CK_PSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK_PSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK_PSC` writer - HRTIM Master Clock prescaler"]
pub struct CK_PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CK_PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Burst DMA Update"]
    #[inline(always)]
    pub fn brstdma(&self) -> BRSTDMA_R {
        BRSTDMA_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29 - Master Timer Repetition update"]
    #[inline(always)]
    pub fn mrepu(&self) -> MREPU_R {
        MREPU_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    pub fn preen(&self) -> PREEN_R {
        PREEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    pub fn dacsync(&self) -> DACSYNC_R {
        DACSYNC_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Timer F counter enable"]
    #[inline(always)]
    pub fn tfcen(&self) -> TFCEN_R {
        TFCEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Timer E counter enable"]
    #[inline(always)]
    pub fn tecen(&self) -> TECEN_R {
        TECEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Timer D counter enable"]
    #[inline(always)]
    pub fn tdcen(&self) -> TDCEN_R {
        TDCEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Timer C counter enable"]
    #[inline(always)]
    pub fn tccen(&self) -> TCCEN_R {
        TCCEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timer B counter enable"]
    #[inline(always)]
    pub fn tbcen(&self) -> TBCEN_R {
        TBCEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Timer A counter enable"]
    #[inline(always)]
    pub fn tacen(&self) -> TACEN_R {
        TACEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Master Counter enable"]
    #[inline(always)]
    pub fn mcen(&self) -> MCEN_R {
        MCEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Synchronization source"]
    #[inline(always)]
    pub fn sync_src(&self) -> SYNC_SRC_R {
        SYNC_SRC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Synchronization output"]
    #[inline(always)]
    pub fn sync_out(&self) -> SYNC_OUT_R {
        SYNC_OUT_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Synchronization Starts Master"]
    #[inline(always)]
    pub fn syncstrtm(&self) -> SYNCSTRTM_R {
        SYNCSTRTM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Synchronization Resets Master"]
    #[inline(always)]
    pub fn syncrstm(&self) -> SYNCRSTM_R {
        SYNCRSTM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - synchronization input"]
    #[inline(always)]
    pub fn sync_in(&self) -> SYNC_IN_R {
        SYNC_IN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Interleaved mode"]
    #[inline(always)]
    pub fn intlvd(&self) -> INTLVD_R {
        INTLVD_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master Re-triggerable mode"]
    #[inline(always)]
    pub fn retrig(&self) -> RETRIG_R {
        RETRIG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Master Continuous mode"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - HRTIM Master Clock prescaler"]
    #[inline(always)]
    pub fn ck_psc(&self) -> CK_PSC_R {
        CK_PSC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Burst DMA Update"]
    #[inline(always)]
    pub fn brstdma(&mut self) -> BRSTDMA_W {
        BRSTDMA_W { w: self }
    }
    #[doc = "Bit 29 - Master Timer Repetition update"]
    #[inline(always)]
    pub fn mrepu(&mut self) -> MREPU_W {
        MREPU_W { w: self }
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    pub fn preen(&mut self) -> PREEN_W {
        PREEN_W { w: self }
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    pub fn dacsync(&mut self) -> DACSYNC_W {
        DACSYNC_W { w: self }
    }
    #[doc = "Bit 22 - Timer F counter enable"]
    #[inline(always)]
    pub fn tfcen(&mut self) -> TFCEN_W {
        TFCEN_W { w: self }
    }
    #[doc = "Bit 21 - Timer E counter enable"]
    #[inline(always)]
    pub fn tecen(&mut self) -> TECEN_W {
        TECEN_W { w: self }
    }
    #[doc = "Bit 20 - Timer D counter enable"]
    #[inline(always)]
    pub fn tdcen(&mut self) -> TDCEN_W {
        TDCEN_W { w: self }
    }
    #[doc = "Bit 19 - Timer C counter enable"]
    #[inline(always)]
    pub fn tccen(&mut self) -> TCCEN_W {
        TCCEN_W { w: self }
    }
    #[doc = "Bit 18 - Timer B counter enable"]
    #[inline(always)]
    pub fn tbcen(&mut self) -> TBCEN_W {
        TBCEN_W { w: self }
    }
    #[doc = "Bit 17 - Timer A counter enable"]
    #[inline(always)]
    pub fn tacen(&mut self) -> TACEN_W {
        TACEN_W { w: self }
    }
    #[doc = "Bit 16 - Master Counter enable"]
    #[inline(always)]
    pub fn mcen(&mut self) -> MCEN_W {
        MCEN_W { w: self }
    }
    #[doc = "Bits 14:15 - Synchronization source"]
    #[inline(always)]
    pub fn sync_src(&mut self) -> SYNC_SRC_W {
        SYNC_SRC_W { w: self }
    }
    #[doc = "Bits 12:13 - Synchronization output"]
    #[inline(always)]
    pub fn sync_out(&mut self) -> SYNC_OUT_W {
        SYNC_OUT_W { w: self }
    }
    #[doc = "Bit 11 - Synchronization Starts Master"]
    #[inline(always)]
    pub fn syncstrtm(&mut self) -> SYNCSTRTM_W {
        SYNCSTRTM_W { w: self }
    }
    #[doc = "Bit 10 - Synchronization Resets Master"]
    #[inline(always)]
    pub fn syncrstm(&mut self) -> SYNCRSTM_W {
        SYNCRSTM_W { w: self }
    }
    #[doc = "Bits 8:9 - synchronization input"]
    #[inline(always)]
    pub fn sync_in(&mut self) -> SYNC_IN_W {
        SYNC_IN_W { w: self }
    }
    #[doc = "Bits 6:7 - Interleaved mode"]
    #[inline(always)]
    pub fn intlvd(&mut self) -> INTLVD_W {
        INTLVD_W { w: self }
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    pub fn half(&mut self) -> HALF_W {
        HALF_W { w: self }
    }
    #[doc = "Bit 4 - Master Re-triggerable mode"]
    #[inline(always)]
    pub fn retrig(&mut self) -> RETRIG_W {
        RETRIG_W { w: self }
    }
    #[doc = "Bit 3 - Master Continuous mode"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Bits 0:2 - HRTIM Master Clock prescaler"]
    #[inline(always)]
    pub fn ck_psc(&mut self) -> CK_PSC_W {
        CK_PSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
