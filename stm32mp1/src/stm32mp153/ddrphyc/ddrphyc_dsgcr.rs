#[doc = "Register `DDRPHYC_DSGCR` reader"]
pub struct R(crate::R<DDRPHYC_DSGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DSGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DSGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DSGCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DSGCR` writer"]
pub struct W(crate::W<DDRPHYC_DSGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DSGCR_SPEC>;
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
impl From<crate::W<DDRPHYC_DSGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DSGCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PUREN` reader - PUREN"]
pub struct PUREN_R(crate::FieldReader<bool, bool>);
impl PUREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PUREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUREN` writer - PUREN"]
pub struct PUREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PUREN_W<'a> {
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
#[doc = "Field `BDISEN` reader - BDISEN"]
pub struct BDISEN_R(crate::FieldReader<bool, bool>);
impl BDISEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BDISEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BDISEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDISEN` writer - BDISEN"]
pub struct BDISEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BDISEN_W<'a> {
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
#[doc = "Field `ZUEN` reader - ZUEN"]
pub struct ZUEN_R(crate::FieldReader<bool, bool>);
impl ZUEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZUEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZUEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZUEN` writer - ZUEN"]
pub struct ZUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ZUEN_W<'a> {
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
#[doc = "Field `LPIOPD` reader - LPIOPD"]
pub struct LPIOPD_R(crate::FieldReader<bool, bool>);
impl LPIOPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPIOPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPIOPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPIOPD` writer - LPIOPD"]
pub struct LPIOPD_W<'a> {
    w: &'a mut W,
}
impl<'a> LPIOPD_W<'a> {
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
#[doc = "Field `LPDLLPD` reader - LPDLLPD"]
pub struct LPDLLPD_R(crate::FieldReader<bool, bool>);
impl LPDLLPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPDLLPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPDLLPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPDLLPD` writer - LPDLLPD"]
pub struct LPDLLPD_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDLLPD_W<'a> {
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
#[doc = "Field `DQSGX` reader - DQSGX"]
pub struct DQSGX_R(crate::FieldReader<u8, u8>);
impl DQSGX_R {
    pub(crate) fn new(bits: u8) -> Self {
        DQSGX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQSGX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSGX` writer - DQSGX"]
pub struct DQSGX_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSGX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `DQSGE` reader - DQSGE"]
pub struct DQSGE_R(crate::FieldReader<u8, u8>);
impl DQSGE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DQSGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQSGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSGE` writer - DQSGE"]
pub struct DQSGE_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `NOBUB` reader - NOBUB"]
pub struct NOBUB_R(crate::FieldReader<bool, bool>);
impl NOBUB_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOBUB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOBUB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOBUB` writer - NOBUB"]
pub struct NOBUB_W<'a> {
    w: &'a mut W,
}
impl<'a> NOBUB_W<'a> {
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
#[doc = "Field `FXDLAT` reader - FXDLAT"]
pub struct FXDLAT_R(crate::FieldReader<bool, bool>);
impl FXDLAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FXDLAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FXDLAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FXDLAT` writer - FXDLAT"]
pub struct FXDLAT_W<'a> {
    w: &'a mut W,
}
impl<'a> FXDLAT_W<'a> {
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
#[doc = "Field `CKEPDD` reader - CKEPDD"]
pub struct CKEPDD_R(crate::FieldReader<bool, bool>);
impl CKEPDD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKEPDD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKEPDD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKEPDD` writer - CKEPDD"]
pub struct CKEPDD_W<'a> {
    w: &'a mut W,
}
impl<'a> CKEPDD_W<'a> {
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
#[doc = "Field `ODTPDD` reader - ODTPDD"]
pub struct ODTPDD_R(crate::FieldReader<bool, bool>);
impl ODTPDD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODTPDD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODTPDD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODTPDD` writer - ODTPDD"]
pub struct ODTPDD_W<'a> {
    w: &'a mut W,
}
impl<'a> ODTPDD_W<'a> {
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
#[doc = "Field `NL2PD` reader - NL2PD"]
pub struct NL2PD_R(crate::FieldReader<bool, bool>);
impl NL2PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        NL2PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NL2PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NL2PD` writer - NL2PD"]
pub struct NL2PD_W<'a> {
    w: &'a mut W,
}
impl<'a> NL2PD_W<'a> {
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
#[doc = "Field `NL2OE` reader - NL2OE"]
pub struct NL2OE_R(crate::FieldReader<bool, bool>);
impl NL2OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NL2OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NL2OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NL2OE` writer - NL2OE"]
pub struct NL2OE_W<'a> {
    w: &'a mut W,
}
impl<'a> NL2OE_W<'a> {
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
#[doc = "Field `TPDPD` reader - TPDPD"]
pub struct TPDPD_R(crate::FieldReader<bool, bool>);
impl TPDPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        TPDPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPDPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPDPD` writer - TPDPD"]
pub struct TPDPD_W<'a> {
    w: &'a mut W,
}
impl<'a> TPDPD_W<'a> {
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
#[doc = "Field `TPDOE` reader - TPDOE"]
pub struct TPDOE_R(crate::FieldReader<bool, bool>);
impl TPDOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TPDOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPDOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPDOE` writer - TPDOE"]
pub struct TPDOE_W<'a> {
    w: &'a mut W,
}
impl<'a> TPDOE_W<'a> {
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
#[doc = "Field `CKOE` reader - CKOE"]
pub struct CKOE_R(crate::FieldReader<bool, bool>);
impl CKOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKOE` writer - CKOE"]
pub struct CKOE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOE_W<'a> {
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
#[doc = "Field `ODTOE` reader - ODTOE"]
pub struct ODTOE_R(crate::FieldReader<bool, bool>);
impl ODTOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODTOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODTOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODTOE` writer - ODTOE"]
pub struct ODTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> ODTOE_W<'a> {
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
#[doc = "Field `RSTOE` reader - RSTOE"]
pub struct RSTOE_R(crate::FieldReader<bool, bool>);
impl RSTOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTOE` writer - RSTOE"]
pub struct RSTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTOE_W<'a> {
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
#[doc = "Field `CKEOE` reader - CKEOE"]
pub struct CKEOE_R(crate::FieldReader<bool, bool>);
impl CKEOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKEOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKEOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKEOE` writer - CKEOE"]
pub struct CKEOE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKEOE_W<'a> {
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
    #[doc = "Bit 0 - PUREN"]
    #[inline(always)]
    pub fn puren(&self) -> PUREN_R {
        PUREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BDISEN"]
    #[inline(always)]
    pub fn bdisen(&self) -> BDISEN_R {
        BDISEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ZUEN"]
    #[inline(always)]
    pub fn zuen(&self) -> ZUEN_R {
        ZUEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPIOPD"]
    #[inline(always)]
    pub fn lpiopd(&self) -> LPIOPD_R {
        LPIOPD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LPDLLPD"]
    #[inline(always)]
    pub fn lpdllpd(&self) -> LPDLLPD_R {
        LPDLLPD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - DQSGX"]
    #[inline(always)]
    pub fn dqsgx(&self) -> DQSGX_R {
        DQSGX_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - DQSGE"]
    #[inline(always)]
    pub fn dqsge(&self) -> DQSGE_R {
        DQSGE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - NOBUB"]
    #[inline(always)]
    pub fn nobub(&self) -> NOBUB_R {
        NOBUB_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FXDLAT"]
    #[inline(always)]
    pub fn fxdlat(&self) -> FXDLAT_R {
        FXDLAT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CKEPDD"]
    #[inline(always)]
    pub fn ckepdd(&self) -> CKEPDD_R {
        CKEPDD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ODTPDD"]
    #[inline(always)]
    pub fn odtpdd(&self) -> ODTPDD_R {
        ODTPDD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - NL2PD"]
    #[inline(always)]
    pub fn nl2pd(&self) -> NL2PD_R {
        NL2PD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - NL2OE"]
    #[inline(always)]
    pub fn nl2oe(&self) -> NL2OE_R {
        NL2OE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TPDPD"]
    #[inline(always)]
    pub fn tpdpd(&self) -> TPDPD_R {
        TPDPD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TPDOE"]
    #[inline(always)]
    pub fn tpdoe(&self) -> TPDOE_R {
        TPDOE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - CKOE"]
    #[inline(always)]
    pub fn ckoe(&self) -> CKOE_R {
        CKOE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ODTOE"]
    #[inline(always)]
    pub fn odtoe(&self) -> ODTOE_R {
        ODTOE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - RSTOE"]
    #[inline(always)]
    pub fn rstoe(&self) -> RSTOE_R {
        RSTOE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - CKEOE"]
    #[inline(always)]
    pub fn ckeoe(&self) -> CKEOE_R {
        CKEOE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PUREN"]
    #[inline(always)]
    pub fn puren(&mut self) -> PUREN_W {
        PUREN_W { w: self }
    }
    #[doc = "Bit 1 - BDISEN"]
    #[inline(always)]
    pub fn bdisen(&mut self) -> BDISEN_W {
        BDISEN_W { w: self }
    }
    #[doc = "Bit 2 - ZUEN"]
    #[inline(always)]
    pub fn zuen(&mut self) -> ZUEN_W {
        ZUEN_W { w: self }
    }
    #[doc = "Bit 3 - LPIOPD"]
    #[inline(always)]
    pub fn lpiopd(&mut self) -> LPIOPD_W {
        LPIOPD_W { w: self }
    }
    #[doc = "Bit 4 - LPDLLPD"]
    #[inline(always)]
    pub fn lpdllpd(&mut self) -> LPDLLPD_W {
        LPDLLPD_W { w: self }
    }
    #[doc = "Bits 5:7 - DQSGX"]
    #[inline(always)]
    pub fn dqsgx(&mut self) -> DQSGX_W {
        DQSGX_W { w: self }
    }
    #[doc = "Bits 8:10 - DQSGE"]
    #[inline(always)]
    pub fn dqsge(&mut self) -> DQSGE_W {
        DQSGE_W { w: self }
    }
    #[doc = "Bit 11 - NOBUB"]
    #[inline(always)]
    pub fn nobub(&mut self) -> NOBUB_W {
        NOBUB_W { w: self }
    }
    #[doc = "Bit 12 - FXDLAT"]
    #[inline(always)]
    pub fn fxdlat(&mut self) -> FXDLAT_W {
        FXDLAT_W { w: self }
    }
    #[doc = "Bit 16 - CKEPDD"]
    #[inline(always)]
    pub fn ckepdd(&mut self) -> CKEPDD_W {
        CKEPDD_W { w: self }
    }
    #[doc = "Bit 20 - ODTPDD"]
    #[inline(always)]
    pub fn odtpdd(&mut self) -> ODTPDD_W {
        ODTPDD_W { w: self }
    }
    #[doc = "Bit 24 - NL2PD"]
    #[inline(always)]
    pub fn nl2pd(&mut self) -> NL2PD_W {
        NL2PD_W { w: self }
    }
    #[doc = "Bit 25 - NL2OE"]
    #[inline(always)]
    pub fn nl2oe(&mut self) -> NL2OE_W {
        NL2OE_W { w: self }
    }
    #[doc = "Bit 26 - TPDPD"]
    #[inline(always)]
    pub fn tpdpd(&mut self) -> TPDPD_W {
        TPDPD_W { w: self }
    }
    #[doc = "Bit 27 - TPDOE"]
    #[inline(always)]
    pub fn tpdoe(&mut self) -> TPDOE_W {
        TPDOE_W { w: self }
    }
    #[doc = "Bit 28 - CKOE"]
    #[inline(always)]
    pub fn ckoe(&mut self) -> CKOE_W {
        CKOE_W { w: self }
    }
    #[doc = "Bit 29 - ODTOE"]
    #[inline(always)]
    pub fn odtoe(&mut self) -> ODTOE_W {
        ODTOE_W { w: self }
    }
    #[doc = "Bit 30 - RSTOE"]
    #[inline(always)]
    pub fn rstoe(&mut self) -> RSTOE_W {
        RSTOE_W { w: self }
    }
    #[doc = "Bit 31 - CKEOE"]
    #[inline(always)]
    pub fn ckeoe(&mut self) -> CKEOE_W {
        CKEOE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DSGC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dsgcr](index.html) module"]
pub struct DDRPHYC_DSGCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DSGCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dsgcr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DSGCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dsgcr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DSGCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DSGCR to value 0xfa00_001f"]
impl crate::Resettable for DDRPHYC_DSGCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfa00_001f
    }
}
