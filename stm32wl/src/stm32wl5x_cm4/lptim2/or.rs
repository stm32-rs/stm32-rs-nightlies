#[doc = "Register `OR` reader"]
pub struct R(crate::R<OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR` writer"]
pub struct W(crate::W<OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR_SPEC>;
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
impl From<crate::W<OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Option register bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OR__A {
    #[doc = "0: Input 1 is connected to I/O"]
    IO = 0,
    #[doc = "1: Input 1 is connected to COMP1_OUT"]
    COMP1_OUT = 1,
    #[doc = "2: Input 1 is connected to COMP2_OUT"]
    COMP2_OUT = 2,
    #[doc = "3: Input 1 is connected to COMP1_OUT OR COMP2_OUT"]
    OR_COMP1_COMP2 = 3,
}
impl From<OR__A> for u8 {
    #[inline(always)]
    fn from(variant: OR__A) -> Self {
        variant as _
    }
}
#[doc = "Field `OR_` reader - Option register bit 1"]
pub struct OR__R(crate::FieldReader<u8, OR__A>);
impl OR__R {
    pub(crate) fn new(bits: u8) -> Self {
        OR__R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OR__A {
        match self.bits {
            0 => OR__A::IO,
            1 => OR__A::COMP1_OUT,
            2 => OR__A::COMP2_OUT,
            3 => OR__A::OR_COMP1_COMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        **self == OR__A::IO
    }
    #[doc = "Checks if the value of the field is `COMP1_OUT`"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        **self == OR__A::COMP1_OUT
    }
    #[doc = "Checks if the value of the field is `COMP2_OUT`"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        **self == OR__A::COMP2_OUT
    }
    #[doc = "Checks if the value of the field is `OR_COMP1_COMP2`"]
    #[inline(always)]
    pub fn is_or_comp1_comp2(&self) -> bool {
        **self == OR__A::OR_COMP1_COMP2
    }
}
impl core::ops::Deref for OR__R {
    type Target = crate::FieldReader<u8, OR__A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OR_` writer - Option register bit 1"]
pub struct OR__W<'a> {
    w: &'a mut W,
}
impl<'a> OR__W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OR__A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input 1 is connected to I/O"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(OR__A::IO)
    }
    #[doc = "Input 1 is connected to COMP1_OUT"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(OR__A::COMP1_OUT)
    }
    #[doc = "Input 1 is connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(OR__A::COMP2_OUT)
    }
    #[doc = "Input 1 is connected to COMP1_OUT OR COMP2_OUT"]
    #[inline(always)]
    pub fn or_comp1_comp2(self) -> &'a mut W {
        self.variant(OR__A::OR_COMP1_COMP2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Option register bit 1"]
    #[inline(always)]
    pub fn or_(&self) -> OR__R {
        OR__R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Option register bit 1"]
    #[inline(always)]
    pub fn or_(&mut self) -> OR__W {
        OR__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or](index.html) module"]
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or::R](R) reader structure"]
impl crate::Readable for OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or::W](W) writer structure"]
impl crate::Writable for OR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
