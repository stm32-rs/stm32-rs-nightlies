#[doc = "Register `ADC4R` reader"]
pub struct R(crate::R<ADC4R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC4R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC4R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC4R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC4R` writer"]
pub struct W(crate::W<ADC4R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC4R_SPEC>;
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
impl From<crate::W<ADC4R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC4R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD2TERST` reader - AD2TERST"]
pub struct AD2TERST_R(crate::FieldReader<bool, bool>);
impl AD2TERST_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TERST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TERST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TERST` writer - AD2TERST"]
pub struct AD2TERST_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TERST_W<'a> {
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
#[doc = "Field `AD2TEC4` reader - AD2TEC4"]
pub struct AD2TEC4_R(crate::FieldReader<bool, bool>);
impl AD2TEC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TEC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TEC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TEC4` writer - AD2TEC4"]
pub struct AD2TEC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TEC4_W<'a> {
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
#[doc = "Field `AD2TEC3` reader - AD2TEC3"]
pub struct AD2TEC3_R(crate::FieldReader<bool, bool>);
impl AD2TEC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TEC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TEC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TEC3` writer - AD2TEC3"]
pub struct AD2TEC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TEC3_W<'a> {
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
#[doc = "Field `AD2TEC2` reader - AD2TEC2"]
pub struct AD2TEC2_R(crate::FieldReader<bool, bool>);
impl AD2TEC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TEC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TEC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TEC2` writer - AD2TEC2"]
pub struct AD2TEC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TEC2_W<'a> {
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
#[doc = "Field `AD2TDRST` reader - AD2TDRST"]
pub struct AD2TDRST_R(crate::FieldReader<bool, bool>);
impl AD2TDRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TDRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TDRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TDRST` writer - AD2TDRST"]
pub struct AD2TDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TDRST_W<'a> {
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
#[doc = "Field `AD2TDPER` reader - AD2TDPER"]
pub struct AD2TDPER_R(crate::FieldReader<bool, bool>);
impl AD2TDPER_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TDPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TDPER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TDPER` writer - AD2TDPER"]
pub struct AD2TDPER_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TDPER_W<'a> {
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
#[doc = "Field `AD2TDC4` reader - AD2TDC4"]
pub struct AD2TDC4_R(crate::FieldReader<bool, bool>);
impl AD2TDC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TDC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TDC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TDC4` writer - AD2TDC4"]
pub struct AD2TDC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TDC4_W<'a> {
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
#[doc = "Field `AD2TDC3` reader - AD2TDC3"]
pub struct AD2TDC3_R(crate::FieldReader<bool, bool>);
impl AD2TDC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TDC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TDC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TDC3` writer - AD2TDC3"]
pub struct AD2TDC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TDC3_W<'a> {
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
#[doc = "Field `AD2TDC2` reader - AD2TDC2"]
pub struct AD2TDC2_R(crate::FieldReader<bool, bool>);
impl AD2TDC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TDC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TDC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TDC2` writer - AD2TDC2"]
pub struct AD2TDC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TDC2_W<'a> {
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
#[doc = "Field `AD2TCRST` reader - AD2TCRST"]
pub struct AD2TCRST_R(crate::FieldReader<bool, bool>);
impl AD2TCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TCRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TCRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TCRST` writer - AD2TCRST"]
pub struct AD2TCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TCRST_W<'a> {
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
#[doc = "Field `AD2TCPER` reader - AD2TCPER"]
pub struct AD2TCPER_R(crate::FieldReader<bool, bool>);
impl AD2TCPER_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TCPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TCPER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TCPER` writer - AD2TCPER"]
pub struct AD2TCPER_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TCPER_W<'a> {
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
#[doc = "Field `AD2TCC4` reader - AD2TCC4"]
pub struct AD2TCC4_R(crate::FieldReader<bool, bool>);
impl AD2TCC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TCC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TCC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TCC4` writer - AD2TCC4"]
pub struct AD2TCC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TCC4_W<'a> {
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
#[doc = "Field `AD2TCC3` reader - AD2TCC3"]
pub struct AD2TCC3_R(crate::FieldReader<bool, bool>);
impl AD2TCC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TCC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TCC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TCC3` writer - AD2TCC3"]
pub struct AD2TCC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TCC3_W<'a> {
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
#[doc = "Field `AD2TCC2` reader - AD2TCC2"]
pub struct AD2TCC2_R(crate::FieldReader<bool, bool>);
impl AD2TCC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TCC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TCC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TCC2` writer - AD2TCC2"]
pub struct AD2TCC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TCC2_W<'a> {
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
#[doc = "Field `AD2TBPER` reader - AD2TBPER"]
pub struct AD2TBPER_R(crate::FieldReader<bool, bool>);
impl AD2TBPER_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TBPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TBPER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TBPER` writer - AD2TBPER"]
pub struct AD2TBPER_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TBPER_W<'a> {
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
#[doc = "Field `AD2TBC4` reader - AD2TBC4"]
pub struct AD2TBC4_R(crate::FieldReader<bool, bool>);
impl AD2TBC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TBC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TBC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TBC4` writer - AD2TBC4"]
pub struct AD2TBC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TBC4_W<'a> {
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
#[doc = "Field `AD2TBC3` reader - AD2TBC3"]
pub struct AD2TBC3_R(crate::FieldReader<bool, bool>);
impl AD2TBC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TBC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TBC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TBC3` writer - AD2TBC3"]
pub struct AD2TBC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TBC3_W<'a> {
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
#[doc = "Field `AD2TBC2` reader - AD2TBC2"]
pub struct AD2TBC2_R(crate::FieldReader<bool, bool>);
impl AD2TBC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TBC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TBC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TBC2` writer - AD2TBC2"]
pub struct AD2TBC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TBC2_W<'a> {
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
#[doc = "Field `AD2TAPER` reader - AD2TAPER"]
pub struct AD2TAPER_R(crate::FieldReader<bool, bool>);
impl AD2TAPER_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TAPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TAPER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TAPER` writer - AD2TAPER"]
pub struct AD2TAPER_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TAPER_W<'a> {
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
#[doc = "Field `AD2TAC4` reader - AD2TAC4"]
pub struct AD2TAC4_R(crate::FieldReader<bool, bool>);
impl AD2TAC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TAC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TAC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TAC4` writer - AD2TAC4"]
pub struct AD2TAC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TAC4_W<'a> {
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
#[doc = "Field `AD2TAC3` reader - AD2TAC3"]
pub struct AD2TAC3_R(crate::FieldReader<bool, bool>);
impl AD2TAC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TAC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TAC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TAC3` writer - AD2TAC3"]
pub struct AD2TAC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TAC3_W<'a> {
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
#[doc = "Field `AD2TAC2` reader - AD2TAC2"]
pub struct AD2TAC2_R(crate::FieldReader<bool, bool>);
impl AD2TAC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2TAC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2TAC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2TAC2` writer - AD2TAC2"]
pub struct AD2TAC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TAC2_W<'a> {
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
#[doc = "Field `AD2EEV10` reader - AD2EEV10"]
pub struct AD2EEV10_R(crate::FieldReader<bool, bool>);
impl AD2EEV10_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2EEV10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2EEV10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2EEV10` writer - AD2EEV10"]
pub struct AD2EEV10_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2EEV10_W<'a> {
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
#[doc = "Field `AD2EEV9` reader - AD2EEV9"]
pub struct AD2EEV9_R(crate::FieldReader<bool, bool>);
impl AD2EEV9_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2EEV9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2EEV9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2EEV9` writer - AD2EEV9"]
pub struct AD2EEV9_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2EEV9_W<'a> {
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
#[doc = "Field `AD2EEV8` reader - AD2EEV8"]
pub struct AD2EEV8_R(crate::FieldReader<bool, bool>);
impl AD2EEV8_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2EEV8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2EEV8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2EEV8` writer - AD2EEV8"]
pub struct AD2EEV8_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2EEV8_W<'a> {
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
#[doc = "Field `AD2EEV7` reader - AD2EEV7"]
pub struct AD2EEV7_R(crate::FieldReader<bool, bool>);
impl AD2EEV7_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2EEV7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2EEV7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2EEV7` writer - AD2EEV7"]
pub struct AD2EEV7_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2EEV7_W<'a> {
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
#[doc = "Field `AD2EEV6` reader - AD2EEV6"]
pub struct AD2EEV6_R(crate::FieldReader<bool, bool>);
impl AD2EEV6_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2EEV6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2EEV6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2EEV6` writer - AD2EEV6"]
pub struct AD2EEV6_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2EEV6_W<'a> {
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
#[doc = "Field `AD2MPER` reader - AD2MPER"]
pub struct AD2MPER_R(crate::FieldReader<bool, bool>);
impl AD2MPER_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2MPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2MPER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2MPER` writer - AD2MPER"]
pub struct AD2MPER_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2MPER_W<'a> {
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
#[doc = "Field `AD2MC4` reader - AD2MC4"]
pub struct AD2MC4_R(crate::FieldReader<bool, bool>);
impl AD2MC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2MC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2MC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2MC4` writer - AD2MC4"]
pub struct AD2MC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2MC4_W<'a> {
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
#[doc = "Field `AD2MC3` reader - AD2MC3"]
pub struct AD2MC3_R(crate::FieldReader<bool, bool>);
impl AD2MC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2MC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2MC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2MC3` writer - AD2MC3"]
pub struct AD2MC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2MC3_W<'a> {
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
#[doc = "Field `AD2MC2` reader - AD2MC2"]
pub struct AD2MC2_R(crate::FieldReader<bool, bool>);
impl AD2MC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2MC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2MC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2MC2` writer - AD2MC2"]
pub struct AD2MC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2MC2_W<'a> {
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
#[doc = "Field `AD2MC1` reader - AD2MC1"]
pub struct AD2MC1_R(crate::FieldReader<bool, bool>);
impl AD2MC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD2MC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD2MC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD2MC1` writer - AD2MC1"]
pub struct AD2MC1_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2MC1_W<'a> {
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
    #[doc = "Bit 31 - AD2TERST"]
    #[inline(always)]
    pub fn ad2terst(&self) -> AD2TERST_R {
        AD2TERST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - AD2TEC4"]
    #[inline(always)]
    pub fn ad2tec4(&self) -> AD2TEC4_R {
        AD2TEC4_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - AD2TEC3"]
    #[inline(always)]
    pub fn ad2tec3(&self) -> AD2TEC3_R {
        AD2TEC3_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - AD2TEC2"]
    #[inline(always)]
    pub fn ad2tec2(&self) -> AD2TEC2_R {
        AD2TEC2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - AD2TDRST"]
    #[inline(always)]
    pub fn ad2tdrst(&self) -> AD2TDRST_R {
        AD2TDRST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - AD2TDPER"]
    #[inline(always)]
    pub fn ad2tdper(&self) -> AD2TDPER_R {
        AD2TDPER_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - AD2TDC4"]
    #[inline(always)]
    pub fn ad2tdc4(&self) -> AD2TDC4_R {
        AD2TDC4_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - AD2TDC3"]
    #[inline(always)]
    pub fn ad2tdc3(&self) -> AD2TDC3_R {
        AD2TDC3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - AD2TDC2"]
    #[inline(always)]
    pub fn ad2tdc2(&self) -> AD2TDC2_R {
        AD2TDC2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - AD2TCRST"]
    #[inline(always)]
    pub fn ad2tcrst(&self) -> AD2TCRST_R {
        AD2TCRST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - AD2TCPER"]
    #[inline(always)]
    pub fn ad2tcper(&self) -> AD2TCPER_R {
        AD2TCPER_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - AD2TCC4"]
    #[inline(always)]
    pub fn ad2tcc4(&self) -> AD2TCC4_R {
        AD2TCC4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - AD2TCC3"]
    #[inline(always)]
    pub fn ad2tcc3(&self) -> AD2TCC3_R {
        AD2TCC3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - AD2TCC2"]
    #[inline(always)]
    pub fn ad2tcc2(&self) -> AD2TCC2_R {
        AD2TCC2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - AD2TBPER"]
    #[inline(always)]
    pub fn ad2tbper(&self) -> AD2TBPER_R {
        AD2TBPER_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AD2TBC4"]
    #[inline(always)]
    pub fn ad2tbc4(&self) -> AD2TBC4_R {
        AD2TBC4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AD2TBC3"]
    #[inline(always)]
    pub fn ad2tbc3(&self) -> AD2TBC3_R {
        AD2TBC3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AD2TBC2"]
    #[inline(always)]
    pub fn ad2tbc2(&self) -> AD2TBC2_R {
        AD2TBC2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AD2TAPER"]
    #[inline(always)]
    pub fn ad2taper(&self) -> AD2TAPER_R {
        AD2TAPER_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AD2TAC4"]
    #[inline(always)]
    pub fn ad2tac4(&self) -> AD2TAC4_R {
        AD2TAC4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AD2TAC3"]
    #[inline(always)]
    pub fn ad2tac3(&self) -> AD2TAC3_R {
        AD2TAC3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AD2TAC2"]
    #[inline(always)]
    pub fn ad2tac2(&self) -> AD2TAC2_R {
        AD2TAC2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AD2EEV10"]
    #[inline(always)]
    pub fn ad2eev10(&self) -> AD2EEV10_R {
        AD2EEV10_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AD2EEV9"]
    #[inline(always)]
    pub fn ad2eev9(&self) -> AD2EEV9_R {
        AD2EEV9_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AD2EEV8"]
    #[inline(always)]
    pub fn ad2eev8(&self) -> AD2EEV8_R {
        AD2EEV8_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AD2EEV7"]
    #[inline(always)]
    pub fn ad2eev7(&self) -> AD2EEV7_R {
        AD2EEV7_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AD2EEV6"]
    #[inline(always)]
    pub fn ad2eev6(&self) -> AD2EEV6_R {
        AD2EEV6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AD2MPER"]
    #[inline(always)]
    pub fn ad2mper(&self) -> AD2MPER_R {
        AD2MPER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AD2MC4"]
    #[inline(always)]
    pub fn ad2mc4(&self) -> AD2MC4_R {
        AD2MC4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AD2MC3"]
    #[inline(always)]
    pub fn ad2mc3(&self) -> AD2MC3_R {
        AD2MC3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - AD2MC2"]
    #[inline(always)]
    pub fn ad2mc2(&self) -> AD2MC2_R {
        AD2MC2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - AD2MC1"]
    #[inline(always)]
    pub fn ad2mc1(&self) -> AD2MC1_R {
        AD2MC1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - AD2TERST"]
    #[inline(always)]
    pub fn ad2terst(&mut self) -> AD2TERST_W {
        AD2TERST_W { w: self }
    }
    #[doc = "Bit 30 - AD2TEC4"]
    #[inline(always)]
    pub fn ad2tec4(&mut self) -> AD2TEC4_W {
        AD2TEC4_W { w: self }
    }
    #[doc = "Bit 29 - AD2TEC3"]
    #[inline(always)]
    pub fn ad2tec3(&mut self) -> AD2TEC3_W {
        AD2TEC3_W { w: self }
    }
    #[doc = "Bit 28 - AD2TEC2"]
    #[inline(always)]
    pub fn ad2tec2(&mut self) -> AD2TEC2_W {
        AD2TEC2_W { w: self }
    }
    #[doc = "Bit 27 - AD2TDRST"]
    #[inline(always)]
    pub fn ad2tdrst(&mut self) -> AD2TDRST_W {
        AD2TDRST_W { w: self }
    }
    #[doc = "Bit 26 - AD2TDPER"]
    #[inline(always)]
    pub fn ad2tdper(&mut self) -> AD2TDPER_W {
        AD2TDPER_W { w: self }
    }
    #[doc = "Bit 25 - AD2TDC4"]
    #[inline(always)]
    pub fn ad2tdc4(&mut self) -> AD2TDC4_W {
        AD2TDC4_W { w: self }
    }
    #[doc = "Bit 24 - AD2TDC3"]
    #[inline(always)]
    pub fn ad2tdc3(&mut self) -> AD2TDC3_W {
        AD2TDC3_W { w: self }
    }
    #[doc = "Bit 23 - AD2TDC2"]
    #[inline(always)]
    pub fn ad2tdc2(&mut self) -> AD2TDC2_W {
        AD2TDC2_W { w: self }
    }
    #[doc = "Bit 22 - AD2TCRST"]
    #[inline(always)]
    pub fn ad2tcrst(&mut self) -> AD2TCRST_W {
        AD2TCRST_W { w: self }
    }
    #[doc = "Bit 21 - AD2TCPER"]
    #[inline(always)]
    pub fn ad2tcper(&mut self) -> AD2TCPER_W {
        AD2TCPER_W { w: self }
    }
    #[doc = "Bit 20 - AD2TCC4"]
    #[inline(always)]
    pub fn ad2tcc4(&mut self) -> AD2TCC4_W {
        AD2TCC4_W { w: self }
    }
    #[doc = "Bit 19 - AD2TCC3"]
    #[inline(always)]
    pub fn ad2tcc3(&mut self) -> AD2TCC3_W {
        AD2TCC3_W { w: self }
    }
    #[doc = "Bit 18 - AD2TCC2"]
    #[inline(always)]
    pub fn ad2tcc2(&mut self) -> AD2TCC2_W {
        AD2TCC2_W { w: self }
    }
    #[doc = "Bit 17 - AD2TBPER"]
    #[inline(always)]
    pub fn ad2tbper(&mut self) -> AD2TBPER_W {
        AD2TBPER_W { w: self }
    }
    #[doc = "Bit 16 - AD2TBC4"]
    #[inline(always)]
    pub fn ad2tbc4(&mut self) -> AD2TBC4_W {
        AD2TBC4_W { w: self }
    }
    #[doc = "Bit 15 - AD2TBC3"]
    #[inline(always)]
    pub fn ad2tbc3(&mut self) -> AD2TBC3_W {
        AD2TBC3_W { w: self }
    }
    #[doc = "Bit 14 - AD2TBC2"]
    #[inline(always)]
    pub fn ad2tbc2(&mut self) -> AD2TBC2_W {
        AD2TBC2_W { w: self }
    }
    #[doc = "Bit 13 - AD2TAPER"]
    #[inline(always)]
    pub fn ad2taper(&mut self) -> AD2TAPER_W {
        AD2TAPER_W { w: self }
    }
    #[doc = "Bit 12 - AD2TAC4"]
    #[inline(always)]
    pub fn ad2tac4(&mut self) -> AD2TAC4_W {
        AD2TAC4_W { w: self }
    }
    #[doc = "Bit 11 - AD2TAC3"]
    #[inline(always)]
    pub fn ad2tac3(&mut self) -> AD2TAC3_W {
        AD2TAC3_W { w: self }
    }
    #[doc = "Bit 10 - AD2TAC2"]
    #[inline(always)]
    pub fn ad2tac2(&mut self) -> AD2TAC2_W {
        AD2TAC2_W { w: self }
    }
    #[doc = "Bit 9 - AD2EEV10"]
    #[inline(always)]
    pub fn ad2eev10(&mut self) -> AD2EEV10_W {
        AD2EEV10_W { w: self }
    }
    #[doc = "Bit 8 - AD2EEV9"]
    #[inline(always)]
    pub fn ad2eev9(&mut self) -> AD2EEV9_W {
        AD2EEV9_W { w: self }
    }
    #[doc = "Bit 7 - AD2EEV8"]
    #[inline(always)]
    pub fn ad2eev8(&mut self) -> AD2EEV8_W {
        AD2EEV8_W { w: self }
    }
    #[doc = "Bit 6 - AD2EEV7"]
    #[inline(always)]
    pub fn ad2eev7(&mut self) -> AD2EEV7_W {
        AD2EEV7_W { w: self }
    }
    #[doc = "Bit 5 - AD2EEV6"]
    #[inline(always)]
    pub fn ad2eev6(&mut self) -> AD2EEV6_W {
        AD2EEV6_W { w: self }
    }
    #[doc = "Bit 4 - AD2MPER"]
    #[inline(always)]
    pub fn ad2mper(&mut self) -> AD2MPER_W {
        AD2MPER_W { w: self }
    }
    #[doc = "Bit 3 - AD2MC4"]
    #[inline(always)]
    pub fn ad2mc4(&mut self) -> AD2MC4_W {
        AD2MC4_W { w: self }
    }
    #[doc = "Bit 2 - AD2MC3"]
    #[inline(always)]
    pub fn ad2mc3(&mut self) -> AD2MC3_W {
        AD2MC3_W { w: self }
    }
    #[doc = "Bit 1 - AD2MC2"]
    #[inline(always)]
    pub fn ad2mc2(&mut self) -> AD2MC2_W {
        AD2MC2_W { w: self }
    }
    #[doc = "Bit 0 - AD2MC1"]
    #[inline(always)]
    pub fn ad2mc1(&mut self) -> AD2MC1_W {
        AD2MC1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Trigger 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc4r](index.html) module"]
pub struct ADC4R_SPEC;
impl crate::RegisterSpec for ADC4R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc4r::R](R) reader structure"]
impl crate::Readable for ADC4R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc4r::W](W) writer structure"]
impl crate::Writable for ADC4R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC4R to value 0"]
impl crate::Resettable for ADC4R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
