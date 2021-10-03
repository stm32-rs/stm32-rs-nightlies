#[doc = "Register `MPCBB2_VCTR27` reader"]
pub struct R(crate::R<MPCBB2_VCTR27_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR27_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR27_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR27_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR27` writer"]
pub struct W(crate::W<MPCBB2_VCTR27_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR27_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR27_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR27_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B864` reader - B864"]
pub struct B864_R(crate::FieldReader<bool, bool>);
impl B864_R {
    pub(crate) fn new(bits: bool) -> Self {
        B864_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B864_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B864` writer - B864"]
pub struct B864_W<'a> {
    w: &'a mut W,
}
impl<'a> B864_W<'a> {
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
#[doc = "Field `B865` reader - B865"]
pub struct B865_R(crate::FieldReader<bool, bool>);
impl B865_R {
    pub(crate) fn new(bits: bool) -> Self {
        B865_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B865_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B865` writer - B865"]
pub struct B865_W<'a> {
    w: &'a mut W,
}
impl<'a> B865_W<'a> {
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
#[doc = "Field `B866` reader - B866"]
pub struct B866_R(crate::FieldReader<bool, bool>);
impl B866_R {
    pub(crate) fn new(bits: bool) -> Self {
        B866_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B866_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B866` writer - B866"]
pub struct B866_W<'a> {
    w: &'a mut W,
}
impl<'a> B866_W<'a> {
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
#[doc = "Field `B867` reader - B867"]
pub struct B867_R(crate::FieldReader<bool, bool>);
impl B867_R {
    pub(crate) fn new(bits: bool) -> Self {
        B867_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B867_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B867` writer - B867"]
pub struct B867_W<'a> {
    w: &'a mut W,
}
impl<'a> B867_W<'a> {
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
#[doc = "Field `B868` reader - B868"]
pub struct B868_R(crate::FieldReader<bool, bool>);
impl B868_R {
    pub(crate) fn new(bits: bool) -> Self {
        B868_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B868_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B868` writer - B868"]
pub struct B868_W<'a> {
    w: &'a mut W,
}
impl<'a> B868_W<'a> {
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
#[doc = "Field `B869` reader - B869"]
pub struct B869_R(crate::FieldReader<bool, bool>);
impl B869_R {
    pub(crate) fn new(bits: bool) -> Self {
        B869_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B869_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B869` writer - B869"]
pub struct B869_W<'a> {
    w: &'a mut W,
}
impl<'a> B869_W<'a> {
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
#[doc = "Field `B870` reader - B870"]
pub struct B870_R(crate::FieldReader<bool, bool>);
impl B870_R {
    pub(crate) fn new(bits: bool) -> Self {
        B870_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B870_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B870` writer - B870"]
pub struct B870_W<'a> {
    w: &'a mut W,
}
impl<'a> B870_W<'a> {
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
#[doc = "Field `B871` reader - B871"]
pub struct B871_R(crate::FieldReader<bool, bool>);
impl B871_R {
    pub(crate) fn new(bits: bool) -> Self {
        B871_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B871_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B871` writer - B871"]
pub struct B871_W<'a> {
    w: &'a mut W,
}
impl<'a> B871_W<'a> {
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
#[doc = "Field `B872` reader - B872"]
pub struct B872_R(crate::FieldReader<bool, bool>);
impl B872_R {
    pub(crate) fn new(bits: bool) -> Self {
        B872_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B872_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B872` writer - B872"]
pub struct B872_W<'a> {
    w: &'a mut W,
}
impl<'a> B872_W<'a> {
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
#[doc = "Field `B873` reader - B873"]
pub struct B873_R(crate::FieldReader<bool, bool>);
impl B873_R {
    pub(crate) fn new(bits: bool) -> Self {
        B873_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B873_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B873` writer - B873"]
pub struct B873_W<'a> {
    w: &'a mut W,
}
impl<'a> B873_W<'a> {
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
#[doc = "Field `B874` reader - B874"]
pub struct B874_R(crate::FieldReader<bool, bool>);
impl B874_R {
    pub(crate) fn new(bits: bool) -> Self {
        B874_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B874_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B874` writer - B874"]
pub struct B874_W<'a> {
    w: &'a mut W,
}
impl<'a> B874_W<'a> {
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
#[doc = "Field `B875` reader - B875"]
pub struct B875_R(crate::FieldReader<bool, bool>);
impl B875_R {
    pub(crate) fn new(bits: bool) -> Self {
        B875_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B875_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B875` writer - B875"]
pub struct B875_W<'a> {
    w: &'a mut W,
}
impl<'a> B875_W<'a> {
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
#[doc = "Field `B876` reader - B876"]
pub struct B876_R(crate::FieldReader<bool, bool>);
impl B876_R {
    pub(crate) fn new(bits: bool) -> Self {
        B876_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B876_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B876` writer - B876"]
pub struct B876_W<'a> {
    w: &'a mut W,
}
impl<'a> B876_W<'a> {
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
#[doc = "Field `B877` reader - B877"]
pub struct B877_R(crate::FieldReader<bool, bool>);
impl B877_R {
    pub(crate) fn new(bits: bool) -> Self {
        B877_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B877_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B877` writer - B877"]
pub struct B877_W<'a> {
    w: &'a mut W,
}
impl<'a> B877_W<'a> {
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
#[doc = "Field `B878` reader - B878"]
pub struct B878_R(crate::FieldReader<bool, bool>);
impl B878_R {
    pub(crate) fn new(bits: bool) -> Self {
        B878_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B878_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B878` writer - B878"]
pub struct B878_W<'a> {
    w: &'a mut W,
}
impl<'a> B878_W<'a> {
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
#[doc = "Field `B879` reader - B879"]
pub struct B879_R(crate::FieldReader<bool, bool>);
impl B879_R {
    pub(crate) fn new(bits: bool) -> Self {
        B879_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B879_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B879` writer - B879"]
pub struct B879_W<'a> {
    w: &'a mut W,
}
impl<'a> B879_W<'a> {
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
#[doc = "Field `B880` reader - B880"]
pub struct B880_R(crate::FieldReader<bool, bool>);
impl B880_R {
    pub(crate) fn new(bits: bool) -> Self {
        B880_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B880_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B880` writer - B880"]
pub struct B880_W<'a> {
    w: &'a mut W,
}
impl<'a> B880_W<'a> {
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
#[doc = "Field `B881` reader - B881"]
pub struct B881_R(crate::FieldReader<bool, bool>);
impl B881_R {
    pub(crate) fn new(bits: bool) -> Self {
        B881_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B881_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B881` writer - B881"]
pub struct B881_W<'a> {
    w: &'a mut W,
}
impl<'a> B881_W<'a> {
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
#[doc = "Field `B882` reader - B882"]
pub struct B882_R(crate::FieldReader<bool, bool>);
impl B882_R {
    pub(crate) fn new(bits: bool) -> Self {
        B882_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B882_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B882` writer - B882"]
pub struct B882_W<'a> {
    w: &'a mut W,
}
impl<'a> B882_W<'a> {
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
#[doc = "Field `B883` reader - B883"]
pub struct B883_R(crate::FieldReader<bool, bool>);
impl B883_R {
    pub(crate) fn new(bits: bool) -> Self {
        B883_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B883_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B883` writer - B883"]
pub struct B883_W<'a> {
    w: &'a mut W,
}
impl<'a> B883_W<'a> {
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
#[doc = "Field `B884` reader - B884"]
pub struct B884_R(crate::FieldReader<bool, bool>);
impl B884_R {
    pub(crate) fn new(bits: bool) -> Self {
        B884_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B884_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B884` writer - B884"]
pub struct B884_W<'a> {
    w: &'a mut W,
}
impl<'a> B884_W<'a> {
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
#[doc = "Field `B885` reader - B885"]
pub struct B885_R(crate::FieldReader<bool, bool>);
impl B885_R {
    pub(crate) fn new(bits: bool) -> Self {
        B885_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B885_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B885` writer - B885"]
pub struct B885_W<'a> {
    w: &'a mut W,
}
impl<'a> B885_W<'a> {
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
#[doc = "Field `B886` reader - B886"]
pub struct B886_R(crate::FieldReader<bool, bool>);
impl B886_R {
    pub(crate) fn new(bits: bool) -> Self {
        B886_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B886_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B886` writer - B886"]
pub struct B886_W<'a> {
    w: &'a mut W,
}
impl<'a> B886_W<'a> {
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
#[doc = "Field `B887` reader - B887"]
pub struct B887_R(crate::FieldReader<bool, bool>);
impl B887_R {
    pub(crate) fn new(bits: bool) -> Self {
        B887_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B887_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B887` writer - B887"]
pub struct B887_W<'a> {
    w: &'a mut W,
}
impl<'a> B887_W<'a> {
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
#[doc = "Field `B888` reader - B888"]
pub struct B888_R(crate::FieldReader<bool, bool>);
impl B888_R {
    pub(crate) fn new(bits: bool) -> Self {
        B888_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B888_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B888` writer - B888"]
pub struct B888_W<'a> {
    w: &'a mut W,
}
impl<'a> B888_W<'a> {
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
#[doc = "Field `B889` reader - B889"]
pub struct B889_R(crate::FieldReader<bool, bool>);
impl B889_R {
    pub(crate) fn new(bits: bool) -> Self {
        B889_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B889_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B889` writer - B889"]
pub struct B889_W<'a> {
    w: &'a mut W,
}
impl<'a> B889_W<'a> {
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
#[doc = "Field `B890` reader - B890"]
pub struct B890_R(crate::FieldReader<bool, bool>);
impl B890_R {
    pub(crate) fn new(bits: bool) -> Self {
        B890_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B890_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B890` writer - B890"]
pub struct B890_W<'a> {
    w: &'a mut W,
}
impl<'a> B890_W<'a> {
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
#[doc = "Field `B891` reader - B891"]
pub struct B891_R(crate::FieldReader<bool, bool>);
impl B891_R {
    pub(crate) fn new(bits: bool) -> Self {
        B891_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B891_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B891` writer - B891"]
pub struct B891_W<'a> {
    w: &'a mut W,
}
impl<'a> B891_W<'a> {
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
#[doc = "Field `B892` reader - B892"]
pub struct B892_R(crate::FieldReader<bool, bool>);
impl B892_R {
    pub(crate) fn new(bits: bool) -> Self {
        B892_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B892_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B892` writer - B892"]
pub struct B892_W<'a> {
    w: &'a mut W,
}
impl<'a> B892_W<'a> {
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
#[doc = "Field `B893` reader - B893"]
pub struct B893_R(crate::FieldReader<bool, bool>);
impl B893_R {
    pub(crate) fn new(bits: bool) -> Self {
        B893_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B893_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B893` writer - B893"]
pub struct B893_W<'a> {
    w: &'a mut W,
}
impl<'a> B893_W<'a> {
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
#[doc = "Field `B894` reader - B894"]
pub struct B894_R(crate::FieldReader<bool, bool>);
impl B894_R {
    pub(crate) fn new(bits: bool) -> Self {
        B894_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B894_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B894` writer - B894"]
pub struct B894_W<'a> {
    w: &'a mut W,
}
impl<'a> B894_W<'a> {
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
#[doc = "Field `B895` reader - B895"]
pub struct B895_R(crate::FieldReader<bool, bool>);
impl B895_R {
    pub(crate) fn new(bits: bool) -> Self {
        B895_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B895_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B895` writer - B895"]
pub struct B895_W<'a> {
    w: &'a mut W,
}
impl<'a> B895_W<'a> {
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
    #[doc = "Bit 0 - B864"]
    #[inline(always)]
    pub fn b864(&self) -> B864_R {
        B864_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B865"]
    #[inline(always)]
    pub fn b865(&self) -> B865_R {
        B865_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B866"]
    #[inline(always)]
    pub fn b866(&self) -> B866_R {
        B866_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B867"]
    #[inline(always)]
    pub fn b867(&self) -> B867_R {
        B867_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B868"]
    #[inline(always)]
    pub fn b868(&self) -> B868_R {
        B868_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B869"]
    #[inline(always)]
    pub fn b869(&self) -> B869_R {
        B869_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B870"]
    #[inline(always)]
    pub fn b870(&self) -> B870_R {
        B870_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B871"]
    #[inline(always)]
    pub fn b871(&self) -> B871_R {
        B871_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B872"]
    #[inline(always)]
    pub fn b872(&self) -> B872_R {
        B872_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B873"]
    #[inline(always)]
    pub fn b873(&self) -> B873_R {
        B873_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B874"]
    #[inline(always)]
    pub fn b874(&self) -> B874_R {
        B874_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B875"]
    #[inline(always)]
    pub fn b875(&self) -> B875_R {
        B875_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B876"]
    #[inline(always)]
    pub fn b876(&self) -> B876_R {
        B876_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B877"]
    #[inline(always)]
    pub fn b877(&self) -> B877_R {
        B877_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B878"]
    #[inline(always)]
    pub fn b878(&self) -> B878_R {
        B878_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B879"]
    #[inline(always)]
    pub fn b879(&self) -> B879_R {
        B879_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B880"]
    #[inline(always)]
    pub fn b880(&self) -> B880_R {
        B880_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B881"]
    #[inline(always)]
    pub fn b881(&self) -> B881_R {
        B881_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B882"]
    #[inline(always)]
    pub fn b882(&self) -> B882_R {
        B882_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B883"]
    #[inline(always)]
    pub fn b883(&self) -> B883_R {
        B883_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B884"]
    #[inline(always)]
    pub fn b884(&self) -> B884_R {
        B884_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B885"]
    #[inline(always)]
    pub fn b885(&self) -> B885_R {
        B885_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B886"]
    #[inline(always)]
    pub fn b886(&self) -> B886_R {
        B886_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B887"]
    #[inline(always)]
    pub fn b887(&self) -> B887_R {
        B887_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B888"]
    #[inline(always)]
    pub fn b888(&self) -> B888_R {
        B888_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B889"]
    #[inline(always)]
    pub fn b889(&self) -> B889_R {
        B889_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B890"]
    #[inline(always)]
    pub fn b890(&self) -> B890_R {
        B890_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B891"]
    #[inline(always)]
    pub fn b891(&self) -> B891_R {
        B891_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B892"]
    #[inline(always)]
    pub fn b892(&self) -> B892_R {
        B892_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B893"]
    #[inline(always)]
    pub fn b893(&self) -> B893_R {
        B893_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B894"]
    #[inline(always)]
    pub fn b894(&self) -> B894_R {
        B894_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B895"]
    #[inline(always)]
    pub fn b895(&self) -> B895_R {
        B895_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B864"]
    #[inline(always)]
    pub fn b864(&mut self) -> B864_W {
        B864_W { w: self }
    }
    #[doc = "Bit 1 - B865"]
    #[inline(always)]
    pub fn b865(&mut self) -> B865_W {
        B865_W { w: self }
    }
    #[doc = "Bit 2 - B866"]
    #[inline(always)]
    pub fn b866(&mut self) -> B866_W {
        B866_W { w: self }
    }
    #[doc = "Bit 3 - B867"]
    #[inline(always)]
    pub fn b867(&mut self) -> B867_W {
        B867_W { w: self }
    }
    #[doc = "Bit 4 - B868"]
    #[inline(always)]
    pub fn b868(&mut self) -> B868_W {
        B868_W { w: self }
    }
    #[doc = "Bit 5 - B869"]
    #[inline(always)]
    pub fn b869(&mut self) -> B869_W {
        B869_W { w: self }
    }
    #[doc = "Bit 6 - B870"]
    #[inline(always)]
    pub fn b870(&mut self) -> B870_W {
        B870_W { w: self }
    }
    #[doc = "Bit 7 - B871"]
    #[inline(always)]
    pub fn b871(&mut self) -> B871_W {
        B871_W { w: self }
    }
    #[doc = "Bit 8 - B872"]
    #[inline(always)]
    pub fn b872(&mut self) -> B872_W {
        B872_W { w: self }
    }
    #[doc = "Bit 9 - B873"]
    #[inline(always)]
    pub fn b873(&mut self) -> B873_W {
        B873_W { w: self }
    }
    #[doc = "Bit 10 - B874"]
    #[inline(always)]
    pub fn b874(&mut self) -> B874_W {
        B874_W { w: self }
    }
    #[doc = "Bit 11 - B875"]
    #[inline(always)]
    pub fn b875(&mut self) -> B875_W {
        B875_W { w: self }
    }
    #[doc = "Bit 12 - B876"]
    #[inline(always)]
    pub fn b876(&mut self) -> B876_W {
        B876_W { w: self }
    }
    #[doc = "Bit 13 - B877"]
    #[inline(always)]
    pub fn b877(&mut self) -> B877_W {
        B877_W { w: self }
    }
    #[doc = "Bit 14 - B878"]
    #[inline(always)]
    pub fn b878(&mut self) -> B878_W {
        B878_W { w: self }
    }
    #[doc = "Bit 15 - B879"]
    #[inline(always)]
    pub fn b879(&mut self) -> B879_W {
        B879_W { w: self }
    }
    #[doc = "Bit 16 - B880"]
    #[inline(always)]
    pub fn b880(&mut self) -> B880_W {
        B880_W { w: self }
    }
    #[doc = "Bit 17 - B881"]
    #[inline(always)]
    pub fn b881(&mut self) -> B881_W {
        B881_W { w: self }
    }
    #[doc = "Bit 18 - B882"]
    #[inline(always)]
    pub fn b882(&mut self) -> B882_W {
        B882_W { w: self }
    }
    #[doc = "Bit 19 - B883"]
    #[inline(always)]
    pub fn b883(&mut self) -> B883_W {
        B883_W { w: self }
    }
    #[doc = "Bit 20 - B884"]
    #[inline(always)]
    pub fn b884(&mut self) -> B884_W {
        B884_W { w: self }
    }
    #[doc = "Bit 21 - B885"]
    #[inline(always)]
    pub fn b885(&mut self) -> B885_W {
        B885_W { w: self }
    }
    #[doc = "Bit 22 - B886"]
    #[inline(always)]
    pub fn b886(&mut self) -> B886_W {
        B886_W { w: self }
    }
    #[doc = "Bit 23 - B887"]
    #[inline(always)]
    pub fn b887(&mut self) -> B887_W {
        B887_W { w: self }
    }
    #[doc = "Bit 24 - B888"]
    #[inline(always)]
    pub fn b888(&mut self) -> B888_W {
        B888_W { w: self }
    }
    #[doc = "Bit 25 - B889"]
    #[inline(always)]
    pub fn b889(&mut self) -> B889_W {
        B889_W { w: self }
    }
    #[doc = "Bit 26 - B890"]
    #[inline(always)]
    pub fn b890(&mut self) -> B890_W {
        B890_W { w: self }
    }
    #[doc = "Bit 27 - B891"]
    #[inline(always)]
    pub fn b891(&mut self) -> B891_W {
        B891_W { w: self }
    }
    #[doc = "Bit 28 - B892"]
    #[inline(always)]
    pub fn b892(&mut self) -> B892_W {
        B892_W { w: self }
    }
    #[doc = "Bit 29 - B893"]
    #[inline(always)]
    pub fn b893(&mut self) -> B893_W {
        B893_W { w: self }
    }
    #[doc = "Bit 30 - B894"]
    #[inline(always)]
    pub fn b894(&mut self) -> B894_W {
        B894_W { w: self }
    }
    #[doc = "Bit 31 - B895"]
    #[inline(always)]
    pub fn b895(&mut self) -> B895_W {
        B895_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr27](index.html) module"]
pub struct MPCBB2_VCTR27_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR27_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr27::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR27_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr27::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR27_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR27 to value 0"]
impl crate::Resettable for MPCBB2_VCTR27_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
