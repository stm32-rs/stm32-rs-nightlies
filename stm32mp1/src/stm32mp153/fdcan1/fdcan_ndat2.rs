#[doc = "Register `FDCAN_NDAT2` reader"]
pub struct R(crate::R<FDCAN_NDAT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_NDAT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_NDAT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_NDAT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_NDAT2` writer"]
pub struct W(crate::W<FDCAN_NDAT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_NDAT2_SPEC>;
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
impl From<crate::W<FDCAN_NDAT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_NDAT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ND32` reader - ND32"]
pub struct ND32_R(crate::FieldReader<bool, bool>);
impl ND32_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND32` writer - ND32"]
pub struct ND32_W<'a> {
    w: &'a mut W,
}
impl<'a> ND32_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND33` reader - ND33"]
pub struct ND33_R(crate::FieldReader<bool, bool>);
impl ND33_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND33_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND33` writer - ND33"]
pub struct ND33_W<'a> {
    w: &'a mut W,
}
impl<'a> ND33_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND34` reader - ND34"]
pub struct ND34_R(crate::FieldReader<bool, bool>);
impl ND34_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND34_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND34_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND34` writer - ND34"]
pub struct ND34_W<'a> {
    w: &'a mut W,
}
impl<'a> ND34_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND35` reader - ND35"]
pub struct ND35_R(crate::FieldReader<bool, bool>);
impl ND35_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND35_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND35_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND35` writer - ND35"]
pub struct ND35_W<'a> {
    w: &'a mut W,
}
impl<'a> ND35_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND36` reader - ND36"]
pub struct ND36_R(crate::FieldReader<bool, bool>);
impl ND36_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND36_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND36_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND36` writer - ND36"]
pub struct ND36_W<'a> {
    w: &'a mut W,
}
impl<'a> ND36_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND37` reader - ND37"]
pub struct ND37_R(crate::FieldReader<bool, bool>);
impl ND37_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND37_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND37_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND37` writer - ND37"]
pub struct ND37_W<'a> {
    w: &'a mut W,
}
impl<'a> ND37_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND38` reader - ND38"]
pub struct ND38_R(crate::FieldReader<bool, bool>);
impl ND38_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND38_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND38_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND38` writer - ND38"]
pub struct ND38_W<'a> {
    w: &'a mut W,
}
impl<'a> ND38_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND39` reader - ND39"]
pub struct ND39_R(crate::FieldReader<bool, bool>);
impl ND39_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND39_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND39_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND39` writer - ND39"]
pub struct ND39_W<'a> {
    w: &'a mut W,
}
impl<'a> ND39_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND40` reader - ND40"]
pub struct ND40_R(crate::FieldReader<bool, bool>);
impl ND40_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND40_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND40_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND40` writer - ND40"]
pub struct ND40_W<'a> {
    w: &'a mut W,
}
impl<'a> ND40_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND41` reader - ND41"]
pub struct ND41_R(crate::FieldReader<bool, bool>);
impl ND41_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND41_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND41_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND41` writer - ND41"]
pub struct ND41_W<'a> {
    w: &'a mut W,
}
impl<'a> ND41_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND42` reader - ND42"]
pub struct ND42_R(crate::FieldReader<bool, bool>);
impl ND42_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND42_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND42_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND42` writer - ND42"]
pub struct ND42_W<'a> {
    w: &'a mut W,
}
impl<'a> ND42_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND43` reader - ND43"]
pub struct ND43_R(crate::FieldReader<bool, bool>);
impl ND43_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND43_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND43_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND43` writer - ND43"]
pub struct ND43_W<'a> {
    w: &'a mut W,
}
impl<'a> ND43_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND44` reader - ND44"]
pub struct ND44_R(crate::FieldReader<bool, bool>);
impl ND44_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND44_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND44_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND44` writer - ND44"]
pub struct ND44_W<'a> {
    w: &'a mut W,
}
impl<'a> ND44_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND45` reader - ND45"]
pub struct ND45_R(crate::FieldReader<bool, bool>);
impl ND45_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND45_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND45_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND45` writer - ND45"]
pub struct ND45_W<'a> {
    w: &'a mut W,
}
impl<'a> ND45_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND46` reader - ND46"]
pub struct ND46_R(crate::FieldReader<bool, bool>);
impl ND46_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND46_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND46_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND46` writer - ND46"]
pub struct ND46_W<'a> {
    w: &'a mut W,
}
impl<'a> ND46_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND47` reader - ND47"]
pub struct ND47_R(crate::FieldReader<bool, bool>);
impl ND47_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND47_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND47_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND47` writer - ND47"]
pub struct ND47_W<'a> {
    w: &'a mut W,
}
impl<'a> ND47_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND48` reader - ND48"]
pub struct ND48_R(crate::FieldReader<bool, bool>);
impl ND48_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND48_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND48_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND48` writer - ND48"]
pub struct ND48_W<'a> {
    w: &'a mut W,
}
impl<'a> ND48_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND49` reader - ND49"]
pub struct ND49_R(crate::FieldReader<bool, bool>);
impl ND49_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND49_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND49_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND49` writer - ND49"]
pub struct ND49_W<'a> {
    w: &'a mut W,
}
impl<'a> ND49_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND50` reader - ND50"]
pub struct ND50_R(crate::FieldReader<bool, bool>);
impl ND50_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND50_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND50_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND50` writer - ND50"]
pub struct ND50_W<'a> {
    w: &'a mut W,
}
impl<'a> ND50_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND51` reader - ND51"]
pub struct ND51_R(crate::FieldReader<bool, bool>);
impl ND51_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND51_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND51_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND51` writer - ND51"]
pub struct ND51_W<'a> {
    w: &'a mut W,
}
impl<'a> ND51_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND52` reader - ND52"]
pub struct ND52_R(crate::FieldReader<bool, bool>);
impl ND52_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND52_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND52_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND52` writer - ND52"]
pub struct ND52_W<'a> {
    w: &'a mut W,
}
impl<'a> ND52_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND53` reader - ND53"]
pub struct ND53_R(crate::FieldReader<bool, bool>);
impl ND53_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND53_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND53_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND53` writer - ND53"]
pub struct ND53_W<'a> {
    w: &'a mut W,
}
impl<'a> ND53_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND54` reader - ND54"]
pub struct ND54_R(crate::FieldReader<bool, bool>);
impl ND54_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND54_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND54_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND54` writer - ND54"]
pub struct ND54_W<'a> {
    w: &'a mut W,
}
impl<'a> ND54_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND55` reader - ND55"]
pub struct ND55_R(crate::FieldReader<bool, bool>);
impl ND55_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND55_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND55_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND55` writer - ND55"]
pub struct ND55_W<'a> {
    w: &'a mut W,
}
impl<'a> ND55_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND56` reader - ND56"]
pub struct ND56_R(crate::FieldReader<bool, bool>);
impl ND56_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND56_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND56_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND56` writer - ND56"]
pub struct ND56_W<'a> {
    w: &'a mut W,
}
impl<'a> ND56_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND57` reader - ND57"]
pub struct ND57_R(crate::FieldReader<bool, bool>);
impl ND57_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND57_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND57_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND57` writer - ND57"]
pub struct ND57_W<'a> {
    w: &'a mut W,
}
impl<'a> ND57_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND58` reader - ND58"]
pub struct ND58_R(crate::FieldReader<bool, bool>);
impl ND58_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND58_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND58_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND58` writer - ND58"]
pub struct ND58_W<'a> {
    w: &'a mut W,
}
impl<'a> ND58_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND59` reader - ND59"]
pub struct ND59_R(crate::FieldReader<bool, bool>);
impl ND59_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND59_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND59_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND59` writer - ND59"]
pub struct ND59_W<'a> {
    w: &'a mut W,
}
impl<'a> ND59_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND60` reader - ND60"]
pub struct ND60_R(crate::FieldReader<bool, bool>);
impl ND60_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND60_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND60_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND60` writer - ND60"]
pub struct ND60_W<'a> {
    w: &'a mut W,
}
impl<'a> ND60_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND61` reader - ND61"]
pub struct ND61_R(crate::FieldReader<bool, bool>);
impl ND61_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND61_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND61_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND61` writer - ND61"]
pub struct ND61_W<'a> {
    w: &'a mut W,
}
impl<'a> ND61_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND62` reader - ND62"]
pub struct ND62_R(crate::FieldReader<bool, bool>);
impl ND62_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND62_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND62_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND62` writer - ND62"]
pub struct ND62_W<'a> {
    w: &'a mut W,
}
impl<'a> ND62_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ND63` reader - ND63"]
pub struct ND63_R(crate::FieldReader<bool, bool>);
impl ND63_R {
    pub(crate) fn new(bits: bool) -> Self {
        ND63_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ND63_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ND63` writer - ND63"]
pub struct ND63_W<'a> {
    w: &'a mut W,
}
impl<'a> ND63_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - ND32"]
    #[inline(always)]
    pub fn nd32(&self) -> ND32_R {
        ND32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ND33"]
    #[inline(always)]
    pub fn nd33(&self) -> ND33_R {
        ND33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ND34"]
    #[inline(always)]
    pub fn nd34(&self) -> ND34_R {
        ND34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ND35"]
    #[inline(always)]
    pub fn nd35(&self) -> ND35_R {
        ND35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ND36"]
    #[inline(always)]
    pub fn nd36(&self) -> ND36_R {
        ND36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ND37"]
    #[inline(always)]
    pub fn nd37(&self) -> ND37_R {
        ND37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ND38"]
    #[inline(always)]
    pub fn nd38(&self) -> ND38_R {
        ND38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ND39"]
    #[inline(always)]
    pub fn nd39(&self) -> ND39_R {
        ND39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ND40"]
    #[inline(always)]
    pub fn nd40(&self) -> ND40_R {
        ND40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ND41"]
    #[inline(always)]
    pub fn nd41(&self) -> ND41_R {
        ND41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ND42"]
    #[inline(always)]
    pub fn nd42(&self) -> ND42_R {
        ND42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ND43"]
    #[inline(always)]
    pub fn nd43(&self) -> ND43_R {
        ND43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ND44"]
    #[inline(always)]
    pub fn nd44(&self) -> ND44_R {
        ND44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ND45"]
    #[inline(always)]
    pub fn nd45(&self) -> ND45_R {
        ND45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ND46"]
    #[inline(always)]
    pub fn nd46(&self) -> ND46_R {
        ND46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ND47"]
    #[inline(always)]
    pub fn nd47(&self) -> ND47_R {
        ND47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ND48"]
    #[inline(always)]
    pub fn nd48(&self) -> ND48_R {
        ND48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ND49"]
    #[inline(always)]
    pub fn nd49(&self) -> ND49_R {
        ND49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ND50"]
    #[inline(always)]
    pub fn nd50(&self) -> ND50_R {
        ND50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ND51"]
    #[inline(always)]
    pub fn nd51(&self) -> ND51_R {
        ND51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ND52"]
    #[inline(always)]
    pub fn nd52(&self) -> ND52_R {
        ND52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ND53"]
    #[inline(always)]
    pub fn nd53(&self) -> ND53_R {
        ND53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ND54"]
    #[inline(always)]
    pub fn nd54(&self) -> ND54_R {
        ND54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ND55"]
    #[inline(always)]
    pub fn nd55(&self) -> ND55_R {
        ND55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ND56"]
    #[inline(always)]
    pub fn nd56(&self) -> ND56_R {
        ND56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ND57"]
    #[inline(always)]
    pub fn nd57(&self) -> ND57_R {
        ND57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - ND58"]
    #[inline(always)]
    pub fn nd58(&self) -> ND58_R {
        ND58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - ND59"]
    #[inline(always)]
    pub fn nd59(&self) -> ND59_R {
        ND59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - ND60"]
    #[inline(always)]
    pub fn nd60(&self) -> ND60_R {
        ND60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ND61"]
    #[inline(always)]
    pub fn nd61(&self) -> ND61_R {
        ND61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ND62"]
    #[inline(always)]
    pub fn nd62(&self) -> ND62_R {
        ND62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ND63"]
    #[inline(always)]
    pub fn nd63(&self) -> ND63_R {
        ND63_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ND32"]
    #[inline(always)]
    pub fn nd32(&mut self) -> ND32_W {
        ND32_W { w: self }
    }
    #[doc = "Bit 1 - ND33"]
    #[inline(always)]
    pub fn nd33(&mut self) -> ND33_W {
        ND33_W { w: self }
    }
    #[doc = "Bit 2 - ND34"]
    #[inline(always)]
    pub fn nd34(&mut self) -> ND34_W {
        ND34_W { w: self }
    }
    #[doc = "Bit 3 - ND35"]
    #[inline(always)]
    pub fn nd35(&mut self) -> ND35_W {
        ND35_W { w: self }
    }
    #[doc = "Bit 4 - ND36"]
    #[inline(always)]
    pub fn nd36(&mut self) -> ND36_W {
        ND36_W { w: self }
    }
    #[doc = "Bit 5 - ND37"]
    #[inline(always)]
    pub fn nd37(&mut self) -> ND37_W {
        ND37_W { w: self }
    }
    #[doc = "Bit 6 - ND38"]
    #[inline(always)]
    pub fn nd38(&mut self) -> ND38_W {
        ND38_W { w: self }
    }
    #[doc = "Bit 7 - ND39"]
    #[inline(always)]
    pub fn nd39(&mut self) -> ND39_W {
        ND39_W { w: self }
    }
    #[doc = "Bit 8 - ND40"]
    #[inline(always)]
    pub fn nd40(&mut self) -> ND40_W {
        ND40_W { w: self }
    }
    #[doc = "Bit 9 - ND41"]
    #[inline(always)]
    pub fn nd41(&mut self) -> ND41_W {
        ND41_W { w: self }
    }
    #[doc = "Bit 10 - ND42"]
    #[inline(always)]
    pub fn nd42(&mut self) -> ND42_W {
        ND42_W { w: self }
    }
    #[doc = "Bit 11 - ND43"]
    #[inline(always)]
    pub fn nd43(&mut self) -> ND43_W {
        ND43_W { w: self }
    }
    #[doc = "Bit 12 - ND44"]
    #[inline(always)]
    pub fn nd44(&mut self) -> ND44_W {
        ND44_W { w: self }
    }
    #[doc = "Bit 13 - ND45"]
    #[inline(always)]
    pub fn nd45(&mut self) -> ND45_W {
        ND45_W { w: self }
    }
    #[doc = "Bit 14 - ND46"]
    #[inline(always)]
    pub fn nd46(&mut self) -> ND46_W {
        ND46_W { w: self }
    }
    #[doc = "Bit 15 - ND47"]
    #[inline(always)]
    pub fn nd47(&mut self) -> ND47_W {
        ND47_W { w: self }
    }
    #[doc = "Bit 16 - ND48"]
    #[inline(always)]
    pub fn nd48(&mut self) -> ND48_W {
        ND48_W { w: self }
    }
    #[doc = "Bit 17 - ND49"]
    #[inline(always)]
    pub fn nd49(&mut self) -> ND49_W {
        ND49_W { w: self }
    }
    #[doc = "Bit 18 - ND50"]
    #[inline(always)]
    pub fn nd50(&mut self) -> ND50_W {
        ND50_W { w: self }
    }
    #[doc = "Bit 19 - ND51"]
    #[inline(always)]
    pub fn nd51(&mut self) -> ND51_W {
        ND51_W { w: self }
    }
    #[doc = "Bit 20 - ND52"]
    #[inline(always)]
    pub fn nd52(&mut self) -> ND52_W {
        ND52_W { w: self }
    }
    #[doc = "Bit 21 - ND53"]
    #[inline(always)]
    pub fn nd53(&mut self) -> ND53_W {
        ND53_W { w: self }
    }
    #[doc = "Bit 22 - ND54"]
    #[inline(always)]
    pub fn nd54(&mut self) -> ND54_W {
        ND54_W { w: self }
    }
    #[doc = "Bit 23 - ND55"]
    #[inline(always)]
    pub fn nd55(&mut self) -> ND55_W {
        ND55_W { w: self }
    }
    #[doc = "Bit 24 - ND56"]
    #[inline(always)]
    pub fn nd56(&mut self) -> ND56_W {
        ND56_W { w: self }
    }
    #[doc = "Bit 25 - ND57"]
    #[inline(always)]
    pub fn nd57(&mut self) -> ND57_W {
        ND57_W { w: self }
    }
    #[doc = "Bit 26 - ND58"]
    #[inline(always)]
    pub fn nd58(&mut self) -> ND58_W {
        ND58_W { w: self }
    }
    #[doc = "Bit 27 - ND59"]
    #[inline(always)]
    pub fn nd59(&mut self) -> ND59_W {
        ND59_W { w: self }
    }
    #[doc = "Bit 28 - ND60"]
    #[inline(always)]
    pub fn nd60(&mut self) -> ND60_W {
        ND60_W { w: self }
    }
    #[doc = "Bit 29 - ND61"]
    #[inline(always)]
    pub fn nd61(&mut self) -> ND61_W {
        ND61_W { w: self }
    }
    #[doc = "Bit 30 - ND62"]
    #[inline(always)]
    pub fn nd62(&mut self) -> ND62_W {
        ND62_W { w: self }
    }
    #[doc = "Bit 31 - ND63"]
    #[inline(always)]
    pub fn nd63(&mut self) -> ND63_W {
        ND63_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN new data 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ndat2](index.html) module"]
pub struct FDCAN_NDAT2_SPEC;
impl crate::RegisterSpec for FDCAN_NDAT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ndat2::R](R) reader structure"]
impl crate::Readable for FDCAN_NDAT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ndat2::W](W) writer structure"]
impl crate::Writable for FDCAN_NDAT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_NDAT2 to value 0"]
impl crate::Resettable for FDCAN_NDAT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
