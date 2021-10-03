#[doc = "Register `C2EMR2` reader"]
pub struct R(crate::R<C2EMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2EMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2EMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2EMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2EMR2` writer"]
pub struct W(crate::W<C2EMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2EMR2_SPEC>;
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
impl From<crate::W<C2EMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2EMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wakeup with event generation Mask on Event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM40_A {
    #[doc = "0: Interrupt request line is masked"]
    MASKED = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    UNMASKED = 1,
}
impl From<EM40_A> for bool {
    #[inline(always)]
    fn from(variant: EM40_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM40` reader - Wakeup with event generation Mask on Event input"]
pub struct EM40_R(crate::FieldReader<bool, EM40_A>);
impl EM40_R {
    pub(crate) fn new(bits: bool) -> Self {
        EM40_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EM40_A {
        match self.bits {
            false => EM40_A::MASKED,
            true => EM40_A::UNMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == EM40_A::MASKED
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == EM40_A::UNMASKED
    }
}
impl core::ops::Deref for EM40_R {
    type Target = crate::FieldReader<bool, EM40_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EM40` writer - Wakeup with event generation Mask on Event input"]
pub struct EM40_W<'a> {
    w: &'a mut W,
}
impl<'a> EM40_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM40_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM40_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM40_A::UNMASKED)
    }
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
#[doc = "Wakeup with event generation Mask on Event input"]
pub type EM41_A = EM40_A;
#[doc = "Field `EM41` reader - Wakeup with event generation Mask on Event input"]
pub type EM41_R = EM40_R;
#[doc = "Field `EM41` writer - Wakeup with event generation Mask on Event input"]
pub struct EM41_W<'a> {
    w: &'a mut W,
}
impl<'a> EM41_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM41_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM41_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM41_A::UNMASKED)
    }
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
impl R {
    #[doc = "Bit 8 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em40(&self) -> EM40_R {
        EM40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em41(&self) -> EM41_R {
        EM41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em40(&mut self) -> EM40_W {
        EM40_W { w: self }
    }
    #[doc = "Bit 9 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em41(&mut self) -> EM41_W {
        EM41_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2emr2](index.html) module"]
pub struct C2EMR2_SPEC;
impl crate::RegisterSpec for C2EMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2emr2::R](R) reader structure"]
impl crate::Readable for C2EMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2emr2::W](W) writer structure"]
impl crate::Writable for C2EMR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2EMR2 to value 0"]
impl crate::Resettable for C2EMR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
