#[doc = "Register `MPCBB2_VCTR51` reader"]
pub struct R(crate::R<MPCBB2_VCTR51_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR51_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR51_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR51_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR51` writer"]
pub struct W(crate::W<MPCBB2_VCTR51_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR51_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR51_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR51_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1632` reader - B1632"]
pub struct B1632_R(crate::FieldReader<bool, bool>);
impl B1632_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1632_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1632_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1632` writer - B1632"]
pub struct B1632_W<'a> {
    w: &'a mut W,
}
impl<'a> B1632_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1633` reader - B1633"]
pub struct B1633_R(crate::FieldReader<bool, bool>);
impl B1633_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1633_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1633_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1633` writer - B1633"]
pub struct B1633_W<'a> {
    w: &'a mut W,
}
impl<'a> B1633_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1634` reader - B1634"]
pub struct B1634_R(crate::FieldReader<bool, bool>);
impl B1634_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1634_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1634_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1634` writer - B1634"]
pub struct B1634_W<'a> {
    w: &'a mut W,
}
impl<'a> B1634_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1635` reader - B1635"]
pub struct B1635_R(crate::FieldReader<bool, bool>);
impl B1635_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1635_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1635_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1635` writer - B1635"]
pub struct B1635_W<'a> {
    w: &'a mut W,
}
impl<'a> B1635_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1636` reader - B1636"]
pub struct B1636_R(crate::FieldReader<bool, bool>);
impl B1636_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1636_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1636_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1636` writer - B1636"]
pub struct B1636_W<'a> {
    w: &'a mut W,
}
impl<'a> B1636_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1637` reader - B1637"]
pub struct B1637_R(crate::FieldReader<bool, bool>);
impl B1637_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1637_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1637_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1637` writer - B1637"]
pub struct B1637_W<'a> {
    w: &'a mut W,
}
impl<'a> B1637_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1638` reader - B1638"]
pub struct B1638_R(crate::FieldReader<bool, bool>);
impl B1638_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1638_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1638_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1638` writer - B1638"]
pub struct B1638_W<'a> {
    w: &'a mut W,
}
impl<'a> B1638_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1639` reader - B1639"]
pub struct B1639_R(crate::FieldReader<bool, bool>);
impl B1639_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1639_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1639_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1639` writer - B1639"]
pub struct B1639_W<'a> {
    w: &'a mut W,
}
impl<'a> B1639_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1640` reader - B1640"]
pub struct B1640_R(crate::FieldReader<bool, bool>);
impl B1640_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1640_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1640_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1640` writer - B1640"]
pub struct B1640_W<'a> {
    w: &'a mut W,
}
impl<'a> B1640_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1641` reader - B1641"]
pub struct B1641_R(crate::FieldReader<bool, bool>);
impl B1641_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1641_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1641_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1641` writer - B1641"]
pub struct B1641_W<'a> {
    w: &'a mut W,
}
impl<'a> B1641_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1642` reader - B1642"]
pub struct B1642_R(crate::FieldReader<bool, bool>);
impl B1642_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1642_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1642_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1642` writer - B1642"]
pub struct B1642_W<'a> {
    w: &'a mut W,
}
impl<'a> B1642_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1643` reader - B1643"]
pub struct B1643_R(crate::FieldReader<bool, bool>);
impl B1643_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1643_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1643_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1643` writer - B1643"]
pub struct B1643_W<'a> {
    w: &'a mut W,
}
impl<'a> B1643_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1644` reader - B1644"]
pub struct B1644_R(crate::FieldReader<bool, bool>);
impl B1644_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1644_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1644_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1644` writer - B1644"]
pub struct B1644_W<'a> {
    w: &'a mut W,
}
impl<'a> B1644_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1645` reader - B1645"]
pub struct B1645_R(crate::FieldReader<bool, bool>);
impl B1645_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1645_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1645_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1645` writer - B1645"]
pub struct B1645_W<'a> {
    w: &'a mut W,
}
impl<'a> B1645_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1646` reader - B1646"]
pub struct B1646_R(crate::FieldReader<bool, bool>);
impl B1646_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1646_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1646_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1646` writer - B1646"]
pub struct B1646_W<'a> {
    w: &'a mut W,
}
impl<'a> B1646_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1647` reader - B1647"]
pub struct B1647_R(crate::FieldReader<bool, bool>);
impl B1647_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1647_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1647_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1647` writer - B1647"]
pub struct B1647_W<'a> {
    w: &'a mut W,
}
impl<'a> B1647_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1648` reader - B1648"]
pub struct B1648_R(crate::FieldReader<bool, bool>);
impl B1648_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1648_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1648_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1648` writer - B1648"]
pub struct B1648_W<'a> {
    w: &'a mut W,
}
impl<'a> B1648_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1649` reader - B1649"]
pub struct B1649_R(crate::FieldReader<bool, bool>);
impl B1649_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1649_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1649_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1649` writer - B1649"]
pub struct B1649_W<'a> {
    w: &'a mut W,
}
impl<'a> B1649_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1650` reader - B1650"]
pub struct B1650_R(crate::FieldReader<bool, bool>);
impl B1650_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1650_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1650_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1650` writer - B1650"]
pub struct B1650_W<'a> {
    w: &'a mut W,
}
impl<'a> B1650_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1651` reader - B1651"]
pub struct B1651_R(crate::FieldReader<bool, bool>);
impl B1651_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1651_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1651_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1651` writer - B1651"]
pub struct B1651_W<'a> {
    w: &'a mut W,
}
impl<'a> B1651_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1652` reader - B1652"]
pub struct B1652_R(crate::FieldReader<bool, bool>);
impl B1652_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1652_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1652_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1652` writer - B1652"]
pub struct B1652_W<'a> {
    w: &'a mut W,
}
impl<'a> B1652_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1653` reader - B1653"]
pub struct B1653_R(crate::FieldReader<bool, bool>);
impl B1653_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1653_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1653_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1653` writer - B1653"]
pub struct B1653_W<'a> {
    w: &'a mut W,
}
impl<'a> B1653_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1654` reader - B1654"]
pub struct B1654_R(crate::FieldReader<bool, bool>);
impl B1654_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1654_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1654_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1654` writer - B1654"]
pub struct B1654_W<'a> {
    w: &'a mut W,
}
impl<'a> B1654_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1655` reader - B1655"]
pub struct B1655_R(crate::FieldReader<bool, bool>);
impl B1655_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1655_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1655_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1655` writer - B1655"]
pub struct B1655_W<'a> {
    w: &'a mut W,
}
impl<'a> B1655_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1656` reader - B1656"]
pub struct B1656_R(crate::FieldReader<bool, bool>);
impl B1656_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1656_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1656_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1656` writer - B1656"]
pub struct B1656_W<'a> {
    w: &'a mut W,
}
impl<'a> B1656_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1657` reader - B1657"]
pub struct B1657_R(crate::FieldReader<bool, bool>);
impl B1657_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1657_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1657_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1657` writer - B1657"]
pub struct B1657_W<'a> {
    w: &'a mut W,
}
impl<'a> B1657_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1658` reader - B1658"]
pub struct B1658_R(crate::FieldReader<bool, bool>);
impl B1658_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1658_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1658_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1658` writer - B1658"]
pub struct B1658_W<'a> {
    w: &'a mut W,
}
impl<'a> B1658_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1659` reader - B1659"]
pub struct B1659_R(crate::FieldReader<bool, bool>);
impl B1659_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1659_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1659_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1659` writer - B1659"]
pub struct B1659_W<'a> {
    w: &'a mut W,
}
impl<'a> B1659_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1660` reader - B1660"]
pub struct B1660_R(crate::FieldReader<bool, bool>);
impl B1660_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1660_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1660_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1660` writer - B1660"]
pub struct B1660_W<'a> {
    w: &'a mut W,
}
impl<'a> B1660_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1661` reader - B1661"]
pub struct B1661_R(crate::FieldReader<bool, bool>);
impl B1661_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1661_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1661_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1661` writer - B1661"]
pub struct B1661_W<'a> {
    w: &'a mut W,
}
impl<'a> B1661_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1662` reader - B1662"]
pub struct B1662_R(crate::FieldReader<bool, bool>);
impl B1662_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1662_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1662_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1662` writer - B1662"]
pub struct B1662_W<'a> {
    w: &'a mut W,
}
impl<'a> B1662_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1663` reader - B1663"]
pub struct B1663_R(crate::FieldReader<bool, bool>);
impl B1663_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1663_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1663_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1663` writer - B1663"]
pub struct B1663_W<'a> {
    w: &'a mut W,
}
impl<'a> B1663_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1632"]
    #[inline(always)]
    pub fn b1632(&self) -> B1632_R {
        B1632_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1633"]
    #[inline(always)]
    pub fn b1633(&self) -> B1633_R {
        B1633_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1634"]
    #[inline(always)]
    pub fn b1634(&self) -> B1634_R {
        B1634_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1635"]
    #[inline(always)]
    pub fn b1635(&self) -> B1635_R {
        B1635_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1636"]
    #[inline(always)]
    pub fn b1636(&self) -> B1636_R {
        B1636_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1637"]
    #[inline(always)]
    pub fn b1637(&self) -> B1637_R {
        B1637_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1638"]
    #[inline(always)]
    pub fn b1638(&self) -> B1638_R {
        B1638_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1639"]
    #[inline(always)]
    pub fn b1639(&self) -> B1639_R {
        B1639_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1640"]
    #[inline(always)]
    pub fn b1640(&self) -> B1640_R {
        B1640_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1641"]
    #[inline(always)]
    pub fn b1641(&self) -> B1641_R {
        B1641_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1642"]
    #[inline(always)]
    pub fn b1642(&self) -> B1642_R {
        B1642_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1643"]
    #[inline(always)]
    pub fn b1643(&self) -> B1643_R {
        B1643_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1644"]
    #[inline(always)]
    pub fn b1644(&self) -> B1644_R {
        B1644_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1645"]
    #[inline(always)]
    pub fn b1645(&self) -> B1645_R {
        B1645_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1646"]
    #[inline(always)]
    pub fn b1646(&self) -> B1646_R {
        B1646_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1647"]
    #[inline(always)]
    pub fn b1647(&self) -> B1647_R {
        B1647_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1648"]
    #[inline(always)]
    pub fn b1648(&self) -> B1648_R {
        B1648_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1649"]
    #[inline(always)]
    pub fn b1649(&self) -> B1649_R {
        B1649_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1650"]
    #[inline(always)]
    pub fn b1650(&self) -> B1650_R {
        B1650_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1651"]
    #[inline(always)]
    pub fn b1651(&self) -> B1651_R {
        B1651_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1652"]
    #[inline(always)]
    pub fn b1652(&self) -> B1652_R {
        B1652_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1653"]
    #[inline(always)]
    pub fn b1653(&self) -> B1653_R {
        B1653_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1654"]
    #[inline(always)]
    pub fn b1654(&self) -> B1654_R {
        B1654_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1655"]
    #[inline(always)]
    pub fn b1655(&self) -> B1655_R {
        B1655_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1656"]
    #[inline(always)]
    pub fn b1656(&self) -> B1656_R {
        B1656_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1657"]
    #[inline(always)]
    pub fn b1657(&self) -> B1657_R {
        B1657_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1658"]
    #[inline(always)]
    pub fn b1658(&self) -> B1658_R {
        B1658_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1659"]
    #[inline(always)]
    pub fn b1659(&self) -> B1659_R {
        B1659_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1660"]
    #[inline(always)]
    pub fn b1660(&self) -> B1660_R {
        B1660_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1661"]
    #[inline(always)]
    pub fn b1661(&self) -> B1661_R {
        B1661_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1662"]
    #[inline(always)]
    pub fn b1662(&self) -> B1662_R {
        B1662_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1663"]
    #[inline(always)]
    pub fn b1663(&self) -> B1663_R {
        B1663_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1632"]
    #[inline(always)]
    pub fn b1632(&mut self) -> B1632_W {
        B1632_W { w: self }
    }
    #[doc = "Bit 1 - B1633"]
    #[inline(always)]
    pub fn b1633(&mut self) -> B1633_W {
        B1633_W { w: self }
    }
    #[doc = "Bit 2 - B1634"]
    #[inline(always)]
    pub fn b1634(&mut self) -> B1634_W {
        B1634_W { w: self }
    }
    #[doc = "Bit 3 - B1635"]
    #[inline(always)]
    pub fn b1635(&mut self) -> B1635_W {
        B1635_W { w: self }
    }
    #[doc = "Bit 4 - B1636"]
    #[inline(always)]
    pub fn b1636(&mut self) -> B1636_W {
        B1636_W { w: self }
    }
    #[doc = "Bit 5 - B1637"]
    #[inline(always)]
    pub fn b1637(&mut self) -> B1637_W {
        B1637_W { w: self }
    }
    #[doc = "Bit 6 - B1638"]
    #[inline(always)]
    pub fn b1638(&mut self) -> B1638_W {
        B1638_W { w: self }
    }
    #[doc = "Bit 7 - B1639"]
    #[inline(always)]
    pub fn b1639(&mut self) -> B1639_W {
        B1639_W { w: self }
    }
    #[doc = "Bit 8 - B1640"]
    #[inline(always)]
    pub fn b1640(&mut self) -> B1640_W {
        B1640_W { w: self }
    }
    #[doc = "Bit 9 - B1641"]
    #[inline(always)]
    pub fn b1641(&mut self) -> B1641_W {
        B1641_W { w: self }
    }
    #[doc = "Bit 10 - B1642"]
    #[inline(always)]
    pub fn b1642(&mut self) -> B1642_W {
        B1642_W { w: self }
    }
    #[doc = "Bit 11 - B1643"]
    #[inline(always)]
    pub fn b1643(&mut self) -> B1643_W {
        B1643_W { w: self }
    }
    #[doc = "Bit 12 - B1644"]
    #[inline(always)]
    pub fn b1644(&mut self) -> B1644_W {
        B1644_W { w: self }
    }
    #[doc = "Bit 13 - B1645"]
    #[inline(always)]
    pub fn b1645(&mut self) -> B1645_W {
        B1645_W { w: self }
    }
    #[doc = "Bit 14 - B1646"]
    #[inline(always)]
    pub fn b1646(&mut self) -> B1646_W {
        B1646_W { w: self }
    }
    #[doc = "Bit 15 - B1647"]
    #[inline(always)]
    pub fn b1647(&mut self) -> B1647_W {
        B1647_W { w: self }
    }
    #[doc = "Bit 16 - B1648"]
    #[inline(always)]
    pub fn b1648(&mut self) -> B1648_W {
        B1648_W { w: self }
    }
    #[doc = "Bit 17 - B1649"]
    #[inline(always)]
    pub fn b1649(&mut self) -> B1649_W {
        B1649_W { w: self }
    }
    #[doc = "Bit 18 - B1650"]
    #[inline(always)]
    pub fn b1650(&mut self) -> B1650_W {
        B1650_W { w: self }
    }
    #[doc = "Bit 19 - B1651"]
    #[inline(always)]
    pub fn b1651(&mut self) -> B1651_W {
        B1651_W { w: self }
    }
    #[doc = "Bit 20 - B1652"]
    #[inline(always)]
    pub fn b1652(&mut self) -> B1652_W {
        B1652_W { w: self }
    }
    #[doc = "Bit 21 - B1653"]
    #[inline(always)]
    pub fn b1653(&mut self) -> B1653_W {
        B1653_W { w: self }
    }
    #[doc = "Bit 22 - B1654"]
    #[inline(always)]
    pub fn b1654(&mut self) -> B1654_W {
        B1654_W { w: self }
    }
    #[doc = "Bit 23 - B1655"]
    #[inline(always)]
    pub fn b1655(&mut self) -> B1655_W {
        B1655_W { w: self }
    }
    #[doc = "Bit 24 - B1656"]
    #[inline(always)]
    pub fn b1656(&mut self) -> B1656_W {
        B1656_W { w: self }
    }
    #[doc = "Bit 25 - B1657"]
    #[inline(always)]
    pub fn b1657(&mut self) -> B1657_W {
        B1657_W { w: self }
    }
    #[doc = "Bit 26 - B1658"]
    #[inline(always)]
    pub fn b1658(&mut self) -> B1658_W {
        B1658_W { w: self }
    }
    #[doc = "Bit 27 - B1659"]
    #[inline(always)]
    pub fn b1659(&mut self) -> B1659_W {
        B1659_W { w: self }
    }
    #[doc = "Bit 28 - B1660"]
    #[inline(always)]
    pub fn b1660(&mut self) -> B1660_W {
        B1660_W { w: self }
    }
    #[doc = "Bit 29 - B1661"]
    #[inline(always)]
    pub fn b1661(&mut self) -> B1661_W {
        B1661_W { w: self }
    }
    #[doc = "Bit 30 - B1662"]
    #[inline(always)]
    pub fn b1662(&mut self) -> B1662_W {
        B1662_W { w: self }
    }
    #[doc = "Bit 31 - B1663"]
    #[inline(always)]
    pub fn b1663(&mut self) -> B1663_W {
        B1663_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr51](index.html) module"]
pub struct MPCBB2_VCTR51_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR51_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr51::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR51_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr51::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR51_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR51 to value 0"]
impl crate::Resettable for MPCBB2_VCTR51_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
