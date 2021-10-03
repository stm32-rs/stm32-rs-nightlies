#[doc = "Register `DTS_OR` reader"]
pub struct R(crate::R<DTS_OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTS_OR` writer"]
pub struct W(crate::W<DTS_OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTS_OR_SPEC>;
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
impl From<crate::W<DTS_OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTS_OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS_Op0` reader - TS_Op0"]
pub struct TS_OP0_R(crate::FieldReader<bool, bool>);
impl TS_OP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op0` writer - TS_Op0"]
pub struct TS_OP0_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op1` reader - TS_Op1"]
pub struct TS_OP1_R(crate::FieldReader<bool, bool>);
impl TS_OP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op1` writer - TS_Op1"]
pub struct TS_OP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op2` reader - TS_Op2"]
pub struct TS_OP2_R(crate::FieldReader<bool, bool>);
impl TS_OP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op2` writer - TS_Op2"]
pub struct TS_OP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op3` reader - TS_Op3"]
pub struct TS_OP3_R(crate::FieldReader<bool, bool>);
impl TS_OP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op3` writer - TS_Op3"]
pub struct TS_OP3_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op4` reader - TS_Op4"]
pub struct TS_OP4_R(crate::FieldReader<bool, bool>);
impl TS_OP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op4` writer - TS_Op4"]
pub struct TS_OP4_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op5` reader - TS_Op5"]
pub struct TS_OP5_R(crate::FieldReader<bool, bool>);
impl TS_OP5_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op5` writer - TS_Op5"]
pub struct TS_OP5_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op6` reader - TS_Op6"]
pub struct TS_OP6_R(crate::FieldReader<bool, bool>);
impl TS_OP6_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op6` writer - TS_Op6"]
pub struct TS_OP6_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op7` reader - TS_Op7"]
pub struct TS_OP7_R(crate::FieldReader<bool, bool>);
impl TS_OP7_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op7` writer - TS_Op7"]
pub struct TS_OP7_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op8` reader - TS_Op8"]
pub struct TS_OP8_R(crate::FieldReader<bool, bool>);
impl TS_OP8_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op8` writer - TS_Op8"]
pub struct TS_OP8_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op9` reader - TS_Op9"]
pub struct TS_OP9_R(crate::FieldReader<bool, bool>);
impl TS_OP9_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op9` writer - TS_Op9"]
pub struct TS_OP9_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op10` reader - TS_Op10"]
pub struct TS_OP10_R(crate::FieldReader<bool, bool>);
impl TS_OP10_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op10` writer - TS_Op10"]
pub struct TS_OP10_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op11` reader - TS_Op11"]
pub struct TS_OP11_R(crate::FieldReader<bool, bool>);
impl TS_OP11_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op11` writer - TS_Op11"]
pub struct TS_OP11_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op12` reader - TS_Op12"]
pub struct TS_OP12_R(crate::FieldReader<bool, bool>);
impl TS_OP12_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op12` writer - TS_Op12"]
pub struct TS_OP12_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op13` reader - TS_Op13"]
pub struct TS_OP13_R(crate::FieldReader<bool, bool>);
impl TS_OP13_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op13` writer - TS_Op13"]
pub struct TS_OP13_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op14` reader - TS_Op14"]
pub struct TS_OP14_R(crate::FieldReader<bool, bool>);
impl TS_OP14_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op14` writer - TS_Op14"]
pub struct TS_OP14_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op15` reader - TS_Op15"]
pub struct TS_OP15_R(crate::FieldReader<bool, bool>);
impl TS_OP15_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op15` writer - TS_Op15"]
pub struct TS_OP15_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op16` reader - TS_Op16"]
pub struct TS_OP16_R(crate::FieldReader<bool, bool>);
impl TS_OP16_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op16` writer - TS_Op16"]
pub struct TS_OP16_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op17` reader - TS_Op17"]
pub struct TS_OP17_R(crate::FieldReader<bool, bool>);
impl TS_OP17_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op17` writer - TS_Op17"]
pub struct TS_OP17_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op18` reader - TS_Op18"]
pub struct TS_OP18_R(crate::FieldReader<bool, bool>);
impl TS_OP18_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op18` writer - TS_Op18"]
pub struct TS_OP18_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op19` reader - TS_Op19"]
pub struct TS_OP19_R(crate::FieldReader<bool, bool>);
impl TS_OP19_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op19` writer - TS_Op19"]
pub struct TS_OP19_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op20` reader - TS_Op20"]
pub struct TS_OP20_R(crate::FieldReader<bool, bool>);
impl TS_OP20_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op20` writer - TS_Op20"]
pub struct TS_OP20_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op21` reader - TS_Op21"]
pub struct TS_OP21_R(crate::FieldReader<bool, bool>);
impl TS_OP21_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op21` writer - TS_Op21"]
pub struct TS_OP21_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op22` reader - TS_Op22"]
pub struct TS_OP22_R(crate::FieldReader<bool, bool>);
impl TS_OP22_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op22` writer - TS_Op22"]
pub struct TS_OP22_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op23` reader - TS_Op23"]
pub struct TS_OP23_R(crate::FieldReader<bool, bool>);
impl TS_OP23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op23` writer - TS_Op23"]
pub struct TS_OP23_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op24` reader - TS_Op24"]
pub struct TS_OP24_R(crate::FieldReader<bool, bool>);
impl TS_OP24_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op24` writer - TS_Op24"]
pub struct TS_OP24_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op25` reader - TS_Op25"]
pub struct TS_OP25_R(crate::FieldReader<bool, bool>);
impl TS_OP25_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op25` writer - TS_Op25"]
pub struct TS_OP25_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op26` reader - TS_Op26"]
pub struct TS_OP26_R(crate::FieldReader<bool, bool>);
impl TS_OP26_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op26` writer - TS_Op26"]
pub struct TS_OP26_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op27` reader - TS_Op27"]
pub struct TS_OP27_R(crate::FieldReader<bool, bool>);
impl TS_OP27_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op27` writer - TS_Op27"]
pub struct TS_OP27_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op28` reader - TS_Op28"]
pub struct TS_OP28_R(crate::FieldReader<bool, bool>);
impl TS_OP28_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op28` writer - TS_Op28"]
pub struct TS_OP28_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op29` reader - TS_Op29"]
pub struct TS_OP29_R(crate::FieldReader<bool, bool>);
impl TS_OP29_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op29` writer - TS_Op29"]
pub struct TS_OP29_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op30` reader - TS_Op30"]
pub struct TS_OP30_R(crate::FieldReader<bool, bool>);
impl TS_OP30_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op30` writer - TS_Op30"]
pub struct TS_OP30_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TS_Op31` reader - TS_Op31"]
pub struct TS_OP31_R(crate::FieldReader<bool, bool>);
impl TS_OP31_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_OP31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_OP31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_Op31` writer - TS_Op31"]
pub struct TS_OP31_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - TS_Op0"]
    #[inline(always)]
    pub fn ts_op0(&self) -> TS_OP0_R {
        TS_OP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TS_Op1"]
    #[inline(always)]
    pub fn ts_op1(&self) -> TS_OP1_R {
        TS_OP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TS_Op2"]
    #[inline(always)]
    pub fn ts_op2(&self) -> TS_OP2_R {
        TS_OP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TS_Op3"]
    #[inline(always)]
    pub fn ts_op3(&self) -> TS_OP3_R {
        TS_OP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TS_Op4"]
    #[inline(always)]
    pub fn ts_op4(&self) -> TS_OP4_R {
        TS_OP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TS_Op5"]
    #[inline(always)]
    pub fn ts_op5(&self) -> TS_OP5_R {
        TS_OP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TS_Op6"]
    #[inline(always)]
    pub fn ts_op6(&self) -> TS_OP6_R {
        TS_OP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TS_Op7"]
    #[inline(always)]
    pub fn ts_op7(&self) -> TS_OP7_R {
        TS_OP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TS_Op8"]
    #[inline(always)]
    pub fn ts_op8(&self) -> TS_OP8_R {
        TS_OP8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TS_Op9"]
    #[inline(always)]
    pub fn ts_op9(&self) -> TS_OP9_R {
        TS_OP9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TS_Op10"]
    #[inline(always)]
    pub fn ts_op10(&self) -> TS_OP10_R {
        TS_OP10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TS_Op11"]
    #[inline(always)]
    pub fn ts_op11(&self) -> TS_OP11_R {
        TS_OP11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TS_Op12"]
    #[inline(always)]
    pub fn ts_op12(&self) -> TS_OP12_R {
        TS_OP12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TS_Op13"]
    #[inline(always)]
    pub fn ts_op13(&self) -> TS_OP13_R {
        TS_OP13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TS_Op14"]
    #[inline(always)]
    pub fn ts_op14(&self) -> TS_OP14_R {
        TS_OP14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TS_Op15"]
    #[inline(always)]
    pub fn ts_op15(&self) -> TS_OP15_R {
        TS_OP15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TS_Op16"]
    #[inline(always)]
    pub fn ts_op16(&self) -> TS_OP16_R {
        TS_OP16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TS_Op17"]
    #[inline(always)]
    pub fn ts_op17(&self) -> TS_OP17_R {
        TS_OP17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TS_Op18"]
    #[inline(always)]
    pub fn ts_op18(&self) -> TS_OP18_R {
        TS_OP18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TS_Op19"]
    #[inline(always)]
    pub fn ts_op19(&self) -> TS_OP19_R {
        TS_OP19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TS_Op20"]
    #[inline(always)]
    pub fn ts_op20(&self) -> TS_OP20_R {
        TS_OP20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TS_Op21"]
    #[inline(always)]
    pub fn ts_op21(&self) -> TS_OP21_R {
        TS_OP21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TS_Op22"]
    #[inline(always)]
    pub fn ts_op22(&self) -> TS_OP22_R {
        TS_OP22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TS_Op23"]
    #[inline(always)]
    pub fn ts_op23(&self) -> TS_OP23_R {
        TS_OP23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TS_Op24"]
    #[inline(always)]
    pub fn ts_op24(&self) -> TS_OP24_R {
        TS_OP24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TS_Op25"]
    #[inline(always)]
    pub fn ts_op25(&self) -> TS_OP25_R {
        TS_OP25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TS_Op26"]
    #[inline(always)]
    pub fn ts_op26(&self) -> TS_OP26_R {
        TS_OP26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TS_Op27"]
    #[inline(always)]
    pub fn ts_op27(&self) -> TS_OP27_R {
        TS_OP27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TS_Op28"]
    #[inline(always)]
    pub fn ts_op28(&self) -> TS_OP28_R {
        TS_OP28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TS_Op29"]
    #[inline(always)]
    pub fn ts_op29(&self) -> TS_OP29_R {
        TS_OP29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TS_Op30"]
    #[inline(always)]
    pub fn ts_op30(&self) -> TS_OP30_R {
        TS_OP30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - TS_Op31"]
    #[inline(always)]
    pub fn ts_op31(&self) -> TS_OP31_R {
        TS_OP31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TS_Op0"]
    #[inline(always)]
    pub fn ts_op0(&mut self) -> TS_OP0_W {
        TS_OP0_W { w: self }
    }
    #[doc = "Bit 1 - TS_Op1"]
    #[inline(always)]
    pub fn ts_op1(&mut self) -> TS_OP1_W {
        TS_OP1_W { w: self }
    }
    #[doc = "Bit 2 - TS_Op2"]
    #[inline(always)]
    pub fn ts_op2(&mut self) -> TS_OP2_W {
        TS_OP2_W { w: self }
    }
    #[doc = "Bit 3 - TS_Op3"]
    #[inline(always)]
    pub fn ts_op3(&mut self) -> TS_OP3_W {
        TS_OP3_W { w: self }
    }
    #[doc = "Bit 4 - TS_Op4"]
    #[inline(always)]
    pub fn ts_op4(&mut self) -> TS_OP4_W {
        TS_OP4_W { w: self }
    }
    #[doc = "Bit 5 - TS_Op5"]
    #[inline(always)]
    pub fn ts_op5(&mut self) -> TS_OP5_W {
        TS_OP5_W { w: self }
    }
    #[doc = "Bit 6 - TS_Op6"]
    #[inline(always)]
    pub fn ts_op6(&mut self) -> TS_OP6_W {
        TS_OP6_W { w: self }
    }
    #[doc = "Bit 7 - TS_Op7"]
    #[inline(always)]
    pub fn ts_op7(&mut self) -> TS_OP7_W {
        TS_OP7_W { w: self }
    }
    #[doc = "Bit 8 - TS_Op8"]
    #[inline(always)]
    pub fn ts_op8(&mut self) -> TS_OP8_W {
        TS_OP8_W { w: self }
    }
    #[doc = "Bit 9 - TS_Op9"]
    #[inline(always)]
    pub fn ts_op9(&mut self) -> TS_OP9_W {
        TS_OP9_W { w: self }
    }
    #[doc = "Bit 10 - TS_Op10"]
    #[inline(always)]
    pub fn ts_op10(&mut self) -> TS_OP10_W {
        TS_OP10_W { w: self }
    }
    #[doc = "Bit 11 - TS_Op11"]
    #[inline(always)]
    pub fn ts_op11(&mut self) -> TS_OP11_W {
        TS_OP11_W { w: self }
    }
    #[doc = "Bit 12 - TS_Op12"]
    #[inline(always)]
    pub fn ts_op12(&mut self) -> TS_OP12_W {
        TS_OP12_W { w: self }
    }
    #[doc = "Bit 13 - TS_Op13"]
    #[inline(always)]
    pub fn ts_op13(&mut self) -> TS_OP13_W {
        TS_OP13_W { w: self }
    }
    #[doc = "Bit 14 - TS_Op14"]
    #[inline(always)]
    pub fn ts_op14(&mut self) -> TS_OP14_W {
        TS_OP14_W { w: self }
    }
    #[doc = "Bit 15 - TS_Op15"]
    #[inline(always)]
    pub fn ts_op15(&mut self) -> TS_OP15_W {
        TS_OP15_W { w: self }
    }
    #[doc = "Bit 16 - TS_Op16"]
    #[inline(always)]
    pub fn ts_op16(&mut self) -> TS_OP16_W {
        TS_OP16_W { w: self }
    }
    #[doc = "Bit 17 - TS_Op17"]
    #[inline(always)]
    pub fn ts_op17(&mut self) -> TS_OP17_W {
        TS_OP17_W { w: self }
    }
    #[doc = "Bit 18 - TS_Op18"]
    #[inline(always)]
    pub fn ts_op18(&mut self) -> TS_OP18_W {
        TS_OP18_W { w: self }
    }
    #[doc = "Bit 19 - TS_Op19"]
    #[inline(always)]
    pub fn ts_op19(&mut self) -> TS_OP19_W {
        TS_OP19_W { w: self }
    }
    #[doc = "Bit 20 - TS_Op20"]
    #[inline(always)]
    pub fn ts_op20(&mut self) -> TS_OP20_W {
        TS_OP20_W { w: self }
    }
    #[doc = "Bit 21 - TS_Op21"]
    #[inline(always)]
    pub fn ts_op21(&mut self) -> TS_OP21_W {
        TS_OP21_W { w: self }
    }
    #[doc = "Bit 22 - TS_Op22"]
    #[inline(always)]
    pub fn ts_op22(&mut self) -> TS_OP22_W {
        TS_OP22_W { w: self }
    }
    #[doc = "Bit 23 - TS_Op23"]
    #[inline(always)]
    pub fn ts_op23(&mut self) -> TS_OP23_W {
        TS_OP23_W { w: self }
    }
    #[doc = "Bit 24 - TS_Op24"]
    #[inline(always)]
    pub fn ts_op24(&mut self) -> TS_OP24_W {
        TS_OP24_W { w: self }
    }
    #[doc = "Bit 25 - TS_Op25"]
    #[inline(always)]
    pub fn ts_op25(&mut self) -> TS_OP25_W {
        TS_OP25_W { w: self }
    }
    #[doc = "Bit 26 - TS_Op26"]
    #[inline(always)]
    pub fn ts_op26(&mut self) -> TS_OP26_W {
        TS_OP26_W { w: self }
    }
    #[doc = "Bit 27 - TS_Op27"]
    #[inline(always)]
    pub fn ts_op27(&mut self) -> TS_OP27_W {
        TS_OP27_W { w: self }
    }
    #[doc = "Bit 28 - TS_Op28"]
    #[inline(always)]
    pub fn ts_op28(&mut self) -> TS_OP28_W {
        TS_OP28_W { w: self }
    }
    #[doc = "Bit 29 - TS_Op29"]
    #[inline(always)]
    pub fn ts_op29(&mut self) -> TS_OP29_W {
        TS_OP29_W { w: self }
    }
    #[doc = "Bit 30 - TS_Op30"]
    #[inline(always)]
    pub fn ts_op30(&mut self) -> TS_OP30_W {
        TS_OP30_W { w: self }
    }
    #[doc = "Bit 31 - TS_Op31"]
    #[inline(always)]
    pub fn ts_op31(&mut self) -> TS_OP31_W {
        TS_OP31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The DTS_OR contains general-purpose option bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_or](index.html) module"]
pub struct DTS_OR_SPEC;
impl crate::RegisterSpec for DTS_OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dts_or::R](R) reader structure"]
impl crate::Readable for DTS_OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dts_or::W](W) writer structure"]
impl crate::Writable for DTS_OR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTS_OR to value 0"]
impl crate::Resettable for DTS_OR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
