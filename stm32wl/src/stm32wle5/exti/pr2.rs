#[doc = "Register `PR2` reader"]
pub struct R(crate::R<PR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR2` writer"]
pub struct W(crate::W<PR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR2_SPEC>;
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
impl From<crate::W<PR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Configurable event inputs 33 Pending bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIF34_A {
    #[doc = "0: No trigger request occurred"]
    NOTPENDING = 0,
    #[doc = "1: Selected trigger request occurred"]
    PENDING = 1,
}
impl From<PIF34_A> for bool {
    #[inline(always)]
    fn from(variant: PIF34_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIF34` reader - Configurable event inputs 33 Pending bit."]
pub struct PIF34_R(crate::FieldReader<bool, PIF34_A>);
impl PIF34_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF34_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIF34_A {
        match self.bits {
            false => PIF34_A::NOTPENDING,
            true => PIF34_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == PIF34_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == PIF34_A::PENDING
    }
}
impl core::ops::Deref for PIF34_R {
    type Target = crate::FieldReader<bool, PIF34_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Configurable event inputs 33 Pending bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIF34_AW {
    #[doc = "1: Clears pending bit"]
    CLEAR = 1,
}
impl From<PIF34_AW> for bool {
    #[inline(always)]
    fn from(variant: PIF34_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIF34` writer - Configurable event inputs 33 Pending bit."]
pub struct PIF34_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF34_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF34_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF34_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Configurable event inputs 45 Pending bit."]
pub type PIF45_A = PIF34_A;
#[doc = "Field `PIF45` reader - Configurable event inputs 45 Pending bit."]
pub type PIF45_R = PIF34_R;
#[doc = "Configurable event inputs 45 Pending bit."]
pub type PIF45_AW = PIF34_AW;
#[doc = "Field `PIF45` writer - Configurable event inputs 45 Pending bit."]
pub struct PIF45_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF45_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF45_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF45_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Configurable event inputs 33 Pending bit."]
    #[inline(always)]
    pub fn pif34(&self) -> PIF34_R {
        PIF34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Configurable event inputs 45 Pending bit."]
    #[inline(always)]
    pub fn pif45(&self) -> PIF45_R {
        PIF45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Configurable event inputs 33 Pending bit."]
    #[inline(always)]
    pub fn pif34(&mut self) -> PIF34_W {
        PIF34_W { w: self }
    }
    #[doc = "Bit 13 - Configurable event inputs 45 Pending bit."]
    #[inline(always)]
    pub fn pif45(&mut self) -> PIF45_W {
        PIF45_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr2](index.html) module"]
pub struct PR2_SPEC;
impl crate::RegisterSpec for PR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr2::R](R) reader structure"]
impl crate::Readable for PR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr2::W](W) writer structure"]
impl crate::Writable for PR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PR2 to value 0"]
impl crate::Resettable for PR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
