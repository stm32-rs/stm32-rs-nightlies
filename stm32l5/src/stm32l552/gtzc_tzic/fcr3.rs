#[doc = "Register `FCR3` reader"]
pub struct R(crate::R<FCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCR3` writer"]
pub struct W(crate::W<FCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR3_SPEC>;
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
impl From<crate::W<FCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZSCFC` reader - TZSCFC"]
pub struct TZSCFC_R(crate::FieldReader<bool, bool>);
impl TZSCFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZSCFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZSCFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZSCFC` writer - TZSCFC"]
pub struct TZSCFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TZSCFC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZICFC` reader - TZICFC"]
pub struct TZICFC_R(crate::FieldReader<bool, bool>);
impl TZICFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZICFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZICFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZICFC` writer - TZICFC"]
pub struct TZICFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TZICFC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MPCWM1FC` reader - MPCWM1FC"]
pub struct MPCWM1FC_R(crate::FieldReader<bool, bool>);
impl MPCWM1FC_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCWM1FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCWM1FC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCWM1FC` writer - MPCWM1FC"]
pub struct MPCWM1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCWM1FC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MPCWM2FC` reader - MPCWM2FC"]
pub struct MPCWM2FC_R(crate::FieldReader<bool, bool>);
impl MPCWM2FC_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCWM2FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCWM2FC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCWM2FC` writer - MPCWM2FC"]
pub struct MPCWM2FC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCWM2FC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MPCBB1FC` reader - MPCBB1FC"]
pub struct MPCBB1FC_R(crate::FieldReader<bool, bool>);
impl MPCBB1FC_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCBB1FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCBB1FC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCBB1FC` writer - MPCBB1FC"]
pub struct MPCBB1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB1FC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MPCBB1_REGFC` reader - MPCBB1_REGFC"]
pub struct MPCBB1_REGFC_R(crate::FieldReader<bool, bool>);
impl MPCBB1_REGFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCBB1_REGFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCBB1_REGFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCBB1_REGFC` writer - MPCBB1_REGFC"]
pub struct MPCBB1_REGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB1_REGFC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MPCBB2FC` reader - MPCBB2FC"]
pub struct MPCBB2FC_R(crate::FieldReader<bool, bool>);
impl MPCBB2FC_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCBB2FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCBB2FC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCBB2FC` writer - MPCBB2FC"]
pub struct MPCBB2FC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB2FC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MPCBB2_REGFC` reader - MPCBB2_REGFC"]
pub struct MPCBB2_REGFC_R(crate::FieldReader<bool, bool>);
impl MPCBB2_REGFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPCBB2_REGFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPCBB2_REGFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPCBB2_REGFC` writer - MPCBB2_REGFC"]
pub struct MPCBB2_REGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB2_REGFC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - TZSCFC"]
    #[inline(always)]
    pub fn tzscfc(&self) -> TZSCFC_R {
        TZSCFC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TZICFC"]
    #[inline(always)]
    pub fn tzicfc(&self) -> TZICFC_R {
        TZICFC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPCWM1FC"]
    #[inline(always)]
    pub fn mpcwm1fc(&self) -> MPCWM1FC_R {
        MPCWM1FC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MPCWM2FC"]
    #[inline(always)]
    pub fn mpcwm2fc(&self) -> MPCWM2FC_R {
        MPCWM2FC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPCBB1FC"]
    #[inline(always)]
    pub fn mpcbb1fc(&self) -> MPCBB1FC_R {
        MPCBB1FC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPCBB1_REGFC"]
    #[inline(always)]
    pub fn mpcbb1_regfc(&self) -> MPCBB1_REGFC_R {
        MPCBB1_REGFC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPCBB2FC"]
    #[inline(always)]
    pub fn mpcbb2fc(&self) -> MPCBB2FC_R {
        MPCBB2FC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPCBB2_REGFC"]
    #[inline(always)]
    pub fn mpcbb2_regfc(&self) -> MPCBB2_REGFC_R {
        MPCBB2_REGFC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZSCFC"]
    #[inline(always)]
    pub fn tzscfc(&mut self) -> TZSCFC_W {
        TZSCFC_W { w: self }
    }
    #[doc = "Bit 1 - TZICFC"]
    #[inline(always)]
    pub fn tzicfc(&mut self) -> TZICFC_W {
        TZICFC_W { w: self }
    }
    #[doc = "Bit 2 - MPCWM1FC"]
    #[inline(always)]
    pub fn mpcwm1fc(&mut self) -> MPCWM1FC_W {
        MPCWM1FC_W { w: self }
    }
    #[doc = "Bit 3 - MPCWM2FC"]
    #[inline(always)]
    pub fn mpcwm2fc(&mut self) -> MPCWM2FC_W {
        MPCWM2FC_W { w: self }
    }
    #[doc = "Bit 4 - MPCBB1FC"]
    #[inline(always)]
    pub fn mpcbb1fc(&mut self) -> MPCBB1FC_W {
        MPCBB1FC_W { w: self }
    }
    #[doc = "Bit 5 - MPCBB1_REGFC"]
    #[inline(always)]
    pub fn mpcbb1_regfc(&mut self) -> MPCBB1_REGFC_W {
        MPCBB1_REGFC_W { w: self }
    }
    #[doc = "Bit 6 - MPCBB2FC"]
    #[inline(always)]
    pub fn mpcbb2fc(&mut self) -> MPCBB2FC_W {
        MPCBB2FC_W { w: self }
    }
    #[doc = "Bit 7 - MPCBB2_REGFC"]
    #[inline(always)]
    pub fn mpcbb2_regfc(&mut self) -> MPCBB2_REGFC_W {
        MPCBB2_REGFC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt clear register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr3](index.html) module"]
pub struct FCR3_SPEC;
impl crate::RegisterSpec for FCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcr3::R](R) reader structure"]
impl crate::Readable for FCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcr3::W](W) writer structure"]
impl crate::Writable for FCR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCR3 to value 0"]
impl crate::Resettable for FCR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
