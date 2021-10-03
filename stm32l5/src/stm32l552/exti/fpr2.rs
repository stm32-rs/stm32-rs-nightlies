#[doc = "Register `FPR2` reader"]
pub struct R(crate::R<FPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPR2` writer"]
pub struct W(crate::W<FPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPR2_SPEC>;
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
impl From<crate::W<FPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPIF35` reader - FPIF35"]
pub struct FPIF35_R(crate::FieldReader<bool, bool>);
impl FPIF35_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF35_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF35_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF35` writer - FPIF35"]
pub struct FPIF35_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF35_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FPIF36` reader - FPIF36"]
pub struct FPIF36_R(crate::FieldReader<bool, bool>);
impl FPIF36_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF36_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF36_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF36` writer - FPIF36"]
pub struct FPIF36_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF36_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FPIF37` reader - FPIF37"]
pub struct FPIF37_R(crate::FieldReader<bool, bool>);
impl FPIF37_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF37_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF37_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF37` writer - FPIF37"]
pub struct FPIF37_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF37_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FPIF38` reader - FPIF38"]
pub struct FPIF38_R(crate::FieldReader<bool, bool>);
impl FPIF38_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF38_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIF38_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIF38` writer - FPIF38"]
pub struct FPIF38_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF38_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 3 - FPIF35"]
    #[inline(always)]
    pub fn fpif35(&self) -> FPIF35_R {
        FPIF35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FPIF36"]
    #[inline(always)]
    pub fn fpif36(&self) -> FPIF36_R {
        FPIF36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FPIF37"]
    #[inline(always)]
    pub fn fpif37(&self) -> FPIF37_R {
        FPIF37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FPIF38"]
    #[inline(always)]
    pub fn fpif38(&self) -> FPIF38_R {
        FPIF38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - FPIF35"]
    #[inline(always)]
    pub fn fpif35(&mut self) -> FPIF35_W {
        FPIF35_W { w: self }
    }
    #[doc = "Bit 4 - FPIF36"]
    #[inline(always)]
    pub fn fpif36(&mut self) -> FPIF36_W {
        FPIF36_W { w: self }
    }
    #[doc = "Bit 5 - FPIF37"]
    #[inline(always)]
    pub fn fpif37(&mut self) -> FPIF37_W {
        FPIF37_W { w: self }
    }
    #[doc = "Bit 6 - FPIF38"]
    #[inline(always)]
    pub fn fpif38(&mut self) -> FPIF38_W {
        FPIF38_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI falling edge pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpr2](index.html) module"]
pub struct FPR2_SPEC;
impl crate::RegisterSpec for FPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpr2::R](R) reader structure"]
impl crate::Readable for FPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpr2::W](W) writer structure"]
impl crate::Writable for FPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPR2 to value 0"]
impl crate::Resettable for FPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
