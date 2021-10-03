#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISEM0` reader - Interrupt semaphore n enable bit"]
pub struct ISEM0_R(crate::FieldReader<bool, bool>);
impl ISEM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM0` writer - Interrupt semaphore n enable bit"]
pub struct ISEM0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM0_W<'a> {
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
#[doc = "Field `ISEM1` reader - Interrupt semaphore n enable bit"]
pub struct ISEM1_R(crate::FieldReader<bool, bool>);
impl ISEM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM1` writer - Interrupt semaphore n enable bit"]
pub struct ISEM1_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM1_W<'a> {
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
#[doc = "Field `ISEM2` reader - Interrupt semaphore n enable bit"]
pub struct ISEM2_R(crate::FieldReader<bool, bool>);
impl ISEM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM2` writer - Interrupt semaphore n enable bit"]
pub struct ISEM2_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM2_W<'a> {
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
#[doc = "Field `ISEM3` reader - Interrupt semaphore n enable bit"]
pub struct ISEM3_R(crate::FieldReader<bool, bool>);
impl ISEM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM3` writer - Interrupt semaphore n enable bit"]
pub struct ISEM3_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM3_W<'a> {
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
#[doc = "Field `ISEM4` reader - Interrupt semaphore n enable bit"]
pub struct ISEM4_R(crate::FieldReader<bool, bool>);
impl ISEM4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM4` writer - Interrupt semaphore n enable bit"]
pub struct ISEM4_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM4_W<'a> {
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
#[doc = "Field `ISEM5` reader - Interrupt semaphore n enable bit"]
pub struct ISEM5_R(crate::FieldReader<bool, bool>);
impl ISEM5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM5` writer - Interrupt semaphore n enable bit"]
pub struct ISEM5_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM5_W<'a> {
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
#[doc = "Field `ISEM6` reader - Interrupt semaphore n enable bit"]
pub struct ISEM6_R(crate::FieldReader<bool, bool>);
impl ISEM6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM6` writer - Interrupt semaphore n enable bit"]
pub struct ISEM6_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM6_W<'a> {
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
#[doc = "Field `ISEM7` reader - Interrupt semaphore n enable bit"]
pub struct ISEM7_R(crate::FieldReader<bool, bool>);
impl ISEM7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM7` writer - Interrupt semaphore n enable bit"]
pub struct ISEM7_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM7_W<'a> {
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
#[doc = "Field `ISEM8` reader - Interrupt semaphore n enable bit"]
pub struct ISEM8_R(crate::FieldReader<bool, bool>);
impl ISEM8_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM8` writer - Interrupt semaphore n enable bit"]
pub struct ISEM8_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM8_W<'a> {
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
#[doc = "Field `ISEM9` reader - Interrupt semaphore n enable bit"]
pub struct ISEM9_R(crate::FieldReader<bool, bool>);
impl ISEM9_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM9` writer - Interrupt semaphore n enable bit"]
pub struct ISEM9_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM9_W<'a> {
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
#[doc = "Field `ISEM10` reader - Interrupt semaphore n enable bit"]
pub struct ISEM10_R(crate::FieldReader<bool, bool>);
impl ISEM10_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM10` writer - Interrupt semaphore n enable bit"]
pub struct ISEM10_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM10_W<'a> {
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
#[doc = "Field `ISEM11` reader - Interrupt semaphore n enable bit"]
pub struct ISEM11_R(crate::FieldReader<bool, bool>);
impl ISEM11_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM11` writer - Interrupt semaphore n enable bit"]
pub struct ISEM11_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM11_W<'a> {
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
#[doc = "Field `ISEM12` reader - Interrupt semaphore n enable bit"]
pub struct ISEM12_R(crate::FieldReader<bool, bool>);
impl ISEM12_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM12` writer - Interrupt semaphore n enable bit"]
pub struct ISEM12_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM12_W<'a> {
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
#[doc = "Field `ISEM13` reader - Interrupt semaphore n enable bit"]
pub struct ISEM13_R(crate::FieldReader<bool, bool>);
impl ISEM13_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM13` writer - Interrupt semaphore n enable bit"]
pub struct ISEM13_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM13_W<'a> {
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
#[doc = "Field `ISEM14` reader - Interrupt semaphore n enable bit"]
pub struct ISEM14_R(crate::FieldReader<bool, bool>);
impl ISEM14_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM14` writer - Interrupt semaphore n enable bit"]
pub struct ISEM14_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM14_W<'a> {
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
#[doc = "Field `ISEM15` reader - Interrupt semaphore n enable bit"]
pub struct ISEM15_R(crate::FieldReader<bool, bool>);
impl ISEM15_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM15` writer - Interrupt semaphore n enable bit"]
pub struct ISEM15_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM15_W<'a> {
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
#[doc = "Field `ISEM16` reader - Interrupt semaphore n enable bit"]
pub struct ISEM16_R(crate::FieldReader<bool, bool>);
impl ISEM16_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM16` writer - Interrupt semaphore n enable bit"]
pub struct ISEM16_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM16_W<'a> {
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
#[doc = "Field `ISEM17` reader - Interrupt semaphore n enable bit"]
pub struct ISEM17_R(crate::FieldReader<bool, bool>);
impl ISEM17_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM17` writer - Interrupt semaphore n enable bit"]
pub struct ISEM17_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM17_W<'a> {
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
#[doc = "Field `ISEM18` reader - Interrupt semaphore n enable bit"]
pub struct ISEM18_R(crate::FieldReader<bool, bool>);
impl ISEM18_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM18` writer - Interrupt semaphore n enable bit"]
pub struct ISEM18_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM18_W<'a> {
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
#[doc = "Field `ISEM19` reader - Interrupt semaphore n enable bit"]
pub struct ISEM19_R(crate::FieldReader<bool, bool>);
impl ISEM19_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM19` writer - Interrupt semaphore n enable bit"]
pub struct ISEM19_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM19_W<'a> {
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
#[doc = "Field `ISEM20` reader - Interrupt semaphore n enable bit"]
pub struct ISEM20_R(crate::FieldReader<bool, bool>);
impl ISEM20_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM20` writer - Interrupt semaphore n enable bit"]
pub struct ISEM20_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM20_W<'a> {
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
#[doc = "Field `ISEM21` reader - Interrupt semaphore n enable bit"]
pub struct ISEM21_R(crate::FieldReader<bool, bool>);
impl ISEM21_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM21` writer - Interrupt semaphore n enable bit"]
pub struct ISEM21_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM21_W<'a> {
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
#[doc = "Field `ISEM22` reader - Interrupt semaphore n enable bit"]
pub struct ISEM22_R(crate::FieldReader<bool, bool>);
impl ISEM22_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM22` writer - Interrupt semaphore n enable bit"]
pub struct ISEM22_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM22_W<'a> {
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
#[doc = "Field `ISEM23` reader - Interrupt semaphore n enable bit"]
pub struct ISEM23_R(crate::FieldReader<bool, bool>);
impl ISEM23_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM23` writer - Interrupt semaphore n enable bit"]
pub struct ISEM23_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM23_W<'a> {
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
#[doc = "Field `ISEM24` reader - Interrupt semaphore n enable bit"]
pub struct ISEM24_R(crate::FieldReader<bool, bool>);
impl ISEM24_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM24` writer - Interrupt semaphore n enable bit"]
pub struct ISEM24_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM24_W<'a> {
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
#[doc = "Field `ISEM25` reader - Interrupt semaphore n enable bit"]
pub struct ISEM25_R(crate::FieldReader<bool, bool>);
impl ISEM25_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM25` writer - Interrupt semaphore n enable bit"]
pub struct ISEM25_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM25_W<'a> {
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
#[doc = "Field `ISEM26` reader - Interrupt semaphore n enable bit"]
pub struct ISEM26_R(crate::FieldReader<bool, bool>);
impl ISEM26_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM26` writer - Interrupt semaphore n enable bit"]
pub struct ISEM26_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM26_W<'a> {
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
#[doc = "Field `ISEM27` reader - Interrupt semaphore n enable bit"]
pub struct ISEM27_R(crate::FieldReader<bool, bool>);
impl ISEM27_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM27` writer - Interrupt semaphore n enable bit"]
pub struct ISEM27_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM27_W<'a> {
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
#[doc = "Field `ISEM28` reader - Interrupt semaphore n enable bit"]
pub struct ISEM28_R(crate::FieldReader<bool, bool>);
impl ISEM28_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM28` writer - Interrupt semaphore n enable bit"]
pub struct ISEM28_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM28_W<'a> {
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
#[doc = "Field `ISEM29` reader - Interrupt semaphore n enable bit"]
pub struct ISEM29_R(crate::FieldReader<bool, bool>);
impl ISEM29_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM29` writer - Interrupt semaphore n enable bit"]
pub struct ISEM29_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM29_W<'a> {
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
#[doc = "Field `ISEM30` reader - Interrupt semaphore n enable bit"]
pub struct ISEM30_R(crate::FieldReader<bool, bool>);
impl ISEM30_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM30` writer - Interrupt semaphore n enable bit"]
pub struct ISEM30_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM30_W<'a> {
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
#[doc = "Field `ISEM31` reader - Interrupt(N) semaphore n enable bit."]
pub struct ISEM31_R(crate::FieldReader<bool, bool>);
impl ISEM31_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISEM31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEM31` writer - Interrupt(N) semaphore n enable bit."]
pub struct ISEM31_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM31_W<'a> {
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
    #[doc = "Bit 0 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem0(&self) -> ISEM0_R {
        ISEM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem1(&self) -> ISEM1_R {
        ISEM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem2(&self) -> ISEM2_R {
        ISEM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem3(&self) -> ISEM3_R {
        ISEM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem4(&self) -> ISEM4_R {
        ISEM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem5(&self) -> ISEM5_R {
        ISEM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem6(&self) -> ISEM6_R {
        ISEM6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem7(&self) -> ISEM7_R {
        ISEM7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem8(&self) -> ISEM8_R {
        ISEM8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem9(&self) -> ISEM9_R {
        ISEM9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem10(&self) -> ISEM10_R {
        ISEM10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem11(&self) -> ISEM11_R {
        ISEM11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem12(&self) -> ISEM12_R {
        ISEM12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem13(&self) -> ISEM13_R {
        ISEM13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem14(&self) -> ISEM14_R {
        ISEM14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem15(&self) -> ISEM15_R {
        ISEM15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem16(&self) -> ISEM16_R {
        ISEM16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem17(&self) -> ISEM17_R {
        ISEM17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem18(&self) -> ISEM18_R {
        ISEM18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem19(&self) -> ISEM19_R {
        ISEM19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem20(&self) -> ISEM20_R {
        ISEM20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem21(&self) -> ISEM21_R {
        ISEM21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem22(&self) -> ISEM22_R {
        ISEM22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem23(&self) -> ISEM23_R {
        ISEM23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem24(&self) -> ISEM24_R {
        ISEM24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem25(&self) -> ISEM25_R {
        ISEM25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem26(&self) -> ISEM26_R {
        ISEM26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem27(&self) -> ISEM27_R {
        ISEM27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem28(&self) -> ISEM28_R {
        ISEM28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem29(&self) -> ISEM29_R {
        ISEM29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem30(&self) -> ISEM30_R {
        ISEM30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Interrupt(N) semaphore n enable bit."]
    #[inline(always)]
    pub fn isem31(&self) -> ISEM31_R {
        ISEM31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem0(&mut self) -> ISEM0_W {
        ISEM0_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem1(&mut self) -> ISEM1_W {
        ISEM1_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem2(&mut self) -> ISEM2_W {
        ISEM2_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem3(&mut self) -> ISEM3_W {
        ISEM3_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem4(&mut self) -> ISEM4_W {
        ISEM4_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem5(&mut self) -> ISEM5_W {
        ISEM5_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem6(&mut self) -> ISEM6_W {
        ISEM6_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem7(&mut self) -> ISEM7_W {
        ISEM7_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem8(&mut self) -> ISEM8_W {
        ISEM8_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem9(&mut self) -> ISEM9_W {
        ISEM9_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem10(&mut self) -> ISEM10_W {
        ISEM10_W { w: self }
    }
    #[doc = "Bit 11 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem11(&mut self) -> ISEM11_W {
        ISEM11_W { w: self }
    }
    #[doc = "Bit 12 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem12(&mut self) -> ISEM12_W {
        ISEM12_W { w: self }
    }
    #[doc = "Bit 13 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem13(&mut self) -> ISEM13_W {
        ISEM13_W { w: self }
    }
    #[doc = "Bit 14 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem14(&mut self) -> ISEM14_W {
        ISEM14_W { w: self }
    }
    #[doc = "Bit 15 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem15(&mut self) -> ISEM15_W {
        ISEM15_W { w: self }
    }
    #[doc = "Bit 16 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem16(&mut self) -> ISEM16_W {
        ISEM16_W { w: self }
    }
    #[doc = "Bit 17 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem17(&mut self) -> ISEM17_W {
        ISEM17_W { w: self }
    }
    #[doc = "Bit 18 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem18(&mut self) -> ISEM18_W {
        ISEM18_W { w: self }
    }
    #[doc = "Bit 19 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem19(&mut self) -> ISEM19_W {
        ISEM19_W { w: self }
    }
    #[doc = "Bit 20 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem20(&mut self) -> ISEM20_W {
        ISEM20_W { w: self }
    }
    #[doc = "Bit 21 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem21(&mut self) -> ISEM21_W {
        ISEM21_W { w: self }
    }
    #[doc = "Bit 22 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem22(&mut self) -> ISEM22_W {
        ISEM22_W { w: self }
    }
    #[doc = "Bit 23 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem23(&mut self) -> ISEM23_W {
        ISEM23_W { w: self }
    }
    #[doc = "Bit 24 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem24(&mut self) -> ISEM24_W {
        ISEM24_W { w: self }
    }
    #[doc = "Bit 25 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem25(&mut self) -> ISEM25_W {
        ISEM25_W { w: self }
    }
    #[doc = "Bit 26 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem26(&mut self) -> ISEM26_W {
        ISEM26_W { w: self }
    }
    #[doc = "Bit 27 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem27(&mut self) -> ISEM27_W {
        ISEM27_W { w: self }
    }
    #[doc = "Bit 28 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem28(&mut self) -> ISEM28_W {
        ISEM28_W { w: self }
    }
    #[doc = "Bit 29 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem29(&mut self) -> ISEM29_W {
        ISEM29_W { w: self }
    }
    #[doc = "Bit 30 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn isem30(&mut self) -> ISEM30_W {
        ISEM30_W { w: self }
    }
    #[doc = "Bit 31 - Interrupt(N) semaphore n enable bit."]
    #[inline(always)]
    pub fn isem31(&mut self) -> ISEM31_W {
        ISEM31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSEM Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
