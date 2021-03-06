#[doc = "Reader of register CCER"]
pub type R = crate::R<u32, super::CCER>;
#[doc = "Writer for register CCER"]
pub type W = crate::W<u32, super::CCER>;
#[doc = "Register CCER `reset()`'s with value 0"]
impl crate::ResetValue for super::CCER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Capture/Compare 4 output Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC4NP_A {
    #[doc = "0: Negative polarity"]
    NEGATIVE = 0,
    #[doc = "1: Positive polarity"]
    POSITIVE = 1,
}
impl From<CC4NP_A> for bool {
    #[inline(always)]
    fn from(variant: CC4NP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CC4NP`"]
pub type CC4NP_R = crate::R<bool, CC4NP_A>;
impl CC4NP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC4NP_A {
        match self.bits {
            false => CC4NP_A::NEGATIVE,
            true => CC4NP_A::POSITIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == CC4NP_A::NEGATIVE
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == CC4NP_A::POSITIVE
    }
}
#[doc = "Write proxy for field `CC4NP`"]
pub struct CC4NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC4NP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Negative polarity"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(CC4NP_A::NEGATIVE)
    }
    #[doc = "Positive polarity"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(CC4NP_A::POSITIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Capture/Compare 3 output Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC4P_A {
    #[doc = "0: Noninverted/rising edge"]
    RISINGEDGE = 0,
    #[doc = "1: Inverted/falling edge"]
    FALLINGEDGE = 1,
}
impl From<CC4P_A> for bool {
    #[inline(always)]
    fn from(variant: CC4P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CC4P`"]
pub type CC4P_R = crate::R<bool, CC4P_A>;
impl CC4P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC4P_A {
        match self.bits {
            false => CC4P_A::RISINGEDGE,
            true => CC4P_A::FALLINGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CC4P_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CC4P_A::FALLINGEDGE
    }
}
#[doc = "Write proxy for field `CC4P`"]
pub struct CC4P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC4P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Noninverted/rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CC4P_A::RISINGEDGE)
    }
    #[doc = "Inverted/falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CC4P_A::FALLINGEDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Capture/Compare 4 output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC4E_A {
    #[doc = "0: Capture disabled"]
    DISABLED = 0,
    #[doc = "1: Capture enabled"]
    ENABLED = 1,
}
impl From<CC4E_A> for bool {
    #[inline(always)]
    fn from(variant: CC4E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CC4E`"]
pub type CC4E_R = crate::R<bool, CC4E_A>;
impl CC4E_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC4E_A {
        match self.bits {
            false => CC4E_A::DISABLED,
            true => CC4E_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC4E_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC4E_A::ENABLED
    }
}
#[doc = "Write proxy for field `CC4E`"]
pub struct CC4E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC4E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4E_A::DISABLED)
    }
    #[doc = "Capture enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4E_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Capture/Compare 3 output Polarity"]
pub type CC3NP_A = CC4NP_A;
#[doc = "Reader of field `CC3NP`"]
pub type CC3NP_R = crate::R<bool, CC4NP_A>;
#[doc = "Write proxy for field `CC3NP`"]
pub struct CC3NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC3NP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Negative polarity"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(CC4NP_A::NEGATIVE)
    }
    #[doc = "Positive polarity"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(CC4NP_A::POSITIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Capture/Compare 3 output Polarity"]
pub type CC3P_A = CC4P_A;
#[doc = "Reader of field `CC3P`"]
pub type CC3P_R = crate::R<bool, CC4P_A>;
#[doc = "Write proxy for field `CC3P`"]
pub struct CC3P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC3P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Noninverted/rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CC4P_A::RISINGEDGE)
    }
    #[doc = "Inverted/falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CC4P_A::FALLINGEDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Capture/Compare 3 output enable"]
pub type CC3E_A = CC4E_A;
#[doc = "Reader of field `CC3E`"]
pub type CC3E_R = crate::R<bool, CC4E_A>;
#[doc = "Write proxy for field `CC3E`"]
pub struct CC3E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC3E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4E_A::DISABLED)
    }
    #[doc = "Capture enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4E_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Capture/Compare 2 output Polarity"]
pub type CC2NP_A = CC4NP_A;
#[doc = "Reader of field `CC2NP`"]
pub type CC2NP_R = crate::R<bool, CC4NP_A>;
#[doc = "Write proxy for field `CC2NP`"]
pub struct CC2NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC2NP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Negative polarity"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(CC4NP_A::NEGATIVE)
    }
    #[doc = "Positive polarity"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(CC4NP_A::POSITIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Capture/Compare 2 output Polarity"]
pub type CC2P_A = CC4P_A;
#[doc = "Reader of field `CC2P`"]
pub type CC2P_R = crate::R<bool, CC4P_A>;
#[doc = "Write proxy for field `CC2P`"]
pub struct CC2P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC2P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Noninverted/rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CC4P_A::RISINGEDGE)
    }
    #[doc = "Inverted/falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CC4P_A::FALLINGEDGE)
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
#[doc = "Capture/Compare 2 output enable"]
pub type CC2E_A = CC4E_A;
#[doc = "Reader of field `CC2E`"]
pub type CC2E_R = crate::R<bool, CC4E_A>;
#[doc = "Write proxy for field `CC2E`"]
pub struct CC2E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC2E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4E_A::DISABLED)
    }
    #[doc = "Capture enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4E_A::ENABLED)
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
#[doc = "Capture/Compare 1 output Polarity"]
pub type CC1NP_A = CC4NP_A;
#[doc = "Reader of field `CC1NP`"]
pub type CC1NP_R = crate::R<bool, CC4NP_A>;
#[doc = "Write proxy for field `CC1NP`"]
pub struct CC1NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1NP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1NP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Negative polarity"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(CC4NP_A::NEGATIVE)
    }
    #[doc = "Positive polarity"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(CC4NP_A::POSITIVE)
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
#[doc = "Capture/Compare 1 output Polarity"]
pub type CC1P_A = CC4P_A;
#[doc = "Reader of field `CC1P`"]
pub type CC1P_R = crate::R<bool, CC4P_A>;
#[doc = "Write proxy for field `CC1P`"]
pub struct CC1P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Noninverted/rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CC4P_A::RISINGEDGE)
    }
    #[doc = "Inverted/falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CC4P_A::FALLINGEDGE)
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
#[doc = "Capture/Compare 1 output enable"]
pub type CC1E_A = CC4E_A;
#[doc = "Reader of field `CC1E`"]
pub type CC1E_R = crate::R<bool, CC4E_A>;
#[doc = "Write proxy for field `CC1E`"]
pub struct CC1E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4E_A::DISABLED)
    }
    #[doc = "Capture enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4E_A::ENABLED)
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
    #[doc = "Bit 15 - Capture/Compare 4 output Polarity"]
    #[inline(always)]
    pub fn cc4np(&self) -> CC4NP_R {
        CC4NP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable"]
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable"]
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Capture/Compare 4 output Polarity"]
    #[inline(always)]
    pub fn cc4np(&mut self) -> CC4NP_W {
        CC4NP_W { w: self }
    }
    #[doc = "Bit 13 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc4p(&mut self) -> CC4P_W {
        CC4P_W { w: self }
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable"]
    #[inline(always)]
    pub fn cc4e(&mut self) -> CC4E_W {
        CC4E_W { w: self }
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3np(&mut self) -> CC3NP_W {
        CC3NP_W { w: self }
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3p(&mut self) -> CC3P_W {
        CC3P_W { w: self }
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable"]
    #[inline(always)]
    pub fn cc3e(&mut self) -> CC3E_W {
        CC3E_W { w: self }
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2np(&mut self) -> CC2NP_W {
        CC2NP_W { w: self }
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2p(&mut self) -> CC2P_W {
        CC2P_W { w: self }
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn cc2e(&mut self) -> CC2E_W {
        CC2E_W { w: self }
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W {
        CC1NP_W { w: self }
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W {
        CC1P_W { w: self }
    }
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W {
        CC1E_W { w: self }
    }
}
