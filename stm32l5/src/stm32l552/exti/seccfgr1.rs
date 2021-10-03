#[doc = "Register `SECCFGR1` reader"]
pub struct R(crate::R<SECCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECCFGR1` writer"]
pub struct W(crate::W<SECCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR1_SPEC>;
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
impl From<crate::W<SECCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC0` reader - Security enable on event input x"]
pub struct SEC0_R(crate::FieldReader<bool, bool>);
impl SEC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC0` writer - Security enable on event input x"]
pub struct SEC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC1` reader - Security enable on event input x"]
pub struct SEC1_R(crate::FieldReader<bool, bool>);
impl SEC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC1` writer - Security enable on event input x"]
pub struct SEC1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC2` reader - Security enable on event input x"]
pub struct SEC2_R(crate::FieldReader<bool, bool>);
impl SEC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC2` writer - Security enable on event input x"]
pub struct SEC2_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC3` reader - Security enable on event input x"]
pub struct SEC3_R(crate::FieldReader<bool, bool>);
impl SEC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC3` writer - Security enable on event input x"]
pub struct SEC3_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC4` reader - Security enable on event input x"]
pub struct SEC4_R(crate::FieldReader<bool, bool>);
impl SEC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC4` writer - Security enable on event input x"]
pub struct SEC4_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC5` reader - Security enable on event input x"]
pub struct SEC5_R(crate::FieldReader<bool, bool>);
impl SEC5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC5` writer - Security enable on event input x"]
pub struct SEC5_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC6` reader - Security enable on event input x"]
pub struct SEC6_R(crate::FieldReader<bool, bool>);
impl SEC6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC6` writer - Security enable on event input x"]
pub struct SEC6_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC7` reader - Security enable on event input x"]
pub struct SEC7_R(crate::FieldReader<bool, bool>);
impl SEC7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC7` writer - Security enable on event input x"]
pub struct SEC7_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC8` reader - Security enable on event input x"]
pub struct SEC8_R(crate::FieldReader<bool, bool>);
impl SEC8_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC8` writer - Security enable on event input x"]
pub struct SEC8_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC9` reader - Security enable on event input x"]
pub struct SEC9_R(crate::FieldReader<bool, bool>);
impl SEC9_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC9` writer - Security enable on event input x"]
pub struct SEC9_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC10` reader - Security enable on event input x"]
pub struct SEC10_R(crate::FieldReader<bool, bool>);
impl SEC10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC10` writer - Security enable on event input x"]
pub struct SEC10_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC11` reader - Security enable on event input x"]
pub struct SEC11_R(crate::FieldReader<bool, bool>);
impl SEC11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC11` writer - Security enable on event input x"]
pub struct SEC11_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC12` reader - Security enable on event input x"]
pub struct SEC12_R(crate::FieldReader<bool, bool>);
impl SEC12_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC12` writer - Security enable on event input x"]
pub struct SEC12_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC13` reader - Security enable on event input x"]
pub struct SEC13_R(crate::FieldReader<bool, bool>);
impl SEC13_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC13` writer - Security enable on event input x"]
pub struct SEC13_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC14` reader - Security enable on event input x"]
pub struct SEC14_R(crate::FieldReader<bool, bool>);
impl SEC14_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC14` writer - Security enable on event input x"]
pub struct SEC14_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC15` reader - Security enable on event input x"]
pub struct SEC15_R(crate::FieldReader<bool, bool>);
impl SEC15_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC15` writer - Security enable on event input x"]
pub struct SEC15_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC16` reader - Security enable on event input x"]
pub struct SEC16_R(crate::FieldReader<bool, bool>);
impl SEC16_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC16` writer - Security enable on event input x"]
pub struct SEC16_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC17` reader - Security enable on event input x"]
pub struct SEC17_R(crate::FieldReader<bool, bool>);
impl SEC17_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC17` writer - Security enable on event input x"]
pub struct SEC17_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC18` reader - Security enable on event input x"]
pub struct SEC18_R(crate::FieldReader<bool, bool>);
impl SEC18_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC18` writer - Security enable on event input x"]
pub struct SEC18_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC19` reader - Security enable on event input x"]
pub struct SEC19_R(crate::FieldReader<bool, bool>);
impl SEC19_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC19` writer - Security enable on event input x"]
pub struct SEC19_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC20` reader - Security enable on event input x"]
pub struct SEC20_R(crate::FieldReader<bool, bool>);
impl SEC20_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC20` writer - Security enable on event input x"]
pub struct SEC20_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC21` reader - Security enable on event input x"]
pub struct SEC21_R(crate::FieldReader<bool, bool>);
impl SEC21_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC21` writer - Security enable on event input x"]
pub struct SEC21_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC22` reader - Security enable on event input x"]
pub struct SEC22_R(crate::FieldReader<bool, bool>);
impl SEC22_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC22` writer - Security enable on event input x"]
pub struct SEC22_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC23` reader - Security enable on event input x"]
pub struct SEC23_R(crate::FieldReader<bool, bool>);
impl SEC23_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC23` writer - Security enable on event input x"]
pub struct SEC23_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC24` reader - Security enable on event input x"]
pub struct SEC24_R(crate::FieldReader<bool, bool>);
impl SEC24_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC24` writer - Security enable on event input x"]
pub struct SEC24_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC25` reader - Security enable on event input x"]
pub struct SEC25_R(crate::FieldReader<bool, bool>);
impl SEC25_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC25` writer - Security enable on event input x"]
pub struct SEC25_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC26` reader - Security enable on event input x"]
pub struct SEC26_R(crate::FieldReader<bool, bool>);
impl SEC26_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC26` writer - Security enable on event input x"]
pub struct SEC26_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC27` reader - Security enable on event input x"]
pub struct SEC27_R(crate::FieldReader<bool, bool>);
impl SEC27_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC27` writer - Security enable on event input x"]
pub struct SEC27_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC28` reader - Security enable on event input x"]
pub struct SEC28_R(crate::FieldReader<bool, bool>);
impl SEC28_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC28` writer - Security enable on event input x"]
pub struct SEC28_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC29` reader - Security enable on event input x"]
pub struct SEC29_R(crate::FieldReader<bool, bool>);
impl SEC29_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC29` writer - Security enable on event input x"]
pub struct SEC29_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC30` reader - Security enable on event input x"]
pub struct SEC30_R(crate::FieldReader<bool, bool>);
impl SEC30_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC30` writer - Security enable on event input x"]
pub struct SEC30_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SEC31` reader - Security enable on event input x"]
pub struct SEC31_R(crate::FieldReader<bool, bool>);
impl SEC31_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC31` writer - Security enable on event input x"]
pub struct SEC31_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec0(&self) -> SEC0_R {
        SEC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec2(&self) -> SEC2_R {
        SEC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec3(&self) -> SEC3_R {
        SEC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec4(&self) -> SEC4_R {
        SEC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec5(&self) -> SEC5_R {
        SEC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec6(&self) -> SEC6_R {
        SEC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec7(&self) -> SEC7_R {
        SEC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec8(&self) -> SEC8_R {
        SEC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec9(&self) -> SEC9_R {
        SEC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec11(&self) -> SEC11_R {
        SEC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec12(&self) -> SEC12_R {
        SEC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec13(&self) -> SEC13_R {
        SEC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec14(&self) -> SEC14_R {
        SEC14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec15(&self) -> SEC15_R {
        SEC15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec16(&self) -> SEC16_R {
        SEC16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec17(&self) -> SEC17_R {
        SEC17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec18(&self) -> SEC18_R {
        SEC18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec19(&self) -> SEC19_R {
        SEC19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec20(&self) -> SEC20_R {
        SEC20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec21(&self) -> SEC21_R {
        SEC21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec22(&self) -> SEC22_R {
        SEC22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec23(&self) -> SEC23_R {
        SEC23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec24(&self) -> SEC24_R {
        SEC24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec25(&self) -> SEC25_R {
        SEC25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec26(&self) -> SEC26_R {
        SEC26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec27(&self) -> SEC27_R {
        SEC27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec28(&self) -> SEC28_R {
        SEC28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec29(&self) -> SEC29_R {
        SEC29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec30(&self) -> SEC30_R {
        SEC30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec31(&self) -> SEC31_R {
        SEC31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec0(&mut self) -> SEC0_W {
        SEC0_W { w: self }
    }
    #[doc = "Bit 1 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec1(&mut self) -> SEC1_W {
        SEC1_W { w: self }
    }
    #[doc = "Bit 2 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec2(&mut self) -> SEC2_W {
        SEC2_W { w: self }
    }
    #[doc = "Bit 3 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec3(&mut self) -> SEC3_W {
        SEC3_W { w: self }
    }
    #[doc = "Bit 4 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec4(&mut self) -> SEC4_W {
        SEC4_W { w: self }
    }
    #[doc = "Bit 5 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec5(&mut self) -> SEC5_W {
        SEC5_W { w: self }
    }
    #[doc = "Bit 6 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec6(&mut self) -> SEC6_W {
        SEC6_W { w: self }
    }
    #[doc = "Bit 7 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec7(&mut self) -> SEC7_W {
        SEC7_W { w: self }
    }
    #[doc = "Bit 8 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec8(&mut self) -> SEC8_W {
        SEC8_W { w: self }
    }
    #[doc = "Bit 9 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec9(&mut self) -> SEC9_W {
        SEC9_W { w: self }
    }
    #[doc = "Bit 10 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec10(&mut self) -> SEC10_W {
        SEC10_W { w: self }
    }
    #[doc = "Bit 11 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec11(&mut self) -> SEC11_W {
        SEC11_W { w: self }
    }
    #[doc = "Bit 12 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec12(&mut self) -> SEC12_W {
        SEC12_W { w: self }
    }
    #[doc = "Bit 13 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec13(&mut self) -> SEC13_W {
        SEC13_W { w: self }
    }
    #[doc = "Bit 14 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec14(&mut self) -> SEC14_W {
        SEC14_W { w: self }
    }
    #[doc = "Bit 15 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec15(&mut self) -> SEC15_W {
        SEC15_W { w: self }
    }
    #[doc = "Bit 16 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec16(&mut self) -> SEC16_W {
        SEC16_W { w: self }
    }
    #[doc = "Bit 17 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec17(&mut self) -> SEC17_W {
        SEC17_W { w: self }
    }
    #[doc = "Bit 18 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec18(&mut self) -> SEC18_W {
        SEC18_W { w: self }
    }
    #[doc = "Bit 19 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec19(&mut self) -> SEC19_W {
        SEC19_W { w: self }
    }
    #[doc = "Bit 20 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec20(&mut self) -> SEC20_W {
        SEC20_W { w: self }
    }
    #[doc = "Bit 21 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec21(&mut self) -> SEC21_W {
        SEC21_W { w: self }
    }
    #[doc = "Bit 22 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec22(&mut self) -> SEC22_W {
        SEC22_W { w: self }
    }
    #[doc = "Bit 23 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec23(&mut self) -> SEC23_W {
        SEC23_W { w: self }
    }
    #[doc = "Bit 24 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec24(&mut self) -> SEC24_W {
        SEC24_W { w: self }
    }
    #[doc = "Bit 25 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec25(&mut self) -> SEC25_W {
        SEC25_W { w: self }
    }
    #[doc = "Bit 26 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec26(&mut self) -> SEC26_W {
        SEC26_W { w: self }
    }
    #[doc = "Bit 27 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec27(&mut self) -> SEC27_W {
        SEC27_W { w: self }
    }
    #[doc = "Bit 28 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec28(&mut self) -> SEC28_W {
        SEC28_W { w: self }
    }
    #[doc = "Bit 29 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec29(&mut self) -> SEC29_W {
        SEC29_W { w: self }
    }
    #[doc = "Bit 30 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec30(&mut self) -> SEC30_W {
        SEC30_W { w: self }
    }
    #[doc = "Bit 31 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec31(&mut self) -> SEC31_W {
        SEC31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI security configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr1](index.html) module"]
pub struct SECCFGR1_SPEC;
impl crate::RegisterSpec for SECCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seccfgr1::R](R) reader structure"]
impl crate::Readable for SECCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seccfgr1::W](W) writer structure"]
impl crate::Writable for SECCFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECCFGR1 to value 0"]
impl crate::Resettable for SECCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
