#[doc = "Register `CRYP_IV1LR` reader"]
pub struct R(crate::R<CRYP_IV1LR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_IV1LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_IV1LR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_IV1LR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYP_IV1LR` writer"]
pub struct W(crate::W<CRYP_IV1LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYP_IV1LR_SPEC>;
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
impl From<crate::W<CRYP_IV1LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYP_IV1LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV95` reader - IV95"]
pub struct IV95_R(crate::FieldReader<bool, bool>);
impl IV95_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV95_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV95_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV95` writer - IV95"]
pub struct IV95_W<'a> {
    w: &'a mut W,
}
impl<'a> IV95_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV94` reader - IV94"]
pub struct IV94_R(crate::FieldReader<bool, bool>);
impl IV94_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV94_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV94_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV94` writer - IV94"]
pub struct IV94_W<'a> {
    w: &'a mut W,
}
impl<'a> IV94_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV93` reader - IV93"]
pub struct IV93_R(crate::FieldReader<bool, bool>);
impl IV93_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV93_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV93_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV93` writer - IV93"]
pub struct IV93_W<'a> {
    w: &'a mut W,
}
impl<'a> IV93_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV92` reader - IV92"]
pub struct IV92_R(crate::FieldReader<bool, bool>);
impl IV92_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV92_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV92_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV92` writer - IV92"]
pub struct IV92_W<'a> {
    w: &'a mut W,
}
impl<'a> IV92_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV91` reader - IV91"]
pub struct IV91_R(crate::FieldReader<bool, bool>);
impl IV91_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV91_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV91_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV91` writer - IV91"]
pub struct IV91_W<'a> {
    w: &'a mut W,
}
impl<'a> IV91_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV90` reader - IV90"]
pub struct IV90_R(crate::FieldReader<bool, bool>);
impl IV90_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV90_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV90_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV90` writer - IV90"]
pub struct IV90_W<'a> {
    w: &'a mut W,
}
impl<'a> IV90_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV89` reader - IV89"]
pub struct IV89_R(crate::FieldReader<bool, bool>);
impl IV89_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV89_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV89_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV89` writer - IV89"]
pub struct IV89_W<'a> {
    w: &'a mut W,
}
impl<'a> IV89_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV88` reader - IV88"]
pub struct IV88_R(crate::FieldReader<bool, bool>);
impl IV88_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV88_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV88_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV88` writer - IV88"]
pub struct IV88_W<'a> {
    w: &'a mut W,
}
impl<'a> IV88_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV87` reader - IV87"]
pub struct IV87_R(crate::FieldReader<bool, bool>);
impl IV87_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV87_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV87_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV87` writer - IV87"]
pub struct IV87_W<'a> {
    w: &'a mut W,
}
impl<'a> IV87_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV86` reader - IV86"]
pub struct IV86_R(crate::FieldReader<bool, bool>);
impl IV86_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV86_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV86_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV86` writer - IV86"]
pub struct IV86_W<'a> {
    w: &'a mut W,
}
impl<'a> IV86_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV85` reader - IV85"]
pub struct IV85_R(crate::FieldReader<bool, bool>);
impl IV85_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV85_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV85_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV85` writer - IV85"]
pub struct IV85_W<'a> {
    w: &'a mut W,
}
impl<'a> IV85_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV84` reader - IV84"]
pub struct IV84_R(crate::FieldReader<bool, bool>);
impl IV84_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV84_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV84_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV84` writer - IV84"]
pub struct IV84_W<'a> {
    w: &'a mut W,
}
impl<'a> IV84_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV83` reader - IV83"]
pub struct IV83_R(crate::FieldReader<bool, bool>);
impl IV83_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV83_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV83_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV83` writer - IV83"]
pub struct IV83_W<'a> {
    w: &'a mut W,
}
impl<'a> IV83_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV82` reader - IV82"]
pub struct IV82_R(crate::FieldReader<bool, bool>);
impl IV82_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV82_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV82_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV82` writer - IV82"]
pub struct IV82_W<'a> {
    w: &'a mut W,
}
impl<'a> IV82_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV81` reader - IV81"]
pub struct IV81_R(crate::FieldReader<bool, bool>);
impl IV81_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV81_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV81_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV81` writer - IV81"]
pub struct IV81_W<'a> {
    w: &'a mut W,
}
impl<'a> IV81_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV80` reader - IV80"]
pub struct IV80_R(crate::FieldReader<bool, bool>);
impl IV80_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV80_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV80_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV80` writer - IV80"]
pub struct IV80_W<'a> {
    w: &'a mut W,
}
impl<'a> IV80_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV79` reader - IV79"]
pub struct IV79_R(crate::FieldReader<bool, bool>);
impl IV79_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV79_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV79_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV79` writer - IV79"]
pub struct IV79_W<'a> {
    w: &'a mut W,
}
impl<'a> IV79_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV78` reader - IV78"]
pub struct IV78_R(crate::FieldReader<bool, bool>);
impl IV78_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV78_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV78_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV78` writer - IV78"]
pub struct IV78_W<'a> {
    w: &'a mut W,
}
impl<'a> IV78_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV77` reader - IV77"]
pub struct IV77_R(crate::FieldReader<bool, bool>);
impl IV77_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV77_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV77_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV77` writer - IV77"]
pub struct IV77_W<'a> {
    w: &'a mut W,
}
impl<'a> IV77_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV76` reader - IV76"]
pub struct IV76_R(crate::FieldReader<bool, bool>);
impl IV76_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV76_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV76_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV76` writer - IV76"]
pub struct IV76_W<'a> {
    w: &'a mut W,
}
impl<'a> IV76_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV75` reader - IV75"]
pub struct IV75_R(crate::FieldReader<bool, bool>);
impl IV75_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV75_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV75_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV75` writer - IV75"]
pub struct IV75_W<'a> {
    w: &'a mut W,
}
impl<'a> IV75_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV74` reader - IV74"]
pub struct IV74_R(crate::FieldReader<bool, bool>);
impl IV74_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV74_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV74_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV74` writer - IV74"]
pub struct IV74_W<'a> {
    w: &'a mut W,
}
impl<'a> IV74_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV73` reader - IV73"]
pub struct IV73_R(crate::FieldReader<bool, bool>);
impl IV73_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV73_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV73_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV73` writer - IV73"]
pub struct IV73_W<'a> {
    w: &'a mut W,
}
impl<'a> IV73_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV72` reader - IV72"]
pub struct IV72_R(crate::FieldReader<bool, bool>);
impl IV72_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV72_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV72_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV72` writer - IV72"]
pub struct IV72_W<'a> {
    w: &'a mut W,
}
impl<'a> IV72_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV71` reader - IV71"]
pub struct IV71_R(crate::FieldReader<bool, bool>);
impl IV71_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV71_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV71_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV71` writer - IV71"]
pub struct IV71_W<'a> {
    w: &'a mut W,
}
impl<'a> IV71_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV70` reader - IV70"]
pub struct IV70_R(crate::FieldReader<bool, bool>);
impl IV70_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV70_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV70_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV70` writer - IV70"]
pub struct IV70_W<'a> {
    w: &'a mut W,
}
impl<'a> IV70_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV69` reader - IV69"]
pub struct IV69_R(crate::FieldReader<bool, bool>);
impl IV69_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV69_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV69_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV69` writer - IV69"]
pub struct IV69_W<'a> {
    w: &'a mut W,
}
impl<'a> IV69_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV68` reader - IV68"]
pub struct IV68_R(crate::FieldReader<bool, bool>);
impl IV68_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV68_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV68_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV68` writer - IV68"]
pub struct IV68_W<'a> {
    w: &'a mut W,
}
impl<'a> IV68_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV67` reader - IV67"]
pub struct IV67_R(crate::FieldReader<bool, bool>);
impl IV67_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV67_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV67_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV67` writer - IV67"]
pub struct IV67_W<'a> {
    w: &'a mut W,
}
impl<'a> IV67_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV66` reader - IV66"]
pub struct IV66_R(crate::FieldReader<bool, bool>);
impl IV66_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV66_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV66_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV66` writer - IV66"]
pub struct IV66_W<'a> {
    w: &'a mut W,
}
impl<'a> IV66_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV65` reader - IV65"]
pub struct IV65_R(crate::FieldReader<bool, bool>);
impl IV65_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV65_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV65_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV65` writer - IV65"]
pub struct IV65_W<'a> {
    w: &'a mut W,
}
impl<'a> IV65_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IV64` reader - IV64"]
pub struct IV64_R(crate::FieldReader<bool, bool>);
impl IV64_R {
    pub(crate) fn new(bits: bool) -> Self {
        IV64_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV64_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IV64` writer - IV64"]
pub struct IV64_W<'a> {
    w: &'a mut W,
}
impl<'a> IV64_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - IV95"]
    #[inline(always)]
    pub fn iv95(&self) -> IV95_R {
        IV95_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IV94"]
    #[inline(always)]
    pub fn iv94(&self) -> IV94_R {
        IV94_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IV93"]
    #[inline(always)]
    pub fn iv93(&self) -> IV93_R {
        IV93_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IV92"]
    #[inline(always)]
    pub fn iv92(&self) -> IV92_R {
        IV92_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IV91"]
    #[inline(always)]
    pub fn iv91(&self) -> IV91_R {
        IV91_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IV90"]
    #[inline(always)]
    pub fn iv90(&self) -> IV90_R {
        IV90_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IV89"]
    #[inline(always)]
    pub fn iv89(&self) -> IV89_R {
        IV89_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IV88"]
    #[inline(always)]
    pub fn iv88(&self) -> IV88_R {
        IV88_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - IV87"]
    #[inline(always)]
    pub fn iv87(&self) -> IV87_R {
        IV87_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - IV86"]
    #[inline(always)]
    pub fn iv86(&self) -> IV86_R {
        IV86_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - IV85"]
    #[inline(always)]
    pub fn iv85(&self) -> IV85_R {
        IV85_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - IV84"]
    #[inline(always)]
    pub fn iv84(&self) -> IV84_R {
        IV84_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IV83"]
    #[inline(always)]
    pub fn iv83(&self) -> IV83_R {
        IV83_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - IV82"]
    #[inline(always)]
    pub fn iv82(&self) -> IV82_R {
        IV82_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - IV81"]
    #[inline(always)]
    pub fn iv81(&self) -> IV81_R {
        IV81_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IV80"]
    #[inline(always)]
    pub fn iv80(&self) -> IV80_R {
        IV80_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - IV79"]
    #[inline(always)]
    pub fn iv79(&self) -> IV79_R {
        IV79_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - IV78"]
    #[inline(always)]
    pub fn iv78(&self) -> IV78_R {
        IV78_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IV77"]
    #[inline(always)]
    pub fn iv77(&self) -> IV77_R {
        IV77_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - IV76"]
    #[inline(always)]
    pub fn iv76(&self) -> IV76_R {
        IV76_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IV75"]
    #[inline(always)]
    pub fn iv75(&self) -> IV75_R {
        IV75_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - IV74"]
    #[inline(always)]
    pub fn iv74(&self) -> IV74_R {
        IV74_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - IV73"]
    #[inline(always)]
    pub fn iv73(&self) -> IV73_R {
        IV73_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - IV72"]
    #[inline(always)]
    pub fn iv72(&self) -> IV72_R {
        IV72_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - IV71"]
    #[inline(always)]
    pub fn iv71(&self) -> IV71_R {
        IV71_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - IV70"]
    #[inline(always)]
    pub fn iv70(&self) -> IV70_R {
        IV70_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - IV69"]
    #[inline(always)]
    pub fn iv69(&self) -> IV69_R {
        IV69_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IV68"]
    #[inline(always)]
    pub fn iv68(&self) -> IV68_R {
        IV68_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - IV67"]
    #[inline(always)]
    pub fn iv67(&self) -> IV67_R {
        IV67_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - IV66"]
    #[inline(always)]
    pub fn iv66(&self) -> IV66_R {
        IV66_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - IV65"]
    #[inline(always)]
    pub fn iv65(&self) -> IV65_R {
        IV65_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - IV64"]
    #[inline(always)]
    pub fn iv64(&self) -> IV64_R {
        IV64_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IV95"]
    #[inline(always)]
    pub fn iv95(&mut self) -> IV95_W {
        IV95_W { w: self }
    }
    #[doc = "Bit 1 - IV94"]
    #[inline(always)]
    pub fn iv94(&mut self) -> IV94_W {
        IV94_W { w: self }
    }
    #[doc = "Bit 2 - IV93"]
    #[inline(always)]
    pub fn iv93(&mut self) -> IV93_W {
        IV93_W { w: self }
    }
    #[doc = "Bit 3 - IV92"]
    #[inline(always)]
    pub fn iv92(&mut self) -> IV92_W {
        IV92_W { w: self }
    }
    #[doc = "Bit 4 - IV91"]
    #[inline(always)]
    pub fn iv91(&mut self) -> IV91_W {
        IV91_W { w: self }
    }
    #[doc = "Bit 5 - IV90"]
    #[inline(always)]
    pub fn iv90(&mut self) -> IV90_W {
        IV90_W { w: self }
    }
    #[doc = "Bit 6 - IV89"]
    #[inline(always)]
    pub fn iv89(&mut self) -> IV89_W {
        IV89_W { w: self }
    }
    #[doc = "Bit 7 - IV88"]
    #[inline(always)]
    pub fn iv88(&mut self) -> IV88_W {
        IV88_W { w: self }
    }
    #[doc = "Bit 8 - IV87"]
    #[inline(always)]
    pub fn iv87(&mut self) -> IV87_W {
        IV87_W { w: self }
    }
    #[doc = "Bit 9 - IV86"]
    #[inline(always)]
    pub fn iv86(&mut self) -> IV86_W {
        IV86_W { w: self }
    }
    #[doc = "Bit 10 - IV85"]
    #[inline(always)]
    pub fn iv85(&mut self) -> IV85_W {
        IV85_W { w: self }
    }
    #[doc = "Bit 11 - IV84"]
    #[inline(always)]
    pub fn iv84(&mut self) -> IV84_W {
        IV84_W { w: self }
    }
    #[doc = "Bit 12 - IV83"]
    #[inline(always)]
    pub fn iv83(&mut self) -> IV83_W {
        IV83_W { w: self }
    }
    #[doc = "Bit 13 - IV82"]
    #[inline(always)]
    pub fn iv82(&mut self) -> IV82_W {
        IV82_W { w: self }
    }
    #[doc = "Bit 14 - IV81"]
    #[inline(always)]
    pub fn iv81(&mut self) -> IV81_W {
        IV81_W { w: self }
    }
    #[doc = "Bit 15 - IV80"]
    #[inline(always)]
    pub fn iv80(&mut self) -> IV80_W {
        IV80_W { w: self }
    }
    #[doc = "Bit 16 - IV79"]
    #[inline(always)]
    pub fn iv79(&mut self) -> IV79_W {
        IV79_W { w: self }
    }
    #[doc = "Bit 17 - IV78"]
    #[inline(always)]
    pub fn iv78(&mut self) -> IV78_W {
        IV78_W { w: self }
    }
    #[doc = "Bit 18 - IV77"]
    #[inline(always)]
    pub fn iv77(&mut self) -> IV77_W {
        IV77_W { w: self }
    }
    #[doc = "Bit 19 - IV76"]
    #[inline(always)]
    pub fn iv76(&mut self) -> IV76_W {
        IV76_W { w: self }
    }
    #[doc = "Bit 20 - IV75"]
    #[inline(always)]
    pub fn iv75(&mut self) -> IV75_W {
        IV75_W { w: self }
    }
    #[doc = "Bit 21 - IV74"]
    #[inline(always)]
    pub fn iv74(&mut self) -> IV74_W {
        IV74_W { w: self }
    }
    #[doc = "Bit 22 - IV73"]
    #[inline(always)]
    pub fn iv73(&mut self) -> IV73_W {
        IV73_W { w: self }
    }
    #[doc = "Bit 23 - IV72"]
    #[inline(always)]
    pub fn iv72(&mut self) -> IV72_W {
        IV72_W { w: self }
    }
    #[doc = "Bit 24 - IV71"]
    #[inline(always)]
    pub fn iv71(&mut self) -> IV71_W {
        IV71_W { w: self }
    }
    #[doc = "Bit 25 - IV70"]
    #[inline(always)]
    pub fn iv70(&mut self) -> IV70_W {
        IV70_W { w: self }
    }
    #[doc = "Bit 26 - IV69"]
    #[inline(always)]
    pub fn iv69(&mut self) -> IV69_W {
        IV69_W { w: self }
    }
    #[doc = "Bit 27 - IV68"]
    #[inline(always)]
    pub fn iv68(&mut self) -> IV68_W {
        IV68_W { w: self }
    }
    #[doc = "Bit 28 - IV67"]
    #[inline(always)]
    pub fn iv67(&mut self) -> IV67_W {
        IV67_W { w: self }
    }
    #[doc = "Bit 29 - IV66"]
    #[inline(always)]
    pub fn iv66(&mut self) -> IV66_W {
        IV66_W { w: self }
    }
    #[doc = "Bit 30 - IV65"]
    #[inline(always)]
    pub fn iv65(&mut self) -> IV65_W {
        IV65_W { w: self }
    }
    #[doc = "Bit 31 - IV64"]
    #[inline(always)]
    pub fn iv64(&mut self) -> IV64_W {
        IV64_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_iv1lr](index.html) module"]
pub struct CRYP_IV1LR_SPEC;
impl crate::RegisterSpec for CRYP_IV1LR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryp_iv1lr::R](R) reader structure"]
impl crate::Readable for CRYP_IV1LR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cryp_iv1lr::W](W) writer structure"]
impl crate::Writable for CRYP_IV1LR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYP_IV1LR to value 0"]
impl crate::Resettable for CRYP_IV1LR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
