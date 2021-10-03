#[doc = "Register `MPCBB2_VCTR36` reader"]
pub struct R(crate::R<MPCBB2_VCTR36_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR36_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR36_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR36_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR36` writer"]
pub struct W(crate::W<MPCBB2_VCTR36_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR36_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR36_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR36_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1152` reader - B1152"]
pub struct B1152_R(crate::FieldReader<bool, bool>);
impl B1152_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1152_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1152_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1152` writer - B1152"]
pub struct B1152_W<'a> {
    w: &'a mut W,
}
impl<'a> B1152_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1153` reader - B1153"]
pub struct B1153_R(crate::FieldReader<bool, bool>);
impl B1153_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1153_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1153_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1153` writer - B1153"]
pub struct B1153_W<'a> {
    w: &'a mut W,
}
impl<'a> B1153_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1154` reader - B1154"]
pub struct B1154_R(crate::FieldReader<bool, bool>);
impl B1154_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1154_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1154_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1154` writer - B1154"]
pub struct B1154_W<'a> {
    w: &'a mut W,
}
impl<'a> B1154_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1155` reader - B1155"]
pub struct B1155_R(crate::FieldReader<bool, bool>);
impl B1155_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1155_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1155_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1155` writer - B1155"]
pub struct B1155_W<'a> {
    w: &'a mut W,
}
impl<'a> B1155_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1156` reader - B1156"]
pub struct B1156_R(crate::FieldReader<bool, bool>);
impl B1156_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1156_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1156_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1156` writer - B1156"]
pub struct B1156_W<'a> {
    w: &'a mut W,
}
impl<'a> B1156_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1157` reader - B1157"]
pub struct B1157_R(crate::FieldReader<bool, bool>);
impl B1157_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1157_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1157_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1157` writer - B1157"]
pub struct B1157_W<'a> {
    w: &'a mut W,
}
impl<'a> B1157_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1158` reader - B1158"]
pub struct B1158_R(crate::FieldReader<bool, bool>);
impl B1158_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1158_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1158_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1158` writer - B1158"]
pub struct B1158_W<'a> {
    w: &'a mut W,
}
impl<'a> B1158_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1159` reader - B1159"]
pub struct B1159_R(crate::FieldReader<bool, bool>);
impl B1159_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1159_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1159_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1159` writer - B1159"]
pub struct B1159_W<'a> {
    w: &'a mut W,
}
impl<'a> B1159_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1160` reader - B1160"]
pub struct B1160_R(crate::FieldReader<bool, bool>);
impl B1160_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1160_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1160_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1160` writer - B1160"]
pub struct B1160_W<'a> {
    w: &'a mut W,
}
impl<'a> B1160_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1161` reader - B1161"]
pub struct B1161_R(crate::FieldReader<bool, bool>);
impl B1161_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1161_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1161_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1161` writer - B1161"]
pub struct B1161_W<'a> {
    w: &'a mut W,
}
impl<'a> B1161_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1162` reader - B1162"]
pub struct B1162_R(crate::FieldReader<bool, bool>);
impl B1162_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1162_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1162_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1162` writer - B1162"]
pub struct B1162_W<'a> {
    w: &'a mut W,
}
impl<'a> B1162_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1163` reader - B1163"]
pub struct B1163_R(crate::FieldReader<bool, bool>);
impl B1163_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1163_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1163_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1163` writer - B1163"]
pub struct B1163_W<'a> {
    w: &'a mut W,
}
impl<'a> B1163_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1164` reader - B1164"]
pub struct B1164_R(crate::FieldReader<bool, bool>);
impl B1164_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1164_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1164_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1164` writer - B1164"]
pub struct B1164_W<'a> {
    w: &'a mut W,
}
impl<'a> B1164_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1165` reader - B1165"]
pub struct B1165_R(crate::FieldReader<bool, bool>);
impl B1165_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1165_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1165_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1165` writer - B1165"]
pub struct B1165_W<'a> {
    w: &'a mut W,
}
impl<'a> B1165_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1166` reader - B1166"]
pub struct B1166_R(crate::FieldReader<bool, bool>);
impl B1166_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1166_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1166_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1166` writer - B1166"]
pub struct B1166_W<'a> {
    w: &'a mut W,
}
impl<'a> B1166_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1167` reader - B1167"]
pub struct B1167_R(crate::FieldReader<bool, bool>);
impl B1167_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1167_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1167_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1167` writer - B1167"]
pub struct B1167_W<'a> {
    w: &'a mut W,
}
impl<'a> B1167_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1168` reader - B1168"]
pub struct B1168_R(crate::FieldReader<bool, bool>);
impl B1168_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1168_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1168_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1168` writer - B1168"]
pub struct B1168_W<'a> {
    w: &'a mut W,
}
impl<'a> B1168_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1169` reader - B1169"]
pub struct B1169_R(crate::FieldReader<bool, bool>);
impl B1169_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1169_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1169_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1169` writer - B1169"]
pub struct B1169_W<'a> {
    w: &'a mut W,
}
impl<'a> B1169_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1170` reader - B1170"]
pub struct B1170_R(crate::FieldReader<bool, bool>);
impl B1170_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1170_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1170_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1170` writer - B1170"]
pub struct B1170_W<'a> {
    w: &'a mut W,
}
impl<'a> B1170_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1171` reader - B1171"]
pub struct B1171_R(crate::FieldReader<bool, bool>);
impl B1171_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1171_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1171_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1171` writer - B1171"]
pub struct B1171_W<'a> {
    w: &'a mut W,
}
impl<'a> B1171_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1172` reader - B1172"]
pub struct B1172_R(crate::FieldReader<bool, bool>);
impl B1172_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1172_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1172_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1172` writer - B1172"]
pub struct B1172_W<'a> {
    w: &'a mut W,
}
impl<'a> B1172_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1173` reader - B1173"]
pub struct B1173_R(crate::FieldReader<bool, bool>);
impl B1173_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1173_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1173_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1173` writer - B1173"]
pub struct B1173_W<'a> {
    w: &'a mut W,
}
impl<'a> B1173_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1174` reader - B1174"]
pub struct B1174_R(crate::FieldReader<bool, bool>);
impl B1174_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1174_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1174_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1174` writer - B1174"]
pub struct B1174_W<'a> {
    w: &'a mut W,
}
impl<'a> B1174_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1175` reader - B1175"]
pub struct B1175_R(crate::FieldReader<bool, bool>);
impl B1175_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1175_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1175_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1175` writer - B1175"]
pub struct B1175_W<'a> {
    w: &'a mut W,
}
impl<'a> B1175_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1176` reader - B1176"]
pub struct B1176_R(crate::FieldReader<bool, bool>);
impl B1176_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1176_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1176_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1176` writer - B1176"]
pub struct B1176_W<'a> {
    w: &'a mut W,
}
impl<'a> B1176_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1177` reader - B1177"]
pub struct B1177_R(crate::FieldReader<bool, bool>);
impl B1177_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1177_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1177_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1177` writer - B1177"]
pub struct B1177_W<'a> {
    w: &'a mut W,
}
impl<'a> B1177_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1178` reader - B1178"]
pub struct B1178_R(crate::FieldReader<bool, bool>);
impl B1178_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1178_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1178_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1178` writer - B1178"]
pub struct B1178_W<'a> {
    w: &'a mut W,
}
impl<'a> B1178_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1179` reader - B1179"]
pub struct B1179_R(crate::FieldReader<bool, bool>);
impl B1179_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1179_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1179_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1179` writer - B1179"]
pub struct B1179_W<'a> {
    w: &'a mut W,
}
impl<'a> B1179_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1180` reader - B1180"]
pub struct B1180_R(crate::FieldReader<bool, bool>);
impl B1180_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1180_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1180_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1180` writer - B1180"]
pub struct B1180_W<'a> {
    w: &'a mut W,
}
impl<'a> B1180_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1181` reader - B1181"]
pub struct B1181_R(crate::FieldReader<bool, bool>);
impl B1181_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1181_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1181_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1181` writer - B1181"]
pub struct B1181_W<'a> {
    w: &'a mut W,
}
impl<'a> B1181_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1182` reader - B1182"]
pub struct B1182_R(crate::FieldReader<bool, bool>);
impl B1182_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1182_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1182_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1182` writer - B1182"]
pub struct B1182_W<'a> {
    w: &'a mut W,
}
impl<'a> B1182_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1183` reader - B1183"]
pub struct B1183_R(crate::FieldReader<bool, bool>);
impl B1183_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1183_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1183_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1183` writer - B1183"]
pub struct B1183_W<'a> {
    w: &'a mut W,
}
impl<'a> B1183_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1152"]
    #[inline(always)]
    pub fn b1152(&self) -> B1152_R {
        B1152_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1153"]
    #[inline(always)]
    pub fn b1153(&self) -> B1153_R {
        B1153_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1154"]
    #[inline(always)]
    pub fn b1154(&self) -> B1154_R {
        B1154_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1155"]
    #[inline(always)]
    pub fn b1155(&self) -> B1155_R {
        B1155_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1156"]
    #[inline(always)]
    pub fn b1156(&self) -> B1156_R {
        B1156_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1157"]
    #[inline(always)]
    pub fn b1157(&self) -> B1157_R {
        B1157_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1158"]
    #[inline(always)]
    pub fn b1158(&self) -> B1158_R {
        B1158_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1159"]
    #[inline(always)]
    pub fn b1159(&self) -> B1159_R {
        B1159_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1160"]
    #[inline(always)]
    pub fn b1160(&self) -> B1160_R {
        B1160_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1161"]
    #[inline(always)]
    pub fn b1161(&self) -> B1161_R {
        B1161_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1162"]
    #[inline(always)]
    pub fn b1162(&self) -> B1162_R {
        B1162_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1163"]
    #[inline(always)]
    pub fn b1163(&self) -> B1163_R {
        B1163_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1164"]
    #[inline(always)]
    pub fn b1164(&self) -> B1164_R {
        B1164_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1165"]
    #[inline(always)]
    pub fn b1165(&self) -> B1165_R {
        B1165_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1166"]
    #[inline(always)]
    pub fn b1166(&self) -> B1166_R {
        B1166_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1167"]
    #[inline(always)]
    pub fn b1167(&self) -> B1167_R {
        B1167_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1168"]
    #[inline(always)]
    pub fn b1168(&self) -> B1168_R {
        B1168_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1169"]
    #[inline(always)]
    pub fn b1169(&self) -> B1169_R {
        B1169_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1170"]
    #[inline(always)]
    pub fn b1170(&self) -> B1170_R {
        B1170_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1171"]
    #[inline(always)]
    pub fn b1171(&self) -> B1171_R {
        B1171_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1172"]
    #[inline(always)]
    pub fn b1172(&self) -> B1172_R {
        B1172_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1173"]
    #[inline(always)]
    pub fn b1173(&self) -> B1173_R {
        B1173_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1174"]
    #[inline(always)]
    pub fn b1174(&self) -> B1174_R {
        B1174_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1175"]
    #[inline(always)]
    pub fn b1175(&self) -> B1175_R {
        B1175_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1176"]
    #[inline(always)]
    pub fn b1176(&self) -> B1176_R {
        B1176_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1177"]
    #[inline(always)]
    pub fn b1177(&self) -> B1177_R {
        B1177_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1178"]
    #[inline(always)]
    pub fn b1178(&self) -> B1178_R {
        B1178_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1179"]
    #[inline(always)]
    pub fn b1179(&self) -> B1179_R {
        B1179_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1180"]
    #[inline(always)]
    pub fn b1180(&self) -> B1180_R {
        B1180_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1181"]
    #[inline(always)]
    pub fn b1181(&self) -> B1181_R {
        B1181_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1182"]
    #[inline(always)]
    pub fn b1182(&self) -> B1182_R {
        B1182_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1183"]
    #[inline(always)]
    pub fn b1183(&self) -> B1183_R {
        B1183_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1152"]
    #[inline(always)]
    pub fn b1152(&mut self) -> B1152_W {
        B1152_W { w: self }
    }
    #[doc = "Bit 1 - B1153"]
    #[inline(always)]
    pub fn b1153(&mut self) -> B1153_W {
        B1153_W { w: self }
    }
    #[doc = "Bit 2 - B1154"]
    #[inline(always)]
    pub fn b1154(&mut self) -> B1154_W {
        B1154_W { w: self }
    }
    #[doc = "Bit 3 - B1155"]
    #[inline(always)]
    pub fn b1155(&mut self) -> B1155_W {
        B1155_W { w: self }
    }
    #[doc = "Bit 4 - B1156"]
    #[inline(always)]
    pub fn b1156(&mut self) -> B1156_W {
        B1156_W { w: self }
    }
    #[doc = "Bit 5 - B1157"]
    #[inline(always)]
    pub fn b1157(&mut self) -> B1157_W {
        B1157_W { w: self }
    }
    #[doc = "Bit 6 - B1158"]
    #[inline(always)]
    pub fn b1158(&mut self) -> B1158_W {
        B1158_W { w: self }
    }
    #[doc = "Bit 7 - B1159"]
    #[inline(always)]
    pub fn b1159(&mut self) -> B1159_W {
        B1159_W { w: self }
    }
    #[doc = "Bit 8 - B1160"]
    #[inline(always)]
    pub fn b1160(&mut self) -> B1160_W {
        B1160_W { w: self }
    }
    #[doc = "Bit 9 - B1161"]
    #[inline(always)]
    pub fn b1161(&mut self) -> B1161_W {
        B1161_W { w: self }
    }
    #[doc = "Bit 10 - B1162"]
    #[inline(always)]
    pub fn b1162(&mut self) -> B1162_W {
        B1162_W { w: self }
    }
    #[doc = "Bit 11 - B1163"]
    #[inline(always)]
    pub fn b1163(&mut self) -> B1163_W {
        B1163_W { w: self }
    }
    #[doc = "Bit 12 - B1164"]
    #[inline(always)]
    pub fn b1164(&mut self) -> B1164_W {
        B1164_W { w: self }
    }
    #[doc = "Bit 13 - B1165"]
    #[inline(always)]
    pub fn b1165(&mut self) -> B1165_W {
        B1165_W { w: self }
    }
    #[doc = "Bit 14 - B1166"]
    #[inline(always)]
    pub fn b1166(&mut self) -> B1166_W {
        B1166_W { w: self }
    }
    #[doc = "Bit 15 - B1167"]
    #[inline(always)]
    pub fn b1167(&mut self) -> B1167_W {
        B1167_W { w: self }
    }
    #[doc = "Bit 16 - B1168"]
    #[inline(always)]
    pub fn b1168(&mut self) -> B1168_W {
        B1168_W { w: self }
    }
    #[doc = "Bit 17 - B1169"]
    #[inline(always)]
    pub fn b1169(&mut self) -> B1169_W {
        B1169_W { w: self }
    }
    #[doc = "Bit 18 - B1170"]
    #[inline(always)]
    pub fn b1170(&mut self) -> B1170_W {
        B1170_W { w: self }
    }
    #[doc = "Bit 19 - B1171"]
    #[inline(always)]
    pub fn b1171(&mut self) -> B1171_W {
        B1171_W { w: self }
    }
    #[doc = "Bit 20 - B1172"]
    #[inline(always)]
    pub fn b1172(&mut self) -> B1172_W {
        B1172_W { w: self }
    }
    #[doc = "Bit 21 - B1173"]
    #[inline(always)]
    pub fn b1173(&mut self) -> B1173_W {
        B1173_W { w: self }
    }
    #[doc = "Bit 22 - B1174"]
    #[inline(always)]
    pub fn b1174(&mut self) -> B1174_W {
        B1174_W { w: self }
    }
    #[doc = "Bit 23 - B1175"]
    #[inline(always)]
    pub fn b1175(&mut self) -> B1175_W {
        B1175_W { w: self }
    }
    #[doc = "Bit 24 - B1176"]
    #[inline(always)]
    pub fn b1176(&mut self) -> B1176_W {
        B1176_W { w: self }
    }
    #[doc = "Bit 25 - B1177"]
    #[inline(always)]
    pub fn b1177(&mut self) -> B1177_W {
        B1177_W { w: self }
    }
    #[doc = "Bit 26 - B1178"]
    #[inline(always)]
    pub fn b1178(&mut self) -> B1178_W {
        B1178_W { w: self }
    }
    #[doc = "Bit 27 - B1179"]
    #[inline(always)]
    pub fn b1179(&mut self) -> B1179_W {
        B1179_W { w: self }
    }
    #[doc = "Bit 28 - B1180"]
    #[inline(always)]
    pub fn b1180(&mut self) -> B1180_W {
        B1180_W { w: self }
    }
    #[doc = "Bit 29 - B1181"]
    #[inline(always)]
    pub fn b1181(&mut self) -> B1181_W {
        B1181_W { w: self }
    }
    #[doc = "Bit 30 - B1182"]
    #[inline(always)]
    pub fn b1182(&mut self) -> B1182_W {
        B1182_W { w: self }
    }
    #[doc = "Bit 31 - B1183"]
    #[inline(always)]
    pub fn b1183(&mut self) -> B1183_W {
        B1183_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr36](index.html) module"]
pub struct MPCBB2_VCTR36_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR36_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr36::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR36_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr36::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR36_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR36 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB2_VCTR36_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
