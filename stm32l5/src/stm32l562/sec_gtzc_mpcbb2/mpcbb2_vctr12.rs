#[doc = "Register `MPCBB2_VCTR12` reader"]
pub struct R(crate::R<MPCBB2_VCTR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR12` writer"]
pub struct W(crate::W<MPCBB2_VCTR12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR12_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B384` reader - B384"]
pub struct B384_R(crate::FieldReader<bool, bool>);
impl B384_R {
    pub(crate) fn new(bits: bool) -> Self {
        B384_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B384_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B384` writer - B384"]
pub struct B384_W<'a> {
    w: &'a mut W,
}
impl<'a> B384_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B385` reader - B385"]
pub struct B385_R(crate::FieldReader<bool, bool>);
impl B385_R {
    pub(crate) fn new(bits: bool) -> Self {
        B385_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B385_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B385` writer - B385"]
pub struct B385_W<'a> {
    w: &'a mut W,
}
impl<'a> B385_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B386` reader - B386"]
pub struct B386_R(crate::FieldReader<bool, bool>);
impl B386_R {
    pub(crate) fn new(bits: bool) -> Self {
        B386_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B386_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B386` writer - B386"]
pub struct B386_W<'a> {
    w: &'a mut W,
}
impl<'a> B386_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B387` reader - B387"]
pub struct B387_R(crate::FieldReader<bool, bool>);
impl B387_R {
    pub(crate) fn new(bits: bool) -> Self {
        B387_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B387_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B387` writer - B387"]
pub struct B387_W<'a> {
    w: &'a mut W,
}
impl<'a> B387_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B388` reader - B388"]
pub struct B388_R(crate::FieldReader<bool, bool>);
impl B388_R {
    pub(crate) fn new(bits: bool) -> Self {
        B388_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B388_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B388` writer - B388"]
pub struct B388_W<'a> {
    w: &'a mut W,
}
impl<'a> B388_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B389` reader - B389"]
pub struct B389_R(crate::FieldReader<bool, bool>);
impl B389_R {
    pub(crate) fn new(bits: bool) -> Self {
        B389_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B389_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B389` writer - B389"]
pub struct B389_W<'a> {
    w: &'a mut W,
}
impl<'a> B389_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B390` reader - B390"]
pub struct B390_R(crate::FieldReader<bool, bool>);
impl B390_R {
    pub(crate) fn new(bits: bool) -> Self {
        B390_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B390_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B390` writer - B390"]
pub struct B390_W<'a> {
    w: &'a mut W,
}
impl<'a> B390_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B391` reader - B391"]
pub struct B391_R(crate::FieldReader<bool, bool>);
impl B391_R {
    pub(crate) fn new(bits: bool) -> Self {
        B391_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B391_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B391` writer - B391"]
pub struct B391_W<'a> {
    w: &'a mut W,
}
impl<'a> B391_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B392` reader - B392"]
pub struct B392_R(crate::FieldReader<bool, bool>);
impl B392_R {
    pub(crate) fn new(bits: bool) -> Self {
        B392_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B392_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B392` writer - B392"]
pub struct B392_W<'a> {
    w: &'a mut W,
}
impl<'a> B392_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B393` reader - B393"]
pub struct B393_R(crate::FieldReader<bool, bool>);
impl B393_R {
    pub(crate) fn new(bits: bool) -> Self {
        B393_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B393_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B393` writer - B393"]
pub struct B393_W<'a> {
    w: &'a mut W,
}
impl<'a> B393_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B394` reader - B394"]
pub struct B394_R(crate::FieldReader<bool, bool>);
impl B394_R {
    pub(crate) fn new(bits: bool) -> Self {
        B394_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B394_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B394` writer - B394"]
pub struct B394_W<'a> {
    w: &'a mut W,
}
impl<'a> B394_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B395` reader - B395"]
pub struct B395_R(crate::FieldReader<bool, bool>);
impl B395_R {
    pub(crate) fn new(bits: bool) -> Self {
        B395_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B395_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B395` writer - B395"]
pub struct B395_W<'a> {
    w: &'a mut W,
}
impl<'a> B395_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B396` reader - B396"]
pub struct B396_R(crate::FieldReader<bool, bool>);
impl B396_R {
    pub(crate) fn new(bits: bool) -> Self {
        B396_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B396_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B396` writer - B396"]
pub struct B396_W<'a> {
    w: &'a mut W,
}
impl<'a> B396_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B397` reader - B397"]
pub struct B397_R(crate::FieldReader<bool, bool>);
impl B397_R {
    pub(crate) fn new(bits: bool) -> Self {
        B397_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B397_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B397` writer - B397"]
pub struct B397_W<'a> {
    w: &'a mut W,
}
impl<'a> B397_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B398` reader - B398"]
pub struct B398_R(crate::FieldReader<bool, bool>);
impl B398_R {
    pub(crate) fn new(bits: bool) -> Self {
        B398_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B398_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B398` writer - B398"]
pub struct B398_W<'a> {
    w: &'a mut W,
}
impl<'a> B398_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B399` reader - B399"]
pub struct B399_R(crate::FieldReader<bool, bool>);
impl B399_R {
    pub(crate) fn new(bits: bool) -> Self {
        B399_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B399_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B399` writer - B399"]
pub struct B399_W<'a> {
    w: &'a mut W,
}
impl<'a> B399_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B400` reader - B400"]
pub struct B400_R(crate::FieldReader<bool, bool>);
impl B400_R {
    pub(crate) fn new(bits: bool) -> Self {
        B400_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B400_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B400` writer - B400"]
pub struct B400_W<'a> {
    w: &'a mut W,
}
impl<'a> B400_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B401` reader - B401"]
pub struct B401_R(crate::FieldReader<bool, bool>);
impl B401_R {
    pub(crate) fn new(bits: bool) -> Self {
        B401_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B401_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B401` writer - B401"]
pub struct B401_W<'a> {
    w: &'a mut W,
}
impl<'a> B401_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B402` reader - B402"]
pub struct B402_R(crate::FieldReader<bool, bool>);
impl B402_R {
    pub(crate) fn new(bits: bool) -> Self {
        B402_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B402_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B402` writer - B402"]
pub struct B402_W<'a> {
    w: &'a mut W,
}
impl<'a> B402_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B403` reader - B403"]
pub struct B403_R(crate::FieldReader<bool, bool>);
impl B403_R {
    pub(crate) fn new(bits: bool) -> Self {
        B403_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B403_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B403` writer - B403"]
pub struct B403_W<'a> {
    w: &'a mut W,
}
impl<'a> B403_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B404` reader - B404"]
pub struct B404_R(crate::FieldReader<bool, bool>);
impl B404_R {
    pub(crate) fn new(bits: bool) -> Self {
        B404_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B404_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B404` writer - B404"]
pub struct B404_W<'a> {
    w: &'a mut W,
}
impl<'a> B404_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B405` reader - B405"]
pub struct B405_R(crate::FieldReader<bool, bool>);
impl B405_R {
    pub(crate) fn new(bits: bool) -> Self {
        B405_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B405_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B405` writer - B405"]
pub struct B405_W<'a> {
    w: &'a mut W,
}
impl<'a> B405_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B406` reader - B406"]
pub struct B406_R(crate::FieldReader<bool, bool>);
impl B406_R {
    pub(crate) fn new(bits: bool) -> Self {
        B406_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B406_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B406` writer - B406"]
pub struct B406_W<'a> {
    w: &'a mut W,
}
impl<'a> B406_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B407` reader - B407"]
pub struct B407_R(crate::FieldReader<bool, bool>);
impl B407_R {
    pub(crate) fn new(bits: bool) -> Self {
        B407_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B407_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B407` writer - B407"]
pub struct B407_W<'a> {
    w: &'a mut W,
}
impl<'a> B407_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B408` reader - B408"]
pub struct B408_R(crate::FieldReader<bool, bool>);
impl B408_R {
    pub(crate) fn new(bits: bool) -> Self {
        B408_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B408_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B408` writer - B408"]
pub struct B408_W<'a> {
    w: &'a mut W,
}
impl<'a> B408_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B409` reader - B409"]
pub struct B409_R(crate::FieldReader<bool, bool>);
impl B409_R {
    pub(crate) fn new(bits: bool) -> Self {
        B409_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B409_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B409` writer - B409"]
pub struct B409_W<'a> {
    w: &'a mut W,
}
impl<'a> B409_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B410` reader - B410"]
pub struct B410_R(crate::FieldReader<bool, bool>);
impl B410_R {
    pub(crate) fn new(bits: bool) -> Self {
        B410_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B410_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B410` writer - B410"]
pub struct B410_W<'a> {
    w: &'a mut W,
}
impl<'a> B410_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B411` reader - B411"]
pub struct B411_R(crate::FieldReader<bool, bool>);
impl B411_R {
    pub(crate) fn new(bits: bool) -> Self {
        B411_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B411_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B411` writer - B411"]
pub struct B411_W<'a> {
    w: &'a mut W,
}
impl<'a> B411_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B412` reader - B412"]
pub struct B412_R(crate::FieldReader<bool, bool>);
impl B412_R {
    pub(crate) fn new(bits: bool) -> Self {
        B412_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B412_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B412` writer - B412"]
pub struct B412_W<'a> {
    w: &'a mut W,
}
impl<'a> B412_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B413` reader - B413"]
pub struct B413_R(crate::FieldReader<bool, bool>);
impl B413_R {
    pub(crate) fn new(bits: bool) -> Self {
        B413_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B413_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B413` writer - B413"]
pub struct B413_W<'a> {
    w: &'a mut W,
}
impl<'a> B413_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B414` reader - B414"]
pub struct B414_R(crate::FieldReader<bool, bool>);
impl B414_R {
    pub(crate) fn new(bits: bool) -> Self {
        B414_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B414_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B414` writer - B414"]
pub struct B414_W<'a> {
    w: &'a mut W,
}
impl<'a> B414_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B415` reader - B415"]
pub struct B415_R(crate::FieldReader<bool, bool>);
impl B415_R {
    pub(crate) fn new(bits: bool) -> Self {
        B415_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B415_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B415` writer - B415"]
pub struct B415_W<'a> {
    w: &'a mut W,
}
impl<'a> B415_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B384"]
    #[inline(always)]
    pub fn b384(&self) -> B384_R {
        B384_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B385"]
    #[inline(always)]
    pub fn b385(&self) -> B385_R {
        B385_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B386"]
    #[inline(always)]
    pub fn b386(&self) -> B386_R {
        B386_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B387"]
    #[inline(always)]
    pub fn b387(&self) -> B387_R {
        B387_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B388"]
    #[inline(always)]
    pub fn b388(&self) -> B388_R {
        B388_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B389"]
    #[inline(always)]
    pub fn b389(&self) -> B389_R {
        B389_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B390"]
    #[inline(always)]
    pub fn b390(&self) -> B390_R {
        B390_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B391"]
    #[inline(always)]
    pub fn b391(&self) -> B391_R {
        B391_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B392"]
    #[inline(always)]
    pub fn b392(&self) -> B392_R {
        B392_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B393"]
    #[inline(always)]
    pub fn b393(&self) -> B393_R {
        B393_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B394"]
    #[inline(always)]
    pub fn b394(&self) -> B394_R {
        B394_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B395"]
    #[inline(always)]
    pub fn b395(&self) -> B395_R {
        B395_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B396"]
    #[inline(always)]
    pub fn b396(&self) -> B396_R {
        B396_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B397"]
    #[inline(always)]
    pub fn b397(&self) -> B397_R {
        B397_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B398"]
    #[inline(always)]
    pub fn b398(&self) -> B398_R {
        B398_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B399"]
    #[inline(always)]
    pub fn b399(&self) -> B399_R {
        B399_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B400"]
    #[inline(always)]
    pub fn b400(&self) -> B400_R {
        B400_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B401"]
    #[inline(always)]
    pub fn b401(&self) -> B401_R {
        B401_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B402"]
    #[inline(always)]
    pub fn b402(&self) -> B402_R {
        B402_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B403"]
    #[inline(always)]
    pub fn b403(&self) -> B403_R {
        B403_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B404"]
    #[inline(always)]
    pub fn b404(&self) -> B404_R {
        B404_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B405"]
    #[inline(always)]
    pub fn b405(&self) -> B405_R {
        B405_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B406"]
    #[inline(always)]
    pub fn b406(&self) -> B406_R {
        B406_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B407"]
    #[inline(always)]
    pub fn b407(&self) -> B407_R {
        B407_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B408"]
    #[inline(always)]
    pub fn b408(&self) -> B408_R {
        B408_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B409"]
    #[inline(always)]
    pub fn b409(&self) -> B409_R {
        B409_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B410"]
    #[inline(always)]
    pub fn b410(&self) -> B410_R {
        B410_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B411"]
    #[inline(always)]
    pub fn b411(&self) -> B411_R {
        B411_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B412"]
    #[inline(always)]
    pub fn b412(&self) -> B412_R {
        B412_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B413"]
    #[inline(always)]
    pub fn b413(&self) -> B413_R {
        B413_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B414"]
    #[inline(always)]
    pub fn b414(&self) -> B414_R {
        B414_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B415"]
    #[inline(always)]
    pub fn b415(&self) -> B415_R {
        B415_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B384"]
    #[inline(always)]
    pub fn b384(&mut self) -> B384_W {
        B384_W { w: self }
    }
    #[doc = "Bit 1 - B385"]
    #[inline(always)]
    pub fn b385(&mut self) -> B385_W {
        B385_W { w: self }
    }
    #[doc = "Bit 2 - B386"]
    #[inline(always)]
    pub fn b386(&mut self) -> B386_W {
        B386_W { w: self }
    }
    #[doc = "Bit 3 - B387"]
    #[inline(always)]
    pub fn b387(&mut self) -> B387_W {
        B387_W { w: self }
    }
    #[doc = "Bit 4 - B388"]
    #[inline(always)]
    pub fn b388(&mut self) -> B388_W {
        B388_W { w: self }
    }
    #[doc = "Bit 5 - B389"]
    #[inline(always)]
    pub fn b389(&mut self) -> B389_W {
        B389_W { w: self }
    }
    #[doc = "Bit 6 - B390"]
    #[inline(always)]
    pub fn b390(&mut self) -> B390_W {
        B390_W { w: self }
    }
    #[doc = "Bit 7 - B391"]
    #[inline(always)]
    pub fn b391(&mut self) -> B391_W {
        B391_W { w: self }
    }
    #[doc = "Bit 8 - B392"]
    #[inline(always)]
    pub fn b392(&mut self) -> B392_W {
        B392_W { w: self }
    }
    #[doc = "Bit 9 - B393"]
    #[inline(always)]
    pub fn b393(&mut self) -> B393_W {
        B393_W { w: self }
    }
    #[doc = "Bit 10 - B394"]
    #[inline(always)]
    pub fn b394(&mut self) -> B394_W {
        B394_W { w: self }
    }
    #[doc = "Bit 11 - B395"]
    #[inline(always)]
    pub fn b395(&mut self) -> B395_W {
        B395_W { w: self }
    }
    #[doc = "Bit 12 - B396"]
    #[inline(always)]
    pub fn b396(&mut self) -> B396_W {
        B396_W { w: self }
    }
    #[doc = "Bit 13 - B397"]
    #[inline(always)]
    pub fn b397(&mut self) -> B397_W {
        B397_W { w: self }
    }
    #[doc = "Bit 14 - B398"]
    #[inline(always)]
    pub fn b398(&mut self) -> B398_W {
        B398_W { w: self }
    }
    #[doc = "Bit 15 - B399"]
    #[inline(always)]
    pub fn b399(&mut self) -> B399_W {
        B399_W { w: self }
    }
    #[doc = "Bit 16 - B400"]
    #[inline(always)]
    pub fn b400(&mut self) -> B400_W {
        B400_W { w: self }
    }
    #[doc = "Bit 17 - B401"]
    #[inline(always)]
    pub fn b401(&mut self) -> B401_W {
        B401_W { w: self }
    }
    #[doc = "Bit 18 - B402"]
    #[inline(always)]
    pub fn b402(&mut self) -> B402_W {
        B402_W { w: self }
    }
    #[doc = "Bit 19 - B403"]
    #[inline(always)]
    pub fn b403(&mut self) -> B403_W {
        B403_W { w: self }
    }
    #[doc = "Bit 20 - B404"]
    #[inline(always)]
    pub fn b404(&mut self) -> B404_W {
        B404_W { w: self }
    }
    #[doc = "Bit 21 - B405"]
    #[inline(always)]
    pub fn b405(&mut self) -> B405_W {
        B405_W { w: self }
    }
    #[doc = "Bit 22 - B406"]
    #[inline(always)]
    pub fn b406(&mut self) -> B406_W {
        B406_W { w: self }
    }
    #[doc = "Bit 23 - B407"]
    #[inline(always)]
    pub fn b407(&mut self) -> B407_W {
        B407_W { w: self }
    }
    #[doc = "Bit 24 - B408"]
    #[inline(always)]
    pub fn b408(&mut self) -> B408_W {
        B408_W { w: self }
    }
    #[doc = "Bit 25 - B409"]
    #[inline(always)]
    pub fn b409(&mut self) -> B409_W {
        B409_W { w: self }
    }
    #[doc = "Bit 26 - B410"]
    #[inline(always)]
    pub fn b410(&mut self) -> B410_W {
        B410_W { w: self }
    }
    #[doc = "Bit 27 - B411"]
    #[inline(always)]
    pub fn b411(&mut self) -> B411_W {
        B411_W { w: self }
    }
    #[doc = "Bit 28 - B412"]
    #[inline(always)]
    pub fn b412(&mut self) -> B412_W {
        B412_W { w: self }
    }
    #[doc = "Bit 29 - B413"]
    #[inline(always)]
    pub fn b413(&mut self) -> B413_W {
        B413_W { w: self }
    }
    #[doc = "Bit 30 - B414"]
    #[inline(always)]
    pub fn b414(&mut self) -> B414_W {
        B414_W { w: self }
    }
    #[doc = "Bit 31 - B415"]
    #[inline(always)]
    pub fn b415(&mut self) -> B415_W {
        B415_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr12](index.html) module"]
pub struct MPCBB2_VCTR12_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr12::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr12::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR12 to value 0"]
impl crate::Resettable for MPCBB2_VCTR12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
