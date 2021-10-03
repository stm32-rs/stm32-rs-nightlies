#[doc = "Register `SWIER2` reader"]
pub struct R(crate::R<SWIER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWIER2` writer"]
pub struct W(crate::W<SWIER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER2_SPEC>;
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
impl From<crate::W<SWIER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software interrupt on line 35\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWI35_A {
    #[doc = "1: Generates an interrupt request"]
    PEND = 1,
}
impl From<SWI35_A> for bool {
    #[inline(always)]
    fn from(variant: SWI35_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI35` reader - Software interrupt on line 35"]
pub struct SWI35_R(crate::FieldReader<bool, SWI35_A>);
impl SWI35_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI35_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWI35_A> {
        match self.bits {
            true => Some(SWI35_A::PEND),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PEND`"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        **self == SWI35_A::PEND
    }
}
impl core::ops::Deref for SWI35_R {
    type Target = crate::FieldReader<bool, SWI35_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI35` writer - Software interrupt on line 35"]
pub struct SWI35_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI35_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWI35_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI35_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Software interrupt on line 36"]
pub type SWI36_A = SWI35_A;
#[doc = "Field `SWI36` reader - Software interrupt on line 36"]
pub type SWI36_R = SWI35_R;
#[doc = "Field `SWI36` writer - Software interrupt on line 36"]
pub struct SWI36_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI36_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWI36_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI36_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Software interrupt on line 37"]
pub type SWI37_A = SWI35_A;
#[doc = "Field `SWI37` reader - Software interrupt on line 37"]
pub type SWI37_R = SWI35_R;
#[doc = "Field `SWI37` writer - Software interrupt on line 37"]
pub struct SWI37_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI37_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWI37_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI37_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Software interrupt on line 38"]
pub type SWI38_A = SWI35_A;
#[doc = "Field `SWI38` reader - Software interrupt on line 38"]
pub type SWI38_R = SWI35_R;
#[doc = "Field `SWI38` writer - Software interrupt on line 38"]
pub struct SWI38_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI38_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWI38_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI38_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Software interrupt on line 35"]
    #[inline(always)]
    pub fn swi35(&self) -> SWI35_R {
        SWI35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software interrupt on line 36"]
    #[inline(always)]
    pub fn swi36(&self) -> SWI36_R {
        SWI36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Software interrupt on line 37"]
    #[inline(always)]
    pub fn swi37(&self) -> SWI37_R {
        SWI37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Software interrupt on line 38"]
    #[inline(always)]
    pub fn swi38(&self) -> SWI38_R {
        SWI38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Software interrupt on line 35"]
    #[inline(always)]
    pub fn swi35(&mut self) -> SWI35_W {
        SWI35_W { w: self }
    }
    #[doc = "Bit 4 - Software interrupt on line 36"]
    #[inline(always)]
    pub fn swi36(&mut self) -> SWI36_W {
        SWI36_W { w: self }
    }
    #[doc = "Bit 5 - Software interrupt on line 37"]
    #[inline(always)]
    pub fn swi37(&mut self) -> SWI37_W {
        SWI37_W { w: self }
    }
    #[doc = "Bit 6 - Software interrupt on line 38"]
    #[inline(always)]
    pub fn swi38(&mut self) -> SWI38_W {
        SWI38_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier2](index.html) module"]
pub struct SWIER2_SPEC;
impl crate::RegisterSpec for SWIER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swier2::R](R) reader structure"]
impl crate::Readable for SWIER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swier2::W](W) writer structure"]
impl crate::Writable for SWIER2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWIER2 to value 0"]
impl crate::Resettable for SWIER2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
