#[doc = "Register `FPR1` reader"]
pub struct R(crate::R<FPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPR1` writer"]
pub struct W(crate::W<FPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPR1_SPEC>;
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
impl From<crate::W<FPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPIF0` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF0_R(crate::FieldReader<bool, bool>);
impl FPIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF0` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF0_W<'a> {
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
#[doc = "Field `FPIF1` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF1_R(crate::FieldReader<bool, bool>);
impl FPIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF1` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF1_W<'a> {
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
#[doc = "Field `FPIF2` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF2_R(crate::FieldReader<bool, bool>);
impl FPIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF2` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF2_W<'a> {
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
#[doc = "Field `FPIF3` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF3_R(crate::FieldReader<bool, bool>);
impl FPIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF3` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF3_W<'a> {
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
#[doc = "Field `FPIF4` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF4_R(crate::FieldReader<bool, bool>);
impl FPIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF4` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF4_W<'a> {
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
#[doc = "Field `FPIF5` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF5_R(crate::FieldReader<bool, bool>);
impl FPIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF5` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF5_W<'a> {
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
#[doc = "Field `FPIF6` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF6_R(crate::FieldReader<bool, bool>);
impl FPIF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF6` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF6_W<'a> {
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
#[doc = "Field `FPIF7` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF7_R(crate::FieldReader<bool, bool>);
impl FPIF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF7` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF7_W<'a> {
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
#[doc = "Field `FPIF8` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF8_R(crate::FieldReader<bool, bool>);
impl FPIF8_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF8` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF8_W<'a> {
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
#[doc = "Field `FPIF9` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF9_R(crate::FieldReader<bool, bool>);
impl FPIF9_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF9` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF9_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF9_W<'a> {
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
#[doc = "Field `FPIF10` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF10_R(crate::FieldReader<bool, bool>);
impl FPIF10_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF10` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF10_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF10_W<'a> {
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
#[doc = "Field `FPIF11` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF11_R(crate::FieldReader<bool, bool>);
impl FPIF11_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF11` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF11_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF11_W<'a> {
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
#[doc = "Field `FPIF12` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF12_R(crate::FieldReader<bool, bool>);
impl FPIF12_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF12` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF12_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF12_W<'a> {
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
#[doc = "Field `FPIF13` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF13_R(crate::FieldReader<bool, bool>);
impl FPIF13_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF13` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF13_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF13_W<'a> {
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
#[doc = "Field `FPIF14` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF14_R(crate::FieldReader<bool, bool>);
impl FPIF14_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF14` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF14_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF14_W<'a> {
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
#[doc = "Field `FPIF15` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF15_R(crate::FieldReader<bool, bool>);
impl FPIF15_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF15` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF15_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF15_W<'a> {
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
#[doc = "Field `FPIF16` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF16_R(crate::FieldReader<bool, bool>);
impl FPIF16_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF16` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF16_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF16_W<'a> {
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
#[doc = "Field `FPIF21` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF21_R(crate::FieldReader<bool, bool>);
impl FPIF21_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF21` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF21_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF21_W<'a> {
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
#[doc = "Field `FPIF22` reader - configurable event inputs x falling edge pending bit."]
pub struct FPIF22_R(crate::FieldReader<bool, bool>);
impl FPIF22_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF22` writer - configurable event inputs x falling edge pending bit."]
pub struct FPIF22_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF22_W<'a> {
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
impl R {
    #[doc = "Bit 0 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif0(&self) -> FPIF0_R {
        FPIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif1(&self) -> FPIF1_R {
        FPIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif2(&self) -> FPIF2_R {
        FPIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif3(&self) -> FPIF3_R {
        FPIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif4(&self) -> FPIF4_R {
        FPIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif5(&self) -> FPIF5_R {
        FPIF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif6(&self) -> FPIF6_R {
        FPIF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif7(&self) -> FPIF7_R {
        FPIF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif8(&self) -> FPIF8_R {
        FPIF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif9(&self) -> FPIF9_R {
        FPIF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif10(&self) -> FPIF10_R {
        FPIF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif11(&self) -> FPIF11_R {
        FPIF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif12(&self) -> FPIF12_R {
        FPIF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif13(&self) -> FPIF13_R {
        FPIF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif14(&self) -> FPIF14_R {
        FPIF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif15(&self) -> FPIF15_R {
        FPIF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif16(&self) -> FPIF16_R {
        FPIF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 21 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif21(&self) -> FPIF21_R {
        FPIF21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif22(&self) -> FPIF22_R {
        FPIF22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif0(&mut self) -> FPIF0_W {
        FPIF0_W { w: self }
    }
    #[doc = "Bit 1 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif1(&mut self) -> FPIF1_W {
        FPIF1_W { w: self }
    }
    #[doc = "Bit 2 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif2(&mut self) -> FPIF2_W {
        FPIF2_W { w: self }
    }
    #[doc = "Bit 3 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif3(&mut self) -> FPIF3_W {
        FPIF3_W { w: self }
    }
    #[doc = "Bit 4 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif4(&mut self) -> FPIF4_W {
        FPIF4_W { w: self }
    }
    #[doc = "Bit 5 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif5(&mut self) -> FPIF5_W {
        FPIF5_W { w: self }
    }
    #[doc = "Bit 6 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif6(&mut self) -> FPIF6_W {
        FPIF6_W { w: self }
    }
    #[doc = "Bit 7 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif7(&mut self) -> FPIF7_W {
        FPIF7_W { w: self }
    }
    #[doc = "Bit 8 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif8(&mut self) -> FPIF8_W {
        FPIF8_W { w: self }
    }
    #[doc = "Bit 9 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif9(&mut self) -> FPIF9_W {
        FPIF9_W { w: self }
    }
    #[doc = "Bit 10 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif10(&mut self) -> FPIF10_W {
        FPIF10_W { w: self }
    }
    #[doc = "Bit 11 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif11(&mut self) -> FPIF11_W {
        FPIF11_W { w: self }
    }
    #[doc = "Bit 12 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif12(&mut self) -> FPIF12_W {
        FPIF12_W { w: self }
    }
    #[doc = "Bit 13 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif13(&mut self) -> FPIF13_W {
        FPIF13_W { w: self }
    }
    #[doc = "Bit 14 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif14(&mut self) -> FPIF14_W {
        FPIF14_W { w: self }
    }
    #[doc = "Bit 15 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif15(&mut self) -> FPIF15_W {
        FPIF15_W { w: self }
    }
    #[doc = "Bit 16 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif16(&mut self) -> FPIF16_W {
        FPIF16_W { w: self }
    }
    #[doc = "Bit 21 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif21(&mut self) -> FPIF21_W {
        FPIF21_W { w: self }
    }
    #[doc = "Bit 22 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif22(&mut self) -> FPIF22_W {
        FPIF22_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI falling edge pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpr1](index.html) module"]
pub struct FPR1_SPEC;
impl crate::RegisterSpec for FPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpr1::R](R) reader structure"]
impl crate::Readable for FPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpr1::W](W) writer structure"]
impl crate::Writable for FPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPR1 to value 0"]
impl crate::Resettable for FPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
