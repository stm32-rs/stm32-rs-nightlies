#[doc = "Register `OTG_GINTSTS` reader"]
pub struct R(crate::R<OTG_GINTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_GINTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_GINTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_GINTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_GINTSTS` writer"]
pub struct W(crate::W<OTG_GINTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_GINTSTS_SPEC>;
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
impl From<crate::W<OTG_GINTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_GINTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMOD` reader - CMOD"]
pub struct CMOD_R(crate::FieldReader<bool, bool>);
impl CMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMIS` reader - MMIS"]
pub struct MMIS_R(crate::FieldReader<bool, bool>);
impl MMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMIS` writer - MMIS"]
pub struct MMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MMIS_W<'a> {
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
#[doc = "Field `OTGINT` reader - OTGINT"]
pub struct OTGINT_R(crate::FieldReader<bool, bool>);
impl OTGINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTGINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTGINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF` reader - SOF"]
pub struct SOF_R(crate::FieldReader<bool, bool>);
impl SOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF` writer - SOF"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
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
#[doc = "Field `RXFLVL` reader - RXFLVL"]
pub struct RXFLVL_R(crate::FieldReader<bool, bool>);
impl RXFLVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFLVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPTXFE` reader - NPTXFE"]
pub struct NPTXFE_R(crate::FieldReader<bool, bool>);
impl NPTXFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NPTXFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPTXFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GINAKEFF` reader - GINAKEFF"]
pub struct GINAKEFF_R(crate::FieldReader<bool, bool>);
impl GINAKEFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GINAKEFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GINAKEFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GONAKEFF` reader - GONAKEFF"]
pub struct GONAKEFF_R(crate::FieldReader<bool, bool>);
impl GONAKEFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GONAKEFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GONAKEFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESUSP` reader - ESUSP"]
pub struct ESUSP_R(crate::FieldReader<bool, bool>);
impl ESUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ESUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESUSP` writer - ESUSP"]
pub struct ESUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> ESUSP_W<'a> {
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
#[doc = "Field `USBSUSP` reader - USBSUSP"]
pub struct USBSUSP_R(crate::FieldReader<bool, bool>);
impl USBSUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBSUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBSUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBSUSP` writer - USBSUSP"]
pub struct USBSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSUSP_W<'a> {
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
#[doc = "Field `USBRST` reader - USBRST"]
pub struct USBRST_R(crate::FieldReader<bool, bool>);
impl USBRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBRST` writer - USBRST"]
pub struct USBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRST_W<'a> {
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
#[doc = "Field `ENUMDNE` reader - ENUMDNE"]
pub struct ENUMDNE_R(crate::FieldReader<bool, bool>);
impl ENUMDNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENUMDNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENUMDNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENUMDNE` writer - ENUMDNE"]
pub struct ENUMDNE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUMDNE_W<'a> {
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
#[doc = "Field `ISOODRP` reader - ISOODRP"]
pub struct ISOODRP_R(crate::FieldReader<bool, bool>);
impl ISOODRP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISOODRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISOODRP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOODRP` writer - ISOODRP"]
pub struct ISOODRP_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOODRP_W<'a> {
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
#[doc = "Field `EOPF` reader - EOPF"]
pub struct EOPF_R(crate::FieldReader<bool, bool>);
impl EOPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOPF` writer - EOPF"]
pub struct EOPF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPF_W<'a> {
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
#[doc = "Field `IEPINT` reader - IEPINT"]
pub struct IEPINT_R(crate::FieldReader<bool, bool>);
impl IEPINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        IEPINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEPINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEPINT` reader - OEPINT"]
pub struct OEPINT_R(crate::FieldReader<bool, bool>);
impl OEPINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OEPINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEPINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IISOIXFR` reader - IISOIXFR"]
pub struct IISOIXFR_R(crate::FieldReader<bool, bool>);
impl IISOIXFR_R {
    pub(crate) fn new(bits: bool) -> Self {
        IISOIXFR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IISOIXFR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IISOIXFR` writer - IISOIXFR"]
pub struct IISOIXFR_W<'a> {
    w: &'a mut W,
}
impl<'a> IISOIXFR_W<'a> {
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
#[doc = "Field `IPXFR` reader - IPXFR"]
pub struct IPXFR_R(crate::FieldReader<bool, bool>);
impl IPXFR_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPXFR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPXFR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPXFR` writer - IPXFR"]
pub struct IPXFR_W<'a> {
    w: &'a mut W,
}
impl<'a> IPXFR_W<'a> {
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
#[doc = "Field `DATAFSUSP` reader - DATAFSUSP"]
pub struct DATAFSUSP_R(crate::FieldReader<bool, bool>);
impl DATAFSUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATAFSUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAFSUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAFSUSP` writer - DATAFSUSP"]
pub struct DATAFSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAFSUSP_W<'a> {
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
#[doc = "Field `HPRTINT` reader - HPRTINT"]
pub struct HPRTINT_R(crate::FieldReader<bool, bool>);
impl HPRTINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPRTINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPRTINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCINT` reader - HCINT"]
pub struct HCINT_R(crate::FieldReader<bool, bool>);
impl HCINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        HCINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTXFE` reader - PTXFE"]
pub struct PTXFE_R(crate::FieldReader<bool, bool>);
impl PTXFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTXFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTXFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIDSCHG` reader - CIDSCHG"]
pub struct CIDSCHG_R(crate::FieldReader<bool, bool>);
impl CIDSCHG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CIDSCHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CIDSCHG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIDSCHG` writer - CIDSCHG"]
pub struct CIDSCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> CIDSCHG_W<'a> {
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
#[doc = "Field `DISCINT` reader - DISCINT"]
pub struct DISCINT_R(crate::FieldReader<bool, bool>);
impl DISCINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISCINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISCINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISCINT` writer - DISCINT"]
pub struct DISCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCINT_W<'a> {
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
#[doc = "Field `SRQINT` reader - SRQINT"]
pub struct SRQINT_R(crate::FieldReader<bool, bool>);
impl SRQINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRQINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRQINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRQINT` writer - SRQINT"]
pub struct SRQINT_W<'a> {
    w: &'a mut W,
}
impl<'a> SRQINT_W<'a> {
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
#[doc = "Field `WKUPINT` reader - WKUPINT"]
pub struct WKUPINT_R(crate::FieldReader<bool, bool>);
impl WKUPINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPINT` writer - WKUPINT"]
pub struct WKUPINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPINT_W<'a> {
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
    #[doc = "Bit 0 - CMOD"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MMIS"]
    #[inline(always)]
    pub fn mmis(&self) -> MMIS_R {
        MMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OTGINT"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SOF"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RXFLVL"]
    #[inline(always)]
    pub fn rxflvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - NPTXFE"]
    #[inline(always)]
    pub fn nptxfe(&self) -> NPTXFE_R {
        NPTXFE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GINAKEFF"]
    #[inline(always)]
    pub fn ginakeff(&self) -> GINAKEFF_R {
        GINAKEFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GONAKEFF"]
    #[inline(always)]
    pub fn gonakeff(&self) -> GONAKEFF_R {
        GONAKEFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ESUSP"]
    #[inline(always)]
    pub fn esusp(&self) -> ESUSP_R {
        ESUSP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USBSUSP"]
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ENUMDNE"]
    #[inline(always)]
    pub fn enumdne(&self) -> ENUMDNE_R {
        ENUMDNE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ISOODRP"]
    #[inline(always)]
    pub fn isoodrp(&self) -> ISOODRP_R {
        ISOODRP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EOPF"]
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IEPINT"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OEPINT"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IISOIXFR"]
    #[inline(always)]
    pub fn iisoixfr(&self) -> IISOIXFR_R {
        IISOIXFR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - IPXFR"]
    #[inline(always)]
    pub fn ipxfr(&self) -> IPXFR_R {
        IPXFR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DATAFSUSP"]
    #[inline(always)]
    pub fn datafsusp(&self) -> DATAFSUSP_R {
        DATAFSUSP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - HPRTINT"]
    #[inline(always)]
    pub fn hprtint(&self) -> HPRTINT_R {
        HPRTINT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - HCINT"]
    #[inline(always)]
    pub fn hcint(&self) -> HCINT_R {
        HCINT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - PTXFE"]
    #[inline(always)]
    pub fn ptxfe(&self) -> PTXFE_R {
        PTXFE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - CIDSCHG"]
    #[inline(always)]
    pub fn cidschg(&self) -> CIDSCHG_R {
        CIDSCHG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DISCINT"]
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - SRQINT"]
    #[inline(always)]
    pub fn srqint(&self) -> SRQINT_R {
        SRQINT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - WKUPINT"]
    #[inline(always)]
    pub fn wkupint(&self) -> WKUPINT_R {
        WKUPINT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - MMIS"]
    #[inline(always)]
    pub fn mmis(&mut self) -> MMIS_W {
        MMIS_W { w: self }
    }
    #[doc = "Bit 3 - SOF"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 10 - ESUSP"]
    #[inline(always)]
    pub fn esusp(&mut self) -> ESUSP_W {
        ESUSP_W { w: self }
    }
    #[doc = "Bit 11 - USBSUSP"]
    #[inline(always)]
    pub fn usbsusp(&mut self) -> USBSUSP_W {
        USBSUSP_W { w: self }
    }
    #[doc = "Bit 12 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W {
        USBRST_W { w: self }
    }
    #[doc = "Bit 13 - ENUMDNE"]
    #[inline(always)]
    pub fn enumdne(&mut self) -> ENUMDNE_W {
        ENUMDNE_W { w: self }
    }
    #[doc = "Bit 14 - ISOODRP"]
    #[inline(always)]
    pub fn isoodrp(&mut self) -> ISOODRP_W {
        ISOODRP_W { w: self }
    }
    #[doc = "Bit 15 - EOPF"]
    #[inline(always)]
    pub fn eopf(&mut self) -> EOPF_W {
        EOPF_W { w: self }
    }
    #[doc = "Bit 20 - IISOIXFR"]
    #[inline(always)]
    pub fn iisoixfr(&mut self) -> IISOIXFR_W {
        IISOIXFR_W { w: self }
    }
    #[doc = "Bit 21 - IPXFR"]
    #[inline(always)]
    pub fn ipxfr(&mut self) -> IPXFR_W {
        IPXFR_W { w: self }
    }
    #[doc = "Bit 22 - DATAFSUSP"]
    #[inline(always)]
    pub fn datafsusp(&mut self) -> DATAFSUSP_W {
        DATAFSUSP_W { w: self }
    }
    #[doc = "Bit 28 - CIDSCHG"]
    #[inline(always)]
    pub fn cidschg(&mut self) -> CIDSCHG_W {
        CIDSCHG_W { w: self }
    }
    #[doc = "Bit 29 - DISCINT"]
    #[inline(always)]
    pub fn discint(&mut self) -> DISCINT_W {
        DISCINT_W { w: self }
    }
    #[doc = "Bit 30 - SRQINT"]
    #[inline(always)]
    pub fn srqint(&mut self) -> SRQINT_W {
        SRQINT_W { w: self }
    }
    #[doc = "Bit 31 - WKUPINT"]
    #[inline(always)]
    pub fn wkupint(&mut self) -> WKUPINT_W {
        WKUPINT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the OTG_GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_gintsts](index.html) module"]
pub struct OTG_GINTSTS_SPEC;
impl crate::RegisterSpec for OTG_GINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_gintsts::R](R) reader structure"]
impl crate::Readable for OTG_GINTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_gintsts::W](W) writer structure"]
impl crate::Writable for OTG_GINTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_GINTSTS to value 0x1400_0020"]
impl crate::Resettable for OTG_GINTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1400_0020
    }
}
