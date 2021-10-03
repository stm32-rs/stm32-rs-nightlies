#[doc = "Register `MPCBB2_VCTR57` reader"]
pub struct R(crate::R<MPCBB2_VCTR57_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR57_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR57_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR57_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR57` writer"]
pub struct W(crate::W<MPCBB2_VCTR57_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR57_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR57_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR57_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1824` reader - B1824"]
pub struct B1824_R(crate::FieldReader<bool, bool>);
impl B1824_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1824_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1824_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1824` writer - B1824"]
pub struct B1824_W<'a> {
    w: &'a mut W,
}
impl<'a> B1824_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1825` reader - B1825"]
pub struct B1825_R(crate::FieldReader<bool, bool>);
impl B1825_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1825_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1825_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1825` writer - B1825"]
pub struct B1825_W<'a> {
    w: &'a mut W,
}
impl<'a> B1825_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1826` reader - B1826"]
pub struct B1826_R(crate::FieldReader<bool, bool>);
impl B1826_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1826_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1826_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1826` writer - B1826"]
pub struct B1826_W<'a> {
    w: &'a mut W,
}
impl<'a> B1826_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1827` reader - B1827"]
pub struct B1827_R(crate::FieldReader<bool, bool>);
impl B1827_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1827_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1827_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1827` writer - B1827"]
pub struct B1827_W<'a> {
    w: &'a mut W,
}
impl<'a> B1827_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1828` reader - B1828"]
pub struct B1828_R(crate::FieldReader<bool, bool>);
impl B1828_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1828_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1828_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1828` writer - B1828"]
pub struct B1828_W<'a> {
    w: &'a mut W,
}
impl<'a> B1828_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1829` reader - B1829"]
pub struct B1829_R(crate::FieldReader<bool, bool>);
impl B1829_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1829_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1829_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1829` writer - B1829"]
pub struct B1829_W<'a> {
    w: &'a mut W,
}
impl<'a> B1829_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1830` reader - B1830"]
pub struct B1830_R(crate::FieldReader<bool, bool>);
impl B1830_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1830_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1830_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1830` writer - B1830"]
pub struct B1830_W<'a> {
    w: &'a mut W,
}
impl<'a> B1830_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1831` reader - B1831"]
pub struct B1831_R(crate::FieldReader<bool, bool>);
impl B1831_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1831_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1831_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1831` writer - B1831"]
pub struct B1831_W<'a> {
    w: &'a mut W,
}
impl<'a> B1831_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1832` reader - B1832"]
pub struct B1832_R(crate::FieldReader<bool, bool>);
impl B1832_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1832_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1832_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1832` writer - B1832"]
pub struct B1832_W<'a> {
    w: &'a mut W,
}
impl<'a> B1832_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1833` reader - B1833"]
pub struct B1833_R(crate::FieldReader<bool, bool>);
impl B1833_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1833_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1833_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1833` writer - B1833"]
pub struct B1833_W<'a> {
    w: &'a mut W,
}
impl<'a> B1833_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1834` reader - B1834"]
pub struct B1834_R(crate::FieldReader<bool, bool>);
impl B1834_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1834_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1834_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1834` writer - B1834"]
pub struct B1834_W<'a> {
    w: &'a mut W,
}
impl<'a> B1834_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1835` reader - B1835"]
pub struct B1835_R(crate::FieldReader<bool, bool>);
impl B1835_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1835_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1835_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1835` writer - B1835"]
pub struct B1835_W<'a> {
    w: &'a mut W,
}
impl<'a> B1835_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1836` reader - B1836"]
pub struct B1836_R(crate::FieldReader<bool, bool>);
impl B1836_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1836_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1836_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1836` writer - B1836"]
pub struct B1836_W<'a> {
    w: &'a mut W,
}
impl<'a> B1836_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1837` reader - B1837"]
pub struct B1837_R(crate::FieldReader<bool, bool>);
impl B1837_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1837_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1837_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1837` writer - B1837"]
pub struct B1837_W<'a> {
    w: &'a mut W,
}
impl<'a> B1837_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1838` reader - B1838"]
pub struct B1838_R(crate::FieldReader<bool, bool>);
impl B1838_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1838_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1838_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1838` writer - B1838"]
pub struct B1838_W<'a> {
    w: &'a mut W,
}
impl<'a> B1838_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1839` reader - B1839"]
pub struct B1839_R(crate::FieldReader<bool, bool>);
impl B1839_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1839_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1839_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1839` writer - B1839"]
pub struct B1839_W<'a> {
    w: &'a mut W,
}
impl<'a> B1839_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1840` reader - B1840"]
pub struct B1840_R(crate::FieldReader<bool, bool>);
impl B1840_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1840_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1840_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1840` writer - B1840"]
pub struct B1840_W<'a> {
    w: &'a mut W,
}
impl<'a> B1840_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1841` reader - B1841"]
pub struct B1841_R(crate::FieldReader<bool, bool>);
impl B1841_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1841_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1841_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1841` writer - B1841"]
pub struct B1841_W<'a> {
    w: &'a mut W,
}
impl<'a> B1841_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1842` reader - B1842"]
pub struct B1842_R(crate::FieldReader<bool, bool>);
impl B1842_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1842_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1842_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1842` writer - B1842"]
pub struct B1842_W<'a> {
    w: &'a mut W,
}
impl<'a> B1842_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1843` reader - B1843"]
pub struct B1843_R(crate::FieldReader<bool, bool>);
impl B1843_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1843_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1843_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1843` writer - B1843"]
pub struct B1843_W<'a> {
    w: &'a mut W,
}
impl<'a> B1843_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1844` reader - B1844"]
pub struct B1844_R(crate::FieldReader<bool, bool>);
impl B1844_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1844_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1844_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1844` writer - B1844"]
pub struct B1844_W<'a> {
    w: &'a mut W,
}
impl<'a> B1844_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1845` reader - B1845"]
pub struct B1845_R(crate::FieldReader<bool, bool>);
impl B1845_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1845_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1845_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1845` writer - B1845"]
pub struct B1845_W<'a> {
    w: &'a mut W,
}
impl<'a> B1845_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1846` reader - B1846"]
pub struct B1846_R(crate::FieldReader<bool, bool>);
impl B1846_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1846_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1846_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1846` writer - B1846"]
pub struct B1846_W<'a> {
    w: &'a mut W,
}
impl<'a> B1846_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1847` reader - B1847"]
pub struct B1847_R(crate::FieldReader<bool, bool>);
impl B1847_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1847_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1847_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1847` writer - B1847"]
pub struct B1847_W<'a> {
    w: &'a mut W,
}
impl<'a> B1847_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1848` reader - B1848"]
pub struct B1848_R(crate::FieldReader<bool, bool>);
impl B1848_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1848_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1848_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1848` writer - B1848"]
pub struct B1848_W<'a> {
    w: &'a mut W,
}
impl<'a> B1848_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1849` reader - B1849"]
pub struct B1849_R(crate::FieldReader<bool, bool>);
impl B1849_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1849_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1849_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1849` writer - B1849"]
pub struct B1849_W<'a> {
    w: &'a mut W,
}
impl<'a> B1849_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1850` reader - B1850"]
pub struct B1850_R(crate::FieldReader<bool, bool>);
impl B1850_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1850_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1850_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1850` writer - B1850"]
pub struct B1850_W<'a> {
    w: &'a mut W,
}
impl<'a> B1850_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1851` reader - B1851"]
pub struct B1851_R(crate::FieldReader<bool, bool>);
impl B1851_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1851_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1851_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1851` writer - B1851"]
pub struct B1851_W<'a> {
    w: &'a mut W,
}
impl<'a> B1851_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1852` reader - B1852"]
pub struct B1852_R(crate::FieldReader<bool, bool>);
impl B1852_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1852_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1852_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1852` writer - B1852"]
pub struct B1852_W<'a> {
    w: &'a mut W,
}
impl<'a> B1852_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1853` reader - B1853"]
pub struct B1853_R(crate::FieldReader<bool, bool>);
impl B1853_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1853_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1853_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1853` writer - B1853"]
pub struct B1853_W<'a> {
    w: &'a mut W,
}
impl<'a> B1853_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1854` reader - B1854"]
pub struct B1854_R(crate::FieldReader<bool, bool>);
impl B1854_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1854_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1854_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1854` writer - B1854"]
pub struct B1854_W<'a> {
    w: &'a mut W,
}
impl<'a> B1854_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1855` reader - B1855"]
pub struct B1855_R(crate::FieldReader<bool, bool>);
impl B1855_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1855_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1855_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1855` writer - B1855"]
pub struct B1855_W<'a> {
    w: &'a mut W,
}
impl<'a> B1855_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1824"]
    #[inline(always)]
    pub fn b1824(&self) -> B1824_R {
        B1824_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1825"]
    #[inline(always)]
    pub fn b1825(&self) -> B1825_R {
        B1825_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1826"]
    #[inline(always)]
    pub fn b1826(&self) -> B1826_R {
        B1826_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1827"]
    #[inline(always)]
    pub fn b1827(&self) -> B1827_R {
        B1827_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1828"]
    #[inline(always)]
    pub fn b1828(&self) -> B1828_R {
        B1828_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1829"]
    #[inline(always)]
    pub fn b1829(&self) -> B1829_R {
        B1829_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1830"]
    #[inline(always)]
    pub fn b1830(&self) -> B1830_R {
        B1830_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1831"]
    #[inline(always)]
    pub fn b1831(&self) -> B1831_R {
        B1831_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1832"]
    #[inline(always)]
    pub fn b1832(&self) -> B1832_R {
        B1832_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1833"]
    #[inline(always)]
    pub fn b1833(&self) -> B1833_R {
        B1833_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1834"]
    #[inline(always)]
    pub fn b1834(&self) -> B1834_R {
        B1834_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1835"]
    #[inline(always)]
    pub fn b1835(&self) -> B1835_R {
        B1835_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1836"]
    #[inline(always)]
    pub fn b1836(&self) -> B1836_R {
        B1836_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1837"]
    #[inline(always)]
    pub fn b1837(&self) -> B1837_R {
        B1837_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1838"]
    #[inline(always)]
    pub fn b1838(&self) -> B1838_R {
        B1838_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1839"]
    #[inline(always)]
    pub fn b1839(&self) -> B1839_R {
        B1839_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1840"]
    #[inline(always)]
    pub fn b1840(&self) -> B1840_R {
        B1840_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1841"]
    #[inline(always)]
    pub fn b1841(&self) -> B1841_R {
        B1841_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1842"]
    #[inline(always)]
    pub fn b1842(&self) -> B1842_R {
        B1842_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1843"]
    #[inline(always)]
    pub fn b1843(&self) -> B1843_R {
        B1843_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1844"]
    #[inline(always)]
    pub fn b1844(&self) -> B1844_R {
        B1844_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1845"]
    #[inline(always)]
    pub fn b1845(&self) -> B1845_R {
        B1845_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1846"]
    #[inline(always)]
    pub fn b1846(&self) -> B1846_R {
        B1846_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1847"]
    #[inline(always)]
    pub fn b1847(&self) -> B1847_R {
        B1847_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1848"]
    #[inline(always)]
    pub fn b1848(&self) -> B1848_R {
        B1848_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1849"]
    #[inline(always)]
    pub fn b1849(&self) -> B1849_R {
        B1849_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1850"]
    #[inline(always)]
    pub fn b1850(&self) -> B1850_R {
        B1850_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1851"]
    #[inline(always)]
    pub fn b1851(&self) -> B1851_R {
        B1851_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1852"]
    #[inline(always)]
    pub fn b1852(&self) -> B1852_R {
        B1852_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1853"]
    #[inline(always)]
    pub fn b1853(&self) -> B1853_R {
        B1853_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1854"]
    #[inline(always)]
    pub fn b1854(&self) -> B1854_R {
        B1854_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1855"]
    #[inline(always)]
    pub fn b1855(&self) -> B1855_R {
        B1855_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1824"]
    #[inline(always)]
    pub fn b1824(&mut self) -> B1824_W {
        B1824_W { w: self }
    }
    #[doc = "Bit 1 - B1825"]
    #[inline(always)]
    pub fn b1825(&mut self) -> B1825_W {
        B1825_W { w: self }
    }
    #[doc = "Bit 2 - B1826"]
    #[inline(always)]
    pub fn b1826(&mut self) -> B1826_W {
        B1826_W { w: self }
    }
    #[doc = "Bit 3 - B1827"]
    #[inline(always)]
    pub fn b1827(&mut self) -> B1827_W {
        B1827_W { w: self }
    }
    #[doc = "Bit 4 - B1828"]
    #[inline(always)]
    pub fn b1828(&mut self) -> B1828_W {
        B1828_W { w: self }
    }
    #[doc = "Bit 5 - B1829"]
    #[inline(always)]
    pub fn b1829(&mut self) -> B1829_W {
        B1829_W { w: self }
    }
    #[doc = "Bit 6 - B1830"]
    #[inline(always)]
    pub fn b1830(&mut self) -> B1830_W {
        B1830_W { w: self }
    }
    #[doc = "Bit 7 - B1831"]
    #[inline(always)]
    pub fn b1831(&mut self) -> B1831_W {
        B1831_W { w: self }
    }
    #[doc = "Bit 8 - B1832"]
    #[inline(always)]
    pub fn b1832(&mut self) -> B1832_W {
        B1832_W { w: self }
    }
    #[doc = "Bit 9 - B1833"]
    #[inline(always)]
    pub fn b1833(&mut self) -> B1833_W {
        B1833_W { w: self }
    }
    #[doc = "Bit 10 - B1834"]
    #[inline(always)]
    pub fn b1834(&mut self) -> B1834_W {
        B1834_W { w: self }
    }
    #[doc = "Bit 11 - B1835"]
    #[inline(always)]
    pub fn b1835(&mut self) -> B1835_W {
        B1835_W { w: self }
    }
    #[doc = "Bit 12 - B1836"]
    #[inline(always)]
    pub fn b1836(&mut self) -> B1836_W {
        B1836_W { w: self }
    }
    #[doc = "Bit 13 - B1837"]
    #[inline(always)]
    pub fn b1837(&mut self) -> B1837_W {
        B1837_W { w: self }
    }
    #[doc = "Bit 14 - B1838"]
    #[inline(always)]
    pub fn b1838(&mut self) -> B1838_W {
        B1838_W { w: self }
    }
    #[doc = "Bit 15 - B1839"]
    #[inline(always)]
    pub fn b1839(&mut self) -> B1839_W {
        B1839_W { w: self }
    }
    #[doc = "Bit 16 - B1840"]
    #[inline(always)]
    pub fn b1840(&mut self) -> B1840_W {
        B1840_W { w: self }
    }
    #[doc = "Bit 17 - B1841"]
    #[inline(always)]
    pub fn b1841(&mut self) -> B1841_W {
        B1841_W { w: self }
    }
    #[doc = "Bit 18 - B1842"]
    #[inline(always)]
    pub fn b1842(&mut self) -> B1842_W {
        B1842_W { w: self }
    }
    #[doc = "Bit 19 - B1843"]
    #[inline(always)]
    pub fn b1843(&mut self) -> B1843_W {
        B1843_W { w: self }
    }
    #[doc = "Bit 20 - B1844"]
    #[inline(always)]
    pub fn b1844(&mut self) -> B1844_W {
        B1844_W { w: self }
    }
    #[doc = "Bit 21 - B1845"]
    #[inline(always)]
    pub fn b1845(&mut self) -> B1845_W {
        B1845_W { w: self }
    }
    #[doc = "Bit 22 - B1846"]
    #[inline(always)]
    pub fn b1846(&mut self) -> B1846_W {
        B1846_W { w: self }
    }
    #[doc = "Bit 23 - B1847"]
    #[inline(always)]
    pub fn b1847(&mut self) -> B1847_W {
        B1847_W { w: self }
    }
    #[doc = "Bit 24 - B1848"]
    #[inline(always)]
    pub fn b1848(&mut self) -> B1848_W {
        B1848_W { w: self }
    }
    #[doc = "Bit 25 - B1849"]
    #[inline(always)]
    pub fn b1849(&mut self) -> B1849_W {
        B1849_W { w: self }
    }
    #[doc = "Bit 26 - B1850"]
    #[inline(always)]
    pub fn b1850(&mut self) -> B1850_W {
        B1850_W { w: self }
    }
    #[doc = "Bit 27 - B1851"]
    #[inline(always)]
    pub fn b1851(&mut self) -> B1851_W {
        B1851_W { w: self }
    }
    #[doc = "Bit 28 - B1852"]
    #[inline(always)]
    pub fn b1852(&mut self) -> B1852_W {
        B1852_W { w: self }
    }
    #[doc = "Bit 29 - B1853"]
    #[inline(always)]
    pub fn b1853(&mut self) -> B1853_W {
        B1853_W { w: self }
    }
    #[doc = "Bit 30 - B1854"]
    #[inline(always)]
    pub fn b1854(&mut self) -> B1854_W {
        B1854_W { w: self }
    }
    #[doc = "Bit 31 - B1855"]
    #[inline(always)]
    pub fn b1855(&mut self) -> B1855_W {
        B1855_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr57](index.html) module"]
pub struct MPCBB2_VCTR57_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR57_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr57::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR57_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr57::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR57_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR57 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB2_VCTR57_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
