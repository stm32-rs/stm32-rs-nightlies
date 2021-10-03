#[doc = "Register `RCC_DDRITFCR` reader"]
pub struct R(crate::R<RCC_DDRITFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_DDRITFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_DDRITFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_DDRITFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_DDRITFCR` writer"]
pub struct W(crate::W<RCC_DDRITFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_DDRITFCR_SPEC>;
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
impl From<crate::W<RCC_DDRITFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_DDRITFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDRC1EN` reader - DDRC1EN"]
pub struct DDRC1EN_R(crate::FieldReader<bool, bool>);
impl DDRC1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRC1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRC1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRC1EN` writer - DDRC1EN"]
pub struct DDRC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRC1EN_W<'a> {
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
#[doc = "Field `DDRC1LPEN` reader - DDRC1LPEN"]
pub struct DDRC1LPEN_R(crate::FieldReader<bool, bool>);
impl DDRC1LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRC1LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRC1LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRC1LPEN` writer - DDRC1LPEN"]
pub struct DDRC1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRC1LPEN_W<'a> {
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
#[doc = "Field `DDRC2EN` reader - DDRC2EN"]
pub struct DDRC2EN_R(crate::FieldReader<bool, bool>);
impl DDRC2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRC2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRC2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRC2EN` writer - DDRC2EN"]
pub struct DDRC2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRC2EN_W<'a> {
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
#[doc = "Field `DDRC2LPEN` reader - DDRC2LPEN"]
pub struct DDRC2LPEN_R(crate::FieldReader<bool, bool>);
impl DDRC2LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRC2LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRC2LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRC2LPEN` writer - DDRC2LPEN"]
pub struct DDRC2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRC2LPEN_W<'a> {
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
#[doc = "Field `DDRPHYCEN` reader - DDRPHYCEN"]
pub struct DDRPHYCEN_R(crate::FieldReader<bool, bool>);
impl DDRPHYCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRPHYCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRPHYCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRPHYCEN` writer - DDRPHYCEN"]
pub struct DDRPHYCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPHYCEN_W<'a> {
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
#[doc = "Field `DDRPHYCLPEN` reader - DDRPHYCLPEN"]
pub struct DDRPHYCLPEN_R(crate::FieldReader<bool, bool>);
impl DDRPHYCLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRPHYCLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRPHYCLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRPHYCLPEN` writer - DDRPHYCLPEN"]
pub struct DDRPHYCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPHYCLPEN_W<'a> {
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
#[doc = "Field `DDRCAPBEN` reader - DDRCAPBEN"]
pub struct DDRCAPBEN_R(crate::FieldReader<bool, bool>);
impl DDRCAPBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRCAPBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRCAPBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRCAPBEN` writer - DDRCAPBEN"]
pub struct DDRCAPBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCAPBEN_W<'a> {
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
#[doc = "Field `DDRCAPBLPEN` reader - DDRCAPBLPEN"]
pub struct DDRCAPBLPEN_R(crate::FieldReader<bool, bool>);
impl DDRCAPBLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRCAPBLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRCAPBLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRCAPBLPEN` writer - DDRCAPBLPEN"]
pub struct DDRCAPBLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCAPBLPEN_W<'a> {
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
#[doc = "Field `AXIDCGEN` reader - AXIDCGEN"]
pub struct AXIDCGEN_R(crate::FieldReader<bool, bool>);
impl AXIDCGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AXIDCGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AXIDCGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AXIDCGEN` writer - AXIDCGEN"]
pub struct AXIDCGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIDCGEN_W<'a> {
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
#[doc = "Field `DDRPHYCAPBEN` reader - DDRPHYCAPBEN"]
pub struct DDRPHYCAPBEN_R(crate::FieldReader<bool, bool>);
impl DDRPHYCAPBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRPHYCAPBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRPHYCAPBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRPHYCAPBEN` writer - DDRPHYCAPBEN"]
pub struct DDRPHYCAPBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPHYCAPBEN_W<'a> {
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
#[doc = "Field `DDRPHYCAPBLPEN` reader - DDRPHYCAPBLPEN"]
pub struct DDRPHYCAPBLPEN_R(crate::FieldReader<bool, bool>);
impl DDRPHYCAPBLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRPHYCAPBLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRPHYCAPBLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRPHYCAPBLPEN` writer - DDRPHYCAPBLPEN"]
pub struct DDRPHYCAPBLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPHYCAPBLPEN_W<'a> {
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
#[doc = "Field `KERDCG_DLY` reader - KERDCG_DLY"]
pub struct KERDCG_DLY_R(crate::FieldReader<u8, u8>);
impl KERDCG_DLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        KERDCG_DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KERDCG_DLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KERDCG_DLY` writer - KERDCG_DLY"]
pub struct KERDCG_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> KERDCG_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Field `DDRCAPBRST` reader - DDRCAPBRST"]
pub struct DDRCAPBRST_R(crate::FieldReader<bool, bool>);
impl DDRCAPBRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRCAPBRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRCAPBRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRCAPBRST` writer - DDRCAPBRST"]
pub struct DDRCAPBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCAPBRST_W<'a> {
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
#[doc = "Field `DDRCAXIRST` reader - DDRCAXIRST"]
pub struct DDRCAXIRST_R(crate::FieldReader<bool, bool>);
impl DDRCAXIRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRCAXIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRCAXIRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRCAXIRST` writer - DDRCAXIRST"]
pub struct DDRCAXIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCAXIRST_W<'a> {
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
#[doc = "Field `DDRCORERST` reader - DDRCORERST"]
pub struct DDRCORERST_R(crate::FieldReader<bool, bool>);
impl DDRCORERST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRCORERST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRCORERST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRCORERST` writer - DDRCORERST"]
pub struct DDRCORERST_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCORERST_W<'a> {
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
#[doc = "Field `DPHYAPBRST` reader - DPHYAPBRST"]
pub struct DPHYAPBRST_R(crate::FieldReader<bool, bool>);
impl DPHYAPBRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPHYAPBRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPHYAPBRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPHYAPBRST` writer - DPHYAPBRST"]
pub struct DPHYAPBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DPHYAPBRST_W<'a> {
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
#[doc = "Field `DPHYRST` reader - DPHYRST"]
pub struct DPHYRST_R(crate::FieldReader<bool, bool>);
impl DPHYRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPHYRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPHYRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPHYRST` writer - DPHYRST"]
pub struct DPHYRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DPHYRST_W<'a> {
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
#[doc = "Field `DPHYCTLRST` reader - DPHYCTLRST"]
pub struct DPHYCTLRST_R(crate::FieldReader<bool, bool>);
impl DPHYCTLRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPHYCTLRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPHYCTLRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPHYCTLRST` writer - DPHYCTLRST"]
pub struct DPHYCTLRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DPHYCTLRST_W<'a> {
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
#[doc = "Field `DDRCKMOD` reader - DDRCKMOD"]
pub struct DDRCKMOD_R(crate::FieldReader<u8, u8>);
impl DDRCKMOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        DDRCKMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRCKMOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRCKMOD` writer - DDRCKMOD"]
pub struct DDRCKMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCKMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `GSKPMOD` reader - GSKPMOD"]
pub struct GSKPMOD_R(crate::FieldReader<bool, bool>);
impl GSKPMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GSKPMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GSKPMOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GSKPMOD` writer - GSKPMOD"]
pub struct GSKPMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> GSKPMOD_W<'a> {
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
#[doc = "Field `GSKPCTRL` reader - GSKPCTRL"]
pub struct GSKPCTRL_R(crate::FieldReader<bool, bool>);
impl GSKPCTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        GSKPCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GSKPCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GSKPCTRL` writer - GSKPCTRL"]
pub struct GSKPCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> GSKPCTRL_W<'a> {
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
#[doc = "Field `DFILP_WIDTH` reader - DFILP_WIDTH"]
pub struct DFILP_WIDTH_R(crate::FieldReader<u8, u8>);
impl DFILP_WIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        DFILP_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFILP_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFILP_WIDTH` writer - DFILP_WIDTH"]
pub struct DFILP_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DFILP_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
#[doc = "Field `GSKP_DUR` reader - GSKP_DUR"]
pub struct GSKP_DUR_R(crate::FieldReader<u8, u8>);
impl GSKP_DUR_R {
    pub(crate) fn new(bits: u8) -> Self {
        GSKP_DUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GSKP_DUR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GSKP_DUR` writer - GSKP_DUR"]
pub struct GSKP_DUR_W<'a> {
    w: &'a mut W,
}
impl<'a> GSKP_DUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DDRC1EN"]
    #[inline(always)]
    pub fn ddrc1en(&self) -> DDRC1EN_R {
        DDRC1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DDRC1LPEN"]
    #[inline(always)]
    pub fn ddrc1lpen(&self) -> DDRC1LPEN_R {
        DDRC1LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DDRC2EN"]
    #[inline(always)]
    pub fn ddrc2en(&self) -> DDRC2EN_R {
        DDRC2EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DDRC2LPEN"]
    #[inline(always)]
    pub fn ddrc2lpen(&self) -> DDRC2LPEN_R {
        DDRC2LPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DDRPHYCEN"]
    #[inline(always)]
    pub fn ddrphycen(&self) -> DDRPHYCEN_R {
        DDRPHYCEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DDRPHYCLPEN"]
    #[inline(always)]
    pub fn ddrphyclpen(&self) -> DDRPHYCLPEN_R {
        DDRPHYCLPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DDRCAPBEN"]
    #[inline(always)]
    pub fn ddrcapben(&self) -> DDRCAPBEN_R {
        DDRCAPBEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DDRCAPBLPEN"]
    #[inline(always)]
    pub fn ddrcapblpen(&self) -> DDRCAPBLPEN_R {
        DDRCAPBLPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AXIDCGEN"]
    #[inline(always)]
    pub fn axidcgen(&self) -> AXIDCGEN_R {
        AXIDCGEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DDRPHYCAPBEN"]
    #[inline(always)]
    pub fn ddrphycapben(&self) -> DDRPHYCAPBEN_R {
        DDRPHYCAPBEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DDRPHYCAPBLPEN"]
    #[inline(always)]
    pub fn ddrphycapblpen(&self) -> DDRPHYCAPBLPEN_R {
        DDRPHYCAPBLPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - KERDCG_DLY"]
    #[inline(always)]
    pub fn kerdcg_dly(&self) -> KERDCG_DLY_R {
        KERDCG_DLY_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 14 - DDRCAPBRST"]
    #[inline(always)]
    pub fn ddrcapbrst(&self) -> DDRCAPBRST_R {
        DDRCAPBRST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DDRCAXIRST"]
    #[inline(always)]
    pub fn ddrcaxirst(&self) -> DDRCAXIRST_R {
        DDRCAXIRST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DDRCORERST"]
    #[inline(always)]
    pub fn ddrcorerst(&self) -> DDRCORERST_R {
        DDRCORERST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DPHYAPBRST"]
    #[inline(always)]
    pub fn dphyapbrst(&self) -> DPHYAPBRST_R {
        DPHYAPBRST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DPHYRST"]
    #[inline(always)]
    pub fn dphyrst(&self) -> DPHYRST_R {
        DPHYRST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DPHYCTLRST"]
    #[inline(always)]
    pub fn dphyctlrst(&self) -> DPHYCTLRST_R {
        DPHYCTLRST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - DDRCKMOD"]
    #[inline(always)]
    pub fn ddrckmod(&self) -> DDRCKMOD_R {
        DDRCKMOD_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - GSKPMOD"]
    #[inline(always)]
    pub fn gskpmod(&self) -> GSKPMOD_R {
        GSKPMOD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - GSKPCTRL"]
    #[inline(always)]
    pub fn gskpctrl(&self) -> GSKPCTRL_R {
        GSKPCTRL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:27 - DFILP_WIDTH"]
    #[inline(always)]
    pub fn dfilp_width(&self) -> DFILP_WIDTH_R {
        DFILP_WIDTH_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 28:31 - GSKP_DUR"]
    #[inline(always)]
    pub fn gskp_dur(&self) -> GSKP_DUR_R {
        GSKP_DUR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DDRC1EN"]
    #[inline(always)]
    pub fn ddrc1en(&mut self) -> DDRC1EN_W {
        DDRC1EN_W { w: self }
    }
    #[doc = "Bit 1 - DDRC1LPEN"]
    #[inline(always)]
    pub fn ddrc1lpen(&mut self) -> DDRC1LPEN_W {
        DDRC1LPEN_W { w: self }
    }
    #[doc = "Bit 2 - DDRC2EN"]
    #[inline(always)]
    pub fn ddrc2en(&mut self) -> DDRC2EN_W {
        DDRC2EN_W { w: self }
    }
    #[doc = "Bit 3 - DDRC2LPEN"]
    #[inline(always)]
    pub fn ddrc2lpen(&mut self) -> DDRC2LPEN_W {
        DDRC2LPEN_W { w: self }
    }
    #[doc = "Bit 4 - DDRPHYCEN"]
    #[inline(always)]
    pub fn ddrphycen(&mut self) -> DDRPHYCEN_W {
        DDRPHYCEN_W { w: self }
    }
    #[doc = "Bit 5 - DDRPHYCLPEN"]
    #[inline(always)]
    pub fn ddrphyclpen(&mut self) -> DDRPHYCLPEN_W {
        DDRPHYCLPEN_W { w: self }
    }
    #[doc = "Bit 6 - DDRCAPBEN"]
    #[inline(always)]
    pub fn ddrcapben(&mut self) -> DDRCAPBEN_W {
        DDRCAPBEN_W { w: self }
    }
    #[doc = "Bit 7 - DDRCAPBLPEN"]
    #[inline(always)]
    pub fn ddrcapblpen(&mut self) -> DDRCAPBLPEN_W {
        DDRCAPBLPEN_W { w: self }
    }
    #[doc = "Bit 8 - AXIDCGEN"]
    #[inline(always)]
    pub fn axidcgen(&mut self) -> AXIDCGEN_W {
        AXIDCGEN_W { w: self }
    }
    #[doc = "Bit 9 - DDRPHYCAPBEN"]
    #[inline(always)]
    pub fn ddrphycapben(&mut self) -> DDRPHYCAPBEN_W {
        DDRPHYCAPBEN_W { w: self }
    }
    #[doc = "Bit 10 - DDRPHYCAPBLPEN"]
    #[inline(always)]
    pub fn ddrphycapblpen(&mut self) -> DDRPHYCAPBLPEN_W {
        DDRPHYCAPBLPEN_W { w: self }
    }
    #[doc = "Bits 11:13 - KERDCG_DLY"]
    #[inline(always)]
    pub fn kerdcg_dly(&mut self) -> KERDCG_DLY_W {
        KERDCG_DLY_W { w: self }
    }
    #[doc = "Bit 14 - DDRCAPBRST"]
    #[inline(always)]
    pub fn ddrcapbrst(&mut self) -> DDRCAPBRST_W {
        DDRCAPBRST_W { w: self }
    }
    #[doc = "Bit 15 - DDRCAXIRST"]
    #[inline(always)]
    pub fn ddrcaxirst(&mut self) -> DDRCAXIRST_W {
        DDRCAXIRST_W { w: self }
    }
    #[doc = "Bit 16 - DDRCORERST"]
    #[inline(always)]
    pub fn ddrcorerst(&mut self) -> DDRCORERST_W {
        DDRCORERST_W { w: self }
    }
    #[doc = "Bit 17 - DPHYAPBRST"]
    #[inline(always)]
    pub fn dphyapbrst(&mut self) -> DPHYAPBRST_W {
        DPHYAPBRST_W { w: self }
    }
    #[doc = "Bit 18 - DPHYRST"]
    #[inline(always)]
    pub fn dphyrst(&mut self) -> DPHYRST_W {
        DPHYRST_W { w: self }
    }
    #[doc = "Bit 19 - DPHYCTLRST"]
    #[inline(always)]
    pub fn dphyctlrst(&mut self) -> DPHYCTLRST_W {
        DPHYCTLRST_W { w: self }
    }
    #[doc = "Bits 20:22 - DDRCKMOD"]
    #[inline(always)]
    pub fn ddrckmod(&mut self) -> DDRCKMOD_W {
        DDRCKMOD_W { w: self }
    }
    #[doc = "Bit 23 - GSKPMOD"]
    #[inline(always)]
    pub fn gskpmod(&mut self) -> GSKPMOD_W {
        GSKPMOD_W { w: self }
    }
    #[doc = "Bit 24 - GSKPCTRL"]
    #[inline(always)]
    pub fn gskpctrl(&mut self) -> GSKPCTRL_W {
        GSKPCTRL_W { w: self }
    }
    #[doc = "Bits 25:27 - DFILP_WIDTH"]
    #[inline(always)]
    pub fn dfilp_width(&mut self) -> DFILP_WIDTH_W {
        DFILP_WIDTH_W { w: self }
    }
    #[doc = "Bits 28:31 - GSKP_DUR"]
    #[inline(always)]
    pub fn gskp_dur(&mut self) -> GSKP_DUR_W {
        GSKP_DUR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ddritfcr](index.html) module"]
pub struct RCC_DDRITFCR_SPEC;
impl crate::RegisterSpec for RCC_DDRITFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ddritfcr::R](R) reader structure"]
impl crate::Readable for RCC_DDRITFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ddritfcr::W](W) writer structure"]
impl crate::Writable for RCC_DDRITFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_DDRITFCR to value 0x000f_d02a"]
impl crate::Resettable for RCC_DDRITFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000f_d02a
    }
}
