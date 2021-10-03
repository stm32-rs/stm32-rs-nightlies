#[doc = "Register `OTG_GINTMSK` reader"]
pub struct R(crate::R<OTG_GINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_GINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_GINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_GINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_GINTMSK` writer"]
pub struct W(crate::W<OTG_GINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_GINTMSK_SPEC>;
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
impl From<crate::W<OTG_GINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_GINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMISM` reader - MMISM"]
pub struct MMISM_R(crate::FieldReader<bool, bool>);
impl MMISM_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMISM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMISM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMISM` writer - MMISM"]
pub struct MMISM_W<'a> {
    w: &'a mut W,
}
impl<'a> MMISM_W<'a> {
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
#[doc = "Field `OTGINT` writer - OTGINT"]
pub struct OTGINT_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGINT_W<'a> {
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
#[doc = "Field `SOFM` reader - SOFM"]
pub struct SOFM_R(crate::FieldReader<bool, bool>);
impl SOFM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFM` writer - SOFM"]
pub struct SOFM_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFM_W<'a> {
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
#[doc = "Field `RXFLVLM` reader - RXFLVLM"]
pub struct RXFLVLM_R(crate::FieldReader<bool, bool>);
impl RXFLVLM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFLVLM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFLVLM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFLVLM` writer - RXFLVLM"]
pub struct RXFLVLM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFLVLM_W<'a> {
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
#[doc = "Field `NPTXFEM` reader - NPTXFEM"]
pub struct NPTXFEM_R(crate::FieldReader<bool, bool>);
impl NPTXFEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        NPTXFEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPTXFEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPTXFEM` writer - NPTXFEM"]
pub struct NPTXFEM_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFEM_W<'a> {
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
#[doc = "Field `GINAKEFFM` reader - GINAKEFFM"]
pub struct GINAKEFFM_R(crate::FieldReader<bool, bool>);
impl GINAKEFFM_R {
    pub(crate) fn new(bits: bool) -> Self {
        GINAKEFFM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GINAKEFFM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GINAKEFFM` writer - GINAKEFFM"]
pub struct GINAKEFFM_W<'a> {
    w: &'a mut W,
}
impl<'a> GINAKEFFM_W<'a> {
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
#[doc = "Field `GONAKEFFM` reader - GONAKEFFM"]
pub struct GONAKEFFM_R(crate::FieldReader<bool, bool>);
impl GONAKEFFM_R {
    pub(crate) fn new(bits: bool) -> Self {
        GONAKEFFM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GONAKEFFM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GONAKEFFM` writer - GONAKEFFM"]
pub struct GONAKEFFM_W<'a> {
    w: &'a mut W,
}
impl<'a> GONAKEFFM_W<'a> {
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
#[doc = "Field `ESUSPM` reader - ESUSPM"]
pub struct ESUSPM_R(crate::FieldReader<bool, bool>);
impl ESUSPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ESUSPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESUSPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESUSPM` writer - ESUSPM"]
pub struct ESUSPM_W<'a> {
    w: &'a mut W,
}
impl<'a> ESUSPM_W<'a> {
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
#[doc = "Field `USBSUSPM` reader - USBSUSPM"]
pub struct USBSUSPM_R(crate::FieldReader<bool, bool>);
impl USBSUSPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBSUSPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBSUSPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBSUSPM` writer - USBSUSPM"]
pub struct USBSUSPM_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSUSPM_W<'a> {
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
#[doc = "Field `ENUMDNEM` reader - ENUMDNEM"]
pub struct ENUMDNEM_R(crate::FieldReader<bool, bool>);
impl ENUMDNEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENUMDNEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENUMDNEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENUMDNEM` writer - ENUMDNEM"]
pub struct ENUMDNEM_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUMDNEM_W<'a> {
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
#[doc = "Field `ISOODRPM` reader - ISOODRPM"]
pub struct ISOODRPM_R(crate::FieldReader<bool, bool>);
impl ISOODRPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISOODRPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISOODRPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOODRPM` writer - ISOODRPM"]
pub struct ISOODRPM_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOODRPM_W<'a> {
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
#[doc = "Field `EOPFM` reader - EOPFM"]
pub struct EOPFM_R(crate::FieldReader<bool, bool>);
impl EOPFM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOPFM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOPFM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOPFM` writer - EOPFM"]
pub struct EOPFM_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPFM_W<'a> {
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
#[doc = "Field `IEPINT` writer - IEPINT"]
pub struct IEPINT_W<'a> {
    w: &'a mut W,
}
impl<'a> IEPINT_W<'a> {
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
#[doc = "Field `OEPINT` writer - OEPINT"]
pub struct OEPINT_W<'a> {
    w: &'a mut W,
}
impl<'a> OEPINT_W<'a> {
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
#[doc = "Field `IISOIXFRM` reader - IISOIXFRM"]
pub struct IISOIXFRM_R(crate::FieldReader<bool, bool>);
impl IISOIXFRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        IISOIXFRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IISOIXFRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IISOIXFRM` writer - IISOIXFRM"]
pub struct IISOIXFRM_W<'a> {
    w: &'a mut W,
}
impl<'a> IISOIXFRM_W<'a> {
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
#[doc = "Field `IPXFRM` reader - IPXFRM"]
pub struct IPXFRM_R(crate::FieldReader<bool, bool>);
impl IPXFRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPXFRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPXFRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPXFRM` writer - IPXFRM"]
pub struct IPXFRM_W<'a> {
    w: &'a mut W,
}
impl<'a> IPXFRM_W<'a> {
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
#[doc = "Field `FSUSPM` reader - FSUSPM"]
pub struct FSUSPM_R(crate::FieldReader<bool, bool>);
impl FSUSPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSUSPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSUSPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSUSPM` writer - FSUSPM"]
pub struct FSUSPM_W<'a> {
    w: &'a mut W,
}
impl<'a> FSUSPM_W<'a> {
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
#[doc = "Field `RSTDETM` reader - RSTDETM"]
pub struct RSTDETM_R(crate::FieldReader<bool, bool>);
impl RSTDETM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTDETM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTDETM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTDETM` writer - RSTDETM"]
pub struct RSTDETM_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTDETM_W<'a> {
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
#[doc = "Field `PRTIM` reader - PRTIM"]
pub struct PRTIM_R(crate::FieldReader<bool, bool>);
impl PRTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCIM` reader - HCIM"]
pub struct HCIM_R(crate::FieldReader<bool, bool>);
impl HCIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        HCIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCIM` writer - HCIM"]
pub struct HCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HCIM_W<'a> {
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
#[doc = "Field `PTXFEM` reader - PTXFEM"]
pub struct PTXFEM_R(crate::FieldReader<bool, bool>);
impl PTXFEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTXFEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTXFEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTXFEM` writer - PTXFEM"]
pub struct PTXFEM_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFEM_W<'a> {
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
#[doc = "Field `LPMINTM` reader - LPMINTM"]
pub struct LPMINTM_R(crate::FieldReader<bool, bool>);
impl LPMINTM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPMINTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMINTM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMINTM` writer - LPMINTM"]
pub struct LPMINTM_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMINTM_W<'a> {
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
#[doc = "Field `CIDSCHGM` reader - CIDSCHGM"]
pub struct CIDSCHGM_R(crate::FieldReader<bool, bool>);
impl CIDSCHGM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CIDSCHGM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CIDSCHGM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIDSCHGM` writer - CIDSCHGM"]
pub struct CIDSCHGM_W<'a> {
    w: &'a mut W,
}
impl<'a> CIDSCHGM_W<'a> {
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
#[doc = "Field `SRQIM` reader - SRQIM"]
pub struct SRQIM_R(crate::FieldReader<bool, bool>);
impl SRQIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRQIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRQIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRQIM` writer - SRQIM"]
pub struct SRQIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SRQIM_W<'a> {
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
#[doc = "Field `WUIM` reader - WUIM"]
pub struct WUIM_R(crate::FieldReader<bool, bool>);
impl WUIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUIM` writer - WUIM"]
pub struct WUIM_W<'a> {
    w: &'a mut W,
}
impl<'a> WUIM_W<'a> {
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
    #[doc = "Bit 1 - MMISM"]
    #[inline(always)]
    pub fn mmism(&self) -> MMISM_R {
        MMISM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OTGINT"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SOFM"]
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RXFLVLM"]
    #[inline(always)]
    pub fn rxflvlm(&self) -> RXFLVLM_R {
        RXFLVLM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - NPTXFEM"]
    #[inline(always)]
    pub fn nptxfem(&self) -> NPTXFEM_R {
        NPTXFEM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GINAKEFFM"]
    #[inline(always)]
    pub fn ginakeffm(&self) -> GINAKEFFM_R {
        GINAKEFFM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GONAKEFFM"]
    #[inline(always)]
    pub fn gonakeffm(&self) -> GONAKEFFM_R {
        GONAKEFFM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ESUSPM"]
    #[inline(always)]
    pub fn esuspm(&self) -> ESUSPM_R {
        ESUSPM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USBSUSPM"]
    #[inline(always)]
    pub fn usbsuspm(&self) -> USBSUSPM_R {
        USBSUSPM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ENUMDNEM"]
    #[inline(always)]
    pub fn enumdnem(&self) -> ENUMDNEM_R {
        ENUMDNEM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ISOODRPM"]
    #[inline(always)]
    pub fn isoodrpm(&self) -> ISOODRPM_R {
        ISOODRPM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EOPFM"]
    #[inline(always)]
    pub fn eopfm(&self) -> EOPFM_R {
        EOPFM_R::new(((self.bits >> 15) & 0x01) != 0)
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
    #[doc = "Bit 20 - IISOIXFRM"]
    #[inline(always)]
    pub fn iisoixfrm(&self) -> IISOIXFRM_R {
        IISOIXFRM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - IPXFRM"]
    #[inline(always)]
    pub fn ipxfrm(&self) -> IPXFRM_R {
        IPXFRM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - FSUSPM"]
    #[inline(always)]
    pub fn fsuspm(&self) -> FSUSPM_R {
        FSUSPM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - RSTDETM"]
    #[inline(always)]
    pub fn rstdetm(&self) -> RSTDETM_R {
        RSTDETM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - PRTIM"]
    #[inline(always)]
    pub fn prtim(&self) -> PRTIM_R {
        PRTIM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - HCIM"]
    #[inline(always)]
    pub fn hcim(&self) -> HCIM_R {
        HCIM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - PTXFEM"]
    #[inline(always)]
    pub fn ptxfem(&self) -> PTXFEM_R {
        PTXFEM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LPMINTM"]
    #[inline(always)]
    pub fn lpmintm(&self) -> LPMINTM_R {
        LPMINTM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - CIDSCHGM"]
    #[inline(always)]
    pub fn cidschgm(&self) -> CIDSCHGM_R {
        CIDSCHGM_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DISCINT"]
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - SRQIM"]
    #[inline(always)]
    pub fn srqim(&self) -> SRQIM_R {
        SRQIM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - WUIM"]
    #[inline(always)]
    pub fn wuim(&self) -> WUIM_R {
        WUIM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - MMISM"]
    #[inline(always)]
    pub fn mmism(&mut self) -> MMISM_W {
        MMISM_W { w: self }
    }
    #[doc = "Bit 2 - OTGINT"]
    #[inline(always)]
    pub fn otgint(&mut self) -> OTGINT_W {
        OTGINT_W { w: self }
    }
    #[doc = "Bit 3 - SOFM"]
    #[inline(always)]
    pub fn sofm(&mut self) -> SOFM_W {
        SOFM_W { w: self }
    }
    #[doc = "Bit 4 - RXFLVLM"]
    #[inline(always)]
    pub fn rxflvlm(&mut self) -> RXFLVLM_W {
        RXFLVLM_W { w: self }
    }
    #[doc = "Bit 5 - NPTXFEM"]
    #[inline(always)]
    pub fn nptxfem(&mut self) -> NPTXFEM_W {
        NPTXFEM_W { w: self }
    }
    #[doc = "Bit 6 - GINAKEFFM"]
    #[inline(always)]
    pub fn ginakeffm(&mut self) -> GINAKEFFM_W {
        GINAKEFFM_W { w: self }
    }
    #[doc = "Bit 7 - GONAKEFFM"]
    #[inline(always)]
    pub fn gonakeffm(&mut self) -> GONAKEFFM_W {
        GONAKEFFM_W { w: self }
    }
    #[doc = "Bit 10 - ESUSPM"]
    #[inline(always)]
    pub fn esuspm(&mut self) -> ESUSPM_W {
        ESUSPM_W { w: self }
    }
    #[doc = "Bit 11 - USBSUSPM"]
    #[inline(always)]
    pub fn usbsuspm(&mut self) -> USBSUSPM_W {
        USBSUSPM_W { w: self }
    }
    #[doc = "Bit 12 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W {
        USBRST_W { w: self }
    }
    #[doc = "Bit 13 - ENUMDNEM"]
    #[inline(always)]
    pub fn enumdnem(&mut self) -> ENUMDNEM_W {
        ENUMDNEM_W { w: self }
    }
    #[doc = "Bit 14 - ISOODRPM"]
    #[inline(always)]
    pub fn isoodrpm(&mut self) -> ISOODRPM_W {
        ISOODRPM_W { w: self }
    }
    #[doc = "Bit 15 - EOPFM"]
    #[inline(always)]
    pub fn eopfm(&mut self) -> EOPFM_W {
        EOPFM_W { w: self }
    }
    #[doc = "Bit 18 - IEPINT"]
    #[inline(always)]
    pub fn iepint(&mut self) -> IEPINT_W {
        IEPINT_W { w: self }
    }
    #[doc = "Bit 19 - OEPINT"]
    #[inline(always)]
    pub fn oepint(&mut self) -> OEPINT_W {
        OEPINT_W { w: self }
    }
    #[doc = "Bit 20 - IISOIXFRM"]
    #[inline(always)]
    pub fn iisoixfrm(&mut self) -> IISOIXFRM_W {
        IISOIXFRM_W { w: self }
    }
    #[doc = "Bit 21 - IPXFRM"]
    #[inline(always)]
    pub fn ipxfrm(&mut self) -> IPXFRM_W {
        IPXFRM_W { w: self }
    }
    #[doc = "Bit 22 - FSUSPM"]
    #[inline(always)]
    pub fn fsuspm(&mut self) -> FSUSPM_W {
        FSUSPM_W { w: self }
    }
    #[doc = "Bit 23 - RSTDETM"]
    #[inline(always)]
    pub fn rstdetm(&mut self) -> RSTDETM_W {
        RSTDETM_W { w: self }
    }
    #[doc = "Bit 25 - HCIM"]
    #[inline(always)]
    pub fn hcim(&mut self) -> HCIM_W {
        HCIM_W { w: self }
    }
    #[doc = "Bit 26 - PTXFEM"]
    #[inline(always)]
    pub fn ptxfem(&mut self) -> PTXFEM_W {
        PTXFEM_W { w: self }
    }
    #[doc = "Bit 27 - LPMINTM"]
    #[inline(always)]
    pub fn lpmintm(&mut self) -> LPMINTM_W {
        LPMINTM_W { w: self }
    }
    #[doc = "Bit 28 - CIDSCHGM"]
    #[inline(always)]
    pub fn cidschgm(&mut self) -> CIDSCHGM_W {
        CIDSCHGM_W { w: self }
    }
    #[doc = "Bit 29 - DISCINT"]
    #[inline(always)]
    pub fn discint(&mut self) -> DISCINT_W {
        DISCINT_W { w: self }
    }
    #[doc = "Bit 30 - SRQIM"]
    #[inline(always)]
    pub fn srqim(&mut self) -> SRQIM_W {
        SRQIM_W { w: self }
    }
    #[doc = "Bit 31 - WUIM"]
    #[inline(always)]
    pub fn wuim(&mut self) -> WUIM_W {
        WUIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (OTG_GINTSTS) register bit corresponding to that interrupt is still set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_gintmsk](index.html) module"]
pub struct OTG_GINTMSK_SPEC;
impl crate::RegisterSpec for OTG_GINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_gintmsk::R](R) reader structure"]
impl crate::Readable for OTG_GINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_gintmsk::W](W) writer structure"]
impl crate::Writable for OTG_GINTMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_GINTMSK to value 0"]
impl crate::Resettable for OTG_GINTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
