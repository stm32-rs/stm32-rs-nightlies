#[doc = "Register `ADC_PCSEL` reader"]
pub struct R(crate::R<ADC_PCSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_PCSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_PCSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_PCSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_PCSEL` writer"]
pub struct W(crate::W<ADC_PCSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_PCSEL_SPEC>;
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
impl From<crate::W<ADC_PCSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_PCSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCSEL0` reader - PCSEL0"]
pub struct PCSEL0_R(crate::FieldReader<bool, bool>);
impl PCSEL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL0` writer - PCSEL0"]
pub struct PCSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL0_W<'a> {
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
#[doc = "Field `PCSEL1` reader - PCSEL1"]
pub struct PCSEL1_R(crate::FieldReader<bool, bool>);
impl PCSEL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL1` writer - PCSEL1"]
pub struct PCSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL1_W<'a> {
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
#[doc = "Field `PCSEL2` reader - PCSEL2"]
pub struct PCSEL2_R(crate::FieldReader<bool, bool>);
impl PCSEL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL2` writer - PCSEL2"]
pub struct PCSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL2_W<'a> {
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
#[doc = "Field `PCSEL3` reader - PCSEL3"]
pub struct PCSEL3_R(crate::FieldReader<bool, bool>);
impl PCSEL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL3` writer - PCSEL3"]
pub struct PCSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL3_W<'a> {
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
#[doc = "Field `PCSEL4` reader - PCSEL4"]
pub struct PCSEL4_R(crate::FieldReader<bool, bool>);
impl PCSEL4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL4` writer - PCSEL4"]
pub struct PCSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL4_W<'a> {
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
#[doc = "Field `PCSEL5` reader - PCSEL5"]
pub struct PCSEL5_R(crate::FieldReader<bool, bool>);
impl PCSEL5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL5` writer - PCSEL5"]
pub struct PCSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL5_W<'a> {
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
#[doc = "Field `PCSEL6` reader - PCSEL6"]
pub struct PCSEL6_R(crate::FieldReader<bool, bool>);
impl PCSEL6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL6` writer - PCSEL6"]
pub struct PCSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL6_W<'a> {
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
#[doc = "Field `PCSEL7` reader - PCSEL7"]
pub struct PCSEL7_R(crate::FieldReader<bool, bool>);
impl PCSEL7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL7` writer - PCSEL7"]
pub struct PCSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL7_W<'a> {
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
#[doc = "Field `PCSEL8` reader - PCSEL8"]
pub struct PCSEL8_R(crate::FieldReader<bool, bool>);
impl PCSEL8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL8` writer - PCSEL8"]
pub struct PCSEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL8_W<'a> {
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
#[doc = "Field `PCSEL9` reader - PCSEL9"]
pub struct PCSEL9_R(crate::FieldReader<bool, bool>);
impl PCSEL9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL9` writer - PCSEL9"]
pub struct PCSEL9_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL9_W<'a> {
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
#[doc = "Field `PCSEL10` reader - PCSEL10"]
pub struct PCSEL10_R(crate::FieldReader<bool, bool>);
impl PCSEL10_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL10` writer - PCSEL10"]
pub struct PCSEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL10_W<'a> {
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
#[doc = "Field `PCSEL11` reader - PCSEL11"]
pub struct PCSEL11_R(crate::FieldReader<bool, bool>);
impl PCSEL11_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL11` writer - PCSEL11"]
pub struct PCSEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL11_W<'a> {
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
#[doc = "Field `PCSEL12` reader - PCSEL12"]
pub struct PCSEL12_R(crate::FieldReader<bool, bool>);
impl PCSEL12_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL12` writer - PCSEL12"]
pub struct PCSEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL12_W<'a> {
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
#[doc = "Field `PCSEL13` reader - PCSEL13"]
pub struct PCSEL13_R(crate::FieldReader<bool, bool>);
impl PCSEL13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL13` writer - PCSEL13"]
pub struct PCSEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL13_W<'a> {
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
#[doc = "Field `PCSEL14` reader - PCSEL14"]
pub struct PCSEL14_R(crate::FieldReader<bool, bool>);
impl PCSEL14_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL14` writer - PCSEL14"]
pub struct PCSEL14_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL14_W<'a> {
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
#[doc = "Field `PCSEL15` reader - PCSEL15"]
pub struct PCSEL15_R(crate::FieldReader<bool, bool>);
impl PCSEL15_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL15` writer - PCSEL15"]
pub struct PCSEL15_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL15_W<'a> {
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
#[doc = "Field `PCSEL16` reader - PCSEL16"]
pub struct PCSEL16_R(crate::FieldReader<bool, bool>);
impl PCSEL16_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL16` writer - PCSEL16"]
pub struct PCSEL16_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL16_W<'a> {
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
#[doc = "Field `PCSEL17` reader - PCSEL17"]
pub struct PCSEL17_R(crate::FieldReader<bool, bool>);
impl PCSEL17_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL17` writer - PCSEL17"]
pub struct PCSEL17_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL17_W<'a> {
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
#[doc = "Field `PCSEL18` reader - PCSEL18"]
pub struct PCSEL18_R(crate::FieldReader<bool, bool>);
impl PCSEL18_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL18` writer - PCSEL18"]
pub struct PCSEL18_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL18_W<'a> {
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
#[doc = "Field `PCSEL19` reader - PCSEL19"]
pub struct PCSEL19_R(crate::FieldReader<bool, bool>);
impl PCSEL19_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSEL19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSEL19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSEL19` writer - PCSEL19"]
pub struct PCSEL19_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL19_W<'a> {
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
impl R {
    #[doc = "Bit 0 - PCSEL0"]
    #[inline(always)]
    pub fn pcsel0(&self) -> PCSEL0_R {
        PCSEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PCSEL1"]
    #[inline(always)]
    pub fn pcsel1(&self) -> PCSEL1_R {
        PCSEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PCSEL2"]
    #[inline(always)]
    pub fn pcsel2(&self) -> PCSEL2_R {
        PCSEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PCSEL3"]
    #[inline(always)]
    pub fn pcsel3(&self) -> PCSEL3_R {
        PCSEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PCSEL4"]
    #[inline(always)]
    pub fn pcsel4(&self) -> PCSEL4_R {
        PCSEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PCSEL5"]
    #[inline(always)]
    pub fn pcsel5(&self) -> PCSEL5_R {
        PCSEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PCSEL6"]
    #[inline(always)]
    pub fn pcsel6(&self) -> PCSEL6_R {
        PCSEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PCSEL7"]
    #[inline(always)]
    pub fn pcsel7(&self) -> PCSEL7_R {
        PCSEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PCSEL8"]
    #[inline(always)]
    pub fn pcsel8(&self) -> PCSEL8_R {
        PCSEL8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PCSEL9"]
    #[inline(always)]
    pub fn pcsel9(&self) -> PCSEL9_R {
        PCSEL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PCSEL10"]
    #[inline(always)]
    pub fn pcsel10(&self) -> PCSEL10_R {
        PCSEL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PCSEL11"]
    #[inline(always)]
    pub fn pcsel11(&self) -> PCSEL11_R {
        PCSEL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PCSEL12"]
    #[inline(always)]
    pub fn pcsel12(&self) -> PCSEL12_R {
        PCSEL12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PCSEL13"]
    #[inline(always)]
    pub fn pcsel13(&self) -> PCSEL13_R {
        PCSEL13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PCSEL14"]
    #[inline(always)]
    pub fn pcsel14(&self) -> PCSEL14_R {
        PCSEL14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PCSEL15"]
    #[inline(always)]
    pub fn pcsel15(&self) -> PCSEL15_R {
        PCSEL15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PCSEL16"]
    #[inline(always)]
    pub fn pcsel16(&self) -> PCSEL16_R {
        PCSEL16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PCSEL17"]
    #[inline(always)]
    pub fn pcsel17(&self) -> PCSEL17_R {
        PCSEL17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PCSEL18"]
    #[inline(always)]
    pub fn pcsel18(&self) -> PCSEL18_R {
        PCSEL18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PCSEL19"]
    #[inline(always)]
    pub fn pcsel19(&self) -> PCSEL19_R {
        PCSEL19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCSEL0"]
    #[inline(always)]
    pub fn pcsel0(&mut self) -> PCSEL0_W {
        PCSEL0_W { w: self }
    }
    #[doc = "Bit 1 - PCSEL1"]
    #[inline(always)]
    pub fn pcsel1(&mut self) -> PCSEL1_W {
        PCSEL1_W { w: self }
    }
    #[doc = "Bit 2 - PCSEL2"]
    #[inline(always)]
    pub fn pcsel2(&mut self) -> PCSEL2_W {
        PCSEL2_W { w: self }
    }
    #[doc = "Bit 3 - PCSEL3"]
    #[inline(always)]
    pub fn pcsel3(&mut self) -> PCSEL3_W {
        PCSEL3_W { w: self }
    }
    #[doc = "Bit 4 - PCSEL4"]
    #[inline(always)]
    pub fn pcsel4(&mut self) -> PCSEL4_W {
        PCSEL4_W { w: self }
    }
    #[doc = "Bit 5 - PCSEL5"]
    #[inline(always)]
    pub fn pcsel5(&mut self) -> PCSEL5_W {
        PCSEL5_W { w: self }
    }
    #[doc = "Bit 6 - PCSEL6"]
    #[inline(always)]
    pub fn pcsel6(&mut self) -> PCSEL6_W {
        PCSEL6_W { w: self }
    }
    #[doc = "Bit 7 - PCSEL7"]
    #[inline(always)]
    pub fn pcsel7(&mut self) -> PCSEL7_W {
        PCSEL7_W { w: self }
    }
    #[doc = "Bit 8 - PCSEL8"]
    #[inline(always)]
    pub fn pcsel8(&mut self) -> PCSEL8_W {
        PCSEL8_W { w: self }
    }
    #[doc = "Bit 9 - PCSEL9"]
    #[inline(always)]
    pub fn pcsel9(&mut self) -> PCSEL9_W {
        PCSEL9_W { w: self }
    }
    #[doc = "Bit 10 - PCSEL10"]
    #[inline(always)]
    pub fn pcsel10(&mut self) -> PCSEL10_W {
        PCSEL10_W { w: self }
    }
    #[doc = "Bit 11 - PCSEL11"]
    #[inline(always)]
    pub fn pcsel11(&mut self) -> PCSEL11_W {
        PCSEL11_W { w: self }
    }
    #[doc = "Bit 12 - PCSEL12"]
    #[inline(always)]
    pub fn pcsel12(&mut self) -> PCSEL12_W {
        PCSEL12_W { w: self }
    }
    #[doc = "Bit 13 - PCSEL13"]
    #[inline(always)]
    pub fn pcsel13(&mut self) -> PCSEL13_W {
        PCSEL13_W { w: self }
    }
    #[doc = "Bit 14 - PCSEL14"]
    #[inline(always)]
    pub fn pcsel14(&mut self) -> PCSEL14_W {
        PCSEL14_W { w: self }
    }
    #[doc = "Bit 15 - PCSEL15"]
    #[inline(always)]
    pub fn pcsel15(&mut self) -> PCSEL15_W {
        PCSEL15_W { w: self }
    }
    #[doc = "Bit 16 - PCSEL16"]
    #[inline(always)]
    pub fn pcsel16(&mut self) -> PCSEL16_W {
        PCSEL16_W { w: self }
    }
    #[doc = "Bit 17 - PCSEL17"]
    #[inline(always)]
    pub fn pcsel17(&mut self) -> PCSEL17_W {
        PCSEL17_W { w: self }
    }
    #[doc = "Bit 18 - PCSEL18"]
    #[inline(always)]
    pub fn pcsel18(&mut self) -> PCSEL18_W {
        PCSEL18_W { w: self }
    }
    #[doc = "Bit 19 - PCSEL19"]
    #[inline(always)]
    pub fn pcsel19(&mut self) -> PCSEL19_W {
        PCSEL19_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC channel preselection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_pcsel](index.html) module"]
pub struct ADC_PCSEL_SPEC;
impl crate::RegisterSpec for ADC_PCSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_pcsel::R](R) reader structure"]
impl crate::Readable for ADC_PCSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_pcsel::W](W) writer structure"]
impl crate::Writable for ADC_PCSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_PCSEL to value 0"]
impl crate::Resettable for ADC_PCSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
