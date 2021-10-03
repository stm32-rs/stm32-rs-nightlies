#[doc = "Register `MPCBB1_VCTR54` reader"]
pub struct R(crate::R<MPCBB1_VCTR54_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR54_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR54_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR54_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR54` writer"]
pub struct W(crate::W<MPCBB1_VCTR54_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR54_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR54_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR54_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1728` reader - B1728"]
pub struct B1728_R(crate::FieldReader<bool, bool>);
impl B1728_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1728_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1728_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1728` writer - B1728"]
pub struct B1728_W<'a> {
    w: &'a mut W,
}
impl<'a> B1728_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1729` reader - B1729"]
pub struct B1729_R(crate::FieldReader<bool, bool>);
impl B1729_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1729_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1729_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1729` writer - B1729"]
pub struct B1729_W<'a> {
    w: &'a mut W,
}
impl<'a> B1729_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1730` reader - B1730"]
pub struct B1730_R(crate::FieldReader<bool, bool>);
impl B1730_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1730_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1730_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1730` writer - B1730"]
pub struct B1730_W<'a> {
    w: &'a mut W,
}
impl<'a> B1730_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1731` reader - B1731"]
pub struct B1731_R(crate::FieldReader<bool, bool>);
impl B1731_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1731_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1731_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1731` writer - B1731"]
pub struct B1731_W<'a> {
    w: &'a mut W,
}
impl<'a> B1731_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1732` reader - B1732"]
pub struct B1732_R(crate::FieldReader<bool, bool>);
impl B1732_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1732_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1732_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1732` writer - B1732"]
pub struct B1732_W<'a> {
    w: &'a mut W,
}
impl<'a> B1732_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1733` reader - B1733"]
pub struct B1733_R(crate::FieldReader<bool, bool>);
impl B1733_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1733_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1733_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1733` writer - B1733"]
pub struct B1733_W<'a> {
    w: &'a mut W,
}
impl<'a> B1733_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1734` reader - B1734"]
pub struct B1734_R(crate::FieldReader<bool, bool>);
impl B1734_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1734_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1734_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1734` writer - B1734"]
pub struct B1734_W<'a> {
    w: &'a mut W,
}
impl<'a> B1734_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1735` reader - B1735"]
pub struct B1735_R(crate::FieldReader<bool, bool>);
impl B1735_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1735_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1735_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1735` writer - B1735"]
pub struct B1735_W<'a> {
    w: &'a mut W,
}
impl<'a> B1735_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1736` reader - B1736"]
pub struct B1736_R(crate::FieldReader<bool, bool>);
impl B1736_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1736_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1736_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1736` writer - B1736"]
pub struct B1736_W<'a> {
    w: &'a mut W,
}
impl<'a> B1736_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1737` reader - B1737"]
pub struct B1737_R(crate::FieldReader<bool, bool>);
impl B1737_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1737_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1737_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1737` writer - B1737"]
pub struct B1737_W<'a> {
    w: &'a mut W,
}
impl<'a> B1737_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1738` reader - B1738"]
pub struct B1738_R(crate::FieldReader<bool, bool>);
impl B1738_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1738_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1738_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1738` writer - B1738"]
pub struct B1738_W<'a> {
    w: &'a mut W,
}
impl<'a> B1738_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1739` reader - B1739"]
pub struct B1739_R(crate::FieldReader<bool, bool>);
impl B1739_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1739_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1739_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1739` writer - B1739"]
pub struct B1739_W<'a> {
    w: &'a mut W,
}
impl<'a> B1739_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1740` reader - B1740"]
pub struct B1740_R(crate::FieldReader<bool, bool>);
impl B1740_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1740_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1740_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1740` writer - B1740"]
pub struct B1740_W<'a> {
    w: &'a mut W,
}
impl<'a> B1740_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1741` reader - B1741"]
pub struct B1741_R(crate::FieldReader<bool, bool>);
impl B1741_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1741_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1741_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1741` writer - B1741"]
pub struct B1741_W<'a> {
    w: &'a mut W,
}
impl<'a> B1741_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1742` reader - B1742"]
pub struct B1742_R(crate::FieldReader<bool, bool>);
impl B1742_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1742_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1742_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1742` writer - B1742"]
pub struct B1742_W<'a> {
    w: &'a mut W,
}
impl<'a> B1742_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1743` reader - B1743"]
pub struct B1743_R(crate::FieldReader<bool, bool>);
impl B1743_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1743_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1743_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1743` writer - B1743"]
pub struct B1743_W<'a> {
    w: &'a mut W,
}
impl<'a> B1743_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1744` reader - B1744"]
pub struct B1744_R(crate::FieldReader<bool, bool>);
impl B1744_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1744_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1744_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1744` writer - B1744"]
pub struct B1744_W<'a> {
    w: &'a mut W,
}
impl<'a> B1744_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1745` reader - B1745"]
pub struct B1745_R(crate::FieldReader<bool, bool>);
impl B1745_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1745_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1745_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1745` writer - B1745"]
pub struct B1745_W<'a> {
    w: &'a mut W,
}
impl<'a> B1745_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1746` reader - B1746"]
pub struct B1746_R(crate::FieldReader<bool, bool>);
impl B1746_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1746_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1746_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1746` writer - B1746"]
pub struct B1746_W<'a> {
    w: &'a mut W,
}
impl<'a> B1746_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1747` reader - B1747"]
pub struct B1747_R(crate::FieldReader<bool, bool>);
impl B1747_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1747_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1747_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1747` writer - B1747"]
pub struct B1747_W<'a> {
    w: &'a mut W,
}
impl<'a> B1747_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1748` reader - B1748"]
pub struct B1748_R(crate::FieldReader<bool, bool>);
impl B1748_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1748_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1748_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1748` writer - B1748"]
pub struct B1748_W<'a> {
    w: &'a mut W,
}
impl<'a> B1748_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1749` reader - B1749"]
pub struct B1749_R(crate::FieldReader<bool, bool>);
impl B1749_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1749_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1749_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1749` writer - B1749"]
pub struct B1749_W<'a> {
    w: &'a mut W,
}
impl<'a> B1749_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1750` reader - B1750"]
pub struct B1750_R(crate::FieldReader<bool, bool>);
impl B1750_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1750_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1750_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1750` writer - B1750"]
pub struct B1750_W<'a> {
    w: &'a mut W,
}
impl<'a> B1750_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1751` reader - B1751"]
pub struct B1751_R(crate::FieldReader<bool, bool>);
impl B1751_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1751_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1751_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1751` writer - B1751"]
pub struct B1751_W<'a> {
    w: &'a mut W,
}
impl<'a> B1751_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1752` reader - B1752"]
pub struct B1752_R(crate::FieldReader<bool, bool>);
impl B1752_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1752_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1752_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1752` writer - B1752"]
pub struct B1752_W<'a> {
    w: &'a mut W,
}
impl<'a> B1752_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1753` reader - B1753"]
pub struct B1753_R(crate::FieldReader<bool, bool>);
impl B1753_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1753_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1753_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1753` writer - B1753"]
pub struct B1753_W<'a> {
    w: &'a mut W,
}
impl<'a> B1753_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1754` reader - B1754"]
pub struct B1754_R(crate::FieldReader<bool, bool>);
impl B1754_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1754_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1754_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1754` writer - B1754"]
pub struct B1754_W<'a> {
    w: &'a mut W,
}
impl<'a> B1754_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1755` reader - B1755"]
pub struct B1755_R(crate::FieldReader<bool, bool>);
impl B1755_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1755_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1755_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1755` writer - B1755"]
pub struct B1755_W<'a> {
    w: &'a mut W,
}
impl<'a> B1755_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1756` reader - B1756"]
pub struct B1756_R(crate::FieldReader<bool, bool>);
impl B1756_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1756_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1756_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1756` writer - B1756"]
pub struct B1756_W<'a> {
    w: &'a mut W,
}
impl<'a> B1756_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1757` reader - B1757"]
pub struct B1757_R(crate::FieldReader<bool, bool>);
impl B1757_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1757_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1757_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1757` writer - B1757"]
pub struct B1757_W<'a> {
    w: &'a mut W,
}
impl<'a> B1757_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1758` reader - B1758"]
pub struct B1758_R(crate::FieldReader<bool, bool>);
impl B1758_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1758_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1758_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1758` writer - B1758"]
pub struct B1758_W<'a> {
    w: &'a mut W,
}
impl<'a> B1758_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1759` reader - B1759"]
pub struct B1759_R(crate::FieldReader<bool, bool>);
impl B1759_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1759_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1759_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1759` writer - B1759"]
pub struct B1759_W<'a> {
    w: &'a mut W,
}
impl<'a> B1759_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1728"]
    #[inline(always)]
    pub fn b1728(&self) -> B1728_R {
        B1728_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1729"]
    #[inline(always)]
    pub fn b1729(&self) -> B1729_R {
        B1729_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1730"]
    #[inline(always)]
    pub fn b1730(&self) -> B1730_R {
        B1730_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1731"]
    #[inline(always)]
    pub fn b1731(&self) -> B1731_R {
        B1731_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1732"]
    #[inline(always)]
    pub fn b1732(&self) -> B1732_R {
        B1732_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1733"]
    #[inline(always)]
    pub fn b1733(&self) -> B1733_R {
        B1733_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1734"]
    #[inline(always)]
    pub fn b1734(&self) -> B1734_R {
        B1734_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1735"]
    #[inline(always)]
    pub fn b1735(&self) -> B1735_R {
        B1735_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1736"]
    #[inline(always)]
    pub fn b1736(&self) -> B1736_R {
        B1736_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1737"]
    #[inline(always)]
    pub fn b1737(&self) -> B1737_R {
        B1737_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1738"]
    #[inline(always)]
    pub fn b1738(&self) -> B1738_R {
        B1738_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1739"]
    #[inline(always)]
    pub fn b1739(&self) -> B1739_R {
        B1739_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1740"]
    #[inline(always)]
    pub fn b1740(&self) -> B1740_R {
        B1740_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1741"]
    #[inline(always)]
    pub fn b1741(&self) -> B1741_R {
        B1741_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1742"]
    #[inline(always)]
    pub fn b1742(&self) -> B1742_R {
        B1742_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1743"]
    #[inline(always)]
    pub fn b1743(&self) -> B1743_R {
        B1743_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1744"]
    #[inline(always)]
    pub fn b1744(&self) -> B1744_R {
        B1744_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1745"]
    #[inline(always)]
    pub fn b1745(&self) -> B1745_R {
        B1745_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1746"]
    #[inline(always)]
    pub fn b1746(&self) -> B1746_R {
        B1746_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1747"]
    #[inline(always)]
    pub fn b1747(&self) -> B1747_R {
        B1747_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1748"]
    #[inline(always)]
    pub fn b1748(&self) -> B1748_R {
        B1748_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1749"]
    #[inline(always)]
    pub fn b1749(&self) -> B1749_R {
        B1749_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1750"]
    #[inline(always)]
    pub fn b1750(&self) -> B1750_R {
        B1750_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1751"]
    #[inline(always)]
    pub fn b1751(&self) -> B1751_R {
        B1751_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1752"]
    #[inline(always)]
    pub fn b1752(&self) -> B1752_R {
        B1752_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1753"]
    #[inline(always)]
    pub fn b1753(&self) -> B1753_R {
        B1753_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1754"]
    #[inline(always)]
    pub fn b1754(&self) -> B1754_R {
        B1754_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1755"]
    #[inline(always)]
    pub fn b1755(&self) -> B1755_R {
        B1755_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1756"]
    #[inline(always)]
    pub fn b1756(&self) -> B1756_R {
        B1756_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1757"]
    #[inline(always)]
    pub fn b1757(&self) -> B1757_R {
        B1757_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1758"]
    #[inline(always)]
    pub fn b1758(&self) -> B1758_R {
        B1758_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1759"]
    #[inline(always)]
    pub fn b1759(&self) -> B1759_R {
        B1759_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1728"]
    #[inline(always)]
    pub fn b1728(&mut self) -> B1728_W {
        B1728_W { w: self }
    }
    #[doc = "Bit 1 - B1729"]
    #[inline(always)]
    pub fn b1729(&mut self) -> B1729_W {
        B1729_W { w: self }
    }
    #[doc = "Bit 2 - B1730"]
    #[inline(always)]
    pub fn b1730(&mut self) -> B1730_W {
        B1730_W { w: self }
    }
    #[doc = "Bit 3 - B1731"]
    #[inline(always)]
    pub fn b1731(&mut self) -> B1731_W {
        B1731_W { w: self }
    }
    #[doc = "Bit 4 - B1732"]
    #[inline(always)]
    pub fn b1732(&mut self) -> B1732_W {
        B1732_W { w: self }
    }
    #[doc = "Bit 5 - B1733"]
    #[inline(always)]
    pub fn b1733(&mut self) -> B1733_W {
        B1733_W { w: self }
    }
    #[doc = "Bit 6 - B1734"]
    #[inline(always)]
    pub fn b1734(&mut self) -> B1734_W {
        B1734_W { w: self }
    }
    #[doc = "Bit 7 - B1735"]
    #[inline(always)]
    pub fn b1735(&mut self) -> B1735_W {
        B1735_W { w: self }
    }
    #[doc = "Bit 8 - B1736"]
    #[inline(always)]
    pub fn b1736(&mut self) -> B1736_W {
        B1736_W { w: self }
    }
    #[doc = "Bit 9 - B1737"]
    #[inline(always)]
    pub fn b1737(&mut self) -> B1737_W {
        B1737_W { w: self }
    }
    #[doc = "Bit 10 - B1738"]
    #[inline(always)]
    pub fn b1738(&mut self) -> B1738_W {
        B1738_W { w: self }
    }
    #[doc = "Bit 11 - B1739"]
    #[inline(always)]
    pub fn b1739(&mut self) -> B1739_W {
        B1739_W { w: self }
    }
    #[doc = "Bit 12 - B1740"]
    #[inline(always)]
    pub fn b1740(&mut self) -> B1740_W {
        B1740_W { w: self }
    }
    #[doc = "Bit 13 - B1741"]
    #[inline(always)]
    pub fn b1741(&mut self) -> B1741_W {
        B1741_W { w: self }
    }
    #[doc = "Bit 14 - B1742"]
    #[inline(always)]
    pub fn b1742(&mut self) -> B1742_W {
        B1742_W { w: self }
    }
    #[doc = "Bit 15 - B1743"]
    #[inline(always)]
    pub fn b1743(&mut self) -> B1743_W {
        B1743_W { w: self }
    }
    #[doc = "Bit 16 - B1744"]
    #[inline(always)]
    pub fn b1744(&mut self) -> B1744_W {
        B1744_W { w: self }
    }
    #[doc = "Bit 17 - B1745"]
    #[inline(always)]
    pub fn b1745(&mut self) -> B1745_W {
        B1745_W { w: self }
    }
    #[doc = "Bit 18 - B1746"]
    #[inline(always)]
    pub fn b1746(&mut self) -> B1746_W {
        B1746_W { w: self }
    }
    #[doc = "Bit 19 - B1747"]
    #[inline(always)]
    pub fn b1747(&mut self) -> B1747_W {
        B1747_W { w: self }
    }
    #[doc = "Bit 20 - B1748"]
    #[inline(always)]
    pub fn b1748(&mut self) -> B1748_W {
        B1748_W { w: self }
    }
    #[doc = "Bit 21 - B1749"]
    #[inline(always)]
    pub fn b1749(&mut self) -> B1749_W {
        B1749_W { w: self }
    }
    #[doc = "Bit 22 - B1750"]
    #[inline(always)]
    pub fn b1750(&mut self) -> B1750_W {
        B1750_W { w: self }
    }
    #[doc = "Bit 23 - B1751"]
    #[inline(always)]
    pub fn b1751(&mut self) -> B1751_W {
        B1751_W { w: self }
    }
    #[doc = "Bit 24 - B1752"]
    #[inline(always)]
    pub fn b1752(&mut self) -> B1752_W {
        B1752_W { w: self }
    }
    #[doc = "Bit 25 - B1753"]
    #[inline(always)]
    pub fn b1753(&mut self) -> B1753_W {
        B1753_W { w: self }
    }
    #[doc = "Bit 26 - B1754"]
    #[inline(always)]
    pub fn b1754(&mut self) -> B1754_W {
        B1754_W { w: self }
    }
    #[doc = "Bit 27 - B1755"]
    #[inline(always)]
    pub fn b1755(&mut self) -> B1755_W {
        B1755_W { w: self }
    }
    #[doc = "Bit 28 - B1756"]
    #[inline(always)]
    pub fn b1756(&mut self) -> B1756_W {
        B1756_W { w: self }
    }
    #[doc = "Bit 29 - B1757"]
    #[inline(always)]
    pub fn b1757(&mut self) -> B1757_W {
        B1757_W { w: self }
    }
    #[doc = "Bit 30 - B1758"]
    #[inline(always)]
    pub fn b1758(&mut self) -> B1758_W {
        B1758_W { w: self }
    }
    #[doc = "Bit 31 - B1759"]
    #[inline(always)]
    pub fn b1759(&mut self) -> B1759_W {
        B1759_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr54](index.html) module"]
pub struct MPCBB1_VCTR54_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR54_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr54::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR54_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr54::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR54_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR54 to value 0"]
impl crate::Resettable for MPCBB1_VCTR54_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
