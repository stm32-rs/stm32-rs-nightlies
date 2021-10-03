#[doc = "Register `SWPR` reader"]
pub struct R(crate::R<SWPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWPR` writer"]
pub struct W(crate::W<SWPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWPR_SPEC>;
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
impl From<crate::W<SWPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Page0_WP` reader - Write protection"]
pub struct PAGE0_WP_R(crate::FieldReader<bool, bool>);
impl PAGE0_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE0_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE0_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page0_WP` writer - Write protection"]
pub struct PAGE0_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE0_WP_W<'a> {
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
#[doc = "Field `Page1_WP` reader - Write protection"]
pub struct PAGE1_WP_R(crate::FieldReader<bool, bool>);
impl PAGE1_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE1_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE1_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page1_WP` writer - Write protection"]
pub struct PAGE1_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE1_WP_W<'a> {
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
#[doc = "Field `Page2_WP` reader - Write protection"]
pub struct PAGE2_WP_R(crate::FieldReader<bool, bool>);
impl PAGE2_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE2_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE2_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page2_WP` writer - Write protection"]
pub struct PAGE2_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE2_WP_W<'a> {
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
#[doc = "Field `Page3_WP` reader - Write protection"]
pub struct PAGE3_WP_R(crate::FieldReader<bool, bool>);
impl PAGE3_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE3_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE3_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page3_WP` writer - Write protection"]
pub struct PAGE3_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE3_WP_W<'a> {
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
#[doc = "Field `Page4_WP` reader - Write protection"]
pub struct PAGE4_WP_R(crate::FieldReader<bool, bool>);
impl PAGE4_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE4_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE4_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page4_WP` writer - Write protection"]
pub struct PAGE4_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE4_WP_W<'a> {
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
#[doc = "Field `Page5_WP` reader - Write protection"]
pub struct PAGE5_WP_R(crate::FieldReader<bool, bool>);
impl PAGE5_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE5_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE5_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page5_WP` writer - Write protection"]
pub struct PAGE5_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE5_WP_W<'a> {
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
#[doc = "Field `Page6_WP` reader - Write protection"]
pub struct PAGE6_WP_R(crate::FieldReader<bool, bool>);
impl PAGE6_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE6_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE6_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page6_WP` writer - Write protection"]
pub struct PAGE6_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE6_WP_W<'a> {
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
#[doc = "Field `Page7_WP` reader - Write protection"]
pub struct PAGE7_WP_R(crate::FieldReader<bool, bool>);
impl PAGE7_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE7_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE7_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page7_WP` writer - Write protection"]
pub struct PAGE7_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE7_WP_W<'a> {
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
#[doc = "Field `Page8_WP` reader - Write protection"]
pub struct PAGE8_WP_R(crate::FieldReader<bool, bool>);
impl PAGE8_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE8_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE8_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page8_WP` writer - Write protection"]
pub struct PAGE8_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE8_WP_W<'a> {
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
#[doc = "Field `Page9_WP` reader - Write protection"]
pub struct PAGE9_WP_R(crate::FieldReader<bool, bool>);
impl PAGE9_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE9_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE9_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page9_WP` writer - Write protection"]
pub struct PAGE9_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE9_WP_W<'a> {
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
#[doc = "Field `Page10_WP` reader - Write protection"]
pub struct PAGE10_WP_R(crate::FieldReader<bool, bool>);
impl PAGE10_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE10_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE10_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page10_WP` writer - Write protection"]
pub struct PAGE10_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE10_WP_W<'a> {
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
#[doc = "Field `Page11_WP` reader - Write protection"]
pub struct PAGE11_WP_R(crate::FieldReader<bool, bool>);
impl PAGE11_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE11_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE11_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page11_WP` writer - Write protection"]
pub struct PAGE11_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE11_WP_W<'a> {
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
#[doc = "Field `Page12_WP` reader - Write protection"]
pub struct PAGE12_WP_R(crate::FieldReader<bool, bool>);
impl PAGE12_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE12_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE12_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page12_WP` writer - Write protection"]
pub struct PAGE12_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE12_WP_W<'a> {
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
#[doc = "Field `Page13_WP` reader - Write protection"]
pub struct PAGE13_WP_R(crate::FieldReader<bool, bool>);
impl PAGE13_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE13_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE13_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page13_WP` writer - Write protection"]
pub struct PAGE13_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE13_WP_W<'a> {
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
#[doc = "Field `Page14_WP` reader - Write protection"]
pub struct PAGE14_WP_R(crate::FieldReader<bool, bool>);
impl PAGE14_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE14_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE14_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page14_WP` writer - Write protection"]
pub struct PAGE14_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE14_WP_W<'a> {
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
#[doc = "Field `Page15_WP` reader - Write protection"]
pub struct PAGE15_WP_R(crate::FieldReader<bool, bool>);
impl PAGE15_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE15_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE15_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page15_WP` writer - Write protection"]
pub struct PAGE15_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE15_WP_W<'a> {
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
#[doc = "Field `Page16_WP` reader - Write protection"]
pub struct PAGE16_WP_R(crate::FieldReader<bool, bool>);
impl PAGE16_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE16_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE16_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page16_WP` writer - Write protection"]
pub struct PAGE16_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE16_WP_W<'a> {
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
#[doc = "Field `Page17_WP` reader - Write protection"]
pub struct PAGE17_WP_R(crate::FieldReader<bool, bool>);
impl PAGE17_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE17_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE17_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page17_WP` writer - Write protection"]
pub struct PAGE17_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE17_WP_W<'a> {
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
#[doc = "Field `Page18_WP` reader - Write protection"]
pub struct PAGE18_WP_R(crate::FieldReader<bool, bool>);
impl PAGE18_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE18_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE18_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page18_WP` writer - Write protection"]
pub struct PAGE18_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE18_WP_W<'a> {
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
#[doc = "Field `Page19_WP` reader - Write protection"]
pub struct PAGE19_WP_R(crate::FieldReader<bool, bool>);
impl PAGE19_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE19_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE19_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page19_WP` writer - Write protection"]
pub struct PAGE19_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE19_WP_W<'a> {
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
#[doc = "Field `Page20_WP` reader - Write protection"]
pub struct PAGE20_WP_R(crate::FieldReader<bool, bool>);
impl PAGE20_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE20_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE20_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page20_WP` writer - Write protection"]
pub struct PAGE20_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE20_WP_W<'a> {
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
#[doc = "Field `Page21_WP` reader - Write protection"]
pub struct PAGE21_WP_R(crate::FieldReader<bool, bool>);
impl PAGE21_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE21_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE21_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page21_WP` writer - Write protection"]
pub struct PAGE21_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE21_WP_W<'a> {
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
#[doc = "Field `Page22_WP` reader - Write protection"]
pub struct PAGE22_WP_R(crate::FieldReader<bool, bool>);
impl PAGE22_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE22_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE22_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page22_WP` writer - Write protection"]
pub struct PAGE22_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE22_WP_W<'a> {
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
#[doc = "Field `Page23_WP` reader - Write protection"]
pub struct PAGE23_WP_R(crate::FieldReader<bool, bool>);
impl PAGE23_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE23_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE23_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page23_WP` writer - Write protection"]
pub struct PAGE23_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE23_WP_W<'a> {
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
#[doc = "Field `Page24_WP` reader - Write protection"]
pub struct PAGE24_WP_R(crate::FieldReader<bool, bool>);
impl PAGE24_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE24_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE24_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page24_WP` writer - Write protection"]
pub struct PAGE24_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE24_WP_W<'a> {
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
#[doc = "Field `Page25_WP` reader - Write protection"]
pub struct PAGE25_WP_R(crate::FieldReader<bool, bool>);
impl PAGE25_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE25_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE25_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page25_WP` writer - Write protection"]
pub struct PAGE25_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE25_WP_W<'a> {
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
#[doc = "Field `Page26_WP` reader - Write protection"]
pub struct PAGE26_WP_R(crate::FieldReader<bool, bool>);
impl PAGE26_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE26_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE26_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page26_WP` writer - Write protection"]
pub struct PAGE26_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE26_WP_W<'a> {
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
#[doc = "Field `Page27_WP` reader - Write protection"]
pub struct PAGE27_WP_R(crate::FieldReader<bool, bool>);
impl PAGE27_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE27_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE27_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page27_WP` writer - Write protection"]
pub struct PAGE27_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE27_WP_W<'a> {
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
#[doc = "Field `Page28_WP` reader - Write protection"]
pub struct PAGE28_WP_R(crate::FieldReader<bool, bool>);
impl PAGE28_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE28_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE28_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page28_WP` writer - Write protection"]
pub struct PAGE28_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE28_WP_W<'a> {
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
#[doc = "Field `Page29_WP` reader - Write protection"]
pub struct PAGE29_WP_R(crate::FieldReader<bool, bool>);
impl PAGE29_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE29_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE29_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page29_WP` writer - Write protection"]
pub struct PAGE29_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE29_WP_W<'a> {
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
#[doc = "Field `Page30_WP` reader - Write protection"]
pub struct PAGE30_WP_R(crate::FieldReader<bool, bool>);
impl PAGE30_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE30_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE30_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page30_WP` writer - Write protection"]
pub struct PAGE30_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE30_WP_W<'a> {
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
#[doc = "Field `Page31_WP` reader - Write protection"]
pub struct PAGE31_WP_R(crate::FieldReader<bool, bool>);
impl PAGE31_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE31_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE31_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Page31_WP` writer - Write protection"]
pub struct PAGE31_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE31_WP_W<'a> {
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
    #[doc = "Bit 0 - Write protection"]
    #[inline(always)]
    pub fn page0_wp(&self) -> PAGE0_WP_R {
        PAGE0_WP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write protection"]
    #[inline(always)]
    pub fn page1_wp(&self) -> PAGE1_WP_R {
        PAGE1_WP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write protection"]
    #[inline(always)]
    pub fn page2_wp(&self) -> PAGE2_WP_R {
        PAGE2_WP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write protection"]
    #[inline(always)]
    pub fn page3_wp(&self) -> PAGE3_WP_R {
        PAGE3_WP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write protection"]
    #[inline(always)]
    pub fn page4_wp(&self) -> PAGE4_WP_R {
        PAGE4_WP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write protection"]
    #[inline(always)]
    pub fn page5_wp(&self) -> PAGE5_WP_R {
        PAGE5_WP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write protection"]
    #[inline(always)]
    pub fn page6_wp(&self) -> PAGE6_WP_R {
        PAGE6_WP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write protection"]
    #[inline(always)]
    pub fn page7_wp(&self) -> PAGE7_WP_R {
        PAGE7_WP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write protection"]
    #[inline(always)]
    pub fn page8_wp(&self) -> PAGE8_WP_R {
        PAGE8_WP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write protection"]
    #[inline(always)]
    pub fn page9_wp(&self) -> PAGE9_WP_R {
        PAGE9_WP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Write protection"]
    #[inline(always)]
    pub fn page10_wp(&self) -> PAGE10_WP_R {
        PAGE10_WP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Write protection"]
    #[inline(always)]
    pub fn page11_wp(&self) -> PAGE11_WP_R {
        PAGE11_WP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Write protection"]
    #[inline(always)]
    pub fn page12_wp(&self) -> PAGE12_WP_R {
        PAGE12_WP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Write protection"]
    #[inline(always)]
    pub fn page13_wp(&self) -> PAGE13_WP_R {
        PAGE13_WP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Write protection"]
    #[inline(always)]
    pub fn page14_wp(&self) -> PAGE14_WP_R {
        PAGE14_WP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Write protection"]
    #[inline(always)]
    pub fn page15_wp(&self) -> PAGE15_WP_R {
        PAGE15_WP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Write protection"]
    #[inline(always)]
    pub fn page16_wp(&self) -> PAGE16_WP_R {
        PAGE16_WP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Write protection"]
    #[inline(always)]
    pub fn page17_wp(&self) -> PAGE17_WP_R {
        PAGE17_WP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Write protection"]
    #[inline(always)]
    pub fn page18_wp(&self) -> PAGE18_WP_R {
        PAGE18_WP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Write protection"]
    #[inline(always)]
    pub fn page19_wp(&self) -> PAGE19_WP_R {
        PAGE19_WP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write protection"]
    #[inline(always)]
    pub fn page20_wp(&self) -> PAGE20_WP_R {
        PAGE20_WP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Write protection"]
    #[inline(always)]
    pub fn page21_wp(&self) -> PAGE21_WP_R {
        PAGE21_WP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Write protection"]
    #[inline(always)]
    pub fn page22_wp(&self) -> PAGE22_WP_R {
        PAGE22_WP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Write protection"]
    #[inline(always)]
    pub fn page23_wp(&self) -> PAGE23_WP_R {
        PAGE23_WP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Write protection"]
    #[inline(always)]
    pub fn page24_wp(&self) -> PAGE24_WP_R {
        PAGE24_WP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Write protection"]
    #[inline(always)]
    pub fn page25_wp(&self) -> PAGE25_WP_R {
        PAGE25_WP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Write protection"]
    #[inline(always)]
    pub fn page26_wp(&self) -> PAGE26_WP_R {
        PAGE26_WP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Write protection"]
    #[inline(always)]
    pub fn page27_wp(&self) -> PAGE27_WP_R {
        PAGE27_WP_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Write protection"]
    #[inline(always)]
    pub fn page28_wp(&self) -> PAGE28_WP_R {
        PAGE28_WP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Write protection"]
    #[inline(always)]
    pub fn page29_wp(&self) -> PAGE29_WP_R {
        PAGE29_WP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Write protection"]
    #[inline(always)]
    pub fn page30_wp(&self) -> PAGE30_WP_R {
        PAGE30_WP_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Write protection"]
    #[inline(always)]
    pub fn page31_wp(&self) -> PAGE31_WP_R {
        PAGE31_WP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write protection"]
    #[inline(always)]
    pub fn page0_wp(&mut self) -> PAGE0_WP_W {
        PAGE0_WP_W { w: self }
    }
    #[doc = "Bit 1 - Write protection"]
    #[inline(always)]
    pub fn page1_wp(&mut self) -> PAGE1_WP_W {
        PAGE1_WP_W { w: self }
    }
    #[doc = "Bit 2 - Write protection"]
    #[inline(always)]
    pub fn page2_wp(&mut self) -> PAGE2_WP_W {
        PAGE2_WP_W { w: self }
    }
    #[doc = "Bit 3 - Write protection"]
    #[inline(always)]
    pub fn page3_wp(&mut self) -> PAGE3_WP_W {
        PAGE3_WP_W { w: self }
    }
    #[doc = "Bit 4 - Write protection"]
    #[inline(always)]
    pub fn page4_wp(&mut self) -> PAGE4_WP_W {
        PAGE4_WP_W { w: self }
    }
    #[doc = "Bit 5 - Write protection"]
    #[inline(always)]
    pub fn page5_wp(&mut self) -> PAGE5_WP_W {
        PAGE5_WP_W { w: self }
    }
    #[doc = "Bit 6 - Write protection"]
    #[inline(always)]
    pub fn page6_wp(&mut self) -> PAGE6_WP_W {
        PAGE6_WP_W { w: self }
    }
    #[doc = "Bit 7 - Write protection"]
    #[inline(always)]
    pub fn page7_wp(&mut self) -> PAGE7_WP_W {
        PAGE7_WP_W { w: self }
    }
    #[doc = "Bit 8 - Write protection"]
    #[inline(always)]
    pub fn page8_wp(&mut self) -> PAGE8_WP_W {
        PAGE8_WP_W { w: self }
    }
    #[doc = "Bit 9 - Write protection"]
    #[inline(always)]
    pub fn page9_wp(&mut self) -> PAGE9_WP_W {
        PAGE9_WP_W { w: self }
    }
    #[doc = "Bit 10 - Write protection"]
    #[inline(always)]
    pub fn page10_wp(&mut self) -> PAGE10_WP_W {
        PAGE10_WP_W { w: self }
    }
    #[doc = "Bit 11 - Write protection"]
    #[inline(always)]
    pub fn page11_wp(&mut self) -> PAGE11_WP_W {
        PAGE11_WP_W { w: self }
    }
    #[doc = "Bit 12 - Write protection"]
    #[inline(always)]
    pub fn page12_wp(&mut self) -> PAGE12_WP_W {
        PAGE12_WP_W { w: self }
    }
    #[doc = "Bit 13 - Write protection"]
    #[inline(always)]
    pub fn page13_wp(&mut self) -> PAGE13_WP_W {
        PAGE13_WP_W { w: self }
    }
    #[doc = "Bit 14 - Write protection"]
    #[inline(always)]
    pub fn page14_wp(&mut self) -> PAGE14_WP_W {
        PAGE14_WP_W { w: self }
    }
    #[doc = "Bit 15 - Write protection"]
    #[inline(always)]
    pub fn page15_wp(&mut self) -> PAGE15_WP_W {
        PAGE15_WP_W { w: self }
    }
    #[doc = "Bit 16 - Write protection"]
    #[inline(always)]
    pub fn page16_wp(&mut self) -> PAGE16_WP_W {
        PAGE16_WP_W { w: self }
    }
    #[doc = "Bit 17 - Write protection"]
    #[inline(always)]
    pub fn page17_wp(&mut self) -> PAGE17_WP_W {
        PAGE17_WP_W { w: self }
    }
    #[doc = "Bit 18 - Write protection"]
    #[inline(always)]
    pub fn page18_wp(&mut self) -> PAGE18_WP_W {
        PAGE18_WP_W { w: self }
    }
    #[doc = "Bit 19 - Write protection"]
    #[inline(always)]
    pub fn page19_wp(&mut self) -> PAGE19_WP_W {
        PAGE19_WP_W { w: self }
    }
    #[doc = "Bit 20 - Write protection"]
    #[inline(always)]
    pub fn page20_wp(&mut self) -> PAGE20_WP_W {
        PAGE20_WP_W { w: self }
    }
    #[doc = "Bit 21 - Write protection"]
    #[inline(always)]
    pub fn page21_wp(&mut self) -> PAGE21_WP_W {
        PAGE21_WP_W { w: self }
    }
    #[doc = "Bit 22 - Write protection"]
    #[inline(always)]
    pub fn page22_wp(&mut self) -> PAGE22_WP_W {
        PAGE22_WP_W { w: self }
    }
    #[doc = "Bit 23 - Write protection"]
    #[inline(always)]
    pub fn page23_wp(&mut self) -> PAGE23_WP_W {
        PAGE23_WP_W { w: self }
    }
    #[doc = "Bit 24 - Write protection"]
    #[inline(always)]
    pub fn page24_wp(&mut self) -> PAGE24_WP_W {
        PAGE24_WP_W { w: self }
    }
    #[doc = "Bit 25 - Write protection"]
    #[inline(always)]
    pub fn page25_wp(&mut self) -> PAGE25_WP_W {
        PAGE25_WP_W { w: self }
    }
    #[doc = "Bit 26 - Write protection"]
    #[inline(always)]
    pub fn page26_wp(&mut self) -> PAGE26_WP_W {
        PAGE26_WP_W { w: self }
    }
    #[doc = "Bit 27 - Write protection"]
    #[inline(always)]
    pub fn page27_wp(&mut self) -> PAGE27_WP_W {
        PAGE27_WP_W { w: self }
    }
    #[doc = "Bit 28 - Write protection"]
    #[inline(always)]
    pub fn page28_wp(&mut self) -> PAGE28_WP_W {
        PAGE28_WP_W { w: self }
    }
    #[doc = "Bit 29 - Write protection"]
    #[inline(always)]
    pub fn page29_wp(&mut self) -> PAGE29_WP_W {
        PAGE29_WP_W { w: self }
    }
    #[doc = "Bit 30 - Write protection"]
    #[inline(always)]
    pub fn page30_wp(&mut self) -> PAGE30_WP_W {
        PAGE30_WP_W { w: self }
    }
    #[doc = "Bit 31 - Write protection"]
    #[inline(always)]
    pub fn page31_wp(&mut self) -> PAGE31_WP_W {
        PAGE31_WP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Write protection register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swpr](index.html) module"]
pub struct SWPR_SPEC;
impl crate::RegisterSpec for SWPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swpr::R](R) reader structure"]
impl crate::Readable for SWPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swpr::W](W) writer structure"]
impl crate::Writable for SWPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWPR to value 0"]
impl crate::Resettable for SWPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
