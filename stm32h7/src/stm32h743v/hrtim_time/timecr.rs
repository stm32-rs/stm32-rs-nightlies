#[doc = "Register `TIMECR` reader"]
pub struct R(crate::R<TIMECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMECR` writer"]
pub struct W(crate::W<TIMECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMECR_SPEC>;
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
impl From<crate::W<TIMECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPDGAT` reader - Update Gating"]
pub struct UPDGAT_R(crate::FieldReader<u8, u8>);
impl UPDGAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        UPDGAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPDGAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDGAT` writer - Update Gating"]
pub struct UPDGAT_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDGAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
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
#[doc = "Field `MSTU` reader - Master Timer update"]
pub struct MSTU_R(crate::FieldReader<bool, bool>);
impl MSTU_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTU` writer - Master Timer update"]
pub struct MSTU_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTU_W<'a> {
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
#[doc = "Field `TEU` reader - TEU"]
pub struct TEU_R(crate::FieldReader<bool, bool>);
impl TEU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEU` writer - TEU"]
pub struct TEU_W<'a> {
    w: &'a mut W,
}
impl<'a> TEU_W<'a> {
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
#[doc = "Field `TDU` reader - TDU"]
pub struct TDU_R(crate::FieldReader<bool, bool>);
impl TDU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDU` writer - TDU"]
pub struct TDU_W<'a> {
    w: &'a mut W,
}
impl<'a> TDU_W<'a> {
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
#[doc = "Field `TCU` reader - TCU"]
pub struct TCU_R(crate::FieldReader<bool, bool>);
impl TCU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCU` writer - TCU"]
pub struct TCU_W<'a> {
    w: &'a mut W,
}
impl<'a> TCU_W<'a> {
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
#[doc = "Field `TBU` reader - TBU"]
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
#[doc = "Field `TBU` writer - TBU"]
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `TxRSTU` reader - Timerx reset update"]
pub struct TXRSTU_R(crate::FieldReader<bool, bool>);
impl TXRSTU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRSTU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRSTU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TxRSTU` writer - Timerx reset update"]
pub struct TXRSTU_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRSTU_W<'a> {
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
#[doc = "Field `TxREPU` reader - Timer x Repetition update"]
pub struct TXREPU_R(crate::FieldReader<bool, bool>);
impl TXREPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXREPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXREPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TxREPU` writer - Timer x Repetition update"]
pub struct TXREPU_W<'a> {
    w: &'a mut W,
}
impl<'a> TXREPU_W<'a> {
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
#[doc = "Field `DELCMP4` reader - Delayed CMP4 mode"]
pub struct DELCMP4_R(crate::FieldReader<u8, u8>);
impl DELCMP4_R {
    pub(crate) fn new(bits: u8) -> Self {
        DELCMP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELCMP4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DELCMP4` writer - Delayed CMP4 mode"]
pub struct DELCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> DELCMP4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `DELCMP2` reader - Delayed CMP2 mode"]
pub struct DELCMP2_R(crate::FieldReader<u8, u8>);
impl DELCMP2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DELCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELCMP2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DELCMP2` writer - Delayed CMP2 mode"]
pub struct DELCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> DELCMP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `SYNCSTRTx` reader - Synchronization Starts Timer x"]
pub struct SYNCSTRTX_R(crate::FieldReader<bool, bool>);
impl SYNCSTRTX_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNCSTRTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCSTRTX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCSTRTx` writer - Synchronization Starts Timer x"]
pub struct SYNCSTRTX_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCSTRTX_W<'a> {
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
#[doc = "Field `SYNCRSTx` reader - Synchronization Resets Timer x"]
pub struct SYNCRSTX_R(crate::FieldReader<bool, bool>);
impl SYNCRSTX_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNCRSTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCRSTX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCRSTx` writer - Synchronization Resets Timer x"]
pub struct SYNCRSTX_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCRSTX_W<'a> {
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
#[doc = "Field `PSHPLL` reader - Push-Pull mode enable"]
pub struct PSHPLL_R(crate::FieldReader<bool, bool>);
impl PSHPLL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSHPLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSHPLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSHPLL` writer - Push-Pull mode enable"]
pub struct PSHPLL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSHPLL_W<'a> {
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
#[doc = "Field `RETRIG` reader - Re-triggerable mode"]
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
#[doc = "Field `RETRIG` writer - Re-triggerable mode"]
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
#[doc = "Field `CONT` reader - Continuous mode"]
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
#[doc = "Field `CONT` writer - Continuous mode"]
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
#[doc = "Field `CK_PSCx` reader - HRTIM Timer x Clock prescaler"]
pub struct CK_PSCX_R(crate::FieldReader<u8, u8>);
impl CK_PSCX_R {
    pub(crate) fn new(bits: u8) -> Self {
        CK_PSCX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK_PSCX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK_PSCx` writer - HRTIM Timer x Clock prescaler"]
pub struct CK_PSCX_W<'a> {
    w: &'a mut W,
}
impl<'a> CK_PSCX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - Update Gating"]
    #[inline(always)]
    pub fn updgat(&self) -> UPDGAT_R {
        UPDGAT_R::new(((self.bits >> 28) & 0x0f) as u8)
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
    #[doc = "Bit 24 - Master Timer update"]
    #[inline(always)]
    pub fn mstu(&self) -> MSTU_R {
        MSTU_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TEU"]
    #[inline(always)]
    pub fn teu(&self) -> TEU_R {
        TEU_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TDU"]
    #[inline(always)]
    pub fn tdu(&self) -> TDU_R {
        TDU_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TCU"]
    #[inline(always)]
    pub fn tcu(&self) -> TCU_R {
        TCU_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TBU"]
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timerx reset update"]
    #[inline(always)]
    pub fn tx_rstu(&self) -> TXRSTU_R {
        TXRSTU_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Timer x Repetition update"]
    #[inline(always)]
    pub fn tx_repu(&self) -> TXREPU_R {
        TXREPU_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Delayed CMP4 mode"]
    #[inline(always)]
    pub fn delcmp4(&self) -> DELCMP4_R {
        DELCMP4_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Delayed CMP2 mode"]
    #[inline(always)]
    pub fn delcmp2(&self) -> DELCMP2_R {
        DELCMP2_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Synchronization Starts Timer x"]
    #[inline(always)]
    pub fn syncstrtx(&self) -> SYNCSTRTX_R {
        SYNCSTRTX_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Synchronization Resets Timer x"]
    #[inline(always)]
    pub fn syncrstx(&self) -> SYNCRSTX_R {
        SYNCRSTX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Push-Pull mode enable"]
    #[inline(always)]
    pub fn pshpll(&self) -> PSHPLL_R {
        PSHPLL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Re-triggerable mode"]
    #[inline(always)]
    pub fn retrig(&self) -> RETRIG_R {
        RETRIG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - HRTIM Timer x Clock prescaler"]
    #[inline(always)]
    pub fn ck_pscx(&self) -> CK_PSCX_R {
        CK_PSCX_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Update Gating"]
    #[inline(always)]
    pub fn updgat(&mut self) -> UPDGAT_W {
        UPDGAT_W { w: self }
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
    #[doc = "Bit 24 - Master Timer update"]
    #[inline(always)]
    pub fn mstu(&mut self) -> MSTU_W {
        MSTU_W { w: self }
    }
    #[doc = "Bit 23 - TEU"]
    #[inline(always)]
    pub fn teu(&mut self) -> TEU_W {
        TEU_W { w: self }
    }
    #[doc = "Bit 22 - TDU"]
    #[inline(always)]
    pub fn tdu(&mut self) -> TDU_W {
        TDU_W { w: self }
    }
    #[doc = "Bit 21 - TCU"]
    #[inline(always)]
    pub fn tcu(&mut self) -> TCU_W {
        TCU_W { w: self }
    }
    #[doc = "Bit 20 - TBU"]
    #[inline(always)]
    pub fn tbu(&mut self) -> TBU_W {
        TBU_W { w: self }
    }
    #[doc = "Bit 18 - Timerx reset update"]
    #[inline(always)]
    pub fn tx_rstu(&mut self) -> TXRSTU_W {
        TXRSTU_W { w: self }
    }
    #[doc = "Bit 17 - Timer x Repetition update"]
    #[inline(always)]
    pub fn tx_repu(&mut self) -> TXREPU_W {
        TXREPU_W { w: self }
    }
    #[doc = "Bits 14:15 - Delayed CMP4 mode"]
    #[inline(always)]
    pub fn delcmp4(&mut self) -> DELCMP4_W {
        DELCMP4_W { w: self }
    }
    #[doc = "Bits 12:13 - Delayed CMP2 mode"]
    #[inline(always)]
    pub fn delcmp2(&mut self) -> DELCMP2_W {
        DELCMP2_W { w: self }
    }
    #[doc = "Bit 11 - Synchronization Starts Timer x"]
    #[inline(always)]
    pub fn syncstrtx(&mut self) -> SYNCSTRTX_W {
        SYNCSTRTX_W { w: self }
    }
    #[doc = "Bit 10 - Synchronization Resets Timer x"]
    #[inline(always)]
    pub fn syncrstx(&mut self) -> SYNCRSTX_W {
        SYNCRSTX_W { w: self }
    }
    #[doc = "Bit 6 - Push-Pull mode enable"]
    #[inline(always)]
    pub fn pshpll(&mut self) -> PSHPLL_W {
        PSHPLL_W { w: self }
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    pub fn half(&mut self) -> HALF_W {
        HALF_W { w: self }
    }
    #[doc = "Bit 4 - Re-triggerable mode"]
    #[inline(always)]
    pub fn retrig(&mut self) -> RETRIG_W {
        RETRIG_W { w: self }
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Bits 0:2 - HRTIM Timer x Clock prescaler"]
    #[inline(always)]
    pub fn ck_pscx(&mut self) -> CK_PSCX_W {
        CK_PSCX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timecr](index.html) module"]
pub struct TIMECR_SPEC;
impl crate::RegisterSpec for TIMECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timecr::R](R) reader structure"]
impl crate::Readable for TIMECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timecr::W](W) writer structure"]
impl crate::Writable for TIMECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMECR to value 0"]
impl crate::Resettable for TIMECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
