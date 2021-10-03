#[doc = "Register `SWIER3` reader"]
pub struct R(crate::R<SWIER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWIER3` writer"]
pub struct W(crate::W<SWIER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER3_SPEC>;
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
impl From<crate::W<SWIER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software interrupt on line x+64\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWIER82_A {
    #[doc = "1: Generates an interrupt request"]
    PEND = 1,
}
impl From<SWIER82_A> for bool {
    #[inline(always)]
    fn from(variant: SWIER82_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWIER82` reader - Software interrupt on line x+64"]
pub struct SWIER82_R(crate::FieldReader<bool, SWIER82_A>);
impl SWIER82_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWIER82_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWIER82_A> {
        match self.bits {
            true => Some(SWIER82_A::PEND),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PEND`"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        **self == SWIER82_A::PEND
    }
}
impl core::ops::Deref for SWIER82_R {
    type Target = crate::FieldReader<bool, SWIER82_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWIER82` writer - Software interrupt on line x+64"]
pub struct SWIER82_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER82_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER82_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER82_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Software interrupt on line x+64"]
pub type SWIER84_A = SWIER82_A;
#[doc = "Field `SWIER84` reader - Software interrupt on line x+64"]
pub type SWIER84_R = SWIER82_R;
#[doc = "Field `SWIER84` writer - Software interrupt on line x+64"]
pub struct SWIER84_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER84_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER84_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER84_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Software interrupt on line x+64"]
pub type SWIER85_A = SWIER82_A;
#[doc = "Field `SWIER85` reader - Software interrupt on line x+64"]
pub type SWIER85_R = SWIER82_R;
#[doc = "Field `SWIER85` writer - Software interrupt on line x+64"]
pub struct SWIER85_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER85_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER85_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER85_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Software interrupt on line x+64"]
pub type SWIER86_A = SWIER82_A;
#[doc = "Field `SWIER86` reader - Software interrupt on line x+64"]
pub type SWIER86_R = SWIER82_R;
#[doc = "Field `SWIER86` writer - Software interrupt on line x+64"]
pub struct SWIER86_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER86_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER86_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER86_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier82(&self) -> SWIER82_R {
        SWIER82_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier84(&self) -> SWIER84_R {
        SWIER84_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier85(&self) -> SWIER85_R {
        SWIER85_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier86(&self) -> SWIER86_R {
        SWIER86_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier82(&mut self) -> SWIER82_W {
        SWIER82_W { w: self }
    }
    #[doc = "Bit 20 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier84(&mut self) -> SWIER84_W {
        SWIER84_W { w: self }
    }
    #[doc = "Bit 21 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier85(&mut self) -> SWIER85_W {
        SWIER85_W { w: self }
    }
    #[doc = "Bit 22 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier86(&mut self) -> SWIER86_W {
        SWIER86_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier3](index.html) module"]
pub struct SWIER3_SPEC;
impl crate::RegisterSpec for SWIER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swier3::R](R) reader structure"]
impl crate::Readable for SWIER3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swier3::W](W) writer structure"]
impl crate::Writable for SWIER3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWIER3 to value 0"]
impl crate::Resettable for SWIER3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
