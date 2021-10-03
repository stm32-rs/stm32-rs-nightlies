#[doc = "Register `FTSR3` reader"]
pub struct R(crate::R<FTSR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTSR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTSR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTSR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTSR3` writer"]
pub struct W(crate::W<FTSR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTSR3_SPEC>;
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
impl From<crate::W<FTSR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTSR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Falling trigger event configuration bit of Configurable Event input x+64\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TR82_A {
    #[doc = "0: Falling edge trigger is disabled"]
    DISABLED = 0,
    #[doc = "1: Falling edge trigger is enabled"]
    ENABLED = 1,
}
impl From<TR82_A> for bool {
    #[inline(always)]
    fn from(variant: TR82_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TR82` reader - Falling trigger event configuration bit of Configurable Event input x+64"]
pub struct TR82_R(crate::FieldReader<bool, TR82_A>);
impl TR82_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR82_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TR82_A {
        match self.bits {
            false => TR82_A::DISABLED,
            true => TR82_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TR82_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TR82_A::ENABLED
    }
}
impl core::ops::Deref for TR82_R {
    type Target = crate::FieldReader<bool, TR82_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR82` writer - Falling trigger event configuration bit of Configurable Event input x+64"]
pub struct TR82_W<'a> {
    w: &'a mut W,
}
impl<'a> TR82_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TR82_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TR82_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TR82_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input x+64"]
pub type TR84_A = TR82_A;
#[doc = "Field `TR84` reader - Falling trigger event configuration bit of Configurable Event input x+64"]
pub type TR84_R = TR82_R;
#[doc = "Field `TR84` writer - Falling trigger event configuration bit of Configurable Event input x+64"]
pub struct TR84_W<'a> {
    w: &'a mut W,
}
impl<'a> TR84_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TR84_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TR84_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TR84_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input x+64"]
pub type TR85_A = TR82_A;
#[doc = "Field `TR85` reader - Falling trigger event configuration bit of Configurable Event input x+64"]
pub type TR85_R = TR82_R;
#[doc = "Field `TR85` writer - Falling trigger event configuration bit of Configurable Event input x+64"]
pub struct TR85_W<'a> {
    w: &'a mut W,
}
impl<'a> TR85_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TR85_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TR85_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TR85_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input x+64"]
pub type TR86_A = TR82_A;
#[doc = "Field `TR86` reader - Falling trigger event configuration bit of Configurable Event input x+64"]
pub type TR86_R = TR82_R;
#[doc = "Field `TR86` writer - Falling trigger event configuration bit of Configurable Event input x+64"]
pub struct TR86_W<'a> {
    w: &'a mut W,
}
impl<'a> TR86_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TR86_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TR86_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TR86_A::ENABLED)
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
    #[doc = "Bit 18 - Falling trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr82(&self) -> TR82_R {
        TR82_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Falling trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr84(&self) -> TR84_R {
        TR84_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Falling trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr85(&self) -> TR85_R {
        TR85_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Falling trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr86(&self) -> TR86_R {
        TR86_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Falling trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr82(&mut self) -> TR82_W {
        TR82_W { w: self }
    }
    #[doc = "Bit 20 - Falling trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr84(&mut self) -> TR84_W {
        TR84_W { w: self }
    }
    #[doc = "Bit 21 - Falling trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr85(&mut self) -> TR85_W {
        TR85_W { w: self }
    }
    #[doc = "Bit 22 - Falling trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr86(&mut self) -> TR86_W {
        TR86_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI falling trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr3](index.html) module"]
pub struct FTSR3_SPEC;
impl crate::RegisterSpec for FTSR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftsr3::R](R) reader structure"]
impl crate::Readable for FTSR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftsr3::W](W) writer structure"]
impl crate::Writable for FTSR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FTSR3 to value 0"]
impl crate::Resettable for FTSR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
