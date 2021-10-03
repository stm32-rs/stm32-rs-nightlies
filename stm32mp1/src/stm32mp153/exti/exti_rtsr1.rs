#[doc = "Register `EXTI_RTSR1` reader"]
pub struct R(crate::R<EXTI_RTSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_RTSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_RTSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_RTSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_RTSR1` writer"]
pub struct W(crate::W<EXTI_RTSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_RTSR1_SPEC>;
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
impl From<crate::W<EXTI_RTSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_RTSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RT0` reader - RT0"]
pub struct RT0_R(crate::FieldReader<bool, bool>);
impl RT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT0` writer - RT0"]
pub struct RT0_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RT1` reader - RT1"]
pub struct RT1_R(crate::FieldReader<bool, bool>);
impl RT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT1` writer - RT1"]
pub struct RT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RT2` reader - RT2"]
pub struct RT2_R(crate::FieldReader<bool, bool>);
impl RT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT2` writer - RT2"]
pub struct RT2_W<'a> {
    w: &'a mut W,
}
impl<'a> RT2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RT3` reader - RT3"]
pub struct RT3_R(crate::FieldReader<bool, bool>);
impl RT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT3` writer - RT3"]
pub struct RT3_W<'a> {
    w: &'a mut W,
}
impl<'a> RT3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RT4` reader - RT4"]
pub struct RT4_R(crate::FieldReader<bool, bool>);
impl RT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT4` writer - RT4"]
pub struct RT4_W<'a> {
    w: &'a mut W,
}
impl<'a> RT4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RT5` reader - RT5"]
pub struct RT5_R(crate::FieldReader<bool, bool>);
impl RT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT5` writer - RT5"]
pub struct RT5_W<'a> {
    w: &'a mut W,
}
impl<'a> RT5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RT6` reader - RT6"]
pub struct RT6_R(crate::FieldReader<bool, bool>);
impl RT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT6` writer - RT6"]
pub struct RT6_W<'a> {
    w: &'a mut W,
}
impl<'a> RT6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RT7` reader - RT7"]
pub struct RT7_R(crate::FieldReader<bool, bool>);
impl RT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT7` writer - RT7"]
pub struct RT7_W<'a> {
    w: &'a mut W,
}
impl<'a> RT7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RT8` reader - RT8"]
pub struct RT8_R(crate::FieldReader<bool, bool>);
impl RT8_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT8` writer - RT8"]
pub struct RT8_W<'a> {
    w: &'a mut W,
}
impl<'a> RT8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RT9` reader - RT9"]
pub struct RT9_R(crate::FieldReader<bool, bool>);
impl RT9_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT9` writer - RT9"]
pub struct RT9_W<'a> {
    w: &'a mut W,
}
impl<'a> RT9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RT10` reader - RT10"]
pub struct RT10_R(crate::FieldReader<bool, bool>);
impl RT10_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT10` writer - RT10"]
pub struct RT10_W<'a> {
    w: &'a mut W,
}
impl<'a> RT10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RT11` reader - RT11"]
pub struct RT11_R(crate::FieldReader<bool, bool>);
impl RT11_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT11` writer - RT11"]
pub struct RT11_W<'a> {
    w: &'a mut W,
}
impl<'a> RT11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RT12` reader - RT12"]
pub struct RT12_R(crate::FieldReader<bool, bool>);
impl RT12_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT12` writer - RT12"]
pub struct RT12_W<'a> {
    w: &'a mut W,
}
impl<'a> RT12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RT13` reader - RT13"]
pub struct RT13_R(crate::FieldReader<bool, bool>);
impl RT13_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT13` writer - RT13"]
pub struct RT13_W<'a> {
    w: &'a mut W,
}
impl<'a> RT13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RT14` reader - RT14"]
pub struct RT14_R(crate::FieldReader<bool, bool>);
impl RT14_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT14` writer - RT14"]
pub struct RT14_W<'a> {
    w: &'a mut W,
}
impl<'a> RT14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RT15` reader - RT15"]
pub struct RT15_R(crate::FieldReader<bool, bool>);
impl RT15_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT15` writer - RT15"]
pub struct RT15_W<'a> {
    w: &'a mut W,
}
impl<'a> RT15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RT16` reader - RT16"]
pub struct RT16_R(crate::FieldReader<bool, bool>);
impl RT16_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT16` writer - RT16"]
pub struct RT16_W<'a> {
    w: &'a mut W,
}
impl<'a> RT16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 0 - RT0"]
    #[inline(always)]
    pub fn rt0(&self) -> RT0_R {
        RT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RT1"]
    #[inline(always)]
    pub fn rt1(&self) -> RT1_R {
        RT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RT2"]
    #[inline(always)]
    pub fn rt2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RT3"]
    #[inline(always)]
    pub fn rt3(&self) -> RT3_R {
        RT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RT4"]
    #[inline(always)]
    pub fn rt4(&self) -> RT4_R {
        RT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RT5"]
    #[inline(always)]
    pub fn rt5(&self) -> RT5_R {
        RT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RT6"]
    #[inline(always)]
    pub fn rt6(&self) -> RT6_R {
        RT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RT7"]
    #[inline(always)]
    pub fn rt7(&self) -> RT7_R {
        RT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RT8"]
    #[inline(always)]
    pub fn rt8(&self) -> RT8_R {
        RT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RT9"]
    #[inline(always)]
    pub fn rt9(&self) -> RT9_R {
        RT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RT10"]
    #[inline(always)]
    pub fn rt10(&self) -> RT10_R {
        RT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RT11"]
    #[inline(always)]
    pub fn rt11(&self) -> RT11_R {
        RT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RT12"]
    #[inline(always)]
    pub fn rt12(&self) -> RT12_R {
        RT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RT13"]
    #[inline(always)]
    pub fn rt13(&self) -> RT13_R {
        RT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RT14"]
    #[inline(always)]
    pub fn rt14(&self) -> RT14_R {
        RT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RT15"]
    #[inline(always)]
    pub fn rt15(&self) -> RT15_R {
        RT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RT16"]
    #[inline(always)]
    pub fn rt16(&self) -> RT16_R {
        RT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RT0"]
    #[inline(always)]
    pub fn rt0(&mut self) -> RT0_W {
        RT0_W { w: self }
    }
    #[doc = "Bit 1 - RT1"]
    #[inline(always)]
    pub fn rt1(&mut self) -> RT1_W {
        RT1_W { w: self }
    }
    #[doc = "Bit 2 - RT2"]
    #[inline(always)]
    pub fn rt2(&mut self) -> RT2_W {
        RT2_W { w: self }
    }
    #[doc = "Bit 3 - RT3"]
    #[inline(always)]
    pub fn rt3(&mut self) -> RT3_W {
        RT3_W { w: self }
    }
    #[doc = "Bit 4 - RT4"]
    #[inline(always)]
    pub fn rt4(&mut self) -> RT4_W {
        RT4_W { w: self }
    }
    #[doc = "Bit 5 - RT5"]
    #[inline(always)]
    pub fn rt5(&mut self) -> RT5_W {
        RT5_W { w: self }
    }
    #[doc = "Bit 6 - RT6"]
    #[inline(always)]
    pub fn rt6(&mut self) -> RT6_W {
        RT6_W { w: self }
    }
    #[doc = "Bit 7 - RT7"]
    #[inline(always)]
    pub fn rt7(&mut self) -> RT7_W {
        RT7_W { w: self }
    }
    #[doc = "Bit 8 - RT8"]
    #[inline(always)]
    pub fn rt8(&mut self) -> RT8_W {
        RT8_W { w: self }
    }
    #[doc = "Bit 9 - RT9"]
    #[inline(always)]
    pub fn rt9(&mut self) -> RT9_W {
        RT9_W { w: self }
    }
    #[doc = "Bit 10 - RT10"]
    #[inline(always)]
    pub fn rt10(&mut self) -> RT10_W {
        RT10_W { w: self }
    }
    #[doc = "Bit 11 - RT11"]
    #[inline(always)]
    pub fn rt11(&mut self) -> RT11_W {
        RT11_W { w: self }
    }
    #[doc = "Bit 12 - RT12"]
    #[inline(always)]
    pub fn rt12(&mut self) -> RT12_W {
        RT12_W { w: self }
    }
    #[doc = "Bit 13 - RT13"]
    #[inline(always)]
    pub fn rt13(&mut self) -> RT13_W {
        RT13_W { w: self }
    }
    #[doc = "Bit 14 - RT14"]
    #[inline(always)]
    pub fn rt14(&mut self) -> RT14_W {
        RT14_W { w: self }
    }
    #[doc = "Bit 15 - RT15"]
    #[inline(always)]
    pub fn rt15(&mut self) -> RT15_W {
        RT15_W { w: self }
    }
    #[doc = "Bit 16 - RT16"]
    #[inline(always)]
    pub fn rt16(&mut self) -> RT16_W {
        RT16_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_rtsr1](index.html) module"]
pub struct EXTI_RTSR1_SPEC;
impl crate::RegisterSpec for EXTI_RTSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_rtsr1::R](R) reader structure"]
impl crate::Readable for EXTI_RTSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_rtsr1::W](W) writer structure"]
impl crate::Writable for EXTI_RTSR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_RTSR1 to value 0"]
impl crate::Resettable for EXTI_RTSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
