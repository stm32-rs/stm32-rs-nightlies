#[doc = "Register `SR3` reader"]
pub struct R(crate::R<SR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR3` writer"]
pub struct W(crate::W<SR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR3_SPEC>;
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
impl From<crate::W<SR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZSCF` reader - TZSCF"]
pub struct TZSCF_R(crate::FieldReader<bool, bool>);
impl TZSCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZSCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZSCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZSCF` writer - TZSCF"]
pub struct TZSCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TZSCF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZICF` reader - TZICF"]
pub struct TZICF_R(crate::FieldReader<bool, bool>);
impl TZICF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZICF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZICF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZICF` writer - TZICF"]
pub struct TZICF_W<'a> {
    w: &'a mut W,
}
impl<'a> TZICF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MPCWM1F` reader - MPCWM1F"]
pub struct MPCWM1F_R(crate::FieldReader<bool, bool>);
impl MPCWM1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCWM1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCWM1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCWM1F` writer - MPCWM1F"]
pub struct MPCWM1F_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCWM1F_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MPCWM2F` reader - MPCWM2F"]
pub struct MPCWM2F_R(crate::FieldReader<bool, bool>);
impl MPCWM2F_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCWM2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCWM2F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCWM2F` writer - MPCWM2F"]
pub struct MPCWM2F_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCWM2F_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MPCBB1F` reader - MPCBB1F"]
pub struct MPCBB1F_R(crate::FieldReader<bool, bool>);
impl MPCBB1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCBB1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCBB1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCBB1F` writer - MPCBB1F"]
pub struct MPCBB1F_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB1F_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MPCBB1_REGF` reader - MPCBB1_REGF"]
pub struct MPCBB1_REGF_R(crate::FieldReader<bool, bool>);
impl MPCBB1_REGF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCBB1_REGF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCBB1_REGF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCBB1_REGF` writer - MPCBB1_REGF"]
pub struct MPCBB1_REGF_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB1_REGF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MPCBB2F` reader - MPCBB2F"]
pub struct MPCBB2F_R(crate::FieldReader<bool, bool>);
impl MPCBB2F_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCBB2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCBB2F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCBB2F` writer - MPCBB2F"]
pub struct MPCBB2F_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB2F_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MPCBB2_REGF` reader - MPCBB2_REGF"]
pub struct MPCBB2_REGF_R(crate::FieldReader<bool, bool>);
impl MPCBB2_REGF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCBB2_REGF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCBB2_REGF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCBB2_REGF` writer - MPCBB2_REGF"]
pub struct MPCBB2_REGF_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB2_REGF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 0 - TZSCF"]
    #[inline(always)]
    pub fn tzscf(&self) -> TZSCF_R {
        TZSCF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TZICF"]
    #[inline(always)]
    pub fn tzicf(&self) -> TZICF_R {
        TZICF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPCWM1F"]
    #[inline(always)]
    pub fn mpcwm1f(&self) -> MPCWM1F_R {
        MPCWM1F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MPCWM2F"]
    #[inline(always)]
    pub fn mpcwm2f(&self) -> MPCWM2F_R {
        MPCWM2F_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPCBB1F"]
    #[inline(always)]
    pub fn mpcbb1f(&self) -> MPCBB1F_R {
        MPCBB1F_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPCBB1_REGF"]
    #[inline(always)]
    pub fn mpcbb1_regf(&self) -> MPCBB1_REGF_R {
        MPCBB1_REGF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPCBB2F"]
    #[inline(always)]
    pub fn mpcbb2f(&self) -> MPCBB2F_R {
        MPCBB2F_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPCBB2_REGF"]
    #[inline(always)]
    pub fn mpcbb2_regf(&self) -> MPCBB2_REGF_R {
        MPCBB2_REGF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZSCF"]
    #[inline(always)]
    pub fn tzscf(&mut self) -> TZSCF_W {
        TZSCF_W { w: self }
    }
    #[doc = "Bit 1 - TZICF"]
    #[inline(always)]
    pub fn tzicf(&mut self) -> TZICF_W {
        TZICF_W { w: self }
    }
    #[doc = "Bit 2 - MPCWM1F"]
    #[inline(always)]
    pub fn mpcwm1f(&mut self) -> MPCWM1F_W {
        MPCWM1F_W { w: self }
    }
    #[doc = "Bit 3 - MPCWM2F"]
    #[inline(always)]
    pub fn mpcwm2f(&mut self) -> MPCWM2F_W {
        MPCWM2F_W { w: self }
    }
    #[doc = "Bit 4 - MPCBB1F"]
    #[inline(always)]
    pub fn mpcbb1f(&mut self) -> MPCBB1F_W {
        MPCBB1F_W { w: self }
    }
    #[doc = "Bit 5 - MPCBB1_REGF"]
    #[inline(always)]
    pub fn mpcbb1_regf(&mut self) -> MPCBB1_REGF_W {
        MPCBB1_REGF_W { w: self }
    }
    #[doc = "Bit 6 - MPCBB2F"]
    #[inline(always)]
    pub fn mpcbb2f(&mut self) -> MPCBB2F_W {
        MPCBB2F_W { w: self }
    }
    #[doc = "Bit 7 - MPCBB2_REGF"]
    #[inline(always)]
    pub fn mpcbb2_regf(&mut self) -> MPCBB2_REGF_W {
        MPCBB2_REGF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt status register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr3](index.html) module"]
pub struct SR3_SPEC;
impl crate::RegisterSpec for SR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr3::R](R) reader structure"]
impl crate::Readable for SR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr3::W](W) writer structure"]
impl crate::Writable for SR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR3 to value 0"]
impl crate::Resettable for SR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
