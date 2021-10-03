#[doc = "Register `MPCBB1_VCTR42` reader"]
pub struct R(crate::R<MPCBB1_VCTR42_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR42_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR42_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR42_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR42` writer"]
pub struct W(crate::W<MPCBB1_VCTR42_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR42_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR42_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR42_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1344` reader - B1344"]
pub struct B1344_R(crate::FieldReader<bool, bool>);
impl B1344_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1344_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1344_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1344` writer - B1344"]
pub struct B1344_W<'a> {
    w: &'a mut W,
}
impl<'a> B1344_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1345` reader - B1345"]
pub struct B1345_R(crate::FieldReader<bool, bool>);
impl B1345_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1345_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1345_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1345` writer - B1345"]
pub struct B1345_W<'a> {
    w: &'a mut W,
}
impl<'a> B1345_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1346` reader - B1346"]
pub struct B1346_R(crate::FieldReader<bool, bool>);
impl B1346_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1346_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1346_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1346` writer - B1346"]
pub struct B1346_W<'a> {
    w: &'a mut W,
}
impl<'a> B1346_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1347` reader - B1347"]
pub struct B1347_R(crate::FieldReader<bool, bool>);
impl B1347_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1347_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1347_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1347` writer - B1347"]
pub struct B1347_W<'a> {
    w: &'a mut W,
}
impl<'a> B1347_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1348` reader - B1348"]
pub struct B1348_R(crate::FieldReader<bool, bool>);
impl B1348_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1348_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1348_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1348` writer - B1348"]
pub struct B1348_W<'a> {
    w: &'a mut W,
}
impl<'a> B1348_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1349` reader - B1349"]
pub struct B1349_R(crate::FieldReader<bool, bool>);
impl B1349_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1349_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1349_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1349` writer - B1349"]
pub struct B1349_W<'a> {
    w: &'a mut W,
}
impl<'a> B1349_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1350` reader - B1350"]
pub struct B1350_R(crate::FieldReader<bool, bool>);
impl B1350_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1350_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1350_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1350` writer - B1350"]
pub struct B1350_W<'a> {
    w: &'a mut W,
}
impl<'a> B1350_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1351` reader - B1351"]
pub struct B1351_R(crate::FieldReader<bool, bool>);
impl B1351_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1351_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1351_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1351` writer - B1351"]
pub struct B1351_W<'a> {
    w: &'a mut W,
}
impl<'a> B1351_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1352` reader - B1352"]
pub struct B1352_R(crate::FieldReader<bool, bool>);
impl B1352_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1352_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1352_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1352` writer - B1352"]
pub struct B1352_W<'a> {
    w: &'a mut W,
}
impl<'a> B1352_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1353` reader - B1353"]
pub struct B1353_R(crate::FieldReader<bool, bool>);
impl B1353_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1353_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1353_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1353` writer - B1353"]
pub struct B1353_W<'a> {
    w: &'a mut W,
}
impl<'a> B1353_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1354` reader - B1354"]
pub struct B1354_R(crate::FieldReader<bool, bool>);
impl B1354_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1354_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1354_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1354` writer - B1354"]
pub struct B1354_W<'a> {
    w: &'a mut W,
}
impl<'a> B1354_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1355` reader - B1355"]
pub struct B1355_R(crate::FieldReader<bool, bool>);
impl B1355_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1355_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1355_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1355` writer - B1355"]
pub struct B1355_W<'a> {
    w: &'a mut W,
}
impl<'a> B1355_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1356` reader - B1356"]
pub struct B1356_R(crate::FieldReader<bool, bool>);
impl B1356_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1356_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1356_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1356` writer - B1356"]
pub struct B1356_W<'a> {
    w: &'a mut W,
}
impl<'a> B1356_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1357` reader - B1357"]
pub struct B1357_R(crate::FieldReader<bool, bool>);
impl B1357_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1357_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1357_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1357` writer - B1357"]
pub struct B1357_W<'a> {
    w: &'a mut W,
}
impl<'a> B1357_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1358` reader - B1358"]
pub struct B1358_R(crate::FieldReader<bool, bool>);
impl B1358_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1358_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1358_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1358` writer - B1358"]
pub struct B1358_W<'a> {
    w: &'a mut W,
}
impl<'a> B1358_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1359` reader - B1359"]
pub struct B1359_R(crate::FieldReader<bool, bool>);
impl B1359_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1359_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1359_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1359` writer - B1359"]
pub struct B1359_W<'a> {
    w: &'a mut W,
}
impl<'a> B1359_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1360` reader - B1360"]
pub struct B1360_R(crate::FieldReader<bool, bool>);
impl B1360_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1360_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1360_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1360` writer - B1360"]
pub struct B1360_W<'a> {
    w: &'a mut W,
}
impl<'a> B1360_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1361` reader - B1361"]
pub struct B1361_R(crate::FieldReader<bool, bool>);
impl B1361_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1361_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1361_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1361` writer - B1361"]
pub struct B1361_W<'a> {
    w: &'a mut W,
}
impl<'a> B1361_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1362` reader - B1362"]
pub struct B1362_R(crate::FieldReader<bool, bool>);
impl B1362_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1362_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1362_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1362` writer - B1362"]
pub struct B1362_W<'a> {
    w: &'a mut W,
}
impl<'a> B1362_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1363` reader - B1363"]
pub struct B1363_R(crate::FieldReader<bool, bool>);
impl B1363_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1363_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1363_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1363` writer - B1363"]
pub struct B1363_W<'a> {
    w: &'a mut W,
}
impl<'a> B1363_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1364` reader - B1364"]
pub struct B1364_R(crate::FieldReader<bool, bool>);
impl B1364_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1364_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1364_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1364` writer - B1364"]
pub struct B1364_W<'a> {
    w: &'a mut W,
}
impl<'a> B1364_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1365` reader - B1365"]
pub struct B1365_R(crate::FieldReader<bool, bool>);
impl B1365_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1365_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1365_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1365` writer - B1365"]
pub struct B1365_W<'a> {
    w: &'a mut W,
}
impl<'a> B1365_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1366` reader - B1366"]
pub struct B1366_R(crate::FieldReader<bool, bool>);
impl B1366_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1366_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1366_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1366` writer - B1366"]
pub struct B1366_W<'a> {
    w: &'a mut W,
}
impl<'a> B1366_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1367` reader - B1367"]
pub struct B1367_R(crate::FieldReader<bool, bool>);
impl B1367_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1367_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1367_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1367` writer - B1367"]
pub struct B1367_W<'a> {
    w: &'a mut W,
}
impl<'a> B1367_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1368` reader - B1368"]
pub struct B1368_R(crate::FieldReader<bool, bool>);
impl B1368_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1368_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1368_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1368` writer - B1368"]
pub struct B1368_W<'a> {
    w: &'a mut W,
}
impl<'a> B1368_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1369` reader - B1369"]
pub struct B1369_R(crate::FieldReader<bool, bool>);
impl B1369_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1369_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1369_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1369` writer - B1369"]
pub struct B1369_W<'a> {
    w: &'a mut W,
}
impl<'a> B1369_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1370` reader - B1370"]
pub struct B1370_R(crate::FieldReader<bool, bool>);
impl B1370_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1370_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1370_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1370` writer - B1370"]
pub struct B1370_W<'a> {
    w: &'a mut W,
}
impl<'a> B1370_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1371` reader - B1371"]
pub struct B1371_R(crate::FieldReader<bool, bool>);
impl B1371_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1371_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1371_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1371` writer - B1371"]
pub struct B1371_W<'a> {
    w: &'a mut W,
}
impl<'a> B1371_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1372` reader - B1372"]
pub struct B1372_R(crate::FieldReader<bool, bool>);
impl B1372_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1372_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1372_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1372` writer - B1372"]
pub struct B1372_W<'a> {
    w: &'a mut W,
}
impl<'a> B1372_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1373` reader - B1373"]
pub struct B1373_R(crate::FieldReader<bool, bool>);
impl B1373_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1373_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1373_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1373` writer - B1373"]
pub struct B1373_W<'a> {
    w: &'a mut W,
}
impl<'a> B1373_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1374` reader - B1374"]
pub struct B1374_R(crate::FieldReader<bool, bool>);
impl B1374_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1374_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1374_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1374` writer - B1374"]
pub struct B1374_W<'a> {
    w: &'a mut W,
}
impl<'a> B1374_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1375` reader - B1375"]
pub struct B1375_R(crate::FieldReader<bool, bool>);
impl B1375_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1375_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1375_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1375` writer - B1375"]
pub struct B1375_W<'a> {
    w: &'a mut W,
}
impl<'a> B1375_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1344"]
    #[inline(always)]
    pub fn b1344(&self) -> B1344_R {
        B1344_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1345"]
    #[inline(always)]
    pub fn b1345(&self) -> B1345_R {
        B1345_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1346"]
    #[inline(always)]
    pub fn b1346(&self) -> B1346_R {
        B1346_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1347"]
    #[inline(always)]
    pub fn b1347(&self) -> B1347_R {
        B1347_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1348"]
    #[inline(always)]
    pub fn b1348(&self) -> B1348_R {
        B1348_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1349"]
    #[inline(always)]
    pub fn b1349(&self) -> B1349_R {
        B1349_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1350"]
    #[inline(always)]
    pub fn b1350(&self) -> B1350_R {
        B1350_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1351"]
    #[inline(always)]
    pub fn b1351(&self) -> B1351_R {
        B1351_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1352"]
    #[inline(always)]
    pub fn b1352(&self) -> B1352_R {
        B1352_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1353"]
    #[inline(always)]
    pub fn b1353(&self) -> B1353_R {
        B1353_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1354"]
    #[inline(always)]
    pub fn b1354(&self) -> B1354_R {
        B1354_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1355"]
    #[inline(always)]
    pub fn b1355(&self) -> B1355_R {
        B1355_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1356"]
    #[inline(always)]
    pub fn b1356(&self) -> B1356_R {
        B1356_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1357"]
    #[inline(always)]
    pub fn b1357(&self) -> B1357_R {
        B1357_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1358"]
    #[inline(always)]
    pub fn b1358(&self) -> B1358_R {
        B1358_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1359"]
    #[inline(always)]
    pub fn b1359(&self) -> B1359_R {
        B1359_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1360"]
    #[inline(always)]
    pub fn b1360(&self) -> B1360_R {
        B1360_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1361"]
    #[inline(always)]
    pub fn b1361(&self) -> B1361_R {
        B1361_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1362"]
    #[inline(always)]
    pub fn b1362(&self) -> B1362_R {
        B1362_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1363"]
    #[inline(always)]
    pub fn b1363(&self) -> B1363_R {
        B1363_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1364"]
    #[inline(always)]
    pub fn b1364(&self) -> B1364_R {
        B1364_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1365"]
    #[inline(always)]
    pub fn b1365(&self) -> B1365_R {
        B1365_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1366"]
    #[inline(always)]
    pub fn b1366(&self) -> B1366_R {
        B1366_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1367"]
    #[inline(always)]
    pub fn b1367(&self) -> B1367_R {
        B1367_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1368"]
    #[inline(always)]
    pub fn b1368(&self) -> B1368_R {
        B1368_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1369"]
    #[inline(always)]
    pub fn b1369(&self) -> B1369_R {
        B1369_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1370"]
    #[inline(always)]
    pub fn b1370(&self) -> B1370_R {
        B1370_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1371"]
    #[inline(always)]
    pub fn b1371(&self) -> B1371_R {
        B1371_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1372"]
    #[inline(always)]
    pub fn b1372(&self) -> B1372_R {
        B1372_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1373"]
    #[inline(always)]
    pub fn b1373(&self) -> B1373_R {
        B1373_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1374"]
    #[inline(always)]
    pub fn b1374(&self) -> B1374_R {
        B1374_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1375"]
    #[inline(always)]
    pub fn b1375(&self) -> B1375_R {
        B1375_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1344"]
    #[inline(always)]
    pub fn b1344(&mut self) -> B1344_W {
        B1344_W { w: self }
    }
    #[doc = "Bit 1 - B1345"]
    #[inline(always)]
    pub fn b1345(&mut self) -> B1345_W {
        B1345_W { w: self }
    }
    #[doc = "Bit 2 - B1346"]
    #[inline(always)]
    pub fn b1346(&mut self) -> B1346_W {
        B1346_W { w: self }
    }
    #[doc = "Bit 3 - B1347"]
    #[inline(always)]
    pub fn b1347(&mut self) -> B1347_W {
        B1347_W { w: self }
    }
    #[doc = "Bit 4 - B1348"]
    #[inline(always)]
    pub fn b1348(&mut self) -> B1348_W {
        B1348_W { w: self }
    }
    #[doc = "Bit 5 - B1349"]
    #[inline(always)]
    pub fn b1349(&mut self) -> B1349_W {
        B1349_W { w: self }
    }
    #[doc = "Bit 6 - B1350"]
    #[inline(always)]
    pub fn b1350(&mut self) -> B1350_W {
        B1350_W { w: self }
    }
    #[doc = "Bit 7 - B1351"]
    #[inline(always)]
    pub fn b1351(&mut self) -> B1351_W {
        B1351_W { w: self }
    }
    #[doc = "Bit 8 - B1352"]
    #[inline(always)]
    pub fn b1352(&mut self) -> B1352_W {
        B1352_W { w: self }
    }
    #[doc = "Bit 9 - B1353"]
    #[inline(always)]
    pub fn b1353(&mut self) -> B1353_W {
        B1353_W { w: self }
    }
    #[doc = "Bit 10 - B1354"]
    #[inline(always)]
    pub fn b1354(&mut self) -> B1354_W {
        B1354_W { w: self }
    }
    #[doc = "Bit 11 - B1355"]
    #[inline(always)]
    pub fn b1355(&mut self) -> B1355_W {
        B1355_W { w: self }
    }
    #[doc = "Bit 12 - B1356"]
    #[inline(always)]
    pub fn b1356(&mut self) -> B1356_W {
        B1356_W { w: self }
    }
    #[doc = "Bit 13 - B1357"]
    #[inline(always)]
    pub fn b1357(&mut self) -> B1357_W {
        B1357_W { w: self }
    }
    #[doc = "Bit 14 - B1358"]
    #[inline(always)]
    pub fn b1358(&mut self) -> B1358_W {
        B1358_W { w: self }
    }
    #[doc = "Bit 15 - B1359"]
    #[inline(always)]
    pub fn b1359(&mut self) -> B1359_W {
        B1359_W { w: self }
    }
    #[doc = "Bit 16 - B1360"]
    #[inline(always)]
    pub fn b1360(&mut self) -> B1360_W {
        B1360_W { w: self }
    }
    #[doc = "Bit 17 - B1361"]
    #[inline(always)]
    pub fn b1361(&mut self) -> B1361_W {
        B1361_W { w: self }
    }
    #[doc = "Bit 18 - B1362"]
    #[inline(always)]
    pub fn b1362(&mut self) -> B1362_W {
        B1362_W { w: self }
    }
    #[doc = "Bit 19 - B1363"]
    #[inline(always)]
    pub fn b1363(&mut self) -> B1363_W {
        B1363_W { w: self }
    }
    #[doc = "Bit 20 - B1364"]
    #[inline(always)]
    pub fn b1364(&mut self) -> B1364_W {
        B1364_W { w: self }
    }
    #[doc = "Bit 21 - B1365"]
    #[inline(always)]
    pub fn b1365(&mut self) -> B1365_W {
        B1365_W { w: self }
    }
    #[doc = "Bit 22 - B1366"]
    #[inline(always)]
    pub fn b1366(&mut self) -> B1366_W {
        B1366_W { w: self }
    }
    #[doc = "Bit 23 - B1367"]
    #[inline(always)]
    pub fn b1367(&mut self) -> B1367_W {
        B1367_W { w: self }
    }
    #[doc = "Bit 24 - B1368"]
    #[inline(always)]
    pub fn b1368(&mut self) -> B1368_W {
        B1368_W { w: self }
    }
    #[doc = "Bit 25 - B1369"]
    #[inline(always)]
    pub fn b1369(&mut self) -> B1369_W {
        B1369_W { w: self }
    }
    #[doc = "Bit 26 - B1370"]
    #[inline(always)]
    pub fn b1370(&mut self) -> B1370_W {
        B1370_W { w: self }
    }
    #[doc = "Bit 27 - B1371"]
    #[inline(always)]
    pub fn b1371(&mut self) -> B1371_W {
        B1371_W { w: self }
    }
    #[doc = "Bit 28 - B1372"]
    #[inline(always)]
    pub fn b1372(&mut self) -> B1372_W {
        B1372_W { w: self }
    }
    #[doc = "Bit 29 - B1373"]
    #[inline(always)]
    pub fn b1373(&mut self) -> B1373_W {
        B1373_W { w: self }
    }
    #[doc = "Bit 30 - B1374"]
    #[inline(always)]
    pub fn b1374(&mut self) -> B1374_W {
        B1374_W { w: self }
    }
    #[doc = "Bit 31 - B1375"]
    #[inline(always)]
    pub fn b1375(&mut self) -> B1375_W {
        B1375_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr42](index.html) module"]
pub struct MPCBB1_VCTR42_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR42_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr42::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR42_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr42::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR42_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR42 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB1_VCTR42_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
