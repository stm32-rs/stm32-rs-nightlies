#[doc = "Register `RPR1` reader"]
pub struct R(crate::R<RPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPR1` writer"]
pub struct W(crate::W<RPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPR1_SPEC>;
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
impl From<crate::W<RPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPIF0` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF0_R(crate::FieldReader<bool, bool>);
impl RPIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF0` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF0_W<'a> {
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
#[doc = "Field `RPIF1` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF1_R(crate::FieldReader<bool, bool>);
impl RPIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF1` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF1_W<'a> {
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
#[doc = "Field `RPIF2` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF2_R(crate::FieldReader<bool, bool>);
impl RPIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF2` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF2_W<'a> {
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
#[doc = "Field `RPIF3` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF3_R(crate::FieldReader<bool, bool>);
impl RPIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF3` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF3_W<'a> {
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
#[doc = "Field `RPIF4` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF4_R(crate::FieldReader<bool, bool>);
impl RPIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF4` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF4_W<'a> {
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
#[doc = "Field `RPIF5` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF5_R(crate::FieldReader<bool, bool>);
impl RPIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF5` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF5_W<'a> {
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
#[doc = "Field `RPIF6` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF6_R(crate::FieldReader<bool, bool>);
impl RPIF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF6` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF6_W<'a> {
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
#[doc = "Field `RPIF7` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF7_R(crate::FieldReader<bool, bool>);
impl RPIF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF7` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF7_W<'a> {
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
#[doc = "Field `RPIF8` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF8_R(crate::FieldReader<bool, bool>);
impl RPIF8_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF8` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF8_W<'a> {
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
#[doc = "Field `RPIF9` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF9_R(crate::FieldReader<bool, bool>);
impl RPIF9_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF9` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF9_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF9_W<'a> {
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
#[doc = "Field `RPIF10` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF10_R(crate::FieldReader<bool, bool>);
impl RPIF10_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF10` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF10_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF10_W<'a> {
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
#[doc = "Field `RPIF11` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF11_R(crate::FieldReader<bool, bool>);
impl RPIF11_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF11` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF11_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF11_W<'a> {
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
#[doc = "Field `RPIF12` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF12_R(crate::FieldReader<bool, bool>);
impl RPIF12_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF12` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF12_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF12_W<'a> {
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
#[doc = "Field `RPIF13` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF13_R(crate::FieldReader<bool, bool>);
impl RPIF13_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF13` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF13_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF13_W<'a> {
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
#[doc = "Field `RPIF14` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF14_R(crate::FieldReader<bool, bool>);
impl RPIF14_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF14` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF14_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF14_W<'a> {
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
#[doc = "Field `RPIF15` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF15_R(crate::FieldReader<bool, bool>);
impl RPIF15_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF15` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF15_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF15_W<'a> {
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
#[doc = "Field `RPIF16` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF16_R(crate::FieldReader<bool, bool>);
impl RPIF16_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF16` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF16_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF16_W<'a> {
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
#[doc = "Field `RPIF21` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF21_R(crate::FieldReader<bool, bool>);
impl RPIF21_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF21` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF21_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF21_W<'a> {
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
#[doc = "Field `RPIF22` reader - configurable event inputs x rising edge pending bit"]
pub struct RPIF22_R(crate::FieldReader<bool, bool>);
impl RPIF22_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF22` writer - configurable event inputs x rising edge pending bit"]
pub struct RPIF22_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF22_W<'a> {
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
    #[doc = "Bit 0 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif0(&self) -> RPIF0_R {
        RPIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif1(&self) -> RPIF1_R {
        RPIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif2(&self) -> RPIF2_R {
        RPIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif3(&self) -> RPIF3_R {
        RPIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif4(&self) -> RPIF4_R {
        RPIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif5(&self) -> RPIF5_R {
        RPIF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif6(&self) -> RPIF6_R {
        RPIF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif7(&self) -> RPIF7_R {
        RPIF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif8(&self) -> RPIF8_R {
        RPIF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif9(&self) -> RPIF9_R {
        RPIF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif10(&self) -> RPIF10_R {
        RPIF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif11(&self) -> RPIF11_R {
        RPIF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif12(&self) -> RPIF12_R {
        RPIF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif13(&self) -> RPIF13_R {
        RPIF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif14(&self) -> RPIF14_R {
        RPIF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif15(&self) -> RPIF15_R {
        RPIF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif16(&self) -> RPIF16_R {
        RPIF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 21 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif21(&self) -> RPIF21_R {
        RPIF21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif22(&self) -> RPIF22_R {
        RPIF22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif0(&mut self) -> RPIF0_W {
        RPIF0_W { w: self }
    }
    #[doc = "Bit 1 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif1(&mut self) -> RPIF1_W {
        RPIF1_W { w: self }
    }
    #[doc = "Bit 2 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif2(&mut self) -> RPIF2_W {
        RPIF2_W { w: self }
    }
    #[doc = "Bit 3 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif3(&mut self) -> RPIF3_W {
        RPIF3_W { w: self }
    }
    #[doc = "Bit 4 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif4(&mut self) -> RPIF4_W {
        RPIF4_W { w: self }
    }
    #[doc = "Bit 5 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif5(&mut self) -> RPIF5_W {
        RPIF5_W { w: self }
    }
    #[doc = "Bit 6 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif6(&mut self) -> RPIF6_W {
        RPIF6_W { w: self }
    }
    #[doc = "Bit 7 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif7(&mut self) -> RPIF7_W {
        RPIF7_W { w: self }
    }
    #[doc = "Bit 8 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif8(&mut self) -> RPIF8_W {
        RPIF8_W { w: self }
    }
    #[doc = "Bit 9 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif9(&mut self) -> RPIF9_W {
        RPIF9_W { w: self }
    }
    #[doc = "Bit 10 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif10(&mut self) -> RPIF10_W {
        RPIF10_W { w: self }
    }
    #[doc = "Bit 11 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif11(&mut self) -> RPIF11_W {
        RPIF11_W { w: self }
    }
    #[doc = "Bit 12 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif12(&mut self) -> RPIF12_W {
        RPIF12_W { w: self }
    }
    #[doc = "Bit 13 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif13(&mut self) -> RPIF13_W {
        RPIF13_W { w: self }
    }
    #[doc = "Bit 14 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif14(&mut self) -> RPIF14_W {
        RPIF14_W { w: self }
    }
    #[doc = "Bit 15 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif15(&mut self) -> RPIF15_W {
        RPIF15_W { w: self }
    }
    #[doc = "Bit 16 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif16(&mut self) -> RPIF16_W {
        RPIF16_W { w: self }
    }
    #[doc = "Bit 21 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif21(&mut self) -> RPIF21_W {
        RPIF21_W { w: self }
    }
    #[doc = "Bit 22 - configurable event inputs x rising edge pending bit"]
    #[inline(always)]
    pub fn rpif22(&mut self) -> RPIF22_W {
        RPIF22_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI rising edge pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpr1](index.html) module"]
pub struct RPR1_SPEC;
impl crate::RegisterSpec for RPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpr1::R](R) reader structure"]
impl crate::Readable for RPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpr1::W](W) writer structure"]
impl crate::Writable for RPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RPR1 to value 0"]
impl crate::Resettable for RPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
