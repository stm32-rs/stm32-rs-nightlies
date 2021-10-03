#[doc = "Register `OR1` reader"]
pub struct R(crate::R<OR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR1` writer"]
pub struct W(crate::W<OR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR1_SPEC>;
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
impl From<crate::W<OR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Ocref_clr source selection This bit selects the ocref_clr input source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCREF_CLR_A {
    #[doc = "0: COMP1 output is connected to the OCREF_CLR input"]
    B_0X0 = 0,
    #[doc = "1: COMP2 output is connected to the OCREF_CLR input"]
    B_0X1 = 1,
}
impl From<OCREF_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: OCREF_CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCREF_CLR` reader - Ocref_clr source selection This bit selects the ocref_clr input source."]
pub struct OCREF_CLR_R(crate::FieldReader<bool, OCREF_CLR_A>);
impl OCREF_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCREF_CLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCREF_CLR_A {
        match self.bits {
            false => OCREF_CLR_A::B_0X0,
            true => OCREF_CLR_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == OCREF_CLR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == OCREF_CLR_A::B_0X1
    }
}
impl core::ops::Deref for OCREF_CLR_R {
    type Target = crate::FieldReader<bool, OCREF_CLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCREF_CLR` writer - Ocref_clr source selection This bit selects the ocref_clr input source."]
pub struct OCREF_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OCREF_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCREF_CLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "COMP1 output is connected to the OCREF_CLR input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OCREF_CLR_A::B_0X0)
    }
    #[doc = "COMP2 output is connected to the OCREF_CLR input"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OCREF_CLR_A::B_0X1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Ocref_clr source selection This bit selects the ocref_clr input source."]
    #[inline(always)]
    pub fn ocref_clr(&self) -> OCREF_CLR_R {
        OCREF_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ocref_clr source selection This bit selects the ocref_clr input source."]
    #[inline(always)]
    pub fn ocref_clr(&mut self) -> OCREF_CLR_W {
        OCREF_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or1](index.html) module"]
pub struct OR1_SPEC;
impl crate::RegisterSpec for OR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or1::R](R) reader structure"]
impl crate::Readable for OR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or1::W](W) writer structure"]
impl crate::Writable for OR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for OR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
