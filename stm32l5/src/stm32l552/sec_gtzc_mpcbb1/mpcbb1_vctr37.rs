#[doc = "Register `MPCBB1_VCTR37` reader"]
pub struct R(crate::R<MPCBB1_VCTR37_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR37_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR37_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR37_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR37` writer"]
pub struct W(crate::W<MPCBB1_VCTR37_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR37_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR37_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR37_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1184` reader - B1184"]
pub struct B1184_R(crate::FieldReader<bool, bool>);
impl B1184_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1184_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1184_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1184` writer - B1184"]
pub struct B1184_W<'a> {
    w: &'a mut W,
}
impl<'a> B1184_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1185` reader - B1185"]
pub struct B1185_R(crate::FieldReader<bool, bool>);
impl B1185_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1185_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1185_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1185` writer - B1185"]
pub struct B1185_W<'a> {
    w: &'a mut W,
}
impl<'a> B1185_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1186` reader - B1186"]
pub struct B1186_R(crate::FieldReader<bool, bool>);
impl B1186_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1186_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1186_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1186` writer - B1186"]
pub struct B1186_W<'a> {
    w: &'a mut W,
}
impl<'a> B1186_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1187` reader - B1187"]
pub struct B1187_R(crate::FieldReader<bool, bool>);
impl B1187_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1187_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1187_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1187` writer - B1187"]
pub struct B1187_W<'a> {
    w: &'a mut W,
}
impl<'a> B1187_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1188` reader - B1188"]
pub struct B1188_R(crate::FieldReader<bool, bool>);
impl B1188_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1188_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1188_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1188` writer - B1188"]
pub struct B1188_W<'a> {
    w: &'a mut W,
}
impl<'a> B1188_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1189` reader - B1189"]
pub struct B1189_R(crate::FieldReader<bool, bool>);
impl B1189_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1189_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1189_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1189` writer - B1189"]
pub struct B1189_W<'a> {
    w: &'a mut W,
}
impl<'a> B1189_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1190` reader - B1190"]
pub struct B1190_R(crate::FieldReader<bool, bool>);
impl B1190_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1190_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1190_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1190` writer - B1190"]
pub struct B1190_W<'a> {
    w: &'a mut W,
}
impl<'a> B1190_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1191` reader - B1191"]
pub struct B1191_R(crate::FieldReader<bool, bool>);
impl B1191_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1191_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1191_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1191` writer - B1191"]
pub struct B1191_W<'a> {
    w: &'a mut W,
}
impl<'a> B1191_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1192` reader - B1192"]
pub struct B1192_R(crate::FieldReader<bool, bool>);
impl B1192_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1192_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1192_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1192` writer - B1192"]
pub struct B1192_W<'a> {
    w: &'a mut W,
}
impl<'a> B1192_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1193` reader - B1193"]
pub struct B1193_R(crate::FieldReader<bool, bool>);
impl B1193_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1193_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1193_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1193` writer - B1193"]
pub struct B1193_W<'a> {
    w: &'a mut W,
}
impl<'a> B1193_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1194` reader - B1194"]
pub struct B1194_R(crate::FieldReader<bool, bool>);
impl B1194_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1194_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1194_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1194` writer - B1194"]
pub struct B1194_W<'a> {
    w: &'a mut W,
}
impl<'a> B1194_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1195` reader - B1195"]
pub struct B1195_R(crate::FieldReader<bool, bool>);
impl B1195_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1195_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1195_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1195` writer - B1195"]
pub struct B1195_W<'a> {
    w: &'a mut W,
}
impl<'a> B1195_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1196` reader - B1196"]
pub struct B1196_R(crate::FieldReader<bool, bool>);
impl B1196_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1196_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1196_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1196` writer - B1196"]
pub struct B1196_W<'a> {
    w: &'a mut W,
}
impl<'a> B1196_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1197` reader - B1197"]
pub struct B1197_R(crate::FieldReader<bool, bool>);
impl B1197_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1197_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1197_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1197` writer - B1197"]
pub struct B1197_W<'a> {
    w: &'a mut W,
}
impl<'a> B1197_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1198` reader - B1198"]
pub struct B1198_R(crate::FieldReader<bool, bool>);
impl B1198_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1198_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1198_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1198` writer - B1198"]
pub struct B1198_W<'a> {
    w: &'a mut W,
}
impl<'a> B1198_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1199` reader - B1199"]
pub struct B1199_R(crate::FieldReader<bool, bool>);
impl B1199_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1199_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1199_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1199` writer - B1199"]
pub struct B1199_W<'a> {
    w: &'a mut W,
}
impl<'a> B1199_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1200` reader - B1200"]
pub struct B1200_R(crate::FieldReader<bool, bool>);
impl B1200_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1200_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1200_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1200` writer - B1200"]
pub struct B1200_W<'a> {
    w: &'a mut W,
}
impl<'a> B1200_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1201` reader - B1201"]
pub struct B1201_R(crate::FieldReader<bool, bool>);
impl B1201_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1201_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1201_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1201` writer - B1201"]
pub struct B1201_W<'a> {
    w: &'a mut W,
}
impl<'a> B1201_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1202` reader - B1202"]
pub struct B1202_R(crate::FieldReader<bool, bool>);
impl B1202_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1202_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1202_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1202` writer - B1202"]
pub struct B1202_W<'a> {
    w: &'a mut W,
}
impl<'a> B1202_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1203` reader - B1203"]
pub struct B1203_R(crate::FieldReader<bool, bool>);
impl B1203_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1203_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1203_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1203` writer - B1203"]
pub struct B1203_W<'a> {
    w: &'a mut W,
}
impl<'a> B1203_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1204` reader - B1204"]
pub struct B1204_R(crate::FieldReader<bool, bool>);
impl B1204_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1204_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1204_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1204` writer - B1204"]
pub struct B1204_W<'a> {
    w: &'a mut W,
}
impl<'a> B1204_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1205` reader - B1205"]
pub struct B1205_R(crate::FieldReader<bool, bool>);
impl B1205_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1205_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1205_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1205` writer - B1205"]
pub struct B1205_W<'a> {
    w: &'a mut W,
}
impl<'a> B1205_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1206` reader - B1206"]
pub struct B1206_R(crate::FieldReader<bool, bool>);
impl B1206_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1206_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1206_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1206` writer - B1206"]
pub struct B1206_W<'a> {
    w: &'a mut W,
}
impl<'a> B1206_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1207` reader - B1207"]
pub struct B1207_R(crate::FieldReader<bool, bool>);
impl B1207_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1207_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1207_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1207` writer - B1207"]
pub struct B1207_W<'a> {
    w: &'a mut W,
}
impl<'a> B1207_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1208` reader - B1208"]
pub struct B1208_R(crate::FieldReader<bool, bool>);
impl B1208_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1208_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1208_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1208` writer - B1208"]
pub struct B1208_W<'a> {
    w: &'a mut W,
}
impl<'a> B1208_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1209` reader - B1209"]
pub struct B1209_R(crate::FieldReader<bool, bool>);
impl B1209_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1209_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1209_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1209` writer - B1209"]
pub struct B1209_W<'a> {
    w: &'a mut W,
}
impl<'a> B1209_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1210` reader - B1210"]
pub struct B1210_R(crate::FieldReader<bool, bool>);
impl B1210_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1210_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1210_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1210` writer - B1210"]
pub struct B1210_W<'a> {
    w: &'a mut W,
}
impl<'a> B1210_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1211` reader - B1211"]
pub struct B1211_R(crate::FieldReader<bool, bool>);
impl B1211_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1211_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1211_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1211` writer - B1211"]
pub struct B1211_W<'a> {
    w: &'a mut W,
}
impl<'a> B1211_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1212` reader - B1212"]
pub struct B1212_R(crate::FieldReader<bool, bool>);
impl B1212_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1212_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1212_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1212` writer - B1212"]
pub struct B1212_W<'a> {
    w: &'a mut W,
}
impl<'a> B1212_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1213` reader - B1213"]
pub struct B1213_R(crate::FieldReader<bool, bool>);
impl B1213_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1213_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1213_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1213` writer - B1213"]
pub struct B1213_W<'a> {
    w: &'a mut W,
}
impl<'a> B1213_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1214` reader - B1214"]
pub struct B1214_R(crate::FieldReader<bool, bool>);
impl B1214_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1214_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1214_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1214` writer - B1214"]
pub struct B1214_W<'a> {
    w: &'a mut W,
}
impl<'a> B1214_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1215` reader - B1215"]
pub struct B1215_R(crate::FieldReader<bool, bool>);
impl B1215_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1215_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1215_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1215` writer - B1215"]
pub struct B1215_W<'a> {
    w: &'a mut W,
}
impl<'a> B1215_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1184"]
    #[inline(always)]
    pub fn b1184(&self) -> B1184_R {
        B1184_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1185"]
    #[inline(always)]
    pub fn b1185(&self) -> B1185_R {
        B1185_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1186"]
    #[inline(always)]
    pub fn b1186(&self) -> B1186_R {
        B1186_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1187"]
    #[inline(always)]
    pub fn b1187(&self) -> B1187_R {
        B1187_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1188"]
    #[inline(always)]
    pub fn b1188(&self) -> B1188_R {
        B1188_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1189"]
    #[inline(always)]
    pub fn b1189(&self) -> B1189_R {
        B1189_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1190"]
    #[inline(always)]
    pub fn b1190(&self) -> B1190_R {
        B1190_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1191"]
    #[inline(always)]
    pub fn b1191(&self) -> B1191_R {
        B1191_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1192"]
    #[inline(always)]
    pub fn b1192(&self) -> B1192_R {
        B1192_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1193"]
    #[inline(always)]
    pub fn b1193(&self) -> B1193_R {
        B1193_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1194"]
    #[inline(always)]
    pub fn b1194(&self) -> B1194_R {
        B1194_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1195"]
    #[inline(always)]
    pub fn b1195(&self) -> B1195_R {
        B1195_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1196"]
    #[inline(always)]
    pub fn b1196(&self) -> B1196_R {
        B1196_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1197"]
    #[inline(always)]
    pub fn b1197(&self) -> B1197_R {
        B1197_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1198"]
    #[inline(always)]
    pub fn b1198(&self) -> B1198_R {
        B1198_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1199"]
    #[inline(always)]
    pub fn b1199(&self) -> B1199_R {
        B1199_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1200"]
    #[inline(always)]
    pub fn b1200(&self) -> B1200_R {
        B1200_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1201"]
    #[inline(always)]
    pub fn b1201(&self) -> B1201_R {
        B1201_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1202"]
    #[inline(always)]
    pub fn b1202(&self) -> B1202_R {
        B1202_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1203"]
    #[inline(always)]
    pub fn b1203(&self) -> B1203_R {
        B1203_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1204"]
    #[inline(always)]
    pub fn b1204(&self) -> B1204_R {
        B1204_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1205"]
    #[inline(always)]
    pub fn b1205(&self) -> B1205_R {
        B1205_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1206"]
    #[inline(always)]
    pub fn b1206(&self) -> B1206_R {
        B1206_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1207"]
    #[inline(always)]
    pub fn b1207(&self) -> B1207_R {
        B1207_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1208"]
    #[inline(always)]
    pub fn b1208(&self) -> B1208_R {
        B1208_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1209"]
    #[inline(always)]
    pub fn b1209(&self) -> B1209_R {
        B1209_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1210"]
    #[inline(always)]
    pub fn b1210(&self) -> B1210_R {
        B1210_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1211"]
    #[inline(always)]
    pub fn b1211(&self) -> B1211_R {
        B1211_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1212"]
    #[inline(always)]
    pub fn b1212(&self) -> B1212_R {
        B1212_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1213"]
    #[inline(always)]
    pub fn b1213(&self) -> B1213_R {
        B1213_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1214"]
    #[inline(always)]
    pub fn b1214(&self) -> B1214_R {
        B1214_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1215"]
    #[inline(always)]
    pub fn b1215(&self) -> B1215_R {
        B1215_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1184"]
    #[inline(always)]
    pub fn b1184(&mut self) -> B1184_W {
        B1184_W { w: self }
    }
    #[doc = "Bit 1 - B1185"]
    #[inline(always)]
    pub fn b1185(&mut self) -> B1185_W {
        B1185_W { w: self }
    }
    #[doc = "Bit 2 - B1186"]
    #[inline(always)]
    pub fn b1186(&mut self) -> B1186_W {
        B1186_W { w: self }
    }
    #[doc = "Bit 3 - B1187"]
    #[inline(always)]
    pub fn b1187(&mut self) -> B1187_W {
        B1187_W { w: self }
    }
    #[doc = "Bit 4 - B1188"]
    #[inline(always)]
    pub fn b1188(&mut self) -> B1188_W {
        B1188_W { w: self }
    }
    #[doc = "Bit 5 - B1189"]
    #[inline(always)]
    pub fn b1189(&mut self) -> B1189_W {
        B1189_W { w: self }
    }
    #[doc = "Bit 6 - B1190"]
    #[inline(always)]
    pub fn b1190(&mut self) -> B1190_W {
        B1190_W { w: self }
    }
    #[doc = "Bit 7 - B1191"]
    #[inline(always)]
    pub fn b1191(&mut self) -> B1191_W {
        B1191_W { w: self }
    }
    #[doc = "Bit 8 - B1192"]
    #[inline(always)]
    pub fn b1192(&mut self) -> B1192_W {
        B1192_W { w: self }
    }
    #[doc = "Bit 9 - B1193"]
    #[inline(always)]
    pub fn b1193(&mut self) -> B1193_W {
        B1193_W { w: self }
    }
    #[doc = "Bit 10 - B1194"]
    #[inline(always)]
    pub fn b1194(&mut self) -> B1194_W {
        B1194_W { w: self }
    }
    #[doc = "Bit 11 - B1195"]
    #[inline(always)]
    pub fn b1195(&mut self) -> B1195_W {
        B1195_W { w: self }
    }
    #[doc = "Bit 12 - B1196"]
    #[inline(always)]
    pub fn b1196(&mut self) -> B1196_W {
        B1196_W { w: self }
    }
    #[doc = "Bit 13 - B1197"]
    #[inline(always)]
    pub fn b1197(&mut self) -> B1197_W {
        B1197_W { w: self }
    }
    #[doc = "Bit 14 - B1198"]
    #[inline(always)]
    pub fn b1198(&mut self) -> B1198_W {
        B1198_W { w: self }
    }
    #[doc = "Bit 15 - B1199"]
    #[inline(always)]
    pub fn b1199(&mut self) -> B1199_W {
        B1199_W { w: self }
    }
    #[doc = "Bit 16 - B1200"]
    #[inline(always)]
    pub fn b1200(&mut self) -> B1200_W {
        B1200_W { w: self }
    }
    #[doc = "Bit 17 - B1201"]
    #[inline(always)]
    pub fn b1201(&mut self) -> B1201_W {
        B1201_W { w: self }
    }
    #[doc = "Bit 18 - B1202"]
    #[inline(always)]
    pub fn b1202(&mut self) -> B1202_W {
        B1202_W { w: self }
    }
    #[doc = "Bit 19 - B1203"]
    #[inline(always)]
    pub fn b1203(&mut self) -> B1203_W {
        B1203_W { w: self }
    }
    #[doc = "Bit 20 - B1204"]
    #[inline(always)]
    pub fn b1204(&mut self) -> B1204_W {
        B1204_W { w: self }
    }
    #[doc = "Bit 21 - B1205"]
    #[inline(always)]
    pub fn b1205(&mut self) -> B1205_W {
        B1205_W { w: self }
    }
    #[doc = "Bit 22 - B1206"]
    #[inline(always)]
    pub fn b1206(&mut self) -> B1206_W {
        B1206_W { w: self }
    }
    #[doc = "Bit 23 - B1207"]
    #[inline(always)]
    pub fn b1207(&mut self) -> B1207_W {
        B1207_W { w: self }
    }
    #[doc = "Bit 24 - B1208"]
    #[inline(always)]
    pub fn b1208(&mut self) -> B1208_W {
        B1208_W { w: self }
    }
    #[doc = "Bit 25 - B1209"]
    #[inline(always)]
    pub fn b1209(&mut self) -> B1209_W {
        B1209_W { w: self }
    }
    #[doc = "Bit 26 - B1210"]
    #[inline(always)]
    pub fn b1210(&mut self) -> B1210_W {
        B1210_W { w: self }
    }
    #[doc = "Bit 27 - B1211"]
    #[inline(always)]
    pub fn b1211(&mut self) -> B1211_W {
        B1211_W { w: self }
    }
    #[doc = "Bit 28 - B1212"]
    #[inline(always)]
    pub fn b1212(&mut self) -> B1212_W {
        B1212_W { w: self }
    }
    #[doc = "Bit 29 - B1213"]
    #[inline(always)]
    pub fn b1213(&mut self) -> B1213_W {
        B1213_W { w: self }
    }
    #[doc = "Bit 30 - B1214"]
    #[inline(always)]
    pub fn b1214(&mut self) -> B1214_W {
        B1214_W { w: self }
    }
    #[doc = "Bit 31 - B1215"]
    #[inline(always)]
    pub fn b1215(&mut self) -> B1215_W {
        B1215_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr37](index.html) module"]
pub struct MPCBB1_VCTR37_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR37_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr37::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR37_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr37::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR37_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR37 to value 0"]
impl crate::Resettable for MPCBB1_VCTR37_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
