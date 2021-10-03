#[doc = "Register `C2SCR` reader"]
pub struct R(crate::R<C2SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2SCR` writer"]
pub struct W(crate::W<C2SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2SCR_SPEC>;
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
impl From<crate::W<C2SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CH1C\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1C_A {
    #[doc = "1: Processor receive channel n status bit clear"]
    CLEAR = 1,
    #[doc = "0: No action"]
    NOACTION = 0,
}
impl From<CH1C_A> for bool {
    #[inline(always)]
    fn from(variant: CH1C_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1C` reader - CH1C"]
pub struct CH1C_R(crate::FieldReader<bool, CH1C_A>);
impl CH1C_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1C_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1C_A {
        match self.bits {
            true => CH1C_A::CLEAR,
            false => CH1C_A::NOACTION,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CH1C_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        **self == CH1C_A::NOACTION
    }
}
impl core::ops::Deref for CH1C_R {
    type Target = crate::FieldReader<bool, CH1C_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1C` writer - CH1C"]
pub struct CH1C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1C_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Processor receive channel n status bit clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH1C_A::CLEAR)
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH1C_A::NOACTION)
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
#[doc = "CH2C"]
pub type CH2C_A = CH1C_A;
#[doc = "Field `CH2C` reader - CH2C"]
pub type CH2C_R = CH1C_R;
#[doc = "Field `CH2C` writer - CH2C"]
pub struct CH2C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2C_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Processor receive channel n status bit clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH2C_A::CLEAR)
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH2C_A::NOACTION)
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
#[doc = "CH3C"]
pub type CH3C_A = CH1C_A;
#[doc = "Field `CH3C` reader - CH3C"]
pub type CH3C_R = CH1C_R;
#[doc = "Field `CH3C` writer - CH3C"]
pub struct CH3C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3C_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Processor receive channel n status bit clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH3C_A::CLEAR)
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH3C_A::NOACTION)
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
#[doc = "CH4C"]
pub type CH4C_A = CH1C_A;
#[doc = "Field `CH4C` reader - CH4C"]
pub type CH4C_R = CH1C_R;
#[doc = "Field `CH4C` writer - CH4C"]
pub struct CH4C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4C_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Processor receive channel n status bit clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH4C_A::CLEAR)
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH4C_A::NOACTION)
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
#[doc = "CH5C"]
pub type CH5C_A = CH1C_A;
#[doc = "Field `CH5C` reader - CH5C"]
pub type CH5C_R = CH1C_R;
#[doc = "Field `CH5C` writer - CH5C"]
pub struct CH5C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5C_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Processor receive channel n status bit clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH5C_A::CLEAR)
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH5C_A::NOACTION)
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
#[doc = "CH6C"]
pub type CH6C_A = CH1C_A;
#[doc = "Field `CH6C` reader - CH6C"]
pub type CH6C_R = CH1C_R;
#[doc = "Field `CH6C` writer - CH6C"]
pub struct CH6C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6C_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Processor receive channel n status bit clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH6C_A::CLEAR)
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH6C_A::NOACTION)
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
#[doc = "CH1S\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1S_A {
    #[doc = "1: Processor transmit channel n status bit set"]
    SET = 1,
    #[doc = "0: No action"]
    NOACTION = 0,
}
impl From<CH1S_A> for bool {
    #[inline(always)]
    fn from(variant: CH1S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1S` reader - CH1S"]
pub struct CH1S_R(crate::FieldReader<bool, CH1S_A>);
impl CH1S_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1S_A {
        match self.bits {
            true => CH1S_A::SET,
            false => CH1S_A::NOACTION,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == CH1S_A::SET
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        **self == CH1S_A::NOACTION
    }
}
impl core::ops::Deref for CH1S_R {
    type Target = crate::FieldReader<bool, CH1S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1S` writer - CH1S"]
pub struct CH1S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Processor transmit channel n status bit set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH1S_A::SET)
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH1S_A::NOACTION)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "CH2S"]
pub type CH2S_A = CH1S_A;
#[doc = "Field `CH2S` reader - CH2S"]
pub type CH2S_R = CH1S_R;
#[doc = "Field `CH2S` writer - CH2S"]
pub struct CH2S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Processor transmit channel n status bit set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH2S_A::SET)
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH2S_A::NOACTION)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "CH3S"]
pub type CH3S_A = CH1S_A;
#[doc = "Field `CH3S` reader - CH3S"]
pub type CH3S_R = CH1S_R;
#[doc = "Field `CH3S` writer - CH3S"]
pub struct CH3S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Processor transmit channel n status bit set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH3S_A::SET)
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH3S_A::NOACTION)
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
#[doc = "CH4S"]
pub type CH4S_A = CH1S_A;
#[doc = "Field `CH4S` reader - CH4S"]
pub type CH4S_R = CH1S_R;
#[doc = "Field `CH4S` writer - CH4S"]
pub struct CH4S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Processor transmit channel n status bit set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH4S_A::SET)
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH4S_A::NOACTION)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "CH5S"]
pub type CH5S_A = CH1S_A;
#[doc = "Field `CH5S` reader - CH5S"]
pub type CH5S_R = CH1S_R;
#[doc = "Field `CH5S` writer - CH5S"]
pub struct CH5S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Processor transmit channel n status bit set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH5S_A::SET)
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH5S_A::NOACTION)
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
#[doc = "CH6S"]
pub type CH6S_A = CH1S_A;
#[doc = "Field `CH6S` reader - CH6S"]
pub type CH6S_R = CH1S_R;
#[doc = "Field `CH6S` writer - CH6S"]
pub struct CH6S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Processor transmit channel n status bit set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH6S_A::SET)
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH6S_A::NOACTION)
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
impl R {
    #[doc = "Bit 0 - CH1C"]
    #[inline(always)]
    pub fn ch1c(&self) -> CH1C_R {
        CH1C_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CH2C"]
    #[inline(always)]
    pub fn ch2c(&self) -> CH2C_R {
        CH2C_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CH3C"]
    #[inline(always)]
    pub fn ch3c(&self) -> CH3C_R {
        CH3C_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CH4C"]
    #[inline(always)]
    pub fn ch4c(&self) -> CH4C_R {
        CH4C_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CH5C"]
    #[inline(always)]
    pub fn ch5c(&self) -> CH5C_R {
        CH5C_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CH6C"]
    #[inline(always)]
    pub fn ch6c(&self) -> CH6C_R {
        CH6C_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CH1S"]
    #[inline(always)]
    pub fn ch1s(&self) -> CH1S_R {
        CH1S_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CH2S"]
    #[inline(always)]
    pub fn ch2s(&self) -> CH2S_R {
        CH2S_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CH3S"]
    #[inline(always)]
    pub fn ch3s(&self) -> CH3S_R {
        CH3S_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CH4S"]
    #[inline(always)]
    pub fn ch4s(&self) -> CH4S_R {
        CH4S_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CH5S"]
    #[inline(always)]
    pub fn ch5s(&self) -> CH5S_R {
        CH5S_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CH6S"]
    #[inline(always)]
    pub fn ch6s(&self) -> CH6S_R {
        CH6S_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH1C"]
    #[inline(always)]
    pub fn ch1c(&mut self) -> CH1C_W {
        CH1C_W { w: self }
    }
    #[doc = "Bit 1 - CH2C"]
    #[inline(always)]
    pub fn ch2c(&mut self) -> CH2C_W {
        CH2C_W { w: self }
    }
    #[doc = "Bit 2 - CH3C"]
    #[inline(always)]
    pub fn ch3c(&mut self) -> CH3C_W {
        CH3C_W { w: self }
    }
    #[doc = "Bit 3 - CH4C"]
    #[inline(always)]
    pub fn ch4c(&mut self) -> CH4C_W {
        CH4C_W { w: self }
    }
    #[doc = "Bit 4 - CH5C"]
    #[inline(always)]
    pub fn ch5c(&mut self) -> CH5C_W {
        CH5C_W { w: self }
    }
    #[doc = "Bit 5 - CH6C"]
    #[inline(always)]
    pub fn ch6c(&mut self) -> CH6C_W {
        CH6C_W { w: self }
    }
    #[doc = "Bit 16 - CH1S"]
    #[inline(always)]
    pub fn ch1s(&mut self) -> CH1S_W {
        CH1S_W { w: self }
    }
    #[doc = "Bit 17 - CH2S"]
    #[inline(always)]
    pub fn ch2s(&mut self) -> CH2S_W {
        CH2S_W { w: self }
    }
    #[doc = "Bit 18 - CH3S"]
    #[inline(always)]
    pub fn ch3s(&mut self) -> CH3S_W {
        CH3S_W { w: self }
    }
    #[doc = "Bit 19 - CH4S"]
    #[inline(always)]
    pub fn ch4s(&mut self) -> CH4S_W {
        CH4S_W { w: self }
    }
    #[doc = "Bit 20 - CH5S"]
    #[inline(always)]
    pub fn ch5s(&mut self) -> CH5S_W {
        CH5S_W { w: self }
    }
    #[doc = "Bit 21 - CH6S"]
    #[inline(always)]
    pub fn ch6s(&mut self) -> CH6S_W {
        CH6S_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reading this register will always return 0x0000 0000.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2scr](index.html) module"]
pub struct C2SCR_SPEC;
impl crate::RegisterSpec for C2SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2scr::R](R) reader structure"]
impl crate::Readable for C2SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2scr::W](W) writer structure"]
impl crate::Writable for C2SCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2SCR to value 0"]
impl crate::Resettable for C2SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
