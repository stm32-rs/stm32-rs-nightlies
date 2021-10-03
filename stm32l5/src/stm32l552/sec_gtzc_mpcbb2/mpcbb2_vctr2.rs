#[doc = "Register `MPCBB2_VCTR2` reader"]
pub struct R(crate::R<MPCBB2_VCTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR2` writer"]
pub struct W(crate::W<MPCBB2_VCTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR2_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B64` reader - B64"]
pub struct B64_R(crate::FieldReader<bool, bool>);
impl B64_R {
    pub(crate) fn new(bits: bool) -> Self {
        B64_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B64_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B64` writer - B64"]
pub struct B64_W<'a> {
    w: &'a mut W,
}
impl<'a> B64_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B65` reader - B65"]
pub struct B65_R(crate::FieldReader<bool, bool>);
impl B65_R {
    pub(crate) fn new(bits: bool) -> Self {
        B65_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B65_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B65` writer - B65"]
pub struct B65_W<'a> {
    w: &'a mut W,
}
impl<'a> B65_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B66` reader - B66"]
pub struct B66_R(crate::FieldReader<bool, bool>);
impl B66_R {
    pub(crate) fn new(bits: bool) -> Self {
        B66_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B66_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B66` writer - B66"]
pub struct B66_W<'a> {
    w: &'a mut W,
}
impl<'a> B66_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B67` reader - B67"]
pub struct B67_R(crate::FieldReader<bool, bool>);
impl B67_R {
    pub(crate) fn new(bits: bool) -> Self {
        B67_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B67_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B67` writer - B67"]
pub struct B67_W<'a> {
    w: &'a mut W,
}
impl<'a> B67_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B68` reader - B68"]
pub struct B68_R(crate::FieldReader<bool, bool>);
impl B68_R {
    pub(crate) fn new(bits: bool) -> Self {
        B68_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B68_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B68` writer - B68"]
pub struct B68_W<'a> {
    w: &'a mut W,
}
impl<'a> B68_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B69` reader - B69"]
pub struct B69_R(crate::FieldReader<bool, bool>);
impl B69_R {
    pub(crate) fn new(bits: bool) -> Self {
        B69_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B69_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B69` writer - B69"]
pub struct B69_W<'a> {
    w: &'a mut W,
}
impl<'a> B69_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B70` reader - B70"]
pub struct B70_R(crate::FieldReader<bool, bool>);
impl B70_R {
    pub(crate) fn new(bits: bool) -> Self {
        B70_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B70_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B70` writer - B70"]
pub struct B70_W<'a> {
    w: &'a mut W,
}
impl<'a> B70_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B71` reader - B71"]
pub struct B71_R(crate::FieldReader<bool, bool>);
impl B71_R {
    pub(crate) fn new(bits: bool) -> Self {
        B71_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B71_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B71` writer - B71"]
pub struct B71_W<'a> {
    w: &'a mut W,
}
impl<'a> B71_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B72` reader - B72"]
pub struct B72_R(crate::FieldReader<bool, bool>);
impl B72_R {
    pub(crate) fn new(bits: bool) -> Self {
        B72_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B72_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B72` writer - B72"]
pub struct B72_W<'a> {
    w: &'a mut W,
}
impl<'a> B72_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B73` reader - B73"]
pub struct B73_R(crate::FieldReader<bool, bool>);
impl B73_R {
    pub(crate) fn new(bits: bool) -> Self {
        B73_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B73_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B73` writer - B73"]
pub struct B73_W<'a> {
    w: &'a mut W,
}
impl<'a> B73_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B74` reader - B74"]
pub struct B74_R(crate::FieldReader<bool, bool>);
impl B74_R {
    pub(crate) fn new(bits: bool) -> Self {
        B74_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B74_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B74` writer - B74"]
pub struct B74_W<'a> {
    w: &'a mut W,
}
impl<'a> B74_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B75` reader - B75"]
pub struct B75_R(crate::FieldReader<bool, bool>);
impl B75_R {
    pub(crate) fn new(bits: bool) -> Self {
        B75_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B75_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B75` writer - B75"]
pub struct B75_W<'a> {
    w: &'a mut W,
}
impl<'a> B75_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B76` reader - B76"]
pub struct B76_R(crate::FieldReader<bool, bool>);
impl B76_R {
    pub(crate) fn new(bits: bool) -> Self {
        B76_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B76_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B76` writer - B76"]
pub struct B76_W<'a> {
    w: &'a mut W,
}
impl<'a> B76_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B77` reader - B77"]
pub struct B77_R(crate::FieldReader<bool, bool>);
impl B77_R {
    pub(crate) fn new(bits: bool) -> Self {
        B77_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B77_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B77` writer - B77"]
pub struct B77_W<'a> {
    w: &'a mut W,
}
impl<'a> B77_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B78` reader - B78"]
pub struct B78_R(crate::FieldReader<bool, bool>);
impl B78_R {
    pub(crate) fn new(bits: bool) -> Self {
        B78_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B78_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B78` writer - B78"]
pub struct B78_W<'a> {
    w: &'a mut W,
}
impl<'a> B78_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B79` reader - B79"]
pub struct B79_R(crate::FieldReader<bool, bool>);
impl B79_R {
    pub(crate) fn new(bits: bool) -> Self {
        B79_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B79_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B79` writer - B79"]
pub struct B79_W<'a> {
    w: &'a mut W,
}
impl<'a> B79_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B80` reader - B80"]
pub struct B80_R(crate::FieldReader<bool, bool>);
impl B80_R {
    pub(crate) fn new(bits: bool) -> Self {
        B80_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B80_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B80` writer - B80"]
pub struct B80_W<'a> {
    w: &'a mut W,
}
impl<'a> B80_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B81` reader - B81"]
pub struct B81_R(crate::FieldReader<bool, bool>);
impl B81_R {
    pub(crate) fn new(bits: bool) -> Self {
        B81_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B81_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B81` writer - B81"]
pub struct B81_W<'a> {
    w: &'a mut W,
}
impl<'a> B81_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B82` reader - B82"]
pub struct B82_R(crate::FieldReader<bool, bool>);
impl B82_R {
    pub(crate) fn new(bits: bool) -> Self {
        B82_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B82_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B82` writer - B82"]
pub struct B82_W<'a> {
    w: &'a mut W,
}
impl<'a> B82_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B83` reader - B83"]
pub struct B83_R(crate::FieldReader<bool, bool>);
impl B83_R {
    pub(crate) fn new(bits: bool) -> Self {
        B83_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B83_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B83` writer - B83"]
pub struct B83_W<'a> {
    w: &'a mut W,
}
impl<'a> B83_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B84` reader - B84"]
pub struct B84_R(crate::FieldReader<bool, bool>);
impl B84_R {
    pub(crate) fn new(bits: bool) -> Self {
        B84_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B84_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B84` writer - B84"]
pub struct B84_W<'a> {
    w: &'a mut W,
}
impl<'a> B84_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B85` reader - B85"]
pub struct B85_R(crate::FieldReader<bool, bool>);
impl B85_R {
    pub(crate) fn new(bits: bool) -> Self {
        B85_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B85_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B85` writer - B85"]
pub struct B85_W<'a> {
    w: &'a mut W,
}
impl<'a> B85_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B86` reader - B86"]
pub struct B86_R(crate::FieldReader<bool, bool>);
impl B86_R {
    pub(crate) fn new(bits: bool) -> Self {
        B86_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B86_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B86` writer - B86"]
pub struct B86_W<'a> {
    w: &'a mut W,
}
impl<'a> B86_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B87` reader - B87"]
pub struct B87_R(crate::FieldReader<bool, bool>);
impl B87_R {
    pub(crate) fn new(bits: bool) -> Self {
        B87_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B87_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B87` writer - B87"]
pub struct B87_W<'a> {
    w: &'a mut W,
}
impl<'a> B87_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B88` reader - B88"]
pub struct B88_R(crate::FieldReader<bool, bool>);
impl B88_R {
    pub(crate) fn new(bits: bool) -> Self {
        B88_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B88_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B88` writer - B88"]
pub struct B88_W<'a> {
    w: &'a mut W,
}
impl<'a> B88_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B89` reader - B89"]
pub struct B89_R(crate::FieldReader<bool, bool>);
impl B89_R {
    pub(crate) fn new(bits: bool) -> Self {
        B89_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B89_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B89` writer - B89"]
pub struct B89_W<'a> {
    w: &'a mut W,
}
impl<'a> B89_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B90` reader - B90"]
pub struct B90_R(crate::FieldReader<bool, bool>);
impl B90_R {
    pub(crate) fn new(bits: bool) -> Self {
        B90_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B90_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B90` writer - B90"]
pub struct B90_W<'a> {
    w: &'a mut W,
}
impl<'a> B90_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B91` reader - B91"]
pub struct B91_R(crate::FieldReader<bool, bool>);
impl B91_R {
    pub(crate) fn new(bits: bool) -> Self {
        B91_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B91_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B91` writer - B91"]
pub struct B91_W<'a> {
    w: &'a mut W,
}
impl<'a> B91_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B92` reader - B92"]
pub struct B92_R(crate::FieldReader<bool, bool>);
impl B92_R {
    pub(crate) fn new(bits: bool) -> Self {
        B92_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B92_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B92` writer - B92"]
pub struct B92_W<'a> {
    w: &'a mut W,
}
impl<'a> B92_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B93` reader - B93"]
pub struct B93_R(crate::FieldReader<bool, bool>);
impl B93_R {
    pub(crate) fn new(bits: bool) -> Self {
        B93_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B93_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B93` writer - B93"]
pub struct B93_W<'a> {
    w: &'a mut W,
}
impl<'a> B93_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B94` reader - B94"]
pub struct B94_R(crate::FieldReader<bool, bool>);
impl B94_R {
    pub(crate) fn new(bits: bool) -> Self {
        B94_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B94_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B94` writer - B94"]
pub struct B94_W<'a> {
    w: &'a mut W,
}
impl<'a> B94_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B95` reader - B95"]
pub struct B95_R(crate::FieldReader<bool, bool>);
impl B95_R {
    pub(crate) fn new(bits: bool) -> Self {
        B95_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B95_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B95` writer - B95"]
pub struct B95_W<'a> {
    w: &'a mut W,
}
impl<'a> B95_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B64"]
    #[inline(always)]
    pub fn b64(&self) -> B64_R {
        B64_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B65"]
    #[inline(always)]
    pub fn b65(&self) -> B65_R {
        B65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B66"]
    #[inline(always)]
    pub fn b66(&self) -> B66_R {
        B66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B67"]
    #[inline(always)]
    pub fn b67(&self) -> B67_R {
        B67_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B68"]
    #[inline(always)]
    pub fn b68(&self) -> B68_R {
        B68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B69"]
    #[inline(always)]
    pub fn b69(&self) -> B69_R {
        B69_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B70"]
    #[inline(always)]
    pub fn b70(&self) -> B70_R {
        B70_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B71"]
    #[inline(always)]
    pub fn b71(&self) -> B71_R {
        B71_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B72"]
    #[inline(always)]
    pub fn b72(&self) -> B72_R {
        B72_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B73"]
    #[inline(always)]
    pub fn b73(&self) -> B73_R {
        B73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B74"]
    #[inline(always)]
    pub fn b74(&self) -> B74_R {
        B74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B75"]
    #[inline(always)]
    pub fn b75(&self) -> B75_R {
        B75_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B76"]
    #[inline(always)]
    pub fn b76(&self) -> B76_R {
        B76_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B77"]
    #[inline(always)]
    pub fn b77(&self) -> B77_R {
        B77_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B78"]
    #[inline(always)]
    pub fn b78(&self) -> B78_R {
        B78_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B79"]
    #[inline(always)]
    pub fn b79(&self) -> B79_R {
        B79_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B80"]
    #[inline(always)]
    pub fn b80(&self) -> B80_R {
        B80_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B81"]
    #[inline(always)]
    pub fn b81(&self) -> B81_R {
        B81_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B82"]
    #[inline(always)]
    pub fn b82(&self) -> B82_R {
        B82_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B83"]
    #[inline(always)]
    pub fn b83(&self) -> B83_R {
        B83_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B84"]
    #[inline(always)]
    pub fn b84(&self) -> B84_R {
        B84_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B85"]
    #[inline(always)]
    pub fn b85(&self) -> B85_R {
        B85_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B86"]
    #[inline(always)]
    pub fn b86(&self) -> B86_R {
        B86_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B87"]
    #[inline(always)]
    pub fn b87(&self) -> B87_R {
        B87_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B88"]
    #[inline(always)]
    pub fn b88(&self) -> B88_R {
        B88_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B89"]
    #[inline(always)]
    pub fn b89(&self) -> B89_R {
        B89_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B90"]
    #[inline(always)]
    pub fn b90(&self) -> B90_R {
        B90_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B91"]
    #[inline(always)]
    pub fn b91(&self) -> B91_R {
        B91_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B92"]
    #[inline(always)]
    pub fn b92(&self) -> B92_R {
        B92_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B93"]
    #[inline(always)]
    pub fn b93(&self) -> B93_R {
        B93_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B94"]
    #[inline(always)]
    pub fn b94(&self) -> B94_R {
        B94_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B95"]
    #[inline(always)]
    pub fn b95(&self) -> B95_R {
        B95_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B64"]
    #[inline(always)]
    pub fn b64(&mut self) -> B64_W {
        B64_W { w: self }
    }
    #[doc = "Bit 1 - B65"]
    #[inline(always)]
    pub fn b65(&mut self) -> B65_W {
        B65_W { w: self }
    }
    #[doc = "Bit 2 - B66"]
    #[inline(always)]
    pub fn b66(&mut self) -> B66_W {
        B66_W { w: self }
    }
    #[doc = "Bit 3 - B67"]
    #[inline(always)]
    pub fn b67(&mut self) -> B67_W {
        B67_W { w: self }
    }
    #[doc = "Bit 4 - B68"]
    #[inline(always)]
    pub fn b68(&mut self) -> B68_W {
        B68_W { w: self }
    }
    #[doc = "Bit 5 - B69"]
    #[inline(always)]
    pub fn b69(&mut self) -> B69_W {
        B69_W { w: self }
    }
    #[doc = "Bit 6 - B70"]
    #[inline(always)]
    pub fn b70(&mut self) -> B70_W {
        B70_W { w: self }
    }
    #[doc = "Bit 7 - B71"]
    #[inline(always)]
    pub fn b71(&mut self) -> B71_W {
        B71_W { w: self }
    }
    #[doc = "Bit 8 - B72"]
    #[inline(always)]
    pub fn b72(&mut self) -> B72_W {
        B72_W { w: self }
    }
    #[doc = "Bit 9 - B73"]
    #[inline(always)]
    pub fn b73(&mut self) -> B73_W {
        B73_W { w: self }
    }
    #[doc = "Bit 10 - B74"]
    #[inline(always)]
    pub fn b74(&mut self) -> B74_W {
        B74_W { w: self }
    }
    #[doc = "Bit 11 - B75"]
    #[inline(always)]
    pub fn b75(&mut self) -> B75_W {
        B75_W { w: self }
    }
    #[doc = "Bit 12 - B76"]
    #[inline(always)]
    pub fn b76(&mut self) -> B76_W {
        B76_W { w: self }
    }
    #[doc = "Bit 13 - B77"]
    #[inline(always)]
    pub fn b77(&mut self) -> B77_W {
        B77_W { w: self }
    }
    #[doc = "Bit 14 - B78"]
    #[inline(always)]
    pub fn b78(&mut self) -> B78_W {
        B78_W { w: self }
    }
    #[doc = "Bit 15 - B79"]
    #[inline(always)]
    pub fn b79(&mut self) -> B79_W {
        B79_W { w: self }
    }
    #[doc = "Bit 16 - B80"]
    #[inline(always)]
    pub fn b80(&mut self) -> B80_W {
        B80_W { w: self }
    }
    #[doc = "Bit 17 - B81"]
    #[inline(always)]
    pub fn b81(&mut self) -> B81_W {
        B81_W { w: self }
    }
    #[doc = "Bit 18 - B82"]
    #[inline(always)]
    pub fn b82(&mut self) -> B82_W {
        B82_W { w: self }
    }
    #[doc = "Bit 19 - B83"]
    #[inline(always)]
    pub fn b83(&mut self) -> B83_W {
        B83_W { w: self }
    }
    #[doc = "Bit 20 - B84"]
    #[inline(always)]
    pub fn b84(&mut self) -> B84_W {
        B84_W { w: self }
    }
    #[doc = "Bit 21 - B85"]
    #[inline(always)]
    pub fn b85(&mut self) -> B85_W {
        B85_W { w: self }
    }
    #[doc = "Bit 22 - B86"]
    #[inline(always)]
    pub fn b86(&mut self) -> B86_W {
        B86_W { w: self }
    }
    #[doc = "Bit 23 - B87"]
    #[inline(always)]
    pub fn b87(&mut self) -> B87_W {
        B87_W { w: self }
    }
    #[doc = "Bit 24 - B88"]
    #[inline(always)]
    pub fn b88(&mut self) -> B88_W {
        B88_W { w: self }
    }
    #[doc = "Bit 25 - B89"]
    #[inline(always)]
    pub fn b89(&mut self) -> B89_W {
        B89_W { w: self }
    }
    #[doc = "Bit 26 - B90"]
    #[inline(always)]
    pub fn b90(&mut self) -> B90_W {
        B90_W { w: self }
    }
    #[doc = "Bit 27 - B91"]
    #[inline(always)]
    pub fn b91(&mut self) -> B91_W {
        B91_W { w: self }
    }
    #[doc = "Bit 28 - B92"]
    #[inline(always)]
    pub fn b92(&mut self) -> B92_W {
        B92_W { w: self }
    }
    #[doc = "Bit 29 - B93"]
    #[inline(always)]
    pub fn b93(&mut self) -> B93_W {
        B93_W { w: self }
    }
    #[doc = "Bit 30 - B94"]
    #[inline(always)]
    pub fn b94(&mut self) -> B94_W {
        B94_W { w: self }
    }
    #[doc = "Bit 31 - B95"]
    #[inline(always)]
    pub fn b95(&mut self) -> B95_W {
        B95_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr2](index.html) module"]
pub struct MPCBB2_VCTR2_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr2::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr2::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR2 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB2_VCTR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
