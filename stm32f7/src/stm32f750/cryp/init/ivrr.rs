#[doc = "Register `IVRR` reader"]
pub struct R(crate::R<IVRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IVRR` writer"]
pub struct W(crate::W<IVRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVRR_SPEC>;
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
impl From<crate::W<IVRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV63` reader - IV63"]
pub struct IV63_R(crate::FieldReader<bool, bool>);
impl IV63_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV63_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV63_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV63` writer - IV63"]
pub struct IV63_W<'a> {
    w: &'a mut W,
}
impl<'a> IV63_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV62` reader - IV62"]
pub struct IV62_R(crate::FieldReader<bool, bool>);
impl IV62_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV62_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV62_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV62` writer - IV62"]
pub struct IV62_W<'a> {
    w: &'a mut W,
}
impl<'a> IV62_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV61` reader - IV61"]
pub struct IV61_R(crate::FieldReader<bool, bool>);
impl IV61_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV61_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV61_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV61` writer - IV61"]
pub struct IV61_W<'a> {
    w: &'a mut W,
}
impl<'a> IV61_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV60` reader - IV60"]
pub struct IV60_R(crate::FieldReader<bool, bool>);
impl IV60_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV60_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV60_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV60` writer - IV60"]
pub struct IV60_W<'a> {
    w: &'a mut W,
}
impl<'a> IV60_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV59` reader - IV59"]
pub struct IV59_R(crate::FieldReader<bool, bool>);
impl IV59_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV59_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV59_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV59` writer - IV59"]
pub struct IV59_W<'a> {
    w: &'a mut W,
}
impl<'a> IV59_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV58` reader - IV58"]
pub struct IV58_R(crate::FieldReader<bool, bool>);
impl IV58_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV58_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV58_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV58` writer - IV58"]
pub struct IV58_W<'a> {
    w: &'a mut W,
}
impl<'a> IV58_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV57` reader - IV57"]
pub struct IV57_R(crate::FieldReader<bool, bool>);
impl IV57_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV57_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV57_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV57` writer - IV57"]
pub struct IV57_W<'a> {
    w: &'a mut W,
}
impl<'a> IV57_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV56` reader - IV56"]
pub struct IV56_R(crate::FieldReader<bool, bool>);
impl IV56_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV56_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV56_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV56` writer - IV56"]
pub struct IV56_W<'a> {
    w: &'a mut W,
}
impl<'a> IV56_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV55` reader - IV55"]
pub struct IV55_R(crate::FieldReader<bool, bool>);
impl IV55_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV55_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV55_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV55` writer - IV55"]
pub struct IV55_W<'a> {
    w: &'a mut W,
}
impl<'a> IV55_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV54` reader - IV54"]
pub struct IV54_R(crate::FieldReader<bool, bool>);
impl IV54_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV54_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV54_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV54` writer - IV54"]
pub struct IV54_W<'a> {
    w: &'a mut W,
}
impl<'a> IV54_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV53` reader - IV53"]
pub struct IV53_R(crate::FieldReader<bool, bool>);
impl IV53_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV53_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV53_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV53` writer - IV53"]
pub struct IV53_W<'a> {
    w: &'a mut W,
}
impl<'a> IV53_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV52` reader - IV52"]
pub struct IV52_R(crate::FieldReader<bool, bool>);
impl IV52_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV52_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV52_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV52` writer - IV52"]
pub struct IV52_W<'a> {
    w: &'a mut W,
}
impl<'a> IV52_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV51` reader - IV51"]
pub struct IV51_R(crate::FieldReader<bool, bool>);
impl IV51_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV51_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV51_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV51` writer - IV51"]
pub struct IV51_W<'a> {
    w: &'a mut W,
}
impl<'a> IV51_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV50` reader - IV50"]
pub struct IV50_R(crate::FieldReader<bool, bool>);
impl IV50_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV50_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV50_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV50` writer - IV50"]
pub struct IV50_W<'a> {
    w: &'a mut W,
}
impl<'a> IV50_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV49` reader - IV49"]
pub struct IV49_R(crate::FieldReader<bool, bool>);
impl IV49_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV49_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV49_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV49` writer - IV49"]
pub struct IV49_W<'a> {
    w: &'a mut W,
}
impl<'a> IV49_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV48` reader - IV48"]
pub struct IV48_R(crate::FieldReader<bool, bool>);
impl IV48_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV48_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV48_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV48` writer - IV48"]
pub struct IV48_W<'a> {
    w: &'a mut W,
}
impl<'a> IV48_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV47` reader - IV47"]
pub struct IV47_R(crate::FieldReader<bool, bool>);
impl IV47_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV47_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV47_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV47` writer - IV47"]
pub struct IV47_W<'a> {
    w: &'a mut W,
}
impl<'a> IV47_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV46` reader - IV46"]
pub struct IV46_R(crate::FieldReader<bool, bool>);
impl IV46_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV46_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV46_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV46` writer - IV46"]
pub struct IV46_W<'a> {
    w: &'a mut W,
}
impl<'a> IV46_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV45` reader - IV45"]
pub struct IV45_R(crate::FieldReader<bool, bool>);
impl IV45_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV45_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV45_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV45` writer - IV45"]
pub struct IV45_W<'a> {
    w: &'a mut W,
}
impl<'a> IV45_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV44` reader - IV44"]
pub struct IV44_R(crate::FieldReader<bool, bool>);
impl IV44_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV44_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV44_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV44` writer - IV44"]
pub struct IV44_W<'a> {
    w: &'a mut W,
}
impl<'a> IV44_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV43` reader - IV43"]
pub struct IV43_R(crate::FieldReader<bool, bool>);
impl IV43_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV43_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV43_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV43` writer - IV43"]
pub struct IV43_W<'a> {
    w: &'a mut W,
}
impl<'a> IV43_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV42` reader - IV42"]
pub struct IV42_R(crate::FieldReader<bool, bool>);
impl IV42_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV42_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV42_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV42` writer - IV42"]
pub struct IV42_W<'a> {
    w: &'a mut W,
}
impl<'a> IV42_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV41` reader - IV41"]
pub struct IV41_R(crate::FieldReader<bool, bool>);
impl IV41_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV41_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV41_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV41` writer - IV41"]
pub struct IV41_W<'a> {
    w: &'a mut W,
}
impl<'a> IV41_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV40` reader - IV40"]
pub struct IV40_R(crate::FieldReader<bool, bool>);
impl IV40_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV40_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV40_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV40` writer - IV40"]
pub struct IV40_W<'a> {
    w: &'a mut W,
}
impl<'a> IV40_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV39` reader - IV39"]
pub struct IV39_R(crate::FieldReader<bool, bool>);
impl IV39_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV39_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV39_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV39` writer - IV39"]
pub struct IV39_W<'a> {
    w: &'a mut W,
}
impl<'a> IV39_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV38` reader - IV38"]
pub struct IV38_R(crate::FieldReader<bool, bool>);
impl IV38_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV38_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV38_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV38` writer - IV38"]
pub struct IV38_W<'a> {
    w: &'a mut W,
}
impl<'a> IV38_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV37` reader - IV37"]
pub struct IV37_R(crate::FieldReader<bool, bool>);
impl IV37_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV37_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV37_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV37` writer - IV37"]
pub struct IV37_W<'a> {
    w: &'a mut W,
}
impl<'a> IV37_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV36` reader - IV36"]
pub struct IV36_R(crate::FieldReader<bool, bool>);
impl IV36_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV36_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV36_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV36` writer - IV36"]
pub struct IV36_W<'a> {
    w: &'a mut W,
}
impl<'a> IV36_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV35` reader - IV35"]
pub struct IV35_R(crate::FieldReader<bool, bool>);
impl IV35_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV35_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV35_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV35` writer - IV35"]
pub struct IV35_W<'a> {
    w: &'a mut W,
}
impl<'a> IV35_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV34` reader - IV34"]
pub struct IV34_R(crate::FieldReader<bool, bool>);
impl IV34_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV34_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV34_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV34` writer - IV34"]
pub struct IV34_W<'a> {
    w: &'a mut W,
}
impl<'a> IV34_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV33` reader - IV33"]
pub struct IV33_R(crate::FieldReader<bool, bool>);
impl IV33_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV33_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV33` writer - IV33"]
pub struct IV33_W<'a> {
    w: &'a mut W,
}
impl<'a> IV33_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV32` reader - IV32"]
pub struct IV32_R(crate::FieldReader<bool, bool>);
impl IV32_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV32` writer - IV32"]
pub struct IV32_W<'a> {
    w: &'a mut W,
}
impl<'a> IV32_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - IV63"]
    #[inline(always)]
    pub fn iv63(&self) -> IV63_R {
        IV63_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IV62"]
    #[inline(always)]
    pub fn iv62(&self) -> IV62_R {
        IV62_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IV61"]
    #[inline(always)]
    pub fn iv61(&self) -> IV61_R {
        IV61_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IV60"]
    #[inline(always)]
    pub fn iv60(&self) -> IV60_R {
        IV60_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IV59"]
    #[inline(always)]
    pub fn iv59(&self) -> IV59_R {
        IV59_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IV58"]
    #[inline(always)]
    pub fn iv58(&self) -> IV58_R {
        IV58_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IV57"]
    #[inline(always)]
    pub fn iv57(&self) -> IV57_R {
        IV57_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IV56"]
    #[inline(always)]
    pub fn iv56(&self) -> IV56_R {
        IV56_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - IV55"]
    #[inline(always)]
    pub fn iv55(&self) -> IV55_R {
        IV55_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - IV54"]
    #[inline(always)]
    pub fn iv54(&self) -> IV54_R {
        IV54_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - IV53"]
    #[inline(always)]
    pub fn iv53(&self) -> IV53_R {
        IV53_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - IV52"]
    #[inline(always)]
    pub fn iv52(&self) -> IV52_R {
        IV52_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IV51"]
    #[inline(always)]
    pub fn iv51(&self) -> IV51_R {
        IV51_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - IV50"]
    #[inline(always)]
    pub fn iv50(&self) -> IV50_R {
        IV50_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - IV49"]
    #[inline(always)]
    pub fn iv49(&self) -> IV49_R {
        IV49_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IV48"]
    #[inline(always)]
    pub fn iv48(&self) -> IV48_R {
        IV48_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - IV47"]
    #[inline(always)]
    pub fn iv47(&self) -> IV47_R {
        IV47_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - IV46"]
    #[inline(always)]
    pub fn iv46(&self) -> IV46_R {
        IV46_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IV45"]
    #[inline(always)]
    pub fn iv45(&self) -> IV45_R {
        IV45_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - IV44"]
    #[inline(always)]
    pub fn iv44(&self) -> IV44_R {
        IV44_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IV43"]
    #[inline(always)]
    pub fn iv43(&self) -> IV43_R {
        IV43_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - IV42"]
    #[inline(always)]
    pub fn iv42(&self) -> IV42_R {
        IV42_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - IV41"]
    #[inline(always)]
    pub fn iv41(&self) -> IV41_R {
        IV41_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - IV40"]
    #[inline(always)]
    pub fn iv40(&self) -> IV40_R {
        IV40_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - IV39"]
    #[inline(always)]
    pub fn iv39(&self) -> IV39_R {
        IV39_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - IV38"]
    #[inline(always)]
    pub fn iv38(&self) -> IV38_R {
        IV38_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - IV37"]
    #[inline(always)]
    pub fn iv37(&self) -> IV37_R {
        IV37_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IV36"]
    #[inline(always)]
    pub fn iv36(&self) -> IV36_R {
        IV36_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - IV35"]
    #[inline(always)]
    pub fn iv35(&self) -> IV35_R {
        IV35_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - IV34"]
    #[inline(always)]
    pub fn iv34(&self) -> IV34_R {
        IV34_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - IV33"]
    #[inline(always)]
    pub fn iv33(&self) -> IV33_R {
        IV33_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - IV32"]
    #[inline(always)]
    pub fn iv32(&self) -> IV32_R {
        IV32_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IV63"]
    #[inline(always)]
    pub fn iv63(&mut self) -> IV63_W {
        IV63_W { w: self }
    }
    #[doc = "Bit 1 - IV62"]
    #[inline(always)]
    pub fn iv62(&mut self) -> IV62_W {
        IV62_W { w: self }
    }
    #[doc = "Bit 2 - IV61"]
    #[inline(always)]
    pub fn iv61(&mut self) -> IV61_W {
        IV61_W { w: self }
    }
    #[doc = "Bit 3 - IV60"]
    #[inline(always)]
    pub fn iv60(&mut self) -> IV60_W {
        IV60_W { w: self }
    }
    #[doc = "Bit 4 - IV59"]
    #[inline(always)]
    pub fn iv59(&mut self) -> IV59_W {
        IV59_W { w: self }
    }
    #[doc = "Bit 5 - IV58"]
    #[inline(always)]
    pub fn iv58(&mut self) -> IV58_W {
        IV58_W { w: self }
    }
    #[doc = "Bit 6 - IV57"]
    #[inline(always)]
    pub fn iv57(&mut self) -> IV57_W {
        IV57_W { w: self }
    }
    #[doc = "Bit 7 - IV56"]
    #[inline(always)]
    pub fn iv56(&mut self) -> IV56_W {
        IV56_W { w: self }
    }
    #[doc = "Bit 8 - IV55"]
    #[inline(always)]
    pub fn iv55(&mut self) -> IV55_W {
        IV55_W { w: self }
    }
    #[doc = "Bit 9 - IV54"]
    #[inline(always)]
    pub fn iv54(&mut self) -> IV54_W {
        IV54_W { w: self }
    }
    #[doc = "Bit 10 - IV53"]
    #[inline(always)]
    pub fn iv53(&mut self) -> IV53_W {
        IV53_W { w: self }
    }
    #[doc = "Bit 11 - IV52"]
    #[inline(always)]
    pub fn iv52(&mut self) -> IV52_W {
        IV52_W { w: self }
    }
    #[doc = "Bit 12 - IV51"]
    #[inline(always)]
    pub fn iv51(&mut self) -> IV51_W {
        IV51_W { w: self }
    }
    #[doc = "Bit 13 - IV50"]
    #[inline(always)]
    pub fn iv50(&mut self) -> IV50_W {
        IV50_W { w: self }
    }
    #[doc = "Bit 14 - IV49"]
    #[inline(always)]
    pub fn iv49(&mut self) -> IV49_W {
        IV49_W { w: self }
    }
    #[doc = "Bit 15 - IV48"]
    #[inline(always)]
    pub fn iv48(&mut self) -> IV48_W {
        IV48_W { w: self }
    }
    #[doc = "Bit 16 - IV47"]
    #[inline(always)]
    pub fn iv47(&mut self) -> IV47_W {
        IV47_W { w: self }
    }
    #[doc = "Bit 17 - IV46"]
    #[inline(always)]
    pub fn iv46(&mut self) -> IV46_W {
        IV46_W { w: self }
    }
    #[doc = "Bit 18 - IV45"]
    #[inline(always)]
    pub fn iv45(&mut self) -> IV45_W {
        IV45_W { w: self }
    }
    #[doc = "Bit 19 - IV44"]
    #[inline(always)]
    pub fn iv44(&mut self) -> IV44_W {
        IV44_W { w: self }
    }
    #[doc = "Bit 20 - IV43"]
    #[inline(always)]
    pub fn iv43(&mut self) -> IV43_W {
        IV43_W { w: self }
    }
    #[doc = "Bit 21 - IV42"]
    #[inline(always)]
    pub fn iv42(&mut self) -> IV42_W {
        IV42_W { w: self }
    }
    #[doc = "Bit 22 - IV41"]
    #[inline(always)]
    pub fn iv41(&mut self) -> IV41_W {
        IV41_W { w: self }
    }
    #[doc = "Bit 23 - IV40"]
    #[inline(always)]
    pub fn iv40(&mut self) -> IV40_W {
        IV40_W { w: self }
    }
    #[doc = "Bit 24 - IV39"]
    #[inline(always)]
    pub fn iv39(&mut self) -> IV39_W {
        IV39_W { w: self }
    }
    #[doc = "Bit 25 - IV38"]
    #[inline(always)]
    pub fn iv38(&mut self) -> IV38_W {
        IV38_W { w: self }
    }
    #[doc = "Bit 26 - IV37"]
    #[inline(always)]
    pub fn iv37(&mut self) -> IV37_W {
        IV37_W { w: self }
    }
    #[doc = "Bit 27 - IV36"]
    #[inline(always)]
    pub fn iv36(&mut self) -> IV36_W {
        IV36_W { w: self }
    }
    #[doc = "Bit 28 - IV35"]
    #[inline(always)]
    pub fn iv35(&mut self) -> IV35_W {
        IV35_W { w: self }
    }
    #[doc = "Bit 29 - IV34"]
    #[inline(always)]
    pub fn iv34(&mut self) -> IV34_W {
        IV34_W { w: self }
    }
    #[doc = "Bit 30 - IV33"]
    #[inline(always)]
    pub fn iv33(&mut self) -> IV33_W {
        IV33_W { w: self }
    }
    #[doc = "Bit 31 - IV32"]
    #[inline(always)]
    pub fn iv32(&mut self) -> IV32_W {
        IV32_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "initialization vector registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivrr](index.html) module"]
pub struct IVRR_SPEC;
impl crate::RegisterSpec for IVRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ivrr::R](R) reader structure"]
impl crate::Readable for IVRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ivrr::W](W) writer structure"]
impl crate::Writable for IVRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IVRR to value 0"]
impl crate::Resettable for IVRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
