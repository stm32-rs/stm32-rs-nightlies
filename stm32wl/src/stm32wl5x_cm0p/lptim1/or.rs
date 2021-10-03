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
pub enum OR_1_A {
    #[doc = "0: LPTIM1 input 2 is connected to I/O"]
    IO = 0,
    #[doc = "1: LPTIM1 input 2 is connected to COMP2_OUT"]
    COMP2_OUT = 1,
}
impl From<OR_1_A> for bool {
    #[inline(always)]
    fn from(variant: OR_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OR_1` reader - Option register bit 1"]
pub struct OR_1_R(crate::FieldReader<bool, OR_1_A>);
impl OR_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OR_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OR_1_A {
        match self.bits {
            false => OR_1_A::IO,
            true => OR_1_A::COMP2_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        **self == OR_1_A::IO
    }
    #[doc = "Checks if the value of the field is `COMP2_OUT`"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        **self == OR_1_A::COMP2_OUT
    }
}
impl core::ops::Deref for OR_1_R {
    type Target = crate::FieldReader<bool, OR_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OR_1` writer - Option register bit 1"]
pub struct OR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> OR_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OR_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LPTIM1 input 2 is connected to I/O"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(OR_1_A::IO)
    }
    #[doc = "LPTIM1 input 2 is connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(OR_1_A::COMP2_OUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Option register bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OR_0_A {
    #[doc = "0: LPTIM1 input 1 is connected to I/O"]
    IO = 0,
    #[doc = "1: LPTIM1 input 1 is connected to COMP1_OUT"]
    COMP1_OUT = 1,
}
impl From<OR_0_A> for bool {
    #[inline(always)]
    fn from(variant: OR_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OR_0` reader - Option register bit 0"]
pub struct OR_0_R(crate::FieldReader<bool, OR_0_A>);
impl OR_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        OR_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OR_0_A {
        match self.bits {
            false => OR_0_A::IO,
            true => OR_0_A::COMP1_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        **self == OR_0_A::IO
    }
    #[doc = "Checks if the value of the field is `COMP1_OUT`"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        **self == OR_0_A::COMP1_OUT
    }
}
impl core::ops::Deref for OR_0_R {
    type Target = crate::FieldReader<bool, OR_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OR_0` writer - Option register bit 0"]
pub struct OR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> OR_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OR_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LPTIM1 input 1 is connected to I/O"]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(OR_0_A::IO)
    }
    #[doc = "LPTIM1 input 1 is connected to COMP1_OUT"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(OR_0_A::COMP1_OUT)
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
    #[doc = "Bit 1 - Option register bit 1"]
    #[inline(always)]
    pub fn or_1(&self) -> OR_1_R {
        OR_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Option register bit 0"]
    #[inline(always)]
    pub fn or_0(&self) -> OR_0_R {
        OR_0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Option register bit 1"]
    #[inline(always)]
    pub fn or_1(&mut self) -> OR_1_W {
        OR_1_W { w: self }
    }
    #[doc = "Bit 0 - Option register bit 0"]
    #[inline(always)]
    pub fn or_0(&mut self) -> OR_0_W {
        OR_0_W { w: self }
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
