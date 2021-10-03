#[doc = "Register `DAC_CR` reader"]
pub struct R(crate::R<DAC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_CR` writer"]
pub struct W(crate::W<DAC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_CR_SPEC>;
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
impl From<crate::W<DAC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN1` reader - EN1"]
pub struct EN1_R(crate::FieldReader<bool, bool>);
impl EN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN1` writer - EN1"]
pub struct EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1_W<'a> {
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
#[doc = "Field `TEN1` reader - TEN1"]
pub struct TEN1_R(crate::FieldReader<bool, bool>);
impl TEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEN1` writer - TEN1"]
pub struct TEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN1_W<'a> {
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
#[doc = "Field `TSEL10` reader - TSEL10"]
pub struct TSEL10_R(crate::FieldReader<bool, bool>);
impl TSEL10_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSEL10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEL10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEL10` writer - TSEL10"]
pub struct TSEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL10_W<'a> {
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
#[doc = "Field `TSEL11` reader - TSEL11"]
pub struct TSEL11_R(crate::FieldReader<bool, bool>);
impl TSEL11_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSEL11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEL11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEL11` writer - TSEL11"]
pub struct TSEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL11_W<'a> {
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
#[doc = "Field `TSEL12` reader - TSEL12"]
pub struct TSEL12_R(crate::FieldReader<bool, bool>);
impl TSEL12_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSEL12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEL12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEL12` writer - TSEL12"]
pub struct TSEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL12_W<'a> {
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
#[doc = "Field `TSEL13` reader - TSEL13"]
pub struct TSEL13_R(crate::FieldReader<bool, bool>);
impl TSEL13_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSEL13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEL13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEL13` writer - TSEL13"]
pub struct TSEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL13_W<'a> {
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
#[doc = "Field `WAVE1` reader - WAVE1"]
pub struct WAVE1_R(crate::FieldReader<u8, u8>);
impl WAVE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAVE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAVE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVE1` writer - WAVE1"]
pub struct WAVE1_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `MAMP1` reader - MAMP1"]
pub struct MAMP1_R(crate::FieldReader<u8, u8>);
impl MAMP1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAMP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAMP1` writer - MAMP1"]
pub struct MAMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MAMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `DMAEN1` reader - DMAEN1"]
pub struct DMAEN1_R(crate::FieldReader<bool, bool>);
impl DMAEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN1` writer - DMAEN1"]
pub struct DMAEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN1_W<'a> {
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
#[doc = "Field `DMAUDRIE1` reader - DMAUDRIE1"]
pub struct DMAUDRIE1_R(crate::FieldReader<bool, bool>);
impl DMAUDRIE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAUDRIE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAUDRIE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAUDRIE1` writer - DMAUDRIE1"]
pub struct DMAUDRIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDRIE1_W<'a> {
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
#[doc = "Field `CEN1` reader - CEN1"]
pub struct CEN1_R(crate::FieldReader<bool, bool>);
impl CEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEN1` writer - CEN1"]
pub struct CEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN1_W<'a> {
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
#[doc = "Field `HFSEL` reader - HFSEL"]
pub struct HFSEL_R(crate::FieldReader<bool, bool>);
impl HFSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFSEL` writer - HFSEL"]
pub struct HFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HFSEL_W<'a> {
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
#[doc = "Field `EN2` reader - EN2"]
pub struct EN2_R(crate::FieldReader<bool, bool>);
impl EN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN2` writer - EN2"]
pub struct EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EN2_W<'a> {
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
#[doc = "Field `TEN2` reader - TEN2"]
pub struct TEN2_R(crate::FieldReader<bool, bool>);
impl TEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEN2` writer - TEN2"]
pub struct TEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN2_W<'a> {
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
#[doc = "Field `TSEL20` reader - TSEL20"]
pub struct TSEL20_R(crate::FieldReader<bool, bool>);
impl TSEL20_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSEL20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEL20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEL20` writer - TSEL20"]
pub struct TSEL20_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL20_W<'a> {
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
#[doc = "Field `TSEL21` reader - TSEL21"]
pub struct TSEL21_R(crate::FieldReader<bool, bool>);
impl TSEL21_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSEL21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEL21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEL21` writer - TSEL21"]
pub struct TSEL21_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL21_W<'a> {
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
#[doc = "Field `TSEL22` reader - TSEL22"]
pub struct TSEL22_R(crate::FieldReader<bool, bool>);
impl TSEL22_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSEL22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEL22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEL22` writer - TSEL22"]
pub struct TSEL22_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL22_W<'a> {
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
#[doc = "Field `TSEL23` reader - TSEL23"]
pub struct TSEL23_R(crate::FieldReader<bool, bool>);
impl TSEL23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSEL23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEL23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEL23` writer - TSEL23"]
pub struct TSEL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL23_W<'a> {
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
#[doc = "Field `WAVE2` reader - WAVE2"]
pub struct WAVE2_R(crate::FieldReader<u8, u8>);
impl WAVE2_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAVE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAVE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVE2` writer - WAVE2"]
pub struct WAVE2_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `MAMP2` reader - MAMP2"]
pub struct MAMP2_R(crate::FieldReader<u8, u8>);
impl MAMP2_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAMP2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAMP2` writer - MAMP2"]
pub struct MAMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> MAMP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `DMAEN2` reader - DMAEN2"]
pub struct DMAEN2_R(crate::FieldReader<bool, bool>);
impl DMAEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN2` writer - DMAEN2"]
pub struct DMAEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN2_W<'a> {
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
#[doc = "Field `DMAUDRIE2` reader - DMAUDRIE2"]
pub struct DMAUDRIE2_R(crate::FieldReader<bool, bool>);
impl DMAUDRIE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAUDRIE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAUDRIE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAUDRIE2` writer - DMAUDRIE2"]
pub struct DMAUDRIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDRIE2_W<'a> {
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
#[doc = "Field `CEN2` reader - CEN2"]
pub struct CEN2_R(crate::FieldReader<bool, bool>);
impl CEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEN2` writer - CEN2"]
pub struct CEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN2_W<'a> {
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
impl R {
    #[doc = "Bit 0 - EN1"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TEN1"]
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TSEL10"]
    #[inline(always)]
    pub fn tsel10(&self) -> TSEL10_R {
        TSEL10_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TSEL11"]
    #[inline(always)]
    pub fn tsel11(&self) -> TSEL11_R {
        TSEL11_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TSEL12"]
    #[inline(always)]
    pub fn tsel12(&self) -> TSEL12_R {
        TSEL12_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TSEL13"]
    #[inline(always)]
    pub fn tsel13(&self) -> TSEL13_R {
        TSEL13_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - WAVE1"]
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - MAMP1"]
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DMAEN1"]
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DMAUDRIE1"]
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CEN1"]
    #[inline(always)]
    pub fn cen1(&self) -> CEN1_R {
        CEN1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - HFSEL"]
    #[inline(always)]
    pub fn hfsel(&self) -> HFSEL_R {
        HFSEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - EN2"]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TEN2"]
    #[inline(always)]
    pub fn ten2(&self) -> TEN2_R {
        TEN2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TSEL20"]
    #[inline(always)]
    pub fn tsel20(&self) -> TSEL20_R {
        TSEL20_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TSEL21"]
    #[inline(always)]
    pub fn tsel21(&self) -> TSEL21_R {
        TSEL21_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TSEL22"]
    #[inline(always)]
    pub fn tsel22(&self) -> TSEL22_R {
        TSEL22_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TSEL23"]
    #[inline(always)]
    pub fn tsel23(&self) -> TSEL23_R {
        TSEL23_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - WAVE2"]
    #[inline(always)]
    pub fn wave2(&self) -> WAVE2_R {
        WAVE2_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - MAMP2"]
    #[inline(always)]
    pub fn mamp2(&self) -> MAMP2_R {
        MAMP2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DMAEN2"]
    #[inline(always)]
    pub fn dmaen2(&self) -> DMAEN2_R {
        DMAEN2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DMAUDRIE2"]
    #[inline(always)]
    pub fn dmaudrie2(&self) -> DMAUDRIE2_R {
        DMAUDRIE2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - CEN2"]
    #[inline(always)]
    pub fn cen2(&self) -> CEN2_R {
        CEN2_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN1"]
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W {
        EN1_W { w: self }
    }
    #[doc = "Bit 1 - TEN1"]
    #[inline(always)]
    pub fn ten1(&mut self) -> TEN1_W {
        TEN1_W { w: self }
    }
    #[doc = "Bit 2 - TSEL10"]
    #[inline(always)]
    pub fn tsel10(&mut self) -> TSEL10_W {
        TSEL10_W { w: self }
    }
    #[doc = "Bit 3 - TSEL11"]
    #[inline(always)]
    pub fn tsel11(&mut self) -> TSEL11_W {
        TSEL11_W { w: self }
    }
    #[doc = "Bit 4 - TSEL12"]
    #[inline(always)]
    pub fn tsel12(&mut self) -> TSEL12_W {
        TSEL12_W { w: self }
    }
    #[doc = "Bit 5 - TSEL13"]
    #[inline(always)]
    pub fn tsel13(&mut self) -> TSEL13_W {
        TSEL13_W { w: self }
    }
    #[doc = "Bits 6:7 - WAVE1"]
    #[inline(always)]
    pub fn wave1(&mut self) -> WAVE1_W {
        WAVE1_W { w: self }
    }
    #[doc = "Bits 8:11 - MAMP1"]
    #[inline(always)]
    pub fn mamp1(&mut self) -> MAMP1_W {
        MAMP1_W { w: self }
    }
    #[doc = "Bit 12 - DMAEN1"]
    #[inline(always)]
    pub fn dmaen1(&mut self) -> DMAEN1_W {
        DMAEN1_W { w: self }
    }
    #[doc = "Bit 13 - DMAUDRIE1"]
    #[inline(always)]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE1_W {
        DMAUDRIE1_W { w: self }
    }
    #[doc = "Bit 14 - CEN1"]
    #[inline(always)]
    pub fn cen1(&mut self) -> CEN1_W {
        CEN1_W { w: self }
    }
    #[doc = "Bit 15 - HFSEL"]
    #[inline(always)]
    pub fn hfsel(&mut self) -> HFSEL_W {
        HFSEL_W { w: self }
    }
    #[doc = "Bit 16 - EN2"]
    #[inline(always)]
    pub fn en2(&mut self) -> EN2_W {
        EN2_W { w: self }
    }
    #[doc = "Bit 17 - TEN2"]
    #[inline(always)]
    pub fn ten2(&mut self) -> TEN2_W {
        TEN2_W { w: self }
    }
    #[doc = "Bit 18 - TSEL20"]
    #[inline(always)]
    pub fn tsel20(&mut self) -> TSEL20_W {
        TSEL20_W { w: self }
    }
    #[doc = "Bit 19 - TSEL21"]
    #[inline(always)]
    pub fn tsel21(&mut self) -> TSEL21_W {
        TSEL21_W { w: self }
    }
    #[doc = "Bit 20 - TSEL22"]
    #[inline(always)]
    pub fn tsel22(&mut self) -> TSEL22_W {
        TSEL22_W { w: self }
    }
    #[doc = "Bit 21 - TSEL23"]
    #[inline(always)]
    pub fn tsel23(&mut self) -> TSEL23_W {
        TSEL23_W { w: self }
    }
    #[doc = "Bits 22:23 - WAVE2"]
    #[inline(always)]
    pub fn wave2(&mut self) -> WAVE2_W {
        WAVE2_W { w: self }
    }
    #[doc = "Bits 24:27 - MAMP2"]
    #[inline(always)]
    pub fn mamp2(&mut self) -> MAMP2_W {
        MAMP2_W { w: self }
    }
    #[doc = "Bit 28 - DMAEN2"]
    #[inline(always)]
    pub fn dmaen2(&mut self) -> DMAEN2_W {
        DMAEN2_W { w: self }
    }
    #[doc = "Bit 29 - DMAUDRIE2"]
    #[inline(always)]
    pub fn dmaudrie2(&mut self) -> DMAUDRIE2_W {
        DMAUDRIE2_W { w: self }
    }
    #[doc = "Bit 30 - CEN2"]
    #[inline(always)]
    pub fn cen2(&mut self) -> CEN2_W {
        CEN2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_cr](index.html) module"]
pub struct DAC_CR_SPEC;
impl crate::RegisterSpec for DAC_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_cr::R](R) reader structure"]
impl crate::Readable for DAC_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_cr::W](W) writer structure"]
impl crate::Writable for DAC_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_CR to value 0"]
impl crate::Resettable for DAC_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
