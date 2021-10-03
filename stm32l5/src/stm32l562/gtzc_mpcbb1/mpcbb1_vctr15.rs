#[doc = "Register `MPCBB1_VCTR15` reader"]
pub struct R(crate::R<MPCBB1_VCTR15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR15` writer"]
pub struct W(crate::W<MPCBB1_VCTR15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR15_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B480` reader - B480"]
pub struct B480_R(crate::FieldReader<bool, bool>);
impl B480_R {
    pub(crate) fn new(bits: bool) -> Self {
        B480_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B480_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B480` writer - B480"]
pub struct B480_W<'a> {
    w: &'a mut W,
}
impl<'a> B480_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B481` reader - B481"]
pub struct B481_R(crate::FieldReader<bool, bool>);
impl B481_R {
    pub(crate) fn new(bits: bool) -> Self {
        B481_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B481_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B481` writer - B481"]
pub struct B481_W<'a> {
    w: &'a mut W,
}
impl<'a> B481_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B482` reader - B482"]
pub struct B482_R(crate::FieldReader<bool, bool>);
impl B482_R {
    pub(crate) fn new(bits: bool) -> Self {
        B482_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B482_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B482` writer - B482"]
pub struct B482_W<'a> {
    w: &'a mut W,
}
impl<'a> B482_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B483` reader - B483"]
pub struct B483_R(crate::FieldReader<bool, bool>);
impl B483_R {
    pub(crate) fn new(bits: bool) -> Self {
        B483_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B483_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B483` writer - B483"]
pub struct B483_W<'a> {
    w: &'a mut W,
}
impl<'a> B483_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B484` reader - B484"]
pub struct B484_R(crate::FieldReader<bool, bool>);
impl B484_R {
    pub(crate) fn new(bits: bool) -> Self {
        B484_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B484_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B484` writer - B484"]
pub struct B484_W<'a> {
    w: &'a mut W,
}
impl<'a> B484_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B485` reader - B485"]
pub struct B485_R(crate::FieldReader<bool, bool>);
impl B485_R {
    pub(crate) fn new(bits: bool) -> Self {
        B485_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B485_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B485` writer - B485"]
pub struct B485_W<'a> {
    w: &'a mut W,
}
impl<'a> B485_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B486` reader - B486"]
pub struct B486_R(crate::FieldReader<bool, bool>);
impl B486_R {
    pub(crate) fn new(bits: bool) -> Self {
        B486_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B486_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B486` writer - B486"]
pub struct B486_W<'a> {
    w: &'a mut W,
}
impl<'a> B486_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B487` reader - B487"]
pub struct B487_R(crate::FieldReader<bool, bool>);
impl B487_R {
    pub(crate) fn new(bits: bool) -> Self {
        B487_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B487_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B487` writer - B487"]
pub struct B487_W<'a> {
    w: &'a mut W,
}
impl<'a> B487_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B488` reader - B488"]
pub struct B488_R(crate::FieldReader<bool, bool>);
impl B488_R {
    pub(crate) fn new(bits: bool) -> Self {
        B488_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B488_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B488` writer - B488"]
pub struct B488_W<'a> {
    w: &'a mut W,
}
impl<'a> B488_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B489` reader - B489"]
pub struct B489_R(crate::FieldReader<bool, bool>);
impl B489_R {
    pub(crate) fn new(bits: bool) -> Self {
        B489_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B489_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B489` writer - B489"]
pub struct B489_W<'a> {
    w: &'a mut W,
}
impl<'a> B489_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B490` reader - B490"]
pub struct B490_R(crate::FieldReader<bool, bool>);
impl B490_R {
    pub(crate) fn new(bits: bool) -> Self {
        B490_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B490_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B490` writer - B490"]
pub struct B490_W<'a> {
    w: &'a mut W,
}
impl<'a> B490_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B491` reader - B491"]
pub struct B491_R(crate::FieldReader<bool, bool>);
impl B491_R {
    pub(crate) fn new(bits: bool) -> Self {
        B491_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B491_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B491` writer - B491"]
pub struct B491_W<'a> {
    w: &'a mut W,
}
impl<'a> B491_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B492` reader - B492"]
pub struct B492_R(crate::FieldReader<bool, bool>);
impl B492_R {
    pub(crate) fn new(bits: bool) -> Self {
        B492_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B492_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B492` writer - B492"]
pub struct B492_W<'a> {
    w: &'a mut W,
}
impl<'a> B492_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B493` reader - B493"]
pub struct B493_R(crate::FieldReader<bool, bool>);
impl B493_R {
    pub(crate) fn new(bits: bool) -> Self {
        B493_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B493_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B493` writer - B493"]
pub struct B493_W<'a> {
    w: &'a mut W,
}
impl<'a> B493_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B494` reader - B494"]
pub struct B494_R(crate::FieldReader<bool, bool>);
impl B494_R {
    pub(crate) fn new(bits: bool) -> Self {
        B494_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B494_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B494` writer - B494"]
pub struct B494_W<'a> {
    w: &'a mut W,
}
impl<'a> B494_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B495` reader - B495"]
pub struct B495_R(crate::FieldReader<bool, bool>);
impl B495_R {
    pub(crate) fn new(bits: bool) -> Self {
        B495_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B495_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B495` writer - B495"]
pub struct B495_W<'a> {
    w: &'a mut W,
}
impl<'a> B495_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B496` reader - B496"]
pub struct B496_R(crate::FieldReader<bool, bool>);
impl B496_R {
    pub(crate) fn new(bits: bool) -> Self {
        B496_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B496_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B496` writer - B496"]
pub struct B496_W<'a> {
    w: &'a mut W,
}
impl<'a> B496_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B497` reader - B497"]
pub struct B497_R(crate::FieldReader<bool, bool>);
impl B497_R {
    pub(crate) fn new(bits: bool) -> Self {
        B497_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B497_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B497` writer - B497"]
pub struct B497_W<'a> {
    w: &'a mut W,
}
impl<'a> B497_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B498` reader - B498"]
pub struct B498_R(crate::FieldReader<bool, bool>);
impl B498_R {
    pub(crate) fn new(bits: bool) -> Self {
        B498_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B498_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B498` writer - B498"]
pub struct B498_W<'a> {
    w: &'a mut W,
}
impl<'a> B498_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B499` reader - B499"]
pub struct B499_R(crate::FieldReader<bool, bool>);
impl B499_R {
    pub(crate) fn new(bits: bool) -> Self {
        B499_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B499_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B499` writer - B499"]
pub struct B499_W<'a> {
    w: &'a mut W,
}
impl<'a> B499_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B500` reader - B500"]
pub struct B500_R(crate::FieldReader<bool, bool>);
impl B500_R {
    pub(crate) fn new(bits: bool) -> Self {
        B500_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B500_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B500` writer - B500"]
pub struct B500_W<'a> {
    w: &'a mut W,
}
impl<'a> B500_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B501` reader - B501"]
pub struct B501_R(crate::FieldReader<bool, bool>);
impl B501_R {
    pub(crate) fn new(bits: bool) -> Self {
        B501_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B501_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B501` writer - B501"]
pub struct B501_W<'a> {
    w: &'a mut W,
}
impl<'a> B501_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B502` reader - B502"]
pub struct B502_R(crate::FieldReader<bool, bool>);
impl B502_R {
    pub(crate) fn new(bits: bool) -> Self {
        B502_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B502_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B502` writer - B502"]
pub struct B502_W<'a> {
    w: &'a mut W,
}
impl<'a> B502_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B503` reader - B503"]
pub struct B503_R(crate::FieldReader<bool, bool>);
impl B503_R {
    pub(crate) fn new(bits: bool) -> Self {
        B503_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B503_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B503` writer - B503"]
pub struct B503_W<'a> {
    w: &'a mut W,
}
impl<'a> B503_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B504` reader - B504"]
pub struct B504_R(crate::FieldReader<bool, bool>);
impl B504_R {
    pub(crate) fn new(bits: bool) -> Self {
        B504_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B504_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B504` writer - B504"]
pub struct B504_W<'a> {
    w: &'a mut W,
}
impl<'a> B504_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B505` reader - B505"]
pub struct B505_R(crate::FieldReader<bool, bool>);
impl B505_R {
    pub(crate) fn new(bits: bool) -> Self {
        B505_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B505_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B505` writer - B505"]
pub struct B505_W<'a> {
    w: &'a mut W,
}
impl<'a> B505_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B506` reader - B506"]
pub struct B506_R(crate::FieldReader<bool, bool>);
impl B506_R {
    pub(crate) fn new(bits: bool) -> Self {
        B506_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B506_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B506` writer - B506"]
pub struct B506_W<'a> {
    w: &'a mut W,
}
impl<'a> B506_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B507` reader - B507"]
pub struct B507_R(crate::FieldReader<bool, bool>);
impl B507_R {
    pub(crate) fn new(bits: bool) -> Self {
        B507_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B507_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B507` writer - B507"]
pub struct B507_W<'a> {
    w: &'a mut W,
}
impl<'a> B507_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B508` reader - B508"]
pub struct B508_R(crate::FieldReader<bool, bool>);
impl B508_R {
    pub(crate) fn new(bits: bool) -> Self {
        B508_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B508_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B508` writer - B508"]
pub struct B508_W<'a> {
    w: &'a mut W,
}
impl<'a> B508_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B509` reader - B509"]
pub struct B509_R(crate::FieldReader<bool, bool>);
impl B509_R {
    pub(crate) fn new(bits: bool) -> Self {
        B509_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B509_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B509` writer - B509"]
pub struct B509_W<'a> {
    w: &'a mut W,
}
impl<'a> B509_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B510` reader - B510"]
pub struct B510_R(crate::FieldReader<bool, bool>);
impl B510_R {
    pub(crate) fn new(bits: bool) -> Self {
        B510_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B510_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B510` writer - B510"]
pub struct B510_W<'a> {
    w: &'a mut W,
}
impl<'a> B510_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B511` reader - B511"]
pub struct B511_R(crate::FieldReader<bool, bool>);
impl B511_R {
    pub(crate) fn new(bits: bool) -> Self {
        B511_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B511_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B511` writer - B511"]
pub struct B511_W<'a> {
    w: &'a mut W,
}
impl<'a> B511_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B480"]
    #[inline(always)]
    pub fn b480(&self) -> B480_R {
        B480_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B481"]
    #[inline(always)]
    pub fn b481(&self) -> B481_R {
        B481_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B482"]
    #[inline(always)]
    pub fn b482(&self) -> B482_R {
        B482_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B483"]
    #[inline(always)]
    pub fn b483(&self) -> B483_R {
        B483_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B484"]
    #[inline(always)]
    pub fn b484(&self) -> B484_R {
        B484_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B485"]
    #[inline(always)]
    pub fn b485(&self) -> B485_R {
        B485_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B486"]
    #[inline(always)]
    pub fn b486(&self) -> B486_R {
        B486_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B487"]
    #[inline(always)]
    pub fn b487(&self) -> B487_R {
        B487_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B488"]
    #[inline(always)]
    pub fn b488(&self) -> B488_R {
        B488_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B489"]
    #[inline(always)]
    pub fn b489(&self) -> B489_R {
        B489_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B490"]
    #[inline(always)]
    pub fn b490(&self) -> B490_R {
        B490_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B491"]
    #[inline(always)]
    pub fn b491(&self) -> B491_R {
        B491_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B492"]
    #[inline(always)]
    pub fn b492(&self) -> B492_R {
        B492_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B493"]
    #[inline(always)]
    pub fn b493(&self) -> B493_R {
        B493_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B494"]
    #[inline(always)]
    pub fn b494(&self) -> B494_R {
        B494_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B495"]
    #[inline(always)]
    pub fn b495(&self) -> B495_R {
        B495_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B496"]
    #[inline(always)]
    pub fn b496(&self) -> B496_R {
        B496_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B497"]
    #[inline(always)]
    pub fn b497(&self) -> B497_R {
        B497_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B498"]
    #[inline(always)]
    pub fn b498(&self) -> B498_R {
        B498_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B499"]
    #[inline(always)]
    pub fn b499(&self) -> B499_R {
        B499_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B500"]
    #[inline(always)]
    pub fn b500(&self) -> B500_R {
        B500_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B501"]
    #[inline(always)]
    pub fn b501(&self) -> B501_R {
        B501_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B502"]
    #[inline(always)]
    pub fn b502(&self) -> B502_R {
        B502_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B503"]
    #[inline(always)]
    pub fn b503(&self) -> B503_R {
        B503_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B504"]
    #[inline(always)]
    pub fn b504(&self) -> B504_R {
        B504_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B505"]
    #[inline(always)]
    pub fn b505(&self) -> B505_R {
        B505_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B506"]
    #[inline(always)]
    pub fn b506(&self) -> B506_R {
        B506_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B507"]
    #[inline(always)]
    pub fn b507(&self) -> B507_R {
        B507_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B508"]
    #[inline(always)]
    pub fn b508(&self) -> B508_R {
        B508_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B509"]
    #[inline(always)]
    pub fn b509(&self) -> B509_R {
        B509_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B510"]
    #[inline(always)]
    pub fn b510(&self) -> B510_R {
        B510_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B511"]
    #[inline(always)]
    pub fn b511(&self) -> B511_R {
        B511_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B480"]
    #[inline(always)]
    pub fn b480(&mut self) -> B480_W {
        B480_W { w: self }
    }
    #[doc = "Bit 1 - B481"]
    #[inline(always)]
    pub fn b481(&mut self) -> B481_W {
        B481_W { w: self }
    }
    #[doc = "Bit 2 - B482"]
    #[inline(always)]
    pub fn b482(&mut self) -> B482_W {
        B482_W { w: self }
    }
    #[doc = "Bit 3 - B483"]
    #[inline(always)]
    pub fn b483(&mut self) -> B483_W {
        B483_W { w: self }
    }
    #[doc = "Bit 4 - B484"]
    #[inline(always)]
    pub fn b484(&mut self) -> B484_W {
        B484_W { w: self }
    }
    #[doc = "Bit 5 - B485"]
    #[inline(always)]
    pub fn b485(&mut self) -> B485_W {
        B485_W { w: self }
    }
    #[doc = "Bit 6 - B486"]
    #[inline(always)]
    pub fn b486(&mut self) -> B486_W {
        B486_W { w: self }
    }
    #[doc = "Bit 7 - B487"]
    #[inline(always)]
    pub fn b487(&mut self) -> B487_W {
        B487_W { w: self }
    }
    #[doc = "Bit 8 - B488"]
    #[inline(always)]
    pub fn b488(&mut self) -> B488_W {
        B488_W { w: self }
    }
    #[doc = "Bit 9 - B489"]
    #[inline(always)]
    pub fn b489(&mut self) -> B489_W {
        B489_W { w: self }
    }
    #[doc = "Bit 10 - B490"]
    #[inline(always)]
    pub fn b490(&mut self) -> B490_W {
        B490_W { w: self }
    }
    #[doc = "Bit 11 - B491"]
    #[inline(always)]
    pub fn b491(&mut self) -> B491_W {
        B491_W { w: self }
    }
    #[doc = "Bit 12 - B492"]
    #[inline(always)]
    pub fn b492(&mut self) -> B492_W {
        B492_W { w: self }
    }
    #[doc = "Bit 13 - B493"]
    #[inline(always)]
    pub fn b493(&mut self) -> B493_W {
        B493_W { w: self }
    }
    #[doc = "Bit 14 - B494"]
    #[inline(always)]
    pub fn b494(&mut self) -> B494_W {
        B494_W { w: self }
    }
    #[doc = "Bit 15 - B495"]
    #[inline(always)]
    pub fn b495(&mut self) -> B495_W {
        B495_W { w: self }
    }
    #[doc = "Bit 16 - B496"]
    #[inline(always)]
    pub fn b496(&mut self) -> B496_W {
        B496_W { w: self }
    }
    #[doc = "Bit 17 - B497"]
    #[inline(always)]
    pub fn b497(&mut self) -> B497_W {
        B497_W { w: self }
    }
    #[doc = "Bit 18 - B498"]
    #[inline(always)]
    pub fn b498(&mut self) -> B498_W {
        B498_W { w: self }
    }
    #[doc = "Bit 19 - B499"]
    #[inline(always)]
    pub fn b499(&mut self) -> B499_W {
        B499_W { w: self }
    }
    #[doc = "Bit 20 - B500"]
    #[inline(always)]
    pub fn b500(&mut self) -> B500_W {
        B500_W { w: self }
    }
    #[doc = "Bit 21 - B501"]
    #[inline(always)]
    pub fn b501(&mut self) -> B501_W {
        B501_W { w: self }
    }
    #[doc = "Bit 22 - B502"]
    #[inline(always)]
    pub fn b502(&mut self) -> B502_W {
        B502_W { w: self }
    }
    #[doc = "Bit 23 - B503"]
    #[inline(always)]
    pub fn b503(&mut self) -> B503_W {
        B503_W { w: self }
    }
    #[doc = "Bit 24 - B504"]
    #[inline(always)]
    pub fn b504(&mut self) -> B504_W {
        B504_W { w: self }
    }
    #[doc = "Bit 25 - B505"]
    #[inline(always)]
    pub fn b505(&mut self) -> B505_W {
        B505_W { w: self }
    }
    #[doc = "Bit 26 - B506"]
    #[inline(always)]
    pub fn b506(&mut self) -> B506_W {
        B506_W { w: self }
    }
    #[doc = "Bit 27 - B507"]
    #[inline(always)]
    pub fn b507(&mut self) -> B507_W {
        B507_W { w: self }
    }
    #[doc = "Bit 28 - B508"]
    #[inline(always)]
    pub fn b508(&mut self) -> B508_W {
        B508_W { w: self }
    }
    #[doc = "Bit 29 - B509"]
    #[inline(always)]
    pub fn b509(&mut self) -> B509_W {
        B509_W { w: self }
    }
    #[doc = "Bit 30 - B510"]
    #[inline(always)]
    pub fn b510(&mut self) -> B510_W {
        B510_W { w: self }
    }
    #[doc = "Bit 31 - B511"]
    #[inline(always)]
    pub fn b511(&mut self) -> B511_W {
        B511_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr15](index.html) module"]
pub struct MPCBB1_VCTR15_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr15::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr15::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR15_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR15 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB1_VCTR15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
