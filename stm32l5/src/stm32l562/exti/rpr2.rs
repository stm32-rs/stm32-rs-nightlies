#[doc = "Register `RPR2` reader"]
pub struct R(crate::R<RPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPR2` writer"]
pub struct W(crate::W<RPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPR2_SPEC>;
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
impl From<crate::W<RPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPIF35` reader - RPIF35"]
pub struct RPIF35_R(crate::FieldReader<bool, bool>);
impl RPIF35_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF35_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF35_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF35` writer - RPIF35"]
pub struct RPIF35_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF35_W<'a> {
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
#[doc = "Field `RPIF36` reader - RPIF36"]
pub struct RPIF36_R(crate::FieldReader<bool, bool>);
impl RPIF36_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF36_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF36_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF36` writer - RPIF36"]
pub struct RPIF36_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF36_W<'a> {
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
#[doc = "Field `RPIF37` reader - RPIF37"]
pub struct RPIF37_R(crate::FieldReader<bool, bool>);
impl RPIF37_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF37_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF37_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF37` writer - RPIF37"]
pub struct RPIF37_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF37_W<'a> {
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
#[doc = "Field `RPIF38` reader - RPIF38"]
pub struct RPIF38_R(crate::FieldReader<bool, bool>);
impl RPIF38_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF38_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIF38_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIF38` writer - RPIF38"]
pub struct RPIF38_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF38_W<'a> {
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
    #[doc = "Bit 3 - RPIF35"]
    #[inline(always)]
    pub fn rpif35(&self) -> RPIF35_R {
        RPIF35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RPIF36"]
    #[inline(always)]
    pub fn rpif36(&self) -> RPIF36_R {
        RPIF36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RPIF37"]
    #[inline(always)]
    pub fn rpif37(&self) -> RPIF37_R {
        RPIF37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RPIF38"]
    #[inline(always)]
    pub fn rpif38(&self) -> RPIF38_R {
        RPIF38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - RPIF35"]
    #[inline(always)]
    pub fn rpif35(&mut self) -> RPIF35_W {
        RPIF35_W { w: self }
    }
    #[doc = "Bit 4 - RPIF36"]
    #[inline(always)]
    pub fn rpif36(&mut self) -> RPIF36_W {
        RPIF36_W { w: self }
    }
    #[doc = "Bit 5 - RPIF37"]
    #[inline(always)]
    pub fn rpif37(&mut self) -> RPIF37_W {
        RPIF37_W { w: self }
    }
    #[doc = "Bit 6 - RPIF38"]
    #[inline(always)]
    pub fn rpif38(&mut self) -> RPIF38_W {
        RPIF38_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI rising edge pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpr2](index.html) module"]
pub struct RPR2_SPEC;
impl crate::RegisterSpec for RPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpr2::R](R) reader structure"]
impl crate::Readable for RPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpr2::W](W) writer structure"]
impl crate::Writable for RPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RPR2 to value 0"]
impl crate::Resettable for RPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
