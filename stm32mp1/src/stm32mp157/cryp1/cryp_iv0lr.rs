#[doc = "Register `CRYP_IV0LR` reader"]
pub struct R(crate::R<CRYP_IV0LR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_IV0LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_IV0LR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_IV0LR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYP_IV0LR` writer"]
pub struct W(crate::W<CRYP_IV0LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYP_IV0LR_SPEC>;
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
impl From<crate::W<CRYP_IV0LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYP_IV0LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV31` reader - IV31"]
pub struct IV31_R(crate::FieldReader<bool, bool>);
impl IV31_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV31` writer - IV31"]
pub struct IV31_W<'a> {
    w: &'a mut W,
}
impl<'a> IV31_W<'a> {
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
#[doc = "Field `IV30` reader - IV30"]
pub struct IV30_R(crate::FieldReader<bool, bool>);
impl IV30_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV30` writer - IV30"]
pub struct IV30_W<'a> {
    w: &'a mut W,
}
impl<'a> IV30_W<'a> {
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
#[doc = "Field `IV29` reader - IV29"]
pub struct IV29_R(crate::FieldReader<bool, bool>);
impl IV29_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV29` writer - IV29"]
pub struct IV29_W<'a> {
    w: &'a mut W,
}
impl<'a> IV29_W<'a> {
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
#[doc = "Field `IV28` reader - IV28"]
pub struct IV28_R(crate::FieldReader<bool, bool>);
impl IV28_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV28` writer - IV28"]
pub struct IV28_W<'a> {
    w: &'a mut W,
}
impl<'a> IV28_W<'a> {
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
#[doc = "Field `IV27` reader - IV27"]
pub struct IV27_R(crate::FieldReader<bool, bool>);
impl IV27_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV27` writer - IV27"]
pub struct IV27_W<'a> {
    w: &'a mut W,
}
impl<'a> IV27_W<'a> {
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
#[doc = "Field `IV26` reader - IV26"]
pub struct IV26_R(crate::FieldReader<bool, bool>);
impl IV26_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV26` writer - IV26"]
pub struct IV26_W<'a> {
    w: &'a mut W,
}
impl<'a> IV26_W<'a> {
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
#[doc = "Field `IV25` reader - IV25"]
pub struct IV25_R(crate::FieldReader<bool, bool>);
impl IV25_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV25` writer - IV25"]
pub struct IV25_W<'a> {
    w: &'a mut W,
}
impl<'a> IV25_W<'a> {
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
#[doc = "Field `IV24` reader - IV24"]
pub struct IV24_R(crate::FieldReader<bool, bool>);
impl IV24_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV24` writer - IV24"]
pub struct IV24_W<'a> {
    w: &'a mut W,
}
impl<'a> IV24_W<'a> {
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
#[doc = "Field `IV23` reader - IV23"]
pub struct IV23_R(crate::FieldReader<bool, bool>);
impl IV23_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV23` writer - IV23"]
pub struct IV23_W<'a> {
    w: &'a mut W,
}
impl<'a> IV23_W<'a> {
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
#[doc = "Field `IV22` reader - IV22"]
pub struct IV22_R(crate::FieldReader<bool, bool>);
impl IV22_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV22` writer - IV22"]
pub struct IV22_W<'a> {
    w: &'a mut W,
}
impl<'a> IV22_W<'a> {
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
#[doc = "Field `IV21` reader - IV21"]
pub struct IV21_R(crate::FieldReader<bool, bool>);
impl IV21_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV21` writer - IV21"]
pub struct IV21_W<'a> {
    w: &'a mut W,
}
impl<'a> IV21_W<'a> {
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
#[doc = "Field `IV20` reader - IV20"]
pub struct IV20_R(crate::FieldReader<bool, bool>);
impl IV20_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV20` writer - IV20"]
pub struct IV20_W<'a> {
    w: &'a mut W,
}
impl<'a> IV20_W<'a> {
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
#[doc = "Field `IV19` reader - IV19"]
pub struct IV19_R(crate::FieldReader<bool, bool>);
impl IV19_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV19` writer - IV19"]
pub struct IV19_W<'a> {
    w: &'a mut W,
}
impl<'a> IV19_W<'a> {
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
#[doc = "Field `IV18` reader - IV18"]
pub struct IV18_R(crate::FieldReader<bool, bool>);
impl IV18_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV18` writer - IV18"]
pub struct IV18_W<'a> {
    w: &'a mut W,
}
impl<'a> IV18_W<'a> {
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
#[doc = "Field `IV17` reader - IV17"]
pub struct IV17_R(crate::FieldReader<bool, bool>);
impl IV17_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV17` writer - IV17"]
pub struct IV17_W<'a> {
    w: &'a mut W,
}
impl<'a> IV17_W<'a> {
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
#[doc = "Field `IV16` reader - IV16"]
pub struct IV16_R(crate::FieldReader<bool, bool>);
impl IV16_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV16` writer - IV16"]
pub struct IV16_W<'a> {
    w: &'a mut W,
}
impl<'a> IV16_W<'a> {
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
#[doc = "Field `IV15` reader - IV15"]
pub struct IV15_R(crate::FieldReader<bool, bool>);
impl IV15_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV15` writer - IV15"]
pub struct IV15_W<'a> {
    w: &'a mut W,
}
impl<'a> IV15_W<'a> {
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
#[doc = "Field `IV14` reader - IV14"]
pub struct IV14_R(crate::FieldReader<bool, bool>);
impl IV14_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV14` writer - IV14"]
pub struct IV14_W<'a> {
    w: &'a mut W,
}
impl<'a> IV14_W<'a> {
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
#[doc = "Field `IV13` reader - IV13"]
pub struct IV13_R(crate::FieldReader<bool, bool>);
impl IV13_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV13` writer - IV13"]
pub struct IV13_W<'a> {
    w: &'a mut W,
}
impl<'a> IV13_W<'a> {
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
#[doc = "Field `IV12` reader - IV12"]
pub struct IV12_R(crate::FieldReader<bool, bool>);
impl IV12_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV12` writer - IV12"]
pub struct IV12_W<'a> {
    w: &'a mut W,
}
impl<'a> IV12_W<'a> {
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
#[doc = "Field `IV11` reader - IV11"]
pub struct IV11_R(crate::FieldReader<bool, bool>);
impl IV11_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV11` writer - IV11"]
pub struct IV11_W<'a> {
    w: &'a mut W,
}
impl<'a> IV11_W<'a> {
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
#[doc = "Field `IV10` reader - IV10"]
pub struct IV10_R(crate::FieldReader<bool, bool>);
impl IV10_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV10` writer - IV10"]
pub struct IV10_W<'a> {
    w: &'a mut W,
}
impl<'a> IV10_W<'a> {
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
#[doc = "Field `IV9` reader - IV9"]
pub struct IV9_R(crate::FieldReader<bool, bool>);
impl IV9_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV9` writer - IV9"]
pub struct IV9_W<'a> {
    w: &'a mut W,
}
impl<'a> IV9_W<'a> {
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
#[doc = "Field `IV8` reader - IV8"]
pub struct IV8_R(crate::FieldReader<bool, bool>);
impl IV8_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV8` writer - IV8"]
pub struct IV8_W<'a> {
    w: &'a mut W,
}
impl<'a> IV8_W<'a> {
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
#[doc = "Field `IV7` reader - IV7"]
pub struct IV7_R(crate::FieldReader<bool, bool>);
impl IV7_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV7` writer - IV7"]
pub struct IV7_W<'a> {
    w: &'a mut W,
}
impl<'a> IV7_W<'a> {
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
#[doc = "Field `IV6` reader - IV6"]
pub struct IV6_R(crate::FieldReader<bool, bool>);
impl IV6_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV6` writer - IV6"]
pub struct IV6_W<'a> {
    w: &'a mut W,
}
impl<'a> IV6_W<'a> {
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
#[doc = "Field `IV5` reader - IV5"]
pub struct IV5_R(crate::FieldReader<bool, bool>);
impl IV5_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV5` writer - IV5"]
pub struct IV5_W<'a> {
    w: &'a mut W,
}
impl<'a> IV5_W<'a> {
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
#[doc = "Field `IV4` reader - IV4"]
pub struct IV4_R(crate::FieldReader<bool, bool>);
impl IV4_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV4` writer - IV4"]
pub struct IV4_W<'a> {
    w: &'a mut W,
}
impl<'a> IV4_W<'a> {
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
#[doc = "Field `IV3` reader - IV3"]
pub struct IV3_R(crate::FieldReader<bool, bool>);
impl IV3_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV3` writer - IV3"]
pub struct IV3_W<'a> {
    w: &'a mut W,
}
impl<'a> IV3_W<'a> {
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
#[doc = "Field `IV2` reader - IV2"]
pub struct IV2_R(crate::FieldReader<bool, bool>);
impl IV2_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV2` writer - IV2"]
pub struct IV2_W<'a> {
    w: &'a mut W,
}
impl<'a> IV2_W<'a> {
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
#[doc = "Field `IV1` reader - IV1"]
pub struct IV1_R(crate::FieldReader<bool, bool>);
impl IV1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV1` writer - IV1"]
pub struct IV1_W<'a> {
    w: &'a mut W,
}
impl<'a> IV1_W<'a> {
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
#[doc = "Field `IV0` reader - IV0"]
pub struct IV0_R(crate::FieldReader<bool, bool>);
impl IV0_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV0` writer - IV0"]
pub struct IV0_W<'a> {
    w: &'a mut W,
}
impl<'a> IV0_W<'a> {
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
    #[doc = "Bit 0 - IV31"]
    #[inline(always)]
    pub fn iv31(&self) -> IV31_R {
        IV31_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IV30"]
    #[inline(always)]
    pub fn iv30(&self) -> IV30_R {
        IV30_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IV29"]
    #[inline(always)]
    pub fn iv29(&self) -> IV29_R {
        IV29_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IV28"]
    #[inline(always)]
    pub fn iv28(&self) -> IV28_R {
        IV28_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IV27"]
    #[inline(always)]
    pub fn iv27(&self) -> IV27_R {
        IV27_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IV26"]
    #[inline(always)]
    pub fn iv26(&self) -> IV26_R {
        IV26_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IV25"]
    #[inline(always)]
    pub fn iv25(&self) -> IV25_R {
        IV25_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IV24"]
    #[inline(always)]
    pub fn iv24(&self) -> IV24_R {
        IV24_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - IV23"]
    #[inline(always)]
    pub fn iv23(&self) -> IV23_R {
        IV23_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - IV22"]
    #[inline(always)]
    pub fn iv22(&self) -> IV22_R {
        IV22_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - IV21"]
    #[inline(always)]
    pub fn iv21(&self) -> IV21_R {
        IV21_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - IV20"]
    #[inline(always)]
    pub fn iv20(&self) -> IV20_R {
        IV20_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IV19"]
    #[inline(always)]
    pub fn iv19(&self) -> IV19_R {
        IV19_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - IV18"]
    #[inline(always)]
    pub fn iv18(&self) -> IV18_R {
        IV18_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - IV17"]
    #[inline(always)]
    pub fn iv17(&self) -> IV17_R {
        IV17_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IV16"]
    #[inline(always)]
    pub fn iv16(&self) -> IV16_R {
        IV16_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - IV15"]
    #[inline(always)]
    pub fn iv15(&self) -> IV15_R {
        IV15_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - IV14"]
    #[inline(always)]
    pub fn iv14(&self) -> IV14_R {
        IV14_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IV13"]
    #[inline(always)]
    pub fn iv13(&self) -> IV13_R {
        IV13_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - IV12"]
    #[inline(always)]
    pub fn iv12(&self) -> IV12_R {
        IV12_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IV11"]
    #[inline(always)]
    pub fn iv11(&self) -> IV11_R {
        IV11_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - IV10"]
    #[inline(always)]
    pub fn iv10(&self) -> IV10_R {
        IV10_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - IV9"]
    #[inline(always)]
    pub fn iv9(&self) -> IV9_R {
        IV9_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - IV8"]
    #[inline(always)]
    pub fn iv8(&self) -> IV8_R {
        IV8_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - IV7"]
    #[inline(always)]
    pub fn iv7(&self) -> IV7_R {
        IV7_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - IV6"]
    #[inline(always)]
    pub fn iv6(&self) -> IV6_R {
        IV6_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - IV5"]
    #[inline(always)]
    pub fn iv5(&self) -> IV5_R {
        IV5_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IV4"]
    #[inline(always)]
    pub fn iv4(&self) -> IV4_R {
        IV4_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - IV3"]
    #[inline(always)]
    pub fn iv3(&self) -> IV3_R {
        IV3_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - IV2"]
    #[inline(always)]
    pub fn iv2(&self) -> IV2_R {
        IV2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - IV1"]
    #[inline(always)]
    pub fn iv1(&self) -> IV1_R {
        IV1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - IV0"]
    #[inline(always)]
    pub fn iv0(&self) -> IV0_R {
        IV0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IV31"]
    #[inline(always)]
    pub fn iv31(&mut self) -> IV31_W {
        IV31_W { w: self }
    }
    #[doc = "Bit 1 - IV30"]
    #[inline(always)]
    pub fn iv30(&mut self) -> IV30_W {
        IV30_W { w: self }
    }
    #[doc = "Bit 2 - IV29"]
    #[inline(always)]
    pub fn iv29(&mut self) -> IV29_W {
        IV29_W { w: self }
    }
    #[doc = "Bit 3 - IV28"]
    #[inline(always)]
    pub fn iv28(&mut self) -> IV28_W {
        IV28_W { w: self }
    }
    #[doc = "Bit 4 - IV27"]
    #[inline(always)]
    pub fn iv27(&mut self) -> IV27_W {
        IV27_W { w: self }
    }
    #[doc = "Bit 5 - IV26"]
    #[inline(always)]
    pub fn iv26(&mut self) -> IV26_W {
        IV26_W { w: self }
    }
    #[doc = "Bit 6 - IV25"]
    #[inline(always)]
    pub fn iv25(&mut self) -> IV25_W {
        IV25_W { w: self }
    }
    #[doc = "Bit 7 - IV24"]
    #[inline(always)]
    pub fn iv24(&mut self) -> IV24_W {
        IV24_W { w: self }
    }
    #[doc = "Bit 8 - IV23"]
    #[inline(always)]
    pub fn iv23(&mut self) -> IV23_W {
        IV23_W { w: self }
    }
    #[doc = "Bit 9 - IV22"]
    #[inline(always)]
    pub fn iv22(&mut self) -> IV22_W {
        IV22_W { w: self }
    }
    #[doc = "Bit 10 - IV21"]
    #[inline(always)]
    pub fn iv21(&mut self) -> IV21_W {
        IV21_W { w: self }
    }
    #[doc = "Bit 11 - IV20"]
    #[inline(always)]
    pub fn iv20(&mut self) -> IV20_W {
        IV20_W { w: self }
    }
    #[doc = "Bit 12 - IV19"]
    #[inline(always)]
    pub fn iv19(&mut self) -> IV19_W {
        IV19_W { w: self }
    }
    #[doc = "Bit 13 - IV18"]
    #[inline(always)]
    pub fn iv18(&mut self) -> IV18_W {
        IV18_W { w: self }
    }
    #[doc = "Bit 14 - IV17"]
    #[inline(always)]
    pub fn iv17(&mut self) -> IV17_W {
        IV17_W { w: self }
    }
    #[doc = "Bit 15 - IV16"]
    #[inline(always)]
    pub fn iv16(&mut self) -> IV16_W {
        IV16_W { w: self }
    }
    #[doc = "Bit 16 - IV15"]
    #[inline(always)]
    pub fn iv15(&mut self) -> IV15_W {
        IV15_W { w: self }
    }
    #[doc = "Bit 17 - IV14"]
    #[inline(always)]
    pub fn iv14(&mut self) -> IV14_W {
        IV14_W { w: self }
    }
    #[doc = "Bit 18 - IV13"]
    #[inline(always)]
    pub fn iv13(&mut self) -> IV13_W {
        IV13_W { w: self }
    }
    #[doc = "Bit 19 - IV12"]
    #[inline(always)]
    pub fn iv12(&mut self) -> IV12_W {
        IV12_W { w: self }
    }
    #[doc = "Bit 20 - IV11"]
    #[inline(always)]
    pub fn iv11(&mut self) -> IV11_W {
        IV11_W { w: self }
    }
    #[doc = "Bit 21 - IV10"]
    #[inline(always)]
    pub fn iv10(&mut self) -> IV10_W {
        IV10_W { w: self }
    }
    #[doc = "Bit 22 - IV9"]
    #[inline(always)]
    pub fn iv9(&mut self) -> IV9_W {
        IV9_W { w: self }
    }
    #[doc = "Bit 23 - IV8"]
    #[inline(always)]
    pub fn iv8(&mut self) -> IV8_W {
        IV8_W { w: self }
    }
    #[doc = "Bit 24 - IV7"]
    #[inline(always)]
    pub fn iv7(&mut self) -> IV7_W {
        IV7_W { w: self }
    }
    #[doc = "Bit 25 - IV6"]
    #[inline(always)]
    pub fn iv6(&mut self) -> IV6_W {
        IV6_W { w: self }
    }
    #[doc = "Bit 26 - IV5"]
    #[inline(always)]
    pub fn iv5(&mut self) -> IV5_W {
        IV5_W { w: self }
    }
    #[doc = "Bit 27 - IV4"]
    #[inline(always)]
    pub fn iv4(&mut self) -> IV4_W {
        IV4_W { w: self }
    }
    #[doc = "Bit 28 - IV3"]
    #[inline(always)]
    pub fn iv3(&mut self) -> IV3_W {
        IV3_W { w: self }
    }
    #[doc = "Bit 29 - IV2"]
    #[inline(always)]
    pub fn iv2(&mut self) -> IV2_W {
        IV2_W { w: self }
    }
    #[doc = "Bit 30 - IV1"]
    #[inline(always)]
    pub fn iv1(&mut self) -> IV1_W {
        IV1_W { w: self }
    }
    #[doc = "Bit 31 - IV0"]
    #[inline(always)]
    pub fn iv0(&mut self) -> IV0_W {
        IV0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The CRYP_IV0...1(L/R)R are the left-word and right-word registers for the initialization vector (64 bits for DES/TDES and 128 bits for AES). For more information refer to Section39.3.18: CRYP initialization vector registers. IV0 is the leftmost bit whereas IV63 (DES, TDES) or IV127 (AES) are the rightmost bits of the initialization vector. IV1(L/R)R is used only in the AES. Only CRYP_IV0(L/R) is used in DES/TDES. Write access to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_iv0lr](index.html) module"]
pub struct CRYP_IV0LR_SPEC;
impl crate::RegisterSpec for CRYP_IV0LR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryp_iv0lr::R](R) reader structure"]
impl crate::Readable for CRYP_IV0LR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cryp_iv0lr::W](W) writer structure"]
impl crate::Writable for CRYP_IV0LR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYP_IV0LR to value 0"]
impl crate::Resettable for CRYP_IV0LR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
