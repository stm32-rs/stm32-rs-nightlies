#[doc = "Register `USBPHYC_TUNE2` reader"]
pub struct R(crate::R<USBPHYC_TUNE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPHYC_TUNE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPHYC_TUNE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPHYC_TUNE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPHYC_TUNE2` writer"]
pub struct W(crate::W<USBPHYC_TUNE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPHYC_TUNE2_SPEC>;
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
impl From<crate::W<USBPHYC_TUNE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPHYC_TUNE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INCURREN` reader - INCURREN"]
pub struct INCURREN_R(crate::FieldReader<bool, bool>);
impl INCURREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INCURREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INCURREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INCURREN` writer - INCURREN"]
pub struct INCURREN_W<'a> {
    w: &'a mut W,
}
impl<'a> INCURREN_W<'a> {
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
#[doc = "Field `INCURRINT` reader - INCURRINT"]
pub struct INCURRINT_R(crate::FieldReader<bool, bool>);
impl INCURRINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INCURRINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INCURRINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INCURRINT` writer - INCURRINT"]
pub struct INCURRINT_W<'a> {
    w: &'a mut W,
}
impl<'a> INCURRINT_W<'a> {
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
#[doc = "Field `LFSCAPEN` reader - LFSCAPEN"]
pub struct LFSCAPEN_R(crate::FieldReader<bool, bool>);
impl LFSCAPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFSCAPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFSCAPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFSCAPEN` writer - LFSCAPEN"]
pub struct LFSCAPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSCAPEN_W<'a> {
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
#[doc = "Field `HSDRVSLEW` reader - HSDRVSLEW"]
pub struct HSDRVSLEW_R(crate::FieldReader<bool, bool>);
impl HSDRVSLEW_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSDRVSLEW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSDRVSLEW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSDRVSLEW` writer - HSDRVSLEW"]
pub struct HSDRVSLEW_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDRVSLEW_W<'a> {
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
#[doc = "Field `HSDRVDCCUR` reader - HSDRVDCCUR"]
pub struct HSDRVDCCUR_R(crate::FieldReader<bool, bool>);
impl HSDRVDCCUR_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSDRVDCCUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSDRVDCCUR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSDRVDCCUR` writer - HSDRVDCCUR"]
pub struct HSDRVDCCUR_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDRVDCCUR_W<'a> {
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
#[doc = "Field `HSDRVDCLEV` reader - HSDRVDCLEV"]
pub struct HSDRVDCLEV_R(crate::FieldReader<bool, bool>);
impl HSDRVDCLEV_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSDRVDCLEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSDRVDCLEV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSDRVDCLEV` writer - HSDRVDCLEV"]
pub struct HSDRVDCLEV_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDRVDCLEV_W<'a> {
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
#[doc = "Field `HSDRVCURINCR` reader - HSDRVCURINCR"]
pub struct HSDRVCURINCR_R(crate::FieldReader<bool, bool>);
impl HSDRVCURINCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSDRVCURINCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSDRVCURINCR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSDRVCURINCR` writer - HSDRVCURINCR"]
pub struct HSDRVCURINCR_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDRVCURINCR_W<'a> {
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
#[doc = "Field `FSDRVRFADJ` reader - FSDRVRFADJ"]
pub struct FSDRVRFADJ_R(crate::FieldReader<bool, bool>);
impl FSDRVRFADJ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSDRVRFADJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSDRVRFADJ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSDRVRFADJ` writer - FSDRVRFADJ"]
pub struct FSDRVRFADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> FSDRVRFADJ_W<'a> {
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
#[doc = "Field `HSDRVRFRED` reader - HSDRVRFRED"]
pub struct HSDRVRFRED_R(crate::FieldReader<bool, bool>);
impl HSDRVRFRED_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSDRVRFRED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSDRVRFRED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSDRVRFRED` writer - HSDRVRFRED"]
pub struct HSDRVRFRED_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDRVRFRED_W<'a> {
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
#[doc = "Field `HSDRVCHKITRM` reader - HSDRVCHKITRM"]
pub struct HSDRVCHKITRM_R(crate::FieldReader<u8, u8>);
impl HSDRVCHKITRM_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSDRVCHKITRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSDRVCHKITRM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSDRVCHKITRM` writer - HSDRVCHKITRM"]
pub struct HSDRVCHKITRM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDRVCHKITRM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | ((value as u32 & 0x0f) << 9);
        self.w
    }
}
#[doc = "Field `HSDRVCHKZTRM` reader - HSDRVCHKZTRM"]
pub struct HSDRVCHKZTRM_R(crate::FieldReader<u8, u8>);
impl HSDRVCHKZTRM_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSDRVCHKZTRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSDRVCHKZTRM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSDRVCHKZTRM` writer - HSDRVCHKZTRM"]
pub struct HSDRVCHKZTRM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDRVCHKZTRM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Field `OTPCOMP` reader - OTPCOMP"]
pub struct OTPCOMP_R(crate::FieldReader<u8, u8>);
impl OTPCOMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        OTPCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPCOMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPCOMP` writer - OTPCOMP"]
pub struct OTPCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPCOMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | ((value as u32 & 0x1f) << 15);
        self.w
    }
}
#[doc = "Field `SQLCHCTL` reader - SQLCHCTL"]
pub struct SQLCHCTL_R(crate::FieldReader<u8, u8>);
impl SQLCHCTL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQLCHCTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQLCHCTL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQLCHCTL` writer - SQLCHCTL"]
pub struct SQLCHCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> SQLCHCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `HDRXGNEQEN` reader - HDRXGNEQEN"]
pub struct HDRXGNEQEN_R(crate::FieldReader<bool, bool>);
impl HDRXGNEQEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDRXGNEQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDRXGNEQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDRXGNEQEN` writer - HDRXGNEQEN"]
pub struct HDRXGNEQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HDRXGNEQEN_W<'a> {
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
#[doc = "Field `HSRXOFF` reader - HSRXOFF"]
pub struct HSRXOFF_R(crate::FieldReader<u8, u8>);
impl HSRXOFF_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSRXOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSRXOFF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSRXOFF` writer - HSRXOFF"]
pub struct HSRXOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSRXOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | ((value as u32 & 0x03) << 23);
        self.w
    }
}
#[doc = "Field `HSFALLPREEM` reader - HSFALLPREEM"]
pub struct HSFALLPREEM_R(crate::FieldReader<bool, bool>);
impl HSFALLPREEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSFALLPREEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSFALLPREEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSFALLPREEM` writer - HSFALLPREEM"]
pub struct HSFALLPREEM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSFALLPREEM_W<'a> {
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
#[doc = "Field `SHTCCTCTLPROT` reader - SHTCCTCTLPROT"]
pub struct SHTCCTCTLPROT_R(crate::FieldReader<bool, bool>);
impl SHTCCTCTLPROT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHTCCTCTLPROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHTCCTCTLPROT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHTCCTCTLPROT` writer - SHTCCTCTLPROT"]
pub struct SHTCCTCTLPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHTCCTCTLPROT_W<'a> {
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
#[doc = "Field `STAGSEL` reader - STAGSEL"]
pub struct STAGSEL_R(crate::FieldReader<bool, bool>);
impl STAGSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        STAGSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STAGSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STAGSEL` writer - STAGSEL"]
pub struct STAGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STAGSEL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - INCURREN"]
    #[inline(always)]
    pub fn incurren(&self) -> INCURREN_R {
        INCURREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - INCURRINT"]
    #[inline(always)]
    pub fn incurrint(&self) -> INCURRINT_R {
        INCURRINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LFSCAPEN"]
    #[inline(always)]
    pub fn lfscapen(&self) -> LFSCAPEN_R {
        LFSCAPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSDRVSLEW"]
    #[inline(always)]
    pub fn hsdrvslew(&self) -> HSDRVSLEW_R {
        HSDRVSLEW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HSDRVDCCUR"]
    #[inline(always)]
    pub fn hsdrvdccur(&self) -> HSDRVDCCUR_R {
        HSDRVDCCUR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HSDRVDCLEV"]
    #[inline(always)]
    pub fn hsdrvdclev(&self) -> HSDRVDCLEV_R {
        HSDRVDCLEV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HSDRVCURINCR"]
    #[inline(always)]
    pub fn hsdrvcurincr(&self) -> HSDRVCURINCR_R {
        HSDRVCURINCR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FSDRVRFADJ"]
    #[inline(always)]
    pub fn fsdrvrfadj(&self) -> FSDRVRFADJ_R {
        FSDRVRFADJ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HSDRVRFRED"]
    #[inline(always)]
    pub fn hsdrvrfred(&self) -> HSDRVRFRED_R {
        HSDRVRFRED_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:12 - HSDRVCHKITRM"]
    #[inline(always)]
    pub fn hsdrvchkitrm(&self) -> HSDRVCHKITRM_R {
        HSDRVCHKITRM_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:14 - HSDRVCHKZTRM"]
    #[inline(always)]
    pub fn hsdrvchkztrm(&self) -> HSDRVCHKZTRM_R {
        HSDRVCHKZTRM_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 15:19 - OTPCOMP"]
    #[inline(always)]
    pub fn otpcomp(&self) -> OTPCOMP_R {
        OTPCOMP_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:21 - SQLCHCTL"]
    #[inline(always)]
    pub fn sqlchctl(&self) -> SQLCHCTL_R {
        SQLCHCTL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - HDRXGNEQEN"]
    #[inline(always)]
    pub fn hdrxgneqen(&self) -> HDRXGNEQEN_R {
        HDRXGNEQEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 23:24 - HSRXOFF"]
    #[inline(always)]
    pub fn hsrxoff(&self) -> HSRXOFF_R {
        HSRXOFF_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bit 25 - HSFALLPREEM"]
    #[inline(always)]
    pub fn hsfallpreem(&self) -> HSFALLPREEM_R {
        HSFALLPREEM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SHTCCTCTLPROT"]
    #[inline(always)]
    pub fn shtcctctlprot(&self) -> SHTCCTCTLPROT_R {
        SHTCCTCTLPROT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - STAGSEL"]
    #[inline(always)]
    pub fn stagsel(&self) -> STAGSEL_R {
        STAGSEL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INCURREN"]
    #[inline(always)]
    pub fn incurren(&mut self) -> INCURREN_W {
        INCURREN_W { w: self }
    }
    #[doc = "Bit 1 - INCURRINT"]
    #[inline(always)]
    pub fn incurrint(&mut self) -> INCURRINT_W {
        INCURRINT_W { w: self }
    }
    #[doc = "Bit 2 - LFSCAPEN"]
    #[inline(always)]
    pub fn lfscapen(&mut self) -> LFSCAPEN_W {
        LFSCAPEN_W { w: self }
    }
    #[doc = "Bit 3 - HSDRVSLEW"]
    #[inline(always)]
    pub fn hsdrvslew(&mut self) -> HSDRVSLEW_W {
        HSDRVSLEW_W { w: self }
    }
    #[doc = "Bit 4 - HSDRVDCCUR"]
    #[inline(always)]
    pub fn hsdrvdccur(&mut self) -> HSDRVDCCUR_W {
        HSDRVDCCUR_W { w: self }
    }
    #[doc = "Bit 5 - HSDRVDCLEV"]
    #[inline(always)]
    pub fn hsdrvdclev(&mut self) -> HSDRVDCLEV_W {
        HSDRVDCLEV_W { w: self }
    }
    #[doc = "Bit 6 - HSDRVCURINCR"]
    #[inline(always)]
    pub fn hsdrvcurincr(&mut self) -> HSDRVCURINCR_W {
        HSDRVCURINCR_W { w: self }
    }
    #[doc = "Bit 7 - FSDRVRFADJ"]
    #[inline(always)]
    pub fn fsdrvrfadj(&mut self) -> FSDRVRFADJ_W {
        FSDRVRFADJ_W { w: self }
    }
    #[doc = "Bit 8 - HSDRVRFRED"]
    #[inline(always)]
    pub fn hsdrvrfred(&mut self) -> HSDRVRFRED_W {
        HSDRVRFRED_W { w: self }
    }
    #[doc = "Bits 9:12 - HSDRVCHKITRM"]
    #[inline(always)]
    pub fn hsdrvchkitrm(&mut self) -> HSDRVCHKITRM_W {
        HSDRVCHKITRM_W { w: self }
    }
    #[doc = "Bits 13:14 - HSDRVCHKZTRM"]
    #[inline(always)]
    pub fn hsdrvchkztrm(&mut self) -> HSDRVCHKZTRM_W {
        HSDRVCHKZTRM_W { w: self }
    }
    #[doc = "Bits 15:19 - OTPCOMP"]
    #[inline(always)]
    pub fn otpcomp(&mut self) -> OTPCOMP_W {
        OTPCOMP_W { w: self }
    }
    #[doc = "Bits 20:21 - SQLCHCTL"]
    #[inline(always)]
    pub fn sqlchctl(&mut self) -> SQLCHCTL_W {
        SQLCHCTL_W { w: self }
    }
    #[doc = "Bit 22 - HDRXGNEQEN"]
    #[inline(always)]
    pub fn hdrxgneqen(&mut self) -> HDRXGNEQEN_W {
        HDRXGNEQEN_W { w: self }
    }
    #[doc = "Bits 23:24 - HSRXOFF"]
    #[inline(always)]
    pub fn hsrxoff(&mut self) -> HSRXOFF_W {
        HSRXOFF_W { w: self }
    }
    #[doc = "Bit 25 - HSFALLPREEM"]
    #[inline(always)]
    pub fn hsfallpreem(&mut self) -> HSFALLPREEM_W {
        HSFALLPREEM_W { w: self }
    }
    #[doc = "Bit 26 - SHTCCTCTLPROT"]
    #[inline(always)]
    pub fn shtcctctlprot(&mut self) -> SHTCCTCTLPROT_W {
        SHTCCTCTLPROT_W { w: self }
    }
    #[doc = "Bit 27 - STAGSEL"]
    #[inline(always)]
    pub fn stagsel(&mut self) -> STAGSEL_W {
        STAGSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the tune interface of the HS PHY, port #x.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphyc_tune2](index.html) module"]
pub struct USBPHYC_TUNE2_SPEC;
impl crate::RegisterSpec for USBPHYC_TUNE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbphyc_tune2::R](R) reader structure"]
impl crate::Readable for USBPHYC_TUNE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbphyc_tune2::W](W) writer structure"]
impl crate::Writable for USBPHYC_TUNE2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBPHYC_TUNE2 to value 0x0407_0004"]
impl crate::Resettable for USBPHYC_TUNE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0407_0004
    }
}
