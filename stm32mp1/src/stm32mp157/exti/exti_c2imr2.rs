#[doc = "Register `EXTI_C2IMR2` reader"]
pub struct R(crate::R<EXTI_C2IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_C2IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_C2IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_C2IMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_C2IMR2` writer"]
pub struct W(crate::W<EXTI_C2IMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_C2IMR2_SPEC>;
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
impl From<crate::W<EXTI_C2IMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_C2IMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM32` reader - IM32"]
pub struct IM32_R(crate::FieldReader<bool, bool>);
impl IM32_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM32` writer - IM32"]
pub struct IM32_W<'a> {
    w: &'a mut W,
}
impl<'a> IM32_W<'a> {
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
#[doc = "Field `IM33` reader - IM33"]
pub struct IM33_R(crate::FieldReader<bool, bool>);
impl IM33_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM33_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM33` writer - IM33"]
pub struct IM33_W<'a> {
    w: &'a mut W,
}
impl<'a> IM33_W<'a> {
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
#[doc = "Field `IM34` reader - IM34"]
pub struct IM34_R(crate::FieldReader<bool, bool>);
impl IM34_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM34_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM34_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM34` writer - IM34"]
pub struct IM34_W<'a> {
    w: &'a mut W,
}
impl<'a> IM34_W<'a> {
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
#[doc = "Field `IM35` reader - IM35"]
pub struct IM35_R(crate::FieldReader<bool, bool>);
impl IM35_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM35_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM35_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM35` writer - IM35"]
pub struct IM35_W<'a> {
    w: &'a mut W,
}
impl<'a> IM35_W<'a> {
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
#[doc = "Field `IM36` reader - IM36"]
pub struct IM36_R(crate::FieldReader<bool, bool>);
impl IM36_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM36_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM36_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM36` writer - IM36"]
pub struct IM36_W<'a> {
    w: &'a mut W,
}
impl<'a> IM36_W<'a> {
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
#[doc = "Field `IM37` reader - IM37"]
pub struct IM37_R(crate::FieldReader<bool, bool>);
impl IM37_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM37_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM37_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM37` writer - IM37"]
pub struct IM37_W<'a> {
    w: &'a mut W,
}
impl<'a> IM37_W<'a> {
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
#[doc = "Field `IM38` reader - IM38"]
pub struct IM38_R(crate::FieldReader<bool, bool>);
impl IM38_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM38_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM38_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM38` writer - IM38"]
pub struct IM38_W<'a> {
    w: &'a mut W,
}
impl<'a> IM38_W<'a> {
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
#[doc = "Field `IM39` reader - IM39"]
pub struct IM39_R(crate::FieldReader<bool, bool>);
impl IM39_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM39_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM39_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM39` writer - IM39"]
pub struct IM39_W<'a> {
    w: &'a mut W,
}
impl<'a> IM39_W<'a> {
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
#[doc = "Field `IM40` reader - IM40"]
pub struct IM40_R(crate::FieldReader<bool, bool>);
impl IM40_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM40_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM40_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM40` writer - IM40"]
pub struct IM40_W<'a> {
    w: &'a mut W,
}
impl<'a> IM40_W<'a> {
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
#[doc = "Field `IM41` reader - IM41"]
pub struct IM41_R(crate::FieldReader<bool, bool>);
impl IM41_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM41_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM41_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM41` writer - IM41"]
pub struct IM41_W<'a> {
    w: &'a mut W,
}
impl<'a> IM41_W<'a> {
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
#[doc = "Field `IM42` reader - IM42"]
pub struct IM42_R(crate::FieldReader<bool, bool>);
impl IM42_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM42_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM42_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM42` writer - IM42"]
pub struct IM42_W<'a> {
    w: &'a mut W,
}
impl<'a> IM42_W<'a> {
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
#[doc = "Field `IM43` reader - IM43"]
pub struct IM43_R(crate::FieldReader<bool, bool>);
impl IM43_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM43_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM43_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM43` writer - IM43"]
pub struct IM43_W<'a> {
    w: &'a mut W,
}
impl<'a> IM43_W<'a> {
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
#[doc = "Field `IM44` reader - IM44"]
pub struct IM44_R(crate::FieldReader<bool, bool>);
impl IM44_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM44_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM44_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM44` writer - IM44"]
pub struct IM44_W<'a> {
    w: &'a mut W,
}
impl<'a> IM44_W<'a> {
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
#[doc = "Field `IM45` reader - IM45"]
pub struct IM45_R(crate::FieldReader<bool, bool>);
impl IM45_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM45_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM45_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM45` writer - IM45"]
pub struct IM45_W<'a> {
    w: &'a mut W,
}
impl<'a> IM45_W<'a> {
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
#[doc = "Field `IM46` reader - IM46"]
pub struct IM46_R(crate::FieldReader<bool, bool>);
impl IM46_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM46_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM46_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM46` writer - IM46"]
pub struct IM46_W<'a> {
    w: &'a mut W,
}
impl<'a> IM46_W<'a> {
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
#[doc = "Field `IM47` reader - IM47"]
pub struct IM47_R(crate::FieldReader<bool, bool>);
impl IM47_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM47_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM47_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM47` writer - IM47"]
pub struct IM47_W<'a> {
    w: &'a mut W,
}
impl<'a> IM47_W<'a> {
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
#[doc = "Field `IM48` reader - IM48"]
pub struct IM48_R(crate::FieldReader<bool, bool>);
impl IM48_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM48_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM48_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM48` writer - IM48"]
pub struct IM48_W<'a> {
    w: &'a mut W,
}
impl<'a> IM48_W<'a> {
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
#[doc = "Field `IM49` reader - IM49"]
pub struct IM49_R(crate::FieldReader<bool, bool>);
impl IM49_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM49_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM49_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM49` writer - IM49"]
pub struct IM49_W<'a> {
    w: &'a mut W,
}
impl<'a> IM49_W<'a> {
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
#[doc = "Field `IM50` reader - IM50"]
pub struct IM50_R(crate::FieldReader<bool, bool>);
impl IM50_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM50_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM50_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM50` writer - IM50"]
pub struct IM50_W<'a> {
    w: &'a mut W,
}
impl<'a> IM50_W<'a> {
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
#[doc = "Field `IM51` reader - IM51"]
pub struct IM51_R(crate::FieldReader<bool, bool>);
impl IM51_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM51_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM51_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM51` writer - IM51"]
pub struct IM51_W<'a> {
    w: &'a mut W,
}
impl<'a> IM51_W<'a> {
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
#[doc = "Field `IM52` reader - IM52"]
pub struct IM52_R(crate::FieldReader<bool, bool>);
impl IM52_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM52_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM52_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM52` writer - IM52"]
pub struct IM52_W<'a> {
    w: &'a mut W,
}
impl<'a> IM52_W<'a> {
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
#[doc = "Field `IM53` reader - IM53"]
pub struct IM53_R(crate::FieldReader<bool, bool>);
impl IM53_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM53_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM53_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM53` writer - IM53"]
pub struct IM53_W<'a> {
    w: &'a mut W,
}
impl<'a> IM53_W<'a> {
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
#[doc = "Field `IM54` reader - IM54"]
pub struct IM54_R(crate::FieldReader<bool, bool>);
impl IM54_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM54_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM54_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM54` writer - IM54"]
pub struct IM54_W<'a> {
    w: &'a mut W,
}
impl<'a> IM54_W<'a> {
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
#[doc = "Field `IM55` reader - IM55"]
pub struct IM55_R(crate::FieldReader<bool, bool>);
impl IM55_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM55_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM55_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM55` writer - IM55"]
pub struct IM55_W<'a> {
    w: &'a mut W,
}
impl<'a> IM55_W<'a> {
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
#[doc = "Field `IM56` reader - IM56"]
pub struct IM56_R(crate::FieldReader<bool, bool>);
impl IM56_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM56_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM56_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM56` writer - IM56"]
pub struct IM56_W<'a> {
    w: &'a mut W,
}
impl<'a> IM56_W<'a> {
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
#[doc = "Field `IM57` reader - IM57"]
pub struct IM57_R(crate::FieldReader<bool, bool>);
impl IM57_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM57_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM57_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM57` writer - IM57"]
pub struct IM57_W<'a> {
    w: &'a mut W,
}
impl<'a> IM57_W<'a> {
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
#[doc = "Field `IM58` reader - IM58"]
pub struct IM58_R(crate::FieldReader<bool, bool>);
impl IM58_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM58_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM58_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM58` writer - IM58"]
pub struct IM58_W<'a> {
    w: &'a mut W,
}
impl<'a> IM58_W<'a> {
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
#[doc = "Field `IM59` reader - IM59"]
pub struct IM59_R(crate::FieldReader<bool, bool>);
impl IM59_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM59_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM59_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM59` writer - IM59"]
pub struct IM59_W<'a> {
    w: &'a mut W,
}
impl<'a> IM59_W<'a> {
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
#[doc = "Field `IM60` reader - IM60"]
pub struct IM60_R(crate::FieldReader<bool, bool>);
impl IM60_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM60_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM60_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM60` writer - IM60"]
pub struct IM60_W<'a> {
    w: &'a mut W,
}
impl<'a> IM60_W<'a> {
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
#[doc = "Field `IM61` reader - IM61"]
pub struct IM61_R(crate::FieldReader<bool, bool>);
impl IM61_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM61_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM61_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM61` writer - IM61"]
pub struct IM61_W<'a> {
    w: &'a mut W,
}
impl<'a> IM61_W<'a> {
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
#[doc = "Field `IM62` reader - IM62"]
pub struct IM62_R(crate::FieldReader<bool, bool>);
impl IM62_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM62_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM62_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM62` writer - IM62"]
pub struct IM62_W<'a> {
    w: &'a mut W,
}
impl<'a> IM62_W<'a> {
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
#[doc = "Field `IM63` reader - IM63"]
pub struct IM63_R(crate::FieldReader<bool, bool>);
impl IM63_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM63_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM63_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM63` writer - IM63"]
pub struct IM63_W<'a> {
    w: &'a mut W,
}
impl<'a> IM63_W<'a> {
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
    #[doc = "Bit 0 - IM32"]
    #[inline(always)]
    pub fn im32(&self) -> IM32_R {
        IM32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IM33"]
    #[inline(always)]
    pub fn im33(&self) -> IM33_R {
        IM33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IM34"]
    #[inline(always)]
    pub fn im34(&self) -> IM34_R {
        IM34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IM35"]
    #[inline(always)]
    pub fn im35(&self) -> IM35_R {
        IM35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IM36"]
    #[inline(always)]
    pub fn im36(&self) -> IM36_R {
        IM36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IM37"]
    #[inline(always)]
    pub fn im37(&self) -> IM37_R {
        IM37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IM38"]
    #[inline(always)]
    pub fn im38(&self) -> IM38_R {
        IM38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IM39"]
    #[inline(always)]
    pub fn im39(&self) -> IM39_R {
        IM39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - IM40"]
    #[inline(always)]
    pub fn im40(&self) -> IM40_R {
        IM40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - IM41"]
    #[inline(always)]
    pub fn im41(&self) -> IM41_R {
        IM41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - IM42"]
    #[inline(always)]
    pub fn im42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - IM43"]
    #[inline(always)]
    pub fn im43(&self) -> IM43_R {
        IM43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IM44"]
    #[inline(always)]
    pub fn im44(&self) -> IM44_R {
        IM44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - IM45"]
    #[inline(always)]
    pub fn im45(&self) -> IM45_R {
        IM45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - IM46"]
    #[inline(always)]
    pub fn im46(&self) -> IM46_R {
        IM46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IM47"]
    #[inline(always)]
    pub fn im47(&self) -> IM47_R {
        IM47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - IM48"]
    #[inline(always)]
    pub fn im48(&self) -> IM48_R {
        IM48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - IM49"]
    #[inline(always)]
    pub fn im49(&self) -> IM49_R {
        IM49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IM50"]
    #[inline(always)]
    pub fn im50(&self) -> IM50_R {
        IM50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - IM51"]
    #[inline(always)]
    pub fn im51(&self) -> IM51_R {
        IM51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IM52"]
    #[inline(always)]
    pub fn im52(&self) -> IM52_R {
        IM52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - IM53"]
    #[inline(always)]
    pub fn im53(&self) -> IM53_R {
        IM53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - IM54"]
    #[inline(always)]
    pub fn im54(&self) -> IM54_R {
        IM54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - IM55"]
    #[inline(always)]
    pub fn im55(&self) -> IM55_R {
        IM55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - IM56"]
    #[inline(always)]
    pub fn im56(&self) -> IM56_R {
        IM56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - IM57"]
    #[inline(always)]
    pub fn im57(&self) -> IM57_R {
        IM57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - IM58"]
    #[inline(always)]
    pub fn im58(&self) -> IM58_R {
        IM58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IM59"]
    #[inline(always)]
    pub fn im59(&self) -> IM59_R {
        IM59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - IM60"]
    #[inline(always)]
    pub fn im60(&self) -> IM60_R {
        IM60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - IM61"]
    #[inline(always)]
    pub fn im61(&self) -> IM61_R {
        IM61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - IM62"]
    #[inline(always)]
    pub fn im62(&self) -> IM62_R {
        IM62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - IM63"]
    #[inline(always)]
    pub fn im63(&self) -> IM63_R {
        IM63_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IM32"]
    #[inline(always)]
    pub fn im32(&mut self) -> IM32_W {
        IM32_W { w: self }
    }
    #[doc = "Bit 1 - IM33"]
    #[inline(always)]
    pub fn im33(&mut self) -> IM33_W {
        IM33_W { w: self }
    }
    #[doc = "Bit 2 - IM34"]
    #[inline(always)]
    pub fn im34(&mut self) -> IM34_W {
        IM34_W { w: self }
    }
    #[doc = "Bit 3 - IM35"]
    #[inline(always)]
    pub fn im35(&mut self) -> IM35_W {
        IM35_W { w: self }
    }
    #[doc = "Bit 4 - IM36"]
    #[inline(always)]
    pub fn im36(&mut self) -> IM36_W {
        IM36_W { w: self }
    }
    #[doc = "Bit 5 - IM37"]
    #[inline(always)]
    pub fn im37(&mut self) -> IM37_W {
        IM37_W { w: self }
    }
    #[doc = "Bit 6 - IM38"]
    #[inline(always)]
    pub fn im38(&mut self) -> IM38_W {
        IM38_W { w: self }
    }
    #[doc = "Bit 7 - IM39"]
    #[inline(always)]
    pub fn im39(&mut self) -> IM39_W {
        IM39_W { w: self }
    }
    #[doc = "Bit 8 - IM40"]
    #[inline(always)]
    pub fn im40(&mut self) -> IM40_W {
        IM40_W { w: self }
    }
    #[doc = "Bit 9 - IM41"]
    #[inline(always)]
    pub fn im41(&mut self) -> IM41_W {
        IM41_W { w: self }
    }
    #[doc = "Bit 10 - IM42"]
    #[inline(always)]
    pub fn im42(&mut self) -> IM42_W {
        IM42_W { w: self }
    }
    #[doc = "Bit 11 - IM43"]
    #[inline(always)]
    pub fn im43(&mut self) -> IM43_W {
        IM43_W { w: self }
    }
    #[doc = "Bit 12 - IM44"]
    #[inline(always)]
    pub fn im44(&mut self) -> IM44_W {
        IM44_W { w: self }
    }
    #[doc = "Bit 13 - IM45"]
    #[inline(always)]
    pub fn im45(&mut self) -> IM45_W {
        IM45_W { w: self }
    }
    #[doc = "Bit 14 - IM46"]
    #[inline(always)]
    pub fn im46(&mut self) -> IM46_W {
        IM46_W { w: self }
    }
    #[doc = "Bit 15 - IM47"]
    #[inline(always)]
    pub fn im47(&mut self) -> IM47_W {
        IM47_W { w: self }
    }
    #[doc = "Bit 16 - IM48"]
    #[inline(always)]
    pub fn im48(&mut self) -> IM48_W {
        IM48_W { w: self }
    }
    #[doc = "Bit 17 - IM49"]
    #[inline(always)]
    pub fn im49(&mut self) -> IM49_W {
        IM49_W { w: self }
    }
    #[doc = "Bit 18 - IM50"]
    #[inline(always)]
    pub fn im50(&mut self) -> IM50_W {
        IM50_W { w: self }
    }
    #[doc = "Bit 19 - IM51"]
    #[inline(always)]
    pub fn im51(&mut self) -> IM51_W {
        IM51_W { w: self }
    }
    #[doc = "Bit 20 - IM52"]
    #[inline(always)]
    pub fn im52(&mut self) -> IM52_W {
        IM52_W { w: self }
    }
    #[doc = "Bit 21 - IM53"]
    #[inline(always)]
    pub fn im53(&mut self) -> IM53_W {
        IM53_W { w: self }
    }
    #[doc = "Bit 22 - IM54"]
    #[inline(always)]
    pub fn im54(&mut self) -> IM54_W {
        IM54_W { w: self }
    }
    #[doc = "Bit 23 - IM55"]
    #[inline(always)]
    pub fn im55(&mut self) -> IM55_W {
        IM55_W { w: self }
    }
    #[doc = "Bit 24 - IM56"]
    #[inline(always)]
    pub fn im56(&mut self) -> IM56_W {
        IM56_W { w: self }
    }
    #[doc = "Bit 25 - IM57"]
    #[inline(always)]
    pub fn im57(&mut self) -> IM57_W {
        IM57_W { w: self }
    }
    #[doc = "Bit 26 - IM58"]
    #[inline(always)]
    pub fn im58(&mut self) -> IM58_W {
        IM58_W { w: self }
    }
    #[doc = "Bit 27 - IM59"]
    #[inline(always)]
    pub fn im59(&mut self) -> IM59_W {
        IM59_W { w: self }
    }
    #[doc = "Bit 28 - IM60"]
    #[inline(always)]
    pub fn im60(&mut self) -> IM60_W {
        IM60_W { w: self }
    }
    #[doc = "Bit 29 - IM61"]
    #[inline(always)]
    pub fn im61(&mut self) -> IM61_W {
        IM61_W { w: self }
    }
    #[doc = "Bit 30 - IM62"]
    #[inline(always)]
    pub fn im62(&mut self) -> IM62_W {
        IM62_W { w: self }
    }
    #[doc = "Bit 31 - IM63"]
    #[inline(always)]
    pub fn im63(&mut self) -> IM63_W {
        IM63_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains register bits for configurable events and direct events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_c2imr2](index.html) module"]
pub struct EXTI_C2IMR2_SPEC;
impl crate::RegisterSpec for EXTI_C2IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_c2imr2::R](R) reader structure"]
impl crate::Readable for EXTI_C2IMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_c2imr2::W](W) writer structure"]
impl crate::Writable for EXTI_C2IMR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_C2IMR2 to value 0xffff_ffff"]
impl crate::Resettable for EXTI_C2IMR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
