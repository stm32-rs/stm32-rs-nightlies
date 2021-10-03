#[doc = "Register `MPCBB1_VCTR11` reader"]
pub struct R(crate::R<MPCBB1_VCTR11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR11` writer"]
pub struct W(crate::W<MPCBB1_VCTR11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR11_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B352` reader - B352"]
pub struct B352_R(crate::FieldReader<bool, bool>);
impl B352_R {
    pub(crate) fn new(bits: bool) -> Self {
        B352_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B352_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B352` writer - B352"]
pub struct B352_W<'a> {
    w: &'a mut W,
}
impl<'a> B352_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B353` reader - B353"]
pub struct B353_R(crate::FieldReader<bool, bool>);
impl B353_R {
    pub(crate) fn new(bits: bool) -> Self {
        B353_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B353_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B353` writer - B353"]
pub struct B353_W<'a> {
    w: &'a mut W,
}
impl<'a> B353_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B354` reader - B354"]
pub struct B354_R(crate::FieldReader<bool, bool>);
impl B354_R {
    pub(crate) fn new(bits: bool) -> Self {
        B354_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B354_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B354` writer - B354"]
pub struct B354_W<'a> {
    w: &'a mut W,
}
impl<'a> B354_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B355` reader - B355"]
pub struct B355_R(crate::FieldReader<bool, bool>);
impl B355_R {
    pub(crate) fn new(bits: bool) -> Self {
        B355_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B355_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B355` writer - B355"]
pub struct B355_W<'a> {
    w: &'a mut W,
}
impl<'a> B355_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B356` reader - B356"]
pub struct B356_R(crate::FieldReader<bool, bool>);
impl B356_R {
    pub(crate) fn new(bits: bool) -> Self {
        B356_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B356_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B356` writer - B356"]
pub struct B356_W<'a> {
    w: &'a mut W,
}
impl<'a> B356_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B357` reader - B357"]
pub struct B357_R(crate::FieldReader<bool, bool>);
impl B357_R {
    pub(crate) fn new(bits: bool) -> Self {
        B357_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B357_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B357` writer - B357"]
pub struct B357_W<'a> {
    w: &'a mut W,
}
impl<'a> B357_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B358` reader - B358"]
pub struct B358_R(crate::FieldReader<bool, bool>);
impl B358_R {
    pub(crate) fn new(bits: bool) -> Self {
        B358_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B358_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B358` writer - B358"]
pub struct B358_W<'a> {
    w: &'a mut W,
}
impl<'a> B358_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B359` reader - B359"]
pub struct B359_R(crate::FieldReader<bool, bool>);
impl B359_R {
    pub(crate) fn new(bits: bool) -> Self {
        B359_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B359_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B359` writer - B359"]
pub struct B359_W<'a> {
    w: &'a mut W,
}
impl<'a> B359_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B360` reader - B360"]
pub struct B360_R(crate::FieldReader<bool, bool>);
impl B360_R {
    pub(crate) fn new(bits: bool) -> Self {
        B360_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B360_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B360` writer - B360"]
pub struct B360_W<'a> {
    w: &'a mut W,
}
impl<'a> B360_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B361` reader - B361"]
pub struct B361_R(crate::FieldReader<bool, bool>);
impl B361_R {
    pub(crate) fn new(bits: bool) -> Self {
        B361_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B361_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B361` writer - B361"]
pub struct B361_W<'a> {
    w: &'a mut W,
}
impl<'a> B361_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B362` reader - B362"]
pub struct B362_R(crate::FieldReader<bool, bool>);
impl B362_R {
    pub(crate) fn new(bits: bool) -> Self {
        B362_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B362_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B362` writer - B362"]
pub struct B362_W<'a> {
    w: &'a mut W,
}
impl<'a> B362_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B363` reader - B363"]
pub struct B363_R(crate::FieldReader<bool, bool>);
impl B363_R {
    pub(crate) fn new(bits: bool) -> Self {
        B363_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B363_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B363` writer - B363"]
pub struct B363_W<'a> {
    w: &'a mut W,
}
impl<'a> B363_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B364` reader - B364"]
pub struct B364_R(crate::FieldReader<bool, bool>);
impl B364_R {
    pub(crate) fn new(bits: bool) -> Self {
        B364_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B364_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B364` writer - B364"]
pub struct B364_W<'a> {
    w: &'a mut W,
}
impl<'a> B364_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B365` reader - B365"]
pub struct B365_R(crate::FieldReader<bool, bool>);
impl B365_R {
    pub(crate) fn new(bits: bool) -> Self {
        B365_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B365_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B365` writer - B365"]
pub struct B365_W<'a> {
    w: &'a mut W,
}
impl<'a> B365_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B366` reader - B366"]
pub struct B366_R(crate::FieldReader<bool, bool>);
impl B366_R {
    pub(crate) fn new(bits: bool) -> Self {
        B366_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B366_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B366` writer - B366"]
pub struct B366_W<'a> {
    w: &'a mut W,
}
impl<'a> B366_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B367` reader - B367"]
pub struct B367_R(crate::FieldReader<bool, bool>);
impl B367_R {
    pub(crate) fn new(bits: bool) -> Self {
        B367_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B367_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B367` writer - B367"]
pub struct B367_W<'a> {
    w: &'a mut W,
}
impl<'a> B367_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B368` reader - B368"]
pub struct B368_R(crate::FieldReader<bool, bool>);
impl B368_R {
    pub(crate) fn new(bits: bool) -> Self {
        B368_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B368_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B368` writer - B368"]
pub struct B368_W<'a> {
    w: &'a mut W,
}
impl<'a> B368_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B369` reader - B369"]
pub struct B369_R(crate::FieldReader<bool, bool>);
impl B369_R {
    pub(crate) fn new(bits: bool) -> Self {
        B369_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B369_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B369` writer - B369"]
pub struct B369_W<'a> {
    w: &'a mut W,
}
impl<'a> B369_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B370` reader - B370"]
pub struct B370_R(crate::FieldReader<bool, bool>);
impl B370_R {
    pub(crate) fn new(bits: bool) -> Self {
        B370_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B370_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B370` writer - B370"]
pub struct B370_W<'a> {
    w: &'a mut W,
}
impl<'a> B370_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B371` reader - B371"]
pub struct B371_R(crate::FieldReader<bool, bool>);
impl B371_R {
    pub(crate) fn new(bits: bool) -> Self {
        B371_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B371_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B371` writer - B371"]
pub struct B371_W<'a> {
    w: &'a mut W,
}
impl<'a> B371_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B372` reader - B372"]
pub struct B372_R(crate::FieldReader<bool, bool>);
impl B372_R {
    pub(crate) fn new(bits: bool) -> Self {
        B372_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B372_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B372` writer - B372"]
pub struct B372_W<'a> {
    w: &'a mut W,
}
impl<'a> B372_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B373` reader - B373"]
pub struct B373_R(crate::FieldReader<bool, bool>);
impl B373_R {
    pub(crate) fn new(bits: bool) -> Self {
        B373_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B373_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B373` writer - B373"]
pub struct B373_W<'a> {
    w: &'a mut W,
}
impl<'a> B373_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B374` reader - B374"]
pub struct B374_R(crate::FieldReader<bool, bool>);
impl B374_R {
    pub(crate) fn new(bits: bool) -> Self {
        B374_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B374_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B374` writer - B374"]
pub struct B374_W<'a> {
    w: &'a mut W,
}
impl<'a> B374_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B375` reader - B375"]
pub struct B375_R(crate::FieldReader<bool, bool>);
impl B375_R {
    pub(crate) fn new(bits: bool) -> Self {
        B375_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B375_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B375` writer - B375"]
pub struct B375_W<'a> {
    w: &'a mut W,
}
impl<'a> B375_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B376` reader - B376"]
pub struct B376_R(crate::FieldReader<bool, bool>);
impl B376_R {
    pub(crate) fn new(bits: bool) -> Self {
        B376_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B376_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B376` writer - B376"]
pub struct B376_W<'a> {
    w: &'a mut W,
}
impl<'a> B376_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B377` reader - B377"]
pub struct B377_R(crate::FieldReader<bool, bool>);
impl B377_R {
    pub(crate) fn new(bits: bool) -> Self {
        B377_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B377_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B377` writer - B377"]
pub struct B377_W<'a> {
    w: &'a mut W,
}
impl<'a> B377_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B378` reader - B378"]
pub struct B378_R(crate::FieldReader<bool, bool>);
impl B378_R {
    pub(crate) fn new(bits: bool) -> Self {
        B378_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B378_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B378` writer - B378"]
pub struct B378_W<'a> {
    w: &'a mut W,
}
impl<'a> B378_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B379` reader - B379"]
pub struct B379_R(crate::FieldReader<bool, bool>);
impl B379_R {
    pub(crate) fn new(bits: bool) -> Self {
        B379_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B379_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B379` writer - B379"]
pub struct B379_W<'a> {
    w: &'a mut W,
}
impl<'a> B379_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B380` reader - B380"]
pub struct B380_R(crate::FieldReader<bool, bool>);
impl B380_R {
    pub(crate) fn new(bits: bool) -> Self {
        B380_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B380_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B380` writer - B380"]
pub struct B380_W<'a> {
    w: &'a mut W,
}
impl<'a> B380_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B381` reader - B381"]
pub struct B381_R(crate::FieldReader<bool, bool>);
impl B381_R {
    pub(crate) fn new(bits: bool) -> Self {
        B381_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B381_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B381` writer - B381"]
pub struct B381_W<'a> {
    w: &'a mut W,
}
impl<'a> B381_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B382` reader - B382"]
pub struct B382_R(crate::FieldReader<bool, bool>);
impl B382_R {
    pub(crate) fn new(bits: bool) -> Self {
        B382_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B382_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B382` writer - B382"]
pub struct B382_W<'a> {
    w: &'a mut W,
}
impl<'a> B382_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B383` reader - B383"]
pub struct B383_R(crate::FieldReader<bool, bool>);
impl B383_R {
    pub(crate) fn new(bits: bool) -> Self {
        B383_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B383_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B383` writer - B383"]
pub struct B383_W<'a> {
    w: &'a mut W,
}
impl<'a> B383_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B352"]
    #[inline(always)]
    pub fn b352(&self) -> B352_R {
        B352_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B353"]
    #[inline(always)]
    pub fn b353(&self) -> B353_R {
        B353_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B354"]
    #[inline(always)]
    pub fn b354(&self) -> B354_R {
        B354_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B355"]
    #[inline(always)]
    pub fn b355(&self) -> B355_R {
        B355_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B356"]
    #[inline(always)]
    pub fn b356(&self) -> B356_R {
        B356_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B357"]
    #[inline(always)]
    pub fn b357(&self) -> B357_R {
        B357_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B358"]
    #[inline(always)]
    pub fn b358(&self) -> B358_R {
        B358_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B359"]
    #[inline(always)]
    pub fn b359(&self) -> B359_R {
        B359_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B360"]
    #[inline(always)]
    pub fn b360(&self) -> B360_R {
        B360_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B361"]
    #[inline(always)]
    pub fn b361(&self) -> B361_R {
        B361_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B362"]
    #[inline(always)]
    pub fn b362(&self) -> B362_R {
        B362_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B363"]
    #[inline(always)]
    pub fn b363(&self) -> B363_R {
        B363_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B364"]
    #[inline(always)]
    pub fn b364(&self) -> B364_R {
        B364_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B365"]
    #[inline(always)]
    pub fn b365(&self) -> B365_R {
        B365_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B366"]
    #[inline(always)]
    pub fn b366(&self) -> B366_R {
        B366_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B367"]
    #[inline(always)]
    pub fn b367(&self) -> B367_R {
        B367_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B368"]
    #[inline(always)]
    pub fn b368(&self) -> B368_R {
        B368_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B369"]
    #[inline(always)]
    pub fn b369(&self) -> B369_R {
        B369_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B370"]
    #[inline(always)]
    pub fn b370(&self) -> B370_R {
        B370_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B371"]
    #[inline(always)]
    pub fn b371(&self) -> B371_R {
        B371_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B372"]
    #[inline(always)]
    pub fn b372(&self) -> B372_R {
        B372_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B373"]
    #[inline(always)]
    pub fn b373(&self) -> B373_R {
        B373_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B374"]
    #[inline(always)]
    pub fn b374(&self) -> B374_R {
        B374_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B375"]
    #[inline(always)]
    pub fn b375(&self) -> B375_R {
        B375_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B376"]
    #[inline(always)]
    pub fn b376(&self) -> B376_R {
        B376_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B377"]
    #[inline(always)]
    pub fn b377(&self) -> B377_R {
        B377_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B378"]
    #[inline(always)]
    pub fn b378(&self) -> B378_R {
        B378_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B379"]
    #[inline(always)]
    pub fn b379(&self) -> B379_R {
        B379_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B380"]
    #[inline(always)]
    pub fn b380(&self) -> B380_R {
        B380_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B381"]
    #[inline(always)]
    pub fn b381(&self) -> B381_R {
        B381_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B382"]
    #[inline(always)]
    pub fn b382(&self) -> B382_R {
        B382_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B383"]
    #[inline(always)]
    pub fn b383(&self) -> B383_R {
        B383_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B352"]
    #[inline(always)]
    pub fn b352(&mut self) -> B352_W {
        B352_W { w: self }
    }
    #[doc = "Bit 1 - B353"]
    #[inline(always)]
    pub fn b353(&mut self) -> B353_W {
        B353_W { w: self }
    }
    #[doc = "Bit 2 - B354"]
    #[inline(always)]
    pub fn b354(&mut self) -> B354_W {
        B354_W { w: self }
    }
    #[doc = "Bit 3 - B355"]
    #[inline(always)]
    pub fn b355(&mut self) -> B355_W {
        B355_W { w: self }
    }
    #[doc = "Bit 4 - B356"]
    #[inline(always)]
    pub fn b356(&mut self) -> B356_W {
        B356_W { w: self }
    }
    #[doc = "Bit 5 - B357"]
    #[inline(always)]
    pub fn b357(&mut self) -> B357_W {
        B357_W { w: self }
    }
    #[doc = "Bit 6 - B358"]
    #[inline(always)]
    pub fn b358(&mut self) -> B358_W {
        B358_W { w: self }
    }
    #[doc = "Bit 7 - B359"]
    #[inline(always)]
    pub fn b359(&mut self) -> B359_W {
        B359_W { w: self }
    }
    #[doc = "Bit 8 - B360"]
    #[inline(always)]
    pub fn b360(&mut self) -> B360_W {
        B360_W { w: self }
    }
    #[doc = "Bit 9 - B361"]
    #[inline(always)]
    pub fn b361(&mut self) -> B361_W {
        B361_W { w: self }
    }
    #[doc = "Bit 10 - B362"]
    #[inline(always)]
    pub fn b362(&mut self) -> B362_W {
        B362_W { w: self }
    }
    #[doc = "Bit 11 - B363"]
    #[inline(always)]
    pub fn b363(&mut self) -> B363_W {
        B363_W { w: self }
    }
    #[doc = "Bit 12 - B364"]
    #[inline(always)]
    pub fn b364(&mut self) -> B364_W {
        B364_W { w: self }
    }
    #[doc = "Bit 13 - B365"]
    #[inline(always)]
    pub fn b365(&mut self) -> B365_W {
        B365_W { w: self }
    }
    #[doc = "Bit 14 - B366"]
    #[inline(always)]
    pub fn b366(&mut self) -> B366_W {
        B366_W { w: self }
    }
    #[doc = "Bit 15 - B367"]
    #[inline(always)]
    pub fn b367(&mut self) -> B367_W {
        B367_W { w: self }
    }
    #[doc = "Bit 16 - B368"]
    #[inline(always)]
    pub fn b368(&mut self) -> B368_W {
        B368_W { w: self }
    }
    #[doc = "Bit 17 - B369"]
    #[inline(always)]
    pub fn b369(&mut self) -> B369_W {
        B369_W { w: self }
    }
    #[doc = "Bit 18 - B370"]
    #[inline(always)]
    pub fn b370(&mut self) -> B370_W {
        B370_W { w: self }
    }
    #[doc = "Bit 19 - B371"]
    #[inline(always)]
    pub fn b371(&mut self) -> B371_W {
        B371_W { w: self }
    }
    #[doc = "Bit 20 - B372"]
    #[inline(always)]
    pub fn b372(&mut self) -> B372_W {
        B372_W { w: self }
    }
    #[doc = "Bit 21 - B373"]
    #[inline(always)]
    pub fn b373(&mut self) -> B373_W {
        B373_W { w: self }
    }
    #[doc = "Bit 22 - B374"]
    #[inline(always)]
    pub fn b374(&mut self) -> B374_W {
        B374_W { w: self }
    }
    #[doc = "Bit 23 - B375"]
    #[inline(always)]
    pub fn b375(&mut self) -> B375_W {
        B375_W { w: self }
    }
    #[doc = "Bit 24 - B376"]
    #[inline(always)]
    pub fn b376(&mut self) -> B376_W {
        B376_W { w: self }
    }
    #[doc = "Bit 25 - B377"]
    #[inline(always)]
    pub fn b377(&mut self) -> B377_W {
        B377_W { w: self }
    }
    #[doc = "Bit 26 - B378"]
    #[inline(always)]
    pub fn b378(&mut self) -> B378_W {
        B378_W { w: self }
    }
    #[doc = "Bit 27 - B379"]
    #[inline(always)]
    pub fn b379(&mut self) -> B379_W {
        B379_W { w: self }
    }
    #[doc = "Bit 28 - B380"]
    #[inline(always)]
    pub fn b380(&mut self) -> B380_W {
        B380_W { w: self }
    }
    #[doc = "Bit 29 - B381"]
    #[inline(always)]
    pub fn b381(&mut self) -> B381_W {
        B381_W { w: self }
    }
    #[doc = "Bit 30 - B382"]
    #[inline(always)]
    pub fn b382(&mut self) -> B382_W {
        B382_W { w: self }
    }
    #[doc = "Bit 31 - B383"]
    #[inline(always)]
    pub fn b383(&mut self) -> B383_W {
        B383_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr11](index.html) module"]
pub struct MPCBB1_VCTR11_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr11::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr11::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR11 to value 0"]
impl crate::Resettable for MPCBB1_VCTR11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
