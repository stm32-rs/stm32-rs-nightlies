#[doc = "Register `MPCBB1_VCTR6` reader"]
pub struct R(crate::R<MPCBB1_VCTR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR6` writer"]
pub struct W(crate::W<MPCBB1_VCTR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR6_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B192` reader - B192"]
pub struct B192_R(crate::FieldReader<bool, bool>);
impl B192_R {
    pub(crate) fn new(bits: bool) -> Self {
        B192_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B192_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B192` writer - B192"]
pub struct B192_W<'a> {
    w: &'a mut W,
}
impl<'a> B192_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B193` reader - B193"]
pub struct B193_R(crate::FieldReader<bool, bool>);
impl B193_R {
    pub(crate) fn new(bits: bool) -> Self {
        B193_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B193_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B193` writer - B193"]
pub struct B193_W<'a> {
    w: &'a mut W,
}
impl<'a> B193_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B194` reader - B194"]
pub struct B194_R(crate::FieldReader<bool, bool>);
impl B194_R {
    pub(crate) fn new(bits: bool) -> Self {
        B194_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B194_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B194` writer - B194"]
pub struct B194_W<'a> {
    w: &'a mut W,
}
impl<'a> B194_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B195` reader - B195"]
pub struct B195_R(crate::FieldReader<bool, bool>);
impl B195_R {
    pub(crate) fn new(bits: bool) -> Self {
        B195_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B195_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B195` writer - B195"]
pub struct B195_W<'a> {
    w: &'a mut W,
}
impl<'a> B195_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B196` reader - B196"]
pub struct B196_R(crate::FieldReader<bool, bool>);
impl B196_R {
    pub(crate) fn new(bits: bool) -> Self {
        B196_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B196_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B196` writer - B196"]
pub struct B196_W<'a> {
    w: &'a mut W,
}
impl<'a> B196_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B197` reader - B197"]
pub struct B197_R(crate::FieldReader<bool, bool>);
impl B197_R {
    pub(crate) fn new(bits: bool) -> Self {
        B197_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B197_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B197` writer - B197"]
pub struct B197_W<'a> {
    w: &'a mut W,
}
impl<'a> B197_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B198` reader - B198"]
pub struct B198_R(crate::FieldReader<bool, bool>);
impl B198_R {
    pub(crate) fn new(bits: bool) -> Self {
        B198_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B198_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B198` writer - B198"]
pub struct B198_W<'a> {
    w: &'a mut W,
}
impl<'a> B198_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B199` reader - B199"]
pub struct B199_R(crate::FieldReader<bool, bool>);
impl B199_R {
    pub(crate) fn new(bits: bool) -> Self {
        B199_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B199_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B199` writer - B199"]
pub struct B199_W<'a> {
    w: &'a mut W,
}
impl<'a> B199_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B200` reader - B200"]
pub struct B200_R(crate::FieldReader<bool, bool>);
impl B200_R {
    pub(crate) fn new(bits: bool) -> Self {
        B200_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B200_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B200` writer - B200"]
pub struct B200_W<'a> {
    w: &'a mut W,
}
impl<'a> B200_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B201` reader - B201"]
pub struct B201_R(crate::FieldReader<bool, bool>);
impl B201_R {
    pub(crate) fn new(bits: bool) -> Self {
        B201_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B201_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B201` writer - B201"]
pub struct B201_W<'a> {
    w: &'a mut W,
}
impl<'a> B201_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B202` reader - B202"]
pub struct B202_R(crate::FieldReader<bool, bool>);
impl B202_R {
    pub(crate) fn new(bits: bool) -> Self {
        B202_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B202_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B202` writer - B202"]
pub struct B202_W<'a> {
    w: &'a mut W,
}
impl<'a> B202_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B203` reader - B203"]
pub struct B203_R(crate::FieldReader<bool, bool>);
impl B203_R {
    pub(crate) fn new(bits: bool) -> Self {
        B203_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B203_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B203` writer - B203"]
pub struct B203_W<'a> {
    w: &'a mut W,
}
impl<'a> B203_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B204` reader - B204"]
pub struct B204_R(crate::FieldReader<bool, bool>);
impl B204_R {
    pub(crate) fn new(bits: bool) -> Self {
        B204_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B204_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B204` writer - B204"]
pub struct B204_W<'a> {
    w: &'a mut W,
}
impl<'a> B204_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B205` reader - B205"]
pub struct B205_R(crate::FieldReader<bool, bool>);
impl B205_R {
    pub(crate) fn new(bits: bool) -> Self {
        B205_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B205_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B205` writer - B205"]
pub struct B205_W<'a> {
    w: &'a mut W,
}
impl<'a> B205_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B206` reader - B206"]
pub struct B206_R(crate::FieldReader<bool, bool>);
impl B206_R {
    pub(crate) fn new(bits: bool) -> Self {
        B206_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B206_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B206` writer - B206"]
pub struct B206_W<'a> {
    w: &'a mut W,
}
impl<'a> B206_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B207` reader - B207"]
pub struct B207_R(crate::FieldReader<bool, bool>);
impl B207_R {
    pub(crate) fn new(bits: bool) -> Self {
        B207_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B207_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B207` writer - B207"]
pub struct B207_W<'a> {
    w: &'a mut W,
}
impl<'a> B207_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B208` reader - B208"]
pub struct B208_R(crate::FieldReader<bool, bool>);
impl B208_R {
    pub(crate) fn new(bits: bool) -> Self {
        B208_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B208_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B208` writer - B208"]
pub struct B208_W<'a> {
    w: &'a mut W,
}
impl<'a> B208_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B209` reader - B209"]
pub struct B209_R(crate::FieldReader<bool, bool>);
impl B209_R {
    pub(crate) fn new(bits: bool) -> Self {
        B209_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B209_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B209` writer - B209"]
pub struct B209_W<'a> {
    w: &'a mut W,
}
impl<'a> B209_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B210` reader - B210"]
pub struct B210_R(crate::FieldReader<bool, bool>);
impl B210_R {
    pub(crate) fn new(bits: bool) -> Self {
        B210_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B210_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B210` writer - B210"]
pub struct B210_W<'a> {
    w: &'a mut W,
}
impl<'a> B210_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B211` reader - B211"]
pub struct B211_R(crate::FieldReader<bool, bool>);
impl B211_R {
    pub(crate) fn new(bits: bool) -> Self {
        B211_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B211_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B211` writer - B211"]
pub struct B211_W<'a> {
    w: &'a mut W,
}
impl<'a> B211_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B212` reader - B212"]
pub struct B212_R(crate::FieldReader<bool, bool>);
impl B212_R {
    pub(crate) fn new(bits: bool) -> Self {
        B212_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B212_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B212` writer - B212"]
pub struct B212_W<'a> {
    w: &'a mut W,
}
impl<'a> B212_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B213` reader - B213"]
pub struct B213_R(crate::FieldReader<bool, bool>);
impl B213_R {
    pub(crate) fn new(bits: bool) -> Self {
        B213_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B213_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B213` writer - B213"]
pub struct B213_W<'a> {
    w: &'a mut W,
}
impl<'a> B213_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B214` reader - B214"]
pub struct B214_R(crate::FieldReader<bool, bool>);
impl B214_R {
    pub(crate) fn new(bits: bool) -> Self {
        B214_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B214_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B214` writer - B214"]
pub struct B214_W<'a> {
    w: &'a mut W,
}
impl<'a> B214_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B215` reader - B215"]
pub struct B215_R(crate::FieldReader<bool, bool>);
impl B215_R {
    pub(crate) fn new(bits: bool) -> Self {
        B215_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B215_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B215` writer - B215"]
pub struct B215_W<'a> {
    w: &'a mut W,
}
impl<'a> B215_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B216` reader - B216"]
pub struct B216_R(crate::FieldReader<bool, bool>);
impl B216_R {
    pub(crate) fn new(bits: bool) -> Self {
        B216_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B216_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B216` writer - B216"]
pub struct B216_W<'a> {
    w: &'a mut W,
}
impl<'a> B216_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B217` reader - B217"]
pub struct B217_R(crate::FieldReader<bool, bool>);
impl B217_R {
    pub(crate) fn new(bits: bool) -> Self {
        B217_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B217_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B217` writer - B217"]
pub struct B217_W<'a> {
    w: &'a mut W,
}
impl<'a> B217_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B218` reader - B218"]
pub struct B218_R(crate::FieldReader<bool, bool>);
impl B218_R {
    pub(crate) fn new(bits: bool) -> Self {
        B218_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B218_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B218` writer - B218"]
pub struct B218_W<'a> {
    w: &'a mut W,
}
impl<'a> B218_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B219` reader - B219"]
pub struct B219_R(crate::FieldReader<bool, bool>);
impl B219_R {
    pub(crate) fn new(bits: bool) -> Self {
        B219_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B219_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B219` writer - B219"]
pub struct B219_W<'a> {
    w: &'a mut W,
}
impl<'a> B219_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B220` reader - B220"]
pub struct B220_R(crate::FieldReader<bool, bool>);
impl B220_R {
    pub(crate) fn new(bits: bool) -> Self {
        B220_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B220_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B220` writer - B220"]
pub struct B220_W<'a> {
    w: &'a mut W,
}
impl<'a> B220_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B221` reader - B221"]
pub struct B221_R(crate::FieldReader<bool, bool>);
impl B221_R {
    pub(crate) fn new(bits: bool) -> Self {
        B221_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B221_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B221` writer - B221"]
pub struct B221_W<'a> {
    w: &'a mut W,
}
impl<'a> B221_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B222` reader - B222"]
pub struct B222_R(crate::FieldReader<bool, bool>);
impl B222_R {
    pub(crate) fn new(bits: bool) -> Self {
        B222_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B222_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B222` writer - B222"]
pub struct B222_W<'a> {
    w: &'a mut W,
}
impl<'a> B222_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B223` reader - B223"]
pub struct B223_R(crate::FieldReader<bool, bool>);
impl B223_R {
    pub(crate) fn new(bits: bool) -> Self {
        B223_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B223_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B223` writer - B223"]
pub struct B223_W<'a> {
    w: &'a mut W,
}
impl<'a> B223_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B192"]
    #[inline(always)]
    pub fn b192(&self) -> B192_R {
        B192_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B193"]
    #[inline(always)]
    pub fn b193(&self) -> B193_R {
        B193_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B194"]
    #[inline(always)]
    pub fn b194(&self) -> B194_R {
        B194_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B195"]
    #[inline(always)]
    pub fn b195(&self) -> B195_R {
        B195_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B196"]
    #[inline(always)]
    pub fn b196(&self) -> B196_R {
        B196_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B197"]
    #[inline(always)]
    pub fn b197(&self) -> B197_R {
        B197_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B198"]
    #[inline(always)]
    pub fn b198(&self) -> B198_R {
        B198_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B199"]
    #[inline(always)]
    pub fn b199(&self) -> B199_R {
        B199_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B200"]
    #[inline(always)]
    pub fn b200(&self) -> B200_R {
        B200_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B201"]
    #[inline(always)]
    pub fn b201(&self) -> B201_R {
        B201_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B202"]
    #[inline(always)]
    pub fn b202(&self) -> B202_R {
        B202_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B203"]
    #[inline(always)]
    pub fn b203(&self) -> B203_R {
        B203_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B204"]
    #[inline(always)]
    pub fn b204(&self) -> B204_R {
        B204_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B205"]
    #[inline(always)]
    pub fn b205(&self) -> B205_R {
        B205_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B206"]
    #[inline(always)]
    pub fn b206(&self) -> B206_R {
        B206_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B207"]
    #[inline(always)]
    pub fn b207(&self) -> B207_R {
        B207_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B208"]
    #[inline(always)]
    pub fn b208(&self) -> B208_R {
        B208_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B209"]
    #[inline(always)]
    pub fn b209(&self) -> B209_R {
        B209_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B210"]
    #[inline(always)]
    pub fn b210(&self) -> B210_R {
        B210_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B211"]
    #[inline(always)]
    pub fn b211(&self) -> B211_R {
        B211_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B212"]
    #[inline(always)]
    pub fn b212(&self) -> B212_R {
        B212_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B213"]
    #[inline(always)]
    pub fn b213(&self) -> B213_R {
        B213_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B214"]
    #[inline(always)]
    pub fn b214(&self) -> B214_R {
        B214_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B215"]
    #[inline(always)]
    pub fn b215(&self) -> B215_R {
        B215_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B216"]
    #[inline(always)]
    pub fn b216(&self) -> B216_R {
        B216_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B217"]
    #[inline(always)]
    pub fn b217(&self) -> B217_R {
        B217_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B218"]
    #[inline(always)]
    pub fn b218(&self) -> B218_R {
        B218_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B219"]
    #[inline(always)]
    pub fn b219(&self) -> B219_R {
        B219_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B220"]
    #[inline(always)]
    pub fn b220(&self) -> B220_R {
        B220_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B221"]
    #[inline(always)]
    pub fn b221(&self) -> B221_R {
        B221_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B222"]
    #[inline(always)]
    pub fn b222(&self) -> B222_R {
        B222_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B223"]
    #[inline(always)]
    pub fn b223(&self) -> B223_R {
        B223_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B192"]
    #[inline(always)]
    pub fn b192(&mut self) -> B192_W {
        B192_W { w: self }
    }
    #[doc = "Bit 1 - B193"]
    #[inline(always)]
    pub fn b193(&mut self) -> B193_W {
        B193_W { w: self }
    }
    #[doc = "Bit 2 - B194"]
    #[inline(always)]
    pub fn b194(&mut self) -> B194_W {
        B194_W { w: self }
    }
    #[doc = "Bit 3 - B195"]
    #[inline(always)]
    pub fn b195(&mut self) -> B195_W {
        B195_W { w: self }
    }
    #[doc = "Bit 4 - B196"]
    #[inline(always)]
    pub fn b196(&mut self) -> B196_W {
        B196_W { w: self }
    }
    #[doc = "Bit 5 - B197"]
    #[inline(always)]
    pub fn b197(&mut self) -> B197_W {
        B197_W { w: self }
    }
    #[doc = "Bit 6 - B198"]
    #[inline(always)]
    pub fn b198(&mut self) -> B198_W {
        B198_W { w: self }
    }
    #[doc = "Bit 7 - B199"]
    #[inline(always)]
    pub fn b199(&mut self) -> B199_W {
        B199_W { w: self }
    }
    #[doc = "Bit 8 - B200"]
    #[inline(always)]
    pub fn b200(&mut self) -> B200_W {
        B200_W { w: self }
    }
    #[doc = "Bit 9 - B201"]
    #[inline(always)]
    pub fn b201(&mut self) -> B201_W {
        B201_W { w: self }
    }
    #[doc = "Bit 10 - B202"]
    #[inline(always)]
    pub fn b202(&mut self) -> B202_W {
        B202_W { w: self }
    }
    #[doc = "Bit 11 - B203"]
    #[inline(always)]
    pub fn b203(&mut self) -> B203_W {
        B203_W { w: self }
    }
    #[doc = "Bit 12 - B204"]
    #[inline(always)]
    pub fn b204(&mut self) -> B204_W {
        B204_W { w: self }
    }
    #[doc = "Bit 13 - B205"]
    #[inline(always)]
    pub fn b205(&mut self) -> B205_W {
        B205_W { w: self }
    }
    #[doc = "Bit 14 - B206"]
    #[inline(always)]
    pub fn b206(&mut self) -> B206_W {
        B206_W { w: self }
    }
    #[doc = "Bit 15 - B207"]
    #[inline(always)]
    pub fn b207(&mut self) -> B207_W {
        B207_W { w: self }
    }
    #[doc = "Bit 16 - B208"]
    #[inline(always)]
    pub fn b208(&mut self) -> B208_W {
        B208_W { w: self }
    }
    #[doc = "Bit 17 - B209"]
    #[inline(always)]
    pub fn b209(&mut self) -> B209_W {
        B209_W { w: self }
    }
    #[doc = "Bit 18 - B210"]
    #[inline(always)]
    pub fn b210(&mut self) -> B210_W {
        B210_W { w: self }
    }
    #[doc = "Bit 19 - B211"]
    #[inline(always)]
    pub fn b211(&mut self) -> B211_W {
        B211_W { w: self }
    }
    #[doc = "Bit 20 - B212"]
    #[inline(always)]
    pub fn b212(&mut self) -> B212_W {
        B212_W { w: self }
    }
    #[doc = "Bit 21 - B213"]
    #[inline(always)]
    pub fn b213(&mut self) -> B213_W {
        B213_W { w: self }
    }
    #[doc = "Bit 22 - B214"]
    #[inline(always)]
    pub fn b214(&mut self) -> B214_W {
        B214_W { w: self }
    }
    #[doc = "Bit 23 - B215"]
    #[inline(always)]
    pub fn b215(&mut self) -> B215_W {
        B215_W { w: self }
    }
    #[doc = "Bit 24 - B216"]
    #[inline(always)]
    pub fn b216(&mut self) -> B216_W {
        B216_W { w: self }
    }
    #[doc = "Bit 25 - B217"]
    #[inline(always)]
    pub fn b217(&mut self) -> B217_W {
        B217_W { w: self }
    }
    #[doc = "Bit 26 - B218"]
    #[inline(always)]
    pub fn b218(&mut self) -> B218_W {
        B218_W { w: self }
    }
    #[doc = "Bit 27 - B219"]
    #[inline(always)]
    pub fn b219(&mut self) -> B219_W {
        B219_W { w: self }
    }
    #[doc = "Bit 28 - B220"]
    #[inline(always)]
    pub fn b220(&mut self) -> B220_W {
        B220_W { w: self }
    }
    #[doc = "Bit 29 - B221"]
    #[inline(always)]
    pub fn b221(&mut self) -> B221_W {
        B221_W { w: self }
    }
    #[doc = "Bit 30 - B222"]
    #[inline(always)]
    pub fn b222(&mut self) -> B222_W {
        B222_W { w: self }
    }
    #[doc = "Bit 31 - B223"]
    #[inline(always)]
    pub fn b223(&mut self) -> B223_W {
        B223_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr6](index.html) module"]
pub struct MPCBB1_VCTR6_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr6::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr6::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR6 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB1_VCTR6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
