#[doc = "Reader of register IER"]
pub type R = crate::R<u32, super::IER>;
#[doc = "Writer for register IER"]
pub type W = crate::W<u32, super::IER>;
#[doc = "Register IER `reset()`'s with value 0"]
impl crate::ResetValue for super::IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Direction change to down Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOWNIE_A {
    #[doc = "0: DOWN interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: DOWN interrupt enabled"]
    ENABLED = 1,
}
impl From<DOWNIE_A> for bool {
    #[inline(always)]
    fn from(variant: DOWNIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DOWNIE`"]
pub type DOWNIE_R = crate::R<bool, DOWNIE_A>;
impl DOWNIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOWNIE_A {
        match self.bits {
            false => DOWNIE_A::DISABLED,
            true => DOWNIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DOWNIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DOWNIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `DOWNIE`"]
pub struct DOWNIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWNIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOWNIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DOWN interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DOWNIE_A::DISABLED)
    }
    #[doc = "DOWN interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DOWNIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Direction change to UP Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPIE_A {
    #[doc = "0: UP interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: UP interrupt enabled"]
    ENABLED = 1,
}
impl From<UPIE_A> for bool {
    #[inline(always)]
    fn from(variant: UPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UPIE`"]
pub type UPIE_R = crate::R<bool, UPIE_A>;
impl UPIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPIE_A {
        match self.bits {
            false => UPIE_A::DISABLED,
            true => UPIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `UPIE`"]
pub struct UPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UP interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPIE_A::DISABLED)
    }
    #[doc = "UP interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UPIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Autoreload register update OK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARROKIE_A {
    #[doc = "0: ARROK interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: ARROK interrupt enabled"]
    ENABLED = 1,
}
impl From<ARROKIE_A> for bool {
    #[inline(always)]
    fn from(variant: ARROKIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ARROKIE`"]
pub type ARROKIE_R = crate::R<bool, ARROKIE_A>;
impl ARROKIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARROKIE_A {
        match self.bits {
            false => ARROKIE_A::DISABLED,
            true => ARROKIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARROKIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARROKIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `ARROKIE`"]
pub struct ARROKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARROKIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARROKIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ARROK interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARROKIE_A::DISABLED)
    }
    #[doc = "ARROK interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARROKIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Compare register update OK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPOKIE_A {
    #[doc = "0: CMPOK interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: CMPOK interrupt enabled"]
    ENABLED = 1,
}
impl From<CMPOKIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPOKIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMPOKIE`"]
pub type CMPOKIE_R = crate::R<bool, CMPOKIE_A>;
impl CMPOKIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPOKIE_A {
        match self.bits {
            false => CMPOKIE_A::DISABLED,
            true => CMPOKIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMPOKIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMPOKIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `CMPOKIE`"]
pub struct CMPOKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPOKIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPOKIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CMPOK interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMPOKIE_A::DISABLED)
    }
    #[doc = "CMPOK interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMPOKIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "External trigger valid edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTTRIGIE_A {
    #[doc = "0: EXTTRIG interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: EXTTRIG interrupt enabled"]
    ENABLED = 1,
}
impl From<EXTTRIGIE_A> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIGIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTTRIGIE`"]
pub type EXTTRIGIE_R = crate::R<bool, EXTTRIGIE_A>;
impl EXTTRIGIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTTRIGIE_A {
        match self.bits {
            false => EXTTRIGIE_A::DISABLED,
            true => EXTTRIGIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTTRIGIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXTTRIGIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `EXTTRIGIE`"]
pub struct EXTTRIGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTTRIGIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTTRIGIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EXTTRIG interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTTRIGIE_A::DISABLED)
    }
    #[doc = "EXTTRIG interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EXTTRIGIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Autoreload match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARRMIE_A {
    #[doc = "0: ARRM interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: ARRM interrupt enabled"]
    ENABLED = 1,
}
impl From<ARRMIE_A> for bool {
    #[inline(always)]
    fn from(variant: ARRMIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ARRMIE`"]
pub type ARRMIE_R = crate::R<bool, ARRMIE_A>;
impl ARRMIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARRMIE_A {
        match self.bits {
            false => ARRMIE_A::DISABLED,
            true => ARRMIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARRMIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARRMIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `ARRMIE`"]
pub struct ARRMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARRMIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARRMIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ARRM interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARRMIE_A::DISABLED)
    }
    #[doc = "ARRM interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARRMIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Compare match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPMIE_A {
    #[doc = "0: CMPM interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: CMPM interrupt enabled"]
    ENABLED = 1,
}
impl From<CMPMIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPMIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMPMIE`"]
pub type CMPMIE_R = crate::R<bool, CMPMIE_A>;
impl CMPMIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMIE_A {
        match self.bits {
            false => CMPMIE_A::DISABLED,
            true => CMPMIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMPMIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMPMIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `CMPMIE`"]
pub struct CMPMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPMIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPMIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CMPM interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMPMIE_A::DISABLED)
    }
    #[doc = "CMPM interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMPMIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - Direction change to down Interrupt Enable"]
    #[inline(always)]
    pub fn downie(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable"]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn arrokie(&self) -> ARROKIE_R {
        ARROKIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Compare register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn cmpokie(&self) -> CMPOKIE_R {
        CMPOKIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    pub fn exttrigie(&self) -> EXTTRIGIE_R {
        EXTTRIGIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    pub fn arrmie(&self) -> ARRMIE_R {
        ARRMIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Compare match Interrupt Enable"]
    #[inline(always)]
    pub fn cmpmie(&self) -> CMPMIE_R {
        CMPMIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Direction change to down Interrupt Enable"]
    #[inline(always)]
    pub fn downie(&mut self) -> DOWNIE_W {
        DOWNIE_W { w: self }
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable"]
    #[inline(always)]
    pub fn upie(&mut self) -> UPIE_W {
        UPIE_W { w: self }
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn arrokie(&mut self) -> ARROKIE_W {
        ARROKIE_W { w: self }
    }
    #[doc = "Bit 3 - Compare register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn cmpokie(&mut self) -> CMPOKIE_W {
        CMPOKIE_W { w: self }
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W {
        EXTTRIGIE_W { w: self }
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    pub fn arrmie(&mut self) -> ARRMIE_W {
        ARRMIE_W { w: self }
    }
    #[doc = "Bit 0 - Compare match Interrupt Enable"]
    #[inline(always)]
    pub fn cmpmie(&mut self) -> CMPMIE_W {
        CMPMIE_W { w: self }
    }
}
