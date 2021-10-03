#[doc = "Register `C1PR3` reader"]
pub struct R(crate::R<C1PR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1PR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1PR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1PR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1PR3` writer"]
pub struct W(crate::W<C1PR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1PR3_SPEC>;
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
impl From<crate::W<C1PR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1PR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Configurable event inputs x+64 Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR82_A {
    #[doc = "0: No trigger request occurred"]
    NOTPENDING = 0,
    #[doc = "1: Selected trigger request occurred"]
    PENDING = 1,
}
impl From<PR82_A> for bool {
    #[inline(always)]
    fn from(variant: PR82_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR82` reader - Configurable event inputs x+64 Pending bit"]
pub struct PR82_R(crate::FieldReader<bool, PR82_A>);
impl PR82_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR82_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR82_A {
        match self.bits {
            false => PR82_A::NOTPENDING,
            true => PR82_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == PR82_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == PR82_A::PENDING
    }
}
impl core::ops::Deref for PR82_R {
    type Target = crate::FieldReader<bool, PR82_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Configurable event inputs x+64 Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR82_AW {
    #[doc = "1: Clears pending bit"]
    CLEAR = 1,
}
impl From<PR82_AW> for bool {
    #[inline(always)]
    fn from(variant: PR82_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR82` writer - Configurable event inputs x+64 Pending bit"]
pub struct PR82_W<'a> {
    w: &'a mut W,
}
impl<'a> PR82_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR82_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR82_AW::CLEAR)
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
#[doc = "Configurable event inputs x+64 Pending bit"]
pub type PR84_A = PR82_A;
#[doc = "Field `PR84` reader - Configurable event inputs x+64 Pending bit"]
pub type PR84_R = PR82_R;
#[doc = "Configurable event inputs x+64 Pending bit"]
pub type PR84_AW = PR82_AW;
#[doc = "Field `PR84` writer - Configurable event inputs x+64 Pending bit"]
pub struct PR84_W<'a> {
    w: &'a mut W,
}
impl<'a> PR84_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR84_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR84_AW::CLEAR)
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
#[doc = "Configurable event inputs x+64 Pending bit"]
pub type PR85_A = PR82_A;
#[doc = "Field `PR85` reader - Configurable event inputs x+64 Pending bit"]
pub type PR85_R = PR82_R;
#[doc = "Configurable event inputs x+64 Pending bit"]
pub type PR85_AW = PR82_AW;
#[doc = "Field `PR85` writer - Configurable event inputs x+64 Pending bit"]
pub struct PR85_W<'a> {
    w: &'a mut W,
}
impl<'a> PR85_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR85_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR85_AW::CLEAR)
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
#[doc = "Configurable event inputs x+64 Pending bit"]
pub type PR86_A = PR82_A;
#[doc = "Field `PR86` reader - Configurable event inputs x+64 Pending bit"]
pub type PR86_R = PR82_R;
#[doc = "Configurable event inputs x+64 Pending bit"]
pub type PR86_AW = PR82_AW;
#[doc = "Field `PR86` writer - Configurable event inputs x+64 Pending bit"]
pub struct PR86_W<'a> {
    w: &'a mut W,
}
impl<'a> PR86_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR86_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR86_AW::CLEAR)
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
    #[doc = "Bit 18 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr82(&self) -> PR82_R {
        PR82_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr84(&self) -> PR84_R {
        PR84_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr85(&self) -> PR85_R {
        PR85_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr86(&self) -> PR86_R {
        PR86_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr82(&mut self) -> PR82_W {
        PR82_W { w: self }
    }
    #[doc = "Bit 20 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr84(&mut self) -> PR84_W {
        PR84_W { w: self }
    }
    #[doc = "Bit 21 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr85(&mut self) -> PR85_W {
        PR85_W { w: self }
    }
    #[doc = "Bit 22 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr86(&mut self) -> PR86_W {
        PR86_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1pr3](index.html) module"]
pub struct C1PR3_SPEC;
impl crate::RegisterSpec for C1PR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1pr3::R](R) reader structure"]
impl crate::Readable for C1PR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1pr3::W](W) writer structure"]
impl crate::Writable for C1PR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1PR3 to value 0"]
impl crate::Resettable for C1PR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
