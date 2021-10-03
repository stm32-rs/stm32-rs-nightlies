#[doc = "Register `MPCBB1_VCTR8` reader"]
pub struct R(crate::R<MPCBB1_VCTR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR8` writer"]
pub struct W(crate::W<MPCBB1_VCTR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR8_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B256` reader - B256"]
pub struct B256_R(crate::FieldReader<bool, bool>);
impl B256_R {
    pub(crate) fn new(bits: bool) -> Self {
        B256_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B256_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B256` writer - B256"]
pub struct B256_W<'a> {
    w: &'a mut W,
}
impl<'a> B256_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B257` reader - B257"]
pub struct B257_R(crate::FieldReader<bool, bool>);
impl B257_R {
    pub(crate) fn new(bits: bool) -> Self {
        B257_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B257_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B257` writer - B257"]
pub struct B257_W<'a> {
    w: &'a mut W,
}
impl<'a> B257_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B258` reader - B258"]
pub struct B258_R(crate::FieldReader<bool, bool>);
impl B258_R {
    pub(crate) fn new(bits: bool) -> Self {
        B258_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B258_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B258` writer - B258"]
pub struct B258_W<'a> {
    w: &'a mut W,
}
impl<'a> B258_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B259` reader - B259"]
pub struct B259_R(crate::FieldReader<bool, bool>);
impl B259_R {
    pub(crate) fn new(bits: bool) -> Self {
        B259_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B259_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B259` writer - B259"]
pub struct B259_W<'a> {
    w: &'a mut W,
}
impl<'a> B259_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B260` reader - B260"]
pub struct B260_R(crate::FieldReader<bool, bool>);
impl B260_R {
    pub(crate) fn new(bits: bool) -> Self {
        B260_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B260_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B260` writer - B260"]
pub struct B260_W<'a> {
    w: &'a mut W,
}
impl<'a> B260_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B261` reader - B261"]
pub struct B261_R(crate::FieldReader<bool, bool>);
impl B261_R {
    pub(crate) fn new(bits: bool) -> Self {
        B261_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B261_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B261` writer - B261"]
pub struct B261_W<'a> {
    w: &'a mut W,
}
impl<'a> B261_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B262` reader - B262"]
pub struct B262_R(crate::FieldReader<bool, bool>);
impl B262_R {
    pub(crate) fn new(bits: bool) -> Self {
        B262_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B262_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B262` writer - B262"]
pub struct B262_W<'a> {
    w: &'a mut W,
}
impl<'a> B262_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B263` reader - B263"]
pub struct B263_R(crate::FieldReader<bool, bool>);
impl B263_R {
    pub(crate) fn new(bits: bool) -> Self {
        B263_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B263_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B263` writer - B263"]
pub struct B263_W<'a> {
    w: &'a mut W,
}
impl<'a> B263_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B264` reader - B264"]
pub struct B264_R(crate::FieldReader<bool, bool>);
impl B264_R {
    pub(crate) fn new(bits: bool) -> Self {
        B264_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B264_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B264` writer - B264"]
pub struct B264_W<'a> {
    w: &'a mut W,
}
impl<'a> B264_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B265` reader - B265"]
pub struct B265_R(crate::FieldReader<bool, bool>);
impl B265_R {
    pub(crate) fn new(bits: bool) -> Self {
        B265_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B265_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B265` writer - B265"]
pub struct B265_W<'a> {
    w: &'a mut W,
}
impl<'a> B265_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B266` reader - B266"]
pub struct B266_R(crate::FieldReader<bool, bool>);
impl B266_R {
    pub(crate) fn new(bits: bool) -> Self {
        B266_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B266_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B266` writer - B266"]
pub struct B266_W<'a> {
    w: &'a mut W,
}
impl<'a> B266_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B267` reader - B267"]
pub struct B267_R(crate::FieldReader<bool, bool>);
impl B267_R {
    pub(crate) fn new(bits: bool) -> Self {
        B267_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B267_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B267` writer - B267"]
pub struct B267_W<'a> {
    w: &'a mut W,
}
impl<'a> B267_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B268` reader - B268"]
pub struct B268_R(crate::FieldReader<bool, bool>);
impl B268_R {
    pub(crate) fn new(bits: bool) -> Self {
        B268_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B268_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B268` writer - B268"]
pub struct B268_W<'a> {
    w: &'a mut W,
}
impl<'a> B268_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B269` reader - B269"]
pub struct B269_R(crate::FieldReader<bool, bool>);
impl B269_R {
    pub(crate) fn new(bits: bool) -> Self {
        B269_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B269_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B269` writer - B269"]
pub struct B269_W<'a> {
    w: &'a mut W,
}
impl<'a> B269_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B270` reader - B270"]
pub struct B270_R(crate::FieldReader<bool, bool>);
impl B270_R {
    pub(crate) fn new(bits: bool) -> Self {
        B270_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B270_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B270` writer - B270"]
pub struct B270_W<'a> {
    w: &'a mut W,
}
impl<'a> B270_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B271` reader - B271"]
pub struct B271_R(crate::FieldReader<bool, bool>);
impl B271_R {
    pub(crate) fn new(bits: bool) -> Self {
        B271_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B271_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B271` writer - B271"]
pub struct B271_W<'a> {
    w: &'a mut W,
}
impl<'a> B271_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B272` reader - B272"]
pub struct B272_R(crate::FieldReader<bool, bool>);
impl B272_R {
    pub(crate) fn new(bits: bool) -> Self {
        B272_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B272_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B272` writer - B272"]
pub struct B272_W<'a> {
    w: &'a mut W,
}
impl<'a> B272_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B273` reader - B273"]
pub struct B273_R(crate::FieldReader<bool, bool>);
impl B273_R {
    pub(crate) fn new(bits: bool) -> Self {
        B273_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B273_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B273` writer - B273"]
pub struct B273_W<'a> {
    w: &'a mut W,
}
impl<'a> B273_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B274` reader - B274"]
pub struct B274_R(crate::FieldReader<bool, bool>);
impl B274_R {
    pub(crate) fn new(bits: bool) -> Self {
        B274_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B274_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B274` writer - B274"]
pub struct B274_W<'a> {
    w: &'a mut W,
}
impl<'a> B274_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B275` reader - B275"]
pub struct B275_R(crate::FieldReader<bool, bool>);
impl B275_R {
    pub(crate) fn new(bits: bool) -> Self {
        B275_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B275_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B275` writer - B275"]
pub struct B275_W<'a> {
    w: &'a mut W,
}
impl<'a> B275_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B276` reader - B276"]
pub struct B276_R(crate::FieldReader<bool, bool>);
impl B276_R {
    pub(crate) fn new(bits: bool) -> Self {
        B276_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B276_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B276` writer - B276"]
pub struct B276_W<'a> {
    w: &'a mut W,
}
impl<'a> B276_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B277` reader - B277"]
pub struct B277_R(crate::FieldReader<bool, bool>);
impl B277_R {
    pub(crate) fn new(bits: bool) -> Self {
        B277_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B277_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B277` writer - B277"]
pub struct B277_W<'a> {
    w: &'a mut W,
}
impl<'a> B277_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B278` reader - B278"]
pub struct B278_R(crate::FieldReader<bool, bool>);
impl B278_R {
    pub(crate) fn new(bits: bool) -> Self {
        B278_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B278_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B278` writer - B278"]
pub struct B278_W<'a> {
    w: &'a mut W,
}
impl<'a> B278_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B279` reader - B279"]
pub struct B279_R(crate::FieldReader<bool, bool>);
impl B279_R {
    pub(crate) fn new(bits: bool) -> Self {
        B279_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B279_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B279` writer - B279"]
pub struct B279_W<'a> {
    w: &'a mut W,
}
impl<'a> B279_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B280` reader - B280"]
pub struct B280_R(crate::FieldReader<bool, bool>);
impl B280_R {
    pub(crate) fn new(bits: bool) -> Self {
        B280_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B280_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B280` writer - B280"]
pub struct B280_W<'a> {
    w: &'a mut W,
}
impl<'a> B280_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B281` reader - B281"]
pub struct B281_R(crate::FieldReader<bool, bool>);
impl B281_R {
    pub(crate) fn new(bits: bool) -> Self {
        B281_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B281_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B281` writer - B281"]
pub struct B281_W<'a> {
    w: &'a mut W,
}
impl<'a> B281_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B282` reader - B282"]
pub struct B282_R(crate::FieldReader<bool, bool>);
impl B282_R {
    pub(crate) fn new(bits: bool) -> Self {
        B282_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B282_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B282` writer - B282"]
pub struct B282_W<'a> {
    w: &'a mut W,
}
impl<'a> B282_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B283` reader - B283"]
pub struct B283_R(crate::FieldReader<bool, bool>);
impl B283_R {
    pub(crate) fn new(bits: bool) -> Self {
        B283_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B283_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B283` writer - B283"]
pub struct B283_W<'a> {
    w: &'a mut W,
}
impl<'a> B283_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B284` reader - B284"]
pub struct B284_R(crate::FieldReader<bool, bool>);
impl B284_R {
    pub(crate) fn new(bits: bool) -> Self {
        B284_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B284_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B284` writer - B284"]
pub struct B284_W<'a> {
    w: &'a mut W,
}
impl<'a> B284_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B285` reader - B285"]
pub struct B285_R(crate::FieldReader<bool, bool>);
impl B285_R {
    pub(crate) fn new(bits: bool) -> Self {
        B285_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B285_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B285` writer - B285"]
pub struct B285_W<'a> {
    w: &'a mut W,
}
impl<'a> B285_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B286` reader - B286"]
pub struct B286_R(crate::FieldReader<bool, bool>);
impl B286_R {
    pub(crate) fn new(bits: bool) -> Self {
        B286_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B286_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B286` writer - B286"]
pub struct B286_W<'a> {
    w: &'a mut W,
}
impl<'a> B286_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B287` reader - B287"]
pub struct B287_R(crate::FieldReader<bool, bool>);
impl B287_R {
    pub(crate) fn new(bits: bool) -> Self {
        B287_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B287_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B287` writer - B287"]
pub struct B287_W<'a> {
    w: &'a mut W,
}
impl<'a> B287_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B256"]
    #[inline(always)]
    pub fn b256(&self) -> B256_R {
        B256_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B257"]
    #[inline(always)]
    pub fn b257(&self) -> B257_R {
        B257_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B258"]
    #[inline(always)]
    pub fn b258(&self) -> B258_R {
        B258_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B259"]
    #[inline(always)]
    pub fn b259(&self) -> B259_R {
        B259_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B260"]
    #[inline(always)]
    pub fn b260(&self) -> B260_R {
        B260_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B261"]
    #[inline(always)]
    pub fn b261(&self) -> B261_R {
        B261_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B262"]
    #[inline(always)]
    pub fn b262(&self) -> B262_R {
        B262_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B263"]
    #[inline(always)]
    pub fn b263(&self) -> B263_R {
        B263_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B264"]
    #[inline(always)]
    pub fn b264(&self) -> B264_R {
        B264_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B265"]
    #[inline(always)]
    pub fn b265(&self) -> B265_R {
        B265_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B266"]
    #[inline(always)]
    pub fn b266(&self) -> B266_R {
        B266_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B267"]
    #[inline(always)]
    pub fn b267(&self) -> B267_R {
        B267_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B268"]
    #[inline(always)]
    pub fn b268(&self) -> B268_R {
        B268_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B269"]
    #[inline(always)]
    pub fn b269(&self) -> B269_R {
        B269_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B270"]
    #[inline(always)]
    pub fn b270(&self) -> B270_R {
        B270_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B271"]
    #[inline(always)]
    pub fn b271(&self) -> B271_R {
        B271_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B272"]
    #[inline(always)]
    pub fn b272(&self) -> B272_R {
        B272_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B273"]
    #[inline(always)]
    pub fn b273(&self) -> B273_R {
        B273_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B274"]
    #[inline(always)]
    pub fn b274(&self) -> B274_R {
        B274_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B275"]
    #[inline(always)]
    pub fn b275(&self) -> B275_R {
        B275_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B276"]
    #[inline(always)]
    pub fn b276(&self) -> B276_R {
        B276_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B277"]
    #[inline(always)]
    pub fn b277(&self) -> B277_R {
        B277_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B278"]
    #[inline(always)]
    pub fn b278(&self) -> B278_R {
        B278_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B279"]
    #[inline(always)]
    pub fn b279(&self) -> B279_R {
        B279_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B280"]
    #[inline(always)]
    pub fn b280(&self) -> B280_R {
        B280_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B281"]
    #[inline(always)]
    pub fn b281(&self) -> B281_R {
        B281_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B282"]
    #[inline(always)]
    pub fn b282(&self) -> B282_R {
        B282_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B283"]
    #[inline(always)]
    pub fn b283(&self) -> B283_R {
        B283_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B284"]
    #[inline(always)]
    pub fn b284(&self) -> B284_R {
        B284_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B285"]
    #[inline(always)]
    pub fn b285(&self) -> B285_R {
        B285_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B286"]
    #[inline(always)]
    pub fn b286(&self) -> B286_R {
        B286_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B287"]
    #[inline(always)]
    pub fn b287(&self) -> B287_R {
        B287_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B256"]
    #[inline(always)]
    pub fn b256(&mut self) -> B256_W {
        B256_W { w: self }
    }
    #[doc = "Bit 1 - B257"]
    #[inline(always)]
    pub fn b257(&mut self) -> B257_W {
        B257_W { w: self }
    }
    #[doc = "Bit 2 - B258"]
    #[inline(always)]
    pub fn b258(&mut self) -> B258_W {
        B258_W { w: self }
    }
    #[doc = "Bit 3 - B259"]
    #[inline(always)]
    pub fn b259(&mut self) -> B259_W {
        B259_W { w: self }
    }
    #[doc = "Bit 4 - B260"]
    #[inline(always)]
    pub fn b260(&mut self) -> B260_W {
        B260_W { w: self }
    }
    #[doc = "Bit 5 - B261"]
    #[inline(always)]
    pub fn b261(&mut self) -> B261_W {
        B261_W { w: self }
    }
    #[doc = "Bit 6 - B262"]
    #[inline(always)]
    pub fn b262(&mut self) -> B262_W {
        B262_W { w: self }
    }
    #[doc = "Bit 7 - B263"]
    #[inline(always)]
    pub fn b263(&mut self) -> B263_W {
        B263_W { w: self }
    }
    #[doc = "Bit 8 - B264"]
    #[inline(always)]
    pub fn b264(&mut self) -> B264_W {
        B264_W { w: self }
    }
    #[doc = "Bit 9 - B265"]
    #[inline(always)]
    pub fn b265(&mut self) -> B265_W {
        B265_W { w: self }
    }
    #[doc = "Bit 10 - B266"]
    #[inline(always)]
    pub fn b266(&mut self) -> B266_W {
        B266_W { w: self }
    }
    #[doc = "Bit 11 - B267"]
    #[inline(always)]
    pub fn b267(&mut self) -> B267_W {
        B267_W { w: self }
    }
    #[doc = "Bit 12 - B268"]
    #[inline(always)]
    pub fn b268(&mut self) -> B268_W {
        B268_W { w: self }
    }
    #[doc = "Bit 13 - B269"]
    #[inline(always)]
    pub fn b269(&mut self) -> B269_W {
        B269_W { w: self }
    }
    #[doc = "Bit 14 - B270"]
    #[inline(always)]
    pub fn b270(&mut self) -> B270_W {
        B270_W { w: self }
    }
    #[doc = "Bit 15 - B271"]
    #[inline(always)]
    pub fn b271(&mut self) -> B271_W {
        B271_W { w: self }
    }
    #[doc = "Bit 16 - B272"]
    #[inline(always)]
    pub fn b272(&mut self) -> B272_W {
        B272_W { w: self }
    }
    #[doc = "Bit 17 - B273"]
    #[inline(always)]
    pub fn b273(&mut self) -> B273_W {
        B273_W { w: self }
    }
    #[doc = "Bit 18 - B274"]
    #[inline(always)]
    pub fn b274(&mut self) -> B274_W {
        B274_W { w: self }
    }
    #[doc = "Bit 19 - B275"]
    #[inline(always)]
    pub fn b275(&mut self) -> B275_W {
        B275_W { w: self }
    }
    #[doc = "Bit 20 - B276"]
    #[inline(always)]
    pub fn b276(&mut self) -> B276_W {
        B276_W { w: self }
    }
    #[doc = "Bit 21 - B277"]
    #[inline(always)]
    pub fn b277(&mut self) -> B277_W {
        B277_W { w: self }
    }
    #[doc = "Bit 22 - B278"]
    #[inline(always)]
    pub fn b278(&mut self) -> B278_W {
        B278_W { w: self }
    }
    #[doc = "Bit 23 - B279"]
    #[inline(always)]
    pub fn b279(&mut self) -> B279_W {
        B279_W { w: self }
    }
    #[doc = "Bit 24 - B280"]
    #[inline(always)]
    pub fn b280(&mut self) -> B280_W {
        B280_W { w: self }
    }
    #[doc = "Bit 25 - B281"]
    #[inline(always)]
    pub fn b281(&mut self) -> B281_W {
        B281_W { w: self }
    }
    #[doc = "Bit 26 - B282"]
    #[inline(always)]
    pub fn b282(&mut self) -> B282_W {
        B282_W { w: self }
    }
    #[doc = "Bit 27 - B283"]
    #[inline(always)]
    pub fn b283(&mut self) -> B283_W {
        B283_W { w: self }
    }
    #[doc = "Bit 28 - B284"]
    #[inline(always)]
    pub fn b284(&mut self) -> B284_W {
        B284_W { w: self }
    }
    #[doc = "Bit 29 - B285"]
    #[inline(always)]
    pub fn b285(&mut self) -> B285_W {
        B285_W { w: self }
    }
    #[doc = "Bit 30 - B286"]
    #[inline(always)]
    pub fn b286(&mut self) -> B286_W {
        B286_W { w: self }
    }
    #[doc = "Bit 31 - B287"]
    #[inline(always)]
    pub fn b287(&mut self) -> B287_W {
        B287_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr8](index.html) module"]
pub struct MPCBB1_VCTR8_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr8::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr8::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR8 to value 0"]
impl crate::Resettable for MPCBB1_VCTR8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
