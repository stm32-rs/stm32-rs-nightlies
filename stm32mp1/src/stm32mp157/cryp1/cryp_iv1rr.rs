#[doc = "Register `CRYP_IV1RR` reader"]
pub struct R(crate::R<CRYP_IV1RR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_IV1RR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_IV1RR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_IV1RR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYP_IV1RR` writer"]
pub struct W(crate::W<CRYP_IV1RR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYP_IV1RR_SPEC>;
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
impl From<crate::W<CRYP_IV1RR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYP_IV1RR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV127` reader - IV127"]
pub struct IV127_R(crate::FieldReader<bool, bool>);
impl IV127_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV127_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV127_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV127` writer - IV127"]
pub struct IV127_W<'a> {
    w: &'a mut W,
}
impl<'a> IV127_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV126` reader - IV126"]
pub struct IV126_R(crate::FieldReader<bool, bool>);
impl IV126_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV126_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV126_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV126` writer - IV126"]
pub struct IV126_W<'a> {
    w: &'a mut W,
}
impl<'a> IV126_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV125` reader - IV125"]
pub struct IV125_R(crate::FieldReader<bool, bool>);
impl IV125_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV125_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV125_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV125` writer - IV125"]
pub struct IV125_W<'a> {
    w: &'a mut W,
}
impl<'a> IV125_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV124` reader - IV124"]
pub struct IV124_R(crate::FieldReader<bool, bool>);
impl IV124_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV124_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV124_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV124` writer - IV124"]
pub struct IV124_W<'a> {
    w: &'a mut W,
}
impl<'a> IV124_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV123` reader - IV123"]
pub struct IV123_R(crate::FieldReader<bool, bool>);
impl IV123_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV123_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV123_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV123` writer - IV123"]
pub struct IV123_W<'a> {
    w: &'a mut W,
}
impl<'a> IV123_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV122` reader - IV122"]
pub struct IV122_R(crate::FieldReader<bool, bool>);
impl IV122_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV122_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV122_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV122` writer - IV122"]
pub struct IV122_W<'a> {
    w: &'a mut W,
}
impl<'a> IV122_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV121` reader - IV121"]
pub struct IV121_R(crate::FieldReader<bool, bool>);
impl IV121_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV121_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV121_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV121` writer - IV121"]
pub struct IV121_W<'a> {
    w: &'a mut W,
}
impl<'a> IV121_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV120` reader - IV120"]
pub struct IV120_R(crate::FieldReader<bool, bool>);
impl IV120_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV120_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV120_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV120` writer - IV120"]
pub struct IV120_W<'a> {
    w: &'a mut W,
}
impl<'a> IV120_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV119` reader - IV119"]
pub struct IV119_R(crate::FieldReader<bool, bool>);
impl IV119_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV119_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV119_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV119` writer - IV119"]
pub struct IV119_W<'a> {
    w: &'a mut W,
}
impl<'a> IV119_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV118` reader - IV118"]
pub struct IV118_R(crate::FieldReader<bool, bool>);
impl IV118_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV118_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV118_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV118` writer - IV118"]
pub struct IV118_W<'a> {
    w: &'a mut W,
}
impl<'a> IV118_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV117` reader - IV117"]
pub struct IV117_R(crate::FieldReader<bool, bool>);
impl IV117_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV117_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV117_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV117` writer - IV117"]
pub struct IV117_W<'a> {
    w: &'a mut W,
}
impl<'a> IV117_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV116` reader - IV116"]
pub struct IV116_R(crate::FieldReader<bool, bool>);
impl IV116_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV116_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV116_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV116` writer - IV116"]
pub struct IV116_W<'a> {
    w: &'a mut W,
}
impl<'a> IV116_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV115` reader - IV115"]
pub struct IV115_R(crate::FieldReader<bool, bool>);
impl IV115_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV115_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV115_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV115` writer - IV115"]
pub struct IV115_W<'a> {
    w: &'a mut W,
}
impl<'a> IV115_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV114` reader - IV114"]
pub struct IV114_R(crate::FieldReader<bool, bool>);
impl IV114_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV114_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV114_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV114` writer - IV114"]
pub struct IV114_W<'a> {
    w: &'a mut W,
}
impl<'a> IV114_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV113` reader - IV113"]
pub struct IV113_R(crate::FieldReader<bool, bool>);
impl IV113_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV113_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV113_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV113` writer - IV113"]
pub struct IV113_W<'a> {
    w: &'a mut W,
}
impl<'a> IV113_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV112` reader - IV112"]
pub struct IV112_R(crate::FieldReader<bool, bool>);
impl IV112_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV112_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV112_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV112` writer - IV112"]
pub struct IV112_W<'a> {
    w: &'a mut W,
}
impl<'a> IV112_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV111` reader - IV111"]
pub struct IV111_R(crate::FieldReader<bool, bool>);
impl IV111_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV111_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV111_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV111` writer - IV111"]
pub struct IV111_W<'a> {
    w: &'a mut W,
}
impl<'a> IV111_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV110` reader - IV110"]
pub struct IV110_R(crate::FieldReader<bool, bool>);
impl IV110_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV110_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV110_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV110` writer - IV110"]
pub struct IV110_W<'a> {
    w: &'a mut W,
}
impl<'a> IV110_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV109` reader - IV109"]
pub struct IV109_R(crate::FieldReader<bool, bool>);
impl IV109_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV109_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV109_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV109` writer - IV109"]
pub struct IV109_W<'a> {
    w: &'a mut W,
}
impl<'a> IV109_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV108` reader - IV108"]
pub struct IV108_R(crate::FieldReader<bool, bool>);
impl IV108_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV108_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV108_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV108` writer - IV108"]
pub struct IV108_W<'a> {
    w: &'a mut W,
}
impl<'a> IV108_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV107` reader - IV107"]
pub struct IV107_R(crate::FieldReader<bool, bool>);
impl IV107_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV107_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV107_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV107` writer - IV107"]
pub struct IV107_W<'a> {
    w: &'a mut W,
}
impl<'a> IV107_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV106` reader - IV106"]
pub struct IV106_R(crate::FieldReader<bool, bool>);
impl IV106_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV106_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV106_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV106` writer - IV106"]
pub struct IV106_W<'a> {
    w: &'a mut W,
}
impl<'a> IV106_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV105` reader - IV105"]
pub struct IV105_R(crate::FieldReader<bool, bool>);
impl IV105_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV105_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV105_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV105` writer - IV105"]
pub struct IV105_W<'a> {
    w: &'a mut W,
}
impl<'a> IV105_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV104` reader - IV104"]
pub struct IV104_R(crate::FieldReader<bool, bool>);
impl IV104_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV104_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV104_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV104` writer - IV104"]
pub struct IV104_W<'a> {
    w: &'a mut W,
}
impl<'a> IV104_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV103` reader - IV103"]
pub struct IV103_R(crate::FieldReader<bool, bool>);
impl IV103_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV103_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV103_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV103` writer - IV103"]
pub struct IV103_W<'a> {
    w: &'a mut W,
}
impl<'a> IV103_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV102` reader - IV102"]
pub struct IV102_R(crate::FieldReader<bool, bool>);
impl IV102_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV102_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV102_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV102` writer - IV102"]
pub struct IV102_W<'a> {
    w: &'a mut W,
}
impl<'a> IV102_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV101` reader - IV101"]
pub struct IV101_R(crate::FieldReader<bool, bool>);
impl IV101_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV101_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV101_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV101` writer - IV101"]
pub struct IV101_W<'a> {
    w: &'a mut W,
}
impl<'a> IV101_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV100` reader - IV100"]
pub struct IV100_R(crate::FieldReader<bool, bool>);
impl IV100_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV100_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV100_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV100` writer - IV100"]
pub struct IV100_W<'a> {
    w: &'a mut W,
}
impl<'a> IV100_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV99` reader - IV99"]
pub struct IV99_R(crate::FieldReader<bool, bool>);
impl IV99_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV99_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV99_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV99` writer - IV99"]
pub struct IV99_W<'a> {
    w: &'a mut W,
}
impl<'a> IV99_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV98` reader - IV98"]
pub struct IV98_R(crate::FieldReader<bool, bool>);
impl IV98_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV98_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV98_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV98` writer - IV98"]
pub struct IV98_W<'a> {
    w: &'a mut W,
}
impl<'a> IV98_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV97` reader - IV97"]
pub struct IV97_R(crate::FieldReader<bool, bool>);
impl IV97_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV97_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV97_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV97` writer - IV97"]
pub struct IV97_W<'a> {
    w: &'a mut W,
}
impl<'a> IV97_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV96` reader - IV96"]
pub struct IV96_R(crate::FieldReader<bool, bool>);
impl IV96_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV96_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV96_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV96` writer - IV96"]
pub struct IV96_W<'a> {
    w: &'a mut W,
}
impl<'a> IV96_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - IV127"]
    #[inline(always)]
    pub fn iv127(&self) -> IV127_R {
        IV127_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IV126"]
    #[inline(always)]
    pub fn iv126(&self) -> IV126_R {
        IV126_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IV125"]
    #[inline(always)]
    pub fn iv125(&self) -> IV125_R {
        IV125_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IV124"]
    #[inline(always)]
    pub fn iv124(&self) -> IV124_R {
        IV124_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IV123"]
    #[inline(always)]
    pub fn iv123(&self) -> IV123_R {
        IV123_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IV122"]
    #[inline(always)]
    pub fn iv122(&self) -> IV122_R {
        IV122_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IV121"]
    #[inline(always)]
    pub fn iv121(&self) -> IV121_R {
        IV121_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IV120"]
    #[inline(always)]
    pub fn iv120(&self) -> IV120_R {
        IV120_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - IV119"]
    #[inline(always)]
    pub fn iv119(&self) -> IV119_R {
        IV119_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - IV118"]
    #[inline(always)]
    pub fn iv118(&self) -> IV118_R {
        IV118_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - IV117"]
    #[inline(always)]
    pub fn iv117(&self) -> IV117_R {
        IV117_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - IV116"]
    #[inline(always)]
    pub fn iv116(&self) -> IV116_R {
        IV116_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IV115"]
    #[inline(always)]
    pub fn iv115(&self) -> IV115_R {
        IV115_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - IV114"]
    #[inline(always)]
    pub fn iv114(&self) -> IV114_R {
        IV114_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - IV113"]
    #[inline(always)]
    pub fn iv113(&self) -> IV113_R {
        IV113_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IV112"]
    #[inline(always)]
    pub fn iv112(&self) -> IV112_R {
        IV112_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - IV111"]
    #[inline(always)]
    pub fn iv111(&self) -> IV111_R {
        IV111_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - IV110"]
    #[inline(always)]
    pub fn iv110(&self) -> IV110_R {
        IV110_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IV109"]
    #[inline(always)]
    pub fn iv109(&self) -> IV109_R {
        IV109_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - IV108"]
    #[inline(always)]
    pub fn iv108(&self) -> IV108_R {
        IV108_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IV107"]
    #[inline(always)]
    pub fn iv107(&self) -> IV107_R {
        IV107_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - IV106"]
    #[inline(always)]
    pub fn iv106(&self) -> IV106_R {
        IV106_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - IV105"]
    #[inline(always)]
    pub fn iv105(&self) -> IV105_R {
        IV105_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - IV104"]
    #[inline(always)]
    pub fn iv104(&self) -> IV104_R {
        IV104_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - IV103"]
    #[inline(always)]
    pub fn iv103(&self) -> IV103_R {
        IV103_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - IV102"]
    #[inline(always)]
    pub fn iv102(&self) -> IV102_R {
        IV102_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - IV101"]
    #[inline(always)]
    pub fn iv101(&self) -> IV101_R {
        IV101_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IV100"]
    #[inline(always)]
    pub fn iv100(&self) -> IV100_R {
        IV100_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - IV99"]
    #[inline(always)]
    pub fn iv99(&self) -> IV99_R {
        IV99_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - IV98"]
    #[inline(always)]
    pub fn iv98(&self) -> IV98_R {
        IV98_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - IV97"]
    #[inline(always)]
    pub fn iv97(&self) -> IV97_R {
        IV97_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - IV96"]
    #[inline(always)]
    pub fn iv96(&self) -> IV96_R {
        IV96_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IV127"]
    #[inline(always)]
    pub fn iv127(&mut self) -> IV127_W {
        IV127_W { w: self }
    }
    #[doc = "Bit 1 - IV126"]
    #[inline(always)]
    pub fn iv126(&mut self) -> IV126_W {
        IV126_W { w: self }
    }
    #[doc = "Bit 2 - IV125"]
    #[inline(always)]
    pub fn iv125(&mut self) -> IV125_W {
        IV125_W { w: self }
    }
    #[doc = "Bit 3 - IV124"]
    #[inline(always)]
    pub fn iv124(&mut self) -> IV124_W {
        IV124_W { w: self }
    }
    #[doc = "Bit 4 - IV123"]
    #[inline(always)]
    pub fn iv123(&mut self) -> IV123_W {
        IV123_W { w: self }
    }
    #[doc = "Bit 5 - IV122"]
    #[inline(always)]
    pub fn iv122(&mut self) -> IV122_W {
        IV122_W { w: self }
    }
    #[doc = "Bit 6 - IV121"]
    #[inline(always)]
    pub fn iv121(&mut self) -> IV121_W {
        IV121_W { w: self }
    }
    #[doc = "Bit 7 - IV120"]
    #[inline(always)]
    pub fn iv120(&mut self) -> IV120_W {
        IV120_W { w: self }
    }
    #[doc = "Bit 8 - IV119"]
    #[inline(always)]
    pub fn iv119(&mut self) -> IV119_W {
        IV119_W { w: self }
    }
    #[doc = "Bit 9 - IV118"]
    #[inline(always)]
    pub fn iv118(&mut self) -> IV118_W {
        IV118_W { w: self }
    }
    #[doc = "Bit 10 - IV117"]
    #[inline(always)]
    pub fn iv117(&mut self) -> IV117_W {
        IV117_W { w: self }
    }
    #[doc = "Bit 11 - IV116"]
    #[inline(always)]
    pub fn iv116(&mut self) -> IV116_W {
        IV116_W { w: self }
    }
    #[doc = "Bit 12 - IV115"]
    #[inline(always)]
    pub fn iv115(&mut self) -> IV115_W {
        IV115_W { w: self }
    }
    #[doc = "Bit 13 - IV114"]
    #[inline(always)]
    pub fn iv114(&mut self) -> IV114_W {
        IV114_W { w: self }
    }
    #[doc = "Bit 14 - IV113"]
    #[inline(always)]
    pub fn iv113(&mut self) -> IV113_W {
        IV113_W { w: self }
    }
    #[doc = "Bit 15 - IV112"]
    #[inline(always)]
    pub fn iv112(&mut self) -> IV112_W {
        IV112_W { w: self }
    }
    #[doc = "Bit 16 - IV111"]
    #[inline(always)]
    pub fn iv111(&mut self) -> IV111_W {
        IV111_W { w: self }
    }
    #[doc = "Bit 17 - IV110"]
    #[inline(always)]
    pub fn iv110(&mut self) -> IV110_W {
        IV110_W { w: self }
    }
    #[doc = "Bit 18 - IV109"]
    #[inline(always)]
    pub fn iv109(&mut self) -> IV109_W {
        IV109_W { w: self }
    }
    #[doc = "Bit 19 - IV108"]
    #[inline(always)]
    pub fn iv108(&mut self) -> IV108_W {
        IV108_W { w: self }
    }
    #[doc = "Bit 20 - IV107"]
    #[inline(always)]
    pub fn iv107(&mut self) -> IV107_W {
        IV107_W { w: self }
    }
    #[doc = "Bit 21 - IV106"]
    #[inline(always)]
    pub fn iv106(&mut self) -> IV106_W {
        IV106_W { w: self }
    }
    #[doc = "Bit 22 - IV105"]
    #[inline(always)]
    pub fn iv105(&mut self) -> IV105_W {
        IV105_W { w: self }
    }
    #[doc = "Bit 23 - IV104"]
    #[inline(always)]
    pub fn iv104(&mut self) -> IV104_W {
        IV104_W { w: self }
    }
    #[doc = "Bit 24 - IV103"]
    #[inline(always)]
    pub fn iv103(&mut self) -> IV103_W {
        IV103_W { w: self }
    }
    #[doc = "Bit 25 - IV102"]
    #[inline(always)]
    pub fn iv102(&mut self) -> IV102_W {
        IV102_W { w: self }
    }
    #[doc = "Bit 26 - IV101"]
    #[inline(always)]
    pub fn iv101(&mut self) -> IV101_W {
        IV101_W { w: self }
    }
    #[doc = "Bit 27 - IV100"]
    #[inline(always)]
    pub fn iv100(&mut self) -> IV100_W {
        IV100_W { w: self }
    }
    #[doc = "Bit 28 - IV99"]
    #[inline(always)]
    pub fn iv99(&mut self) -> IV99_W {
        IV99_W { w: self }
    }
    #[doc = "Bit 29 - IV98"]
    #[inline(always)]
    pub fn iv98(&mut self) -> IV98_W {
        IV98_W { w: self }
    }
    #[doc = "Bit 30 - IV97"]
    #[inline(always)]
    pub fn iv97(&mut self) -> IV97_W {
        IV97_W { w: self }
    }
    #[doc = "Bit 31 - IV96"]
    #[inline(always)]
    pub fn iv96(&mut self) -> IV96_W {
        IV96_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_iv1rr](index.html) module"]
pub struct CRYP_IV1RR_SPEC;
impl crate::RegisterSpec for CRYP_IV1RR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryp_iv1rr::R](R) reader structure"]
impl crate::Readable for CRYP_IV1RR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cryp_iv1rr::W](W) writer structure"]
impl crate::Writable for CRYP_IV1RR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYP_IV1RR to value 0"]
impl crate::Resettable for CRYP_IV1RR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
