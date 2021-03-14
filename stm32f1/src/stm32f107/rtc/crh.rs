#[doc = "Reader of register CRH"]
pub type R = crate::R<u32, super::CRH>;
#[doc = "Writer for register CRH"]
pub type W = crate::W<u32, super::CRH>;
#[doc = "Register CRH `reset()`'s with value 0"]
impl crate::ResetValue for super::CRH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Second interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECIE_A {
    #[doc = "0: Second interrupt is masked"]
    DISABLED = 0,
    #[doc = "1: Second interrupt is enabled"]
    ENABLED = 1,
}
impl From<SECIE_A> for bool {
    #[inline(always)]
    fn from(variant: SECIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SECIE`"]
pub type SECIE_R = crate::R<bool, SECIE_A>;
impl SECIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECIE_A {
        match self.bits {
            false => SECIE_A::DISABLED,
            true => SECIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SECIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SECIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `SECIE`"]
pub struct SECIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SECIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SECIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Second interrupt is masked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SECIE_A::DISABLED)
    }
    #[doc = "Second interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SECIE_A::ENABLED)
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
#[doc = "Alarm interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRIE_A {
    #[doc = "0: Alarm interrupt is masked"]
    DISABLED = 0,
    #[doc = "1: Alarm interrupt is enabled"]
    ENABLED = 1,
}
impl From<ALRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALRIE`"]
pub type ALRIE_R = crate::R<bool, ALRIE_A>;
impl ALRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRIE_A {
        match self.bits {
            false => ALRIE_A::DISABLED,
            true => ALRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `ALRIE`"]
pub struct ALRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Alarm interrupt is masked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRIE_A::DISABLED)
    }
    #[doc = "Alarm interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRIE_A::ENABLED)
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
#[doc = "Overflow interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWIE_A {
    #[doc = "0: Overflow interrupt is masked"]
    DISABLED = 0,
    #[doc = "1: Overflow interrupt is enabled"]
    ENABLED = 1,
}
impl From<OWIE_A> for bool {
    #[inline(always)]
    fn from(variant: OWIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OWIE`"]
pub type OWIE_R = crate::R<bool, OWIE_A>;
impl OWIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OWIE_A {
        match self.bits {
            false => OWIE_A::DISABLED,
            true => OWIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OWIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OWIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `OWIE`"]
pub struct OWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OWIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OWIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Overflow interrupt is masked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OWIE_A::DISABLED)
    }
    #[doc = "Overflow interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OWIE_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Second interrupt Enable"]
    #[inline(always)]
    pub fn secie(&self) -> SECIE_R {
        SECIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt Enable"]
    #[inline(always)]
    pub fn alrie(&self) -> ALRIE_R {
        ALRIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Overflow interrupt Enable"]
    #[inline(always)]
    pub fn owie(&self) -> OWIE_R {
        OWIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second interrupt Enable"]
    #[inline(always)]
    pub fn secie(&mut self) -> SECIE_W {
        SECIE_W { w: self }
    }
    #[doc = "Bit 1 - Alarm interrupt Enable"]
    #[inline(always)]
    pub fn alrie(&mut self) -> ALRIE_W {
        ALRIE_W { w: self }
    }
    #[doc = "Bit 2 - Overflow interrupt Enable"]
    #[inline(always)]
    pub fn owie(&mut self) -> OWIE_W {
        OWIE_W { w: self }
    }
}
