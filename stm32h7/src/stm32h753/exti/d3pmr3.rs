#[doc = "Register `D3PMR3` reader"]
pub struct R(crate::R<D3PMR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3PMR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3PMR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3PMR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D3PMR3` writer"]
pub struct W(crate::W<D3PMR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3PMR3_SPEC>;
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
impl From<crate::W<D3PMR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3PMR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "D3 Pending Mask on Event input x+64\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR88_A {
    #[doc = "0: Interrupt request line is masked"]
    MASKED = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    UNMASKED = 1,
}
impl From<MR88_A> for bool {
    #[inline(always)]
    fn from(variant: MR88_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR88` reader - D3 Pending Mask on Event input x+64"]
pub struct MR88_R(crate::FieldReader<bool, MR88_A>);
impl MR88_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR88_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR88_A {
        match self.bits {
            false => MR88_A::MASKED,
            true => MR88_A::UNMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == MR88_A::MASKED
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == MR88_A::UNMASKED
    }
}
impl core::ops::Deref for MR88_R {
    type Target = crate::FieldReader<bool, MR88_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR88` writer - D3 Pending Mask on Event input x+64"]
pub struct MR88_W<'a> {
    w: &'a mut W,
}
impl<'a> MR88_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR88_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR88_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR88_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - D3 Pending Mask on Event input x+64"]
    #[inline(always)]
    pub fn mr88(&self) -> MR88_R {
        MR88_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - D3 Pending Mask on Event input x+64"]
    #[inline(always)]
    pub fn mr88(&mut self) -> MR88_W {
        MR88_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI D3 pending mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pmr3](index.html) module"]
pub struct D3PMR3_SPEC;
impl crate::RegisterSpec for D3PMR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d3pmr3::R](R) reader structure"]
impl crate::Readable for D3PMR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d3pmr3::W](W) writer structure"]
impl crate::Writable for D3PMR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D3PMR3 to value 0"]
impl crate::Resettable for D3PMR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
