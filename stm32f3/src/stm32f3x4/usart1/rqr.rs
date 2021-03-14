#[doc = "Reader of register RQR"]
pub type R = crate::R<u32, super::RQR>;
#[doc = "Writer for register RQR"]
pub type W = crate::W<u32, super::RQR>;
#[doc = "Register RQR `reset()`'s with value 0"]
impl crate::ResetValue for super::RQR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmit data flush request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFRQ_A {
    #[doc = "1: Set the TXE flags. This allows to discard the transmit data"]
    DISCARD = 1,
}
impl From<TXFRQ_A> for bool {
    #[inline(always)]
    fn from(variant: TXFRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXFRQ`"]
pub type TXFRQ_R = crate::R<bool, TXFRQ_A>;
impl TXFRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, TXFRQ_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(TXFRQ_A::DISCARD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISCARD`"]
    #[inline(always)]
    pub fn is_discard(&self) -> bool {
        *self == TXFRQ_A::DISCARD
    }
}
#[doc = "Write proxy for field `TXFRQ`"]
pub struct TXFRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set the TXE flags. This allows to discard the transmit data"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut W {
        self.variant(TXFRQ_A::DISCARD)
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
#[doc = "Receive data flush request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFRQ_A {
    #[doc = "1: clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    DISCARD = 1,
}
impl From<RXFRQ_A> for bool {
    #[inline(always)]
    fn from(variant: RXFRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXFRQ`"]
pub type RXFRQ_R = crate::R<bool, RXFRQ_A>;
impl RXFRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RXFRQ_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(RXFRQ_A::DISCARD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISCARD`"]
    #[inline(always)]
    pub fn is_discard(&self) -> bool {
        *self == RXFRQ_A::DISCARD
    }
}
#[doc = "Write proxy for field `RXFRQ`"]
pub struct RXFRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut W {
        self.variant(RXFRQ_A::DISCARD)
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
#[doc = "Mute mode request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMRQ_A {
    #[doc = "1: Puts the USART in mute mode and sets the RWU flag"]
    MUTE = 1,
}
impl From<MMRQ_A> for bool {
    #[inline(always)]
    fn from(variant: MMRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MMRQ`"]
pub type MMRQ_R = crate::R<bool, MMRQ_A>;
impl MMRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MMRQ_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(MMRQ_A::MUTE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MUTE`"]
    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        *self == MMRQ_A::MUTE
    }
}
#[doc = "Write proxy for field `MMRQ`"]
pub struct MMRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MMRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Puts the USART in mute mode and sets the RWU flag"]
    #[inline(always)]
    pub fn mute(self) -> &'a mut W {
        self.variant(MMRQ_A::MUTE)
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
#[doc = "Send break request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBKRQ_A {
    #[doc = "1: sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    BREAK = 1,
}
impl From<SBKRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SBKRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SBKRQ`"]
pub type SBKRQ_R = crate::R<bool, SBKRQ_A>;
impl SBKRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SBKRQ_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SBKRQ_A::BREAK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BREAK`"]
    #[inline(always)]
    pub fn is_break_(&self) -> bool {
        *self == SBKRQ_A::BREAK
    }
}
#[doc = "Write proxy for field `SBKRQ`"]
pub struct SBKRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SBKRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBKRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut W {
        self.variant(SBKRQ_A::BREAK)
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
#[doc = "Auto baud rate request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRRQ_A {
    #[doc = "1: resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    REQUEST = 1,
}
impl From<ABRRQ_A> for bool {
    #[inline(always)]
    fn from(variant: ABRRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ABRRQ`"]
pub type ABRRQ_R = crate::R<bool, ABRRQ_A>;
impl ABRRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ABRRQ_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(ABRRQ_A::REQUEST),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REQUEST`"]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == ABRRQ_A::REQUEST
    }
}
#[doc = "Write proxy for field `ABRRQ`"]
pub struct ABRRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ABRRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABRRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    #[inline(always)]
    pub fn request(self) -> &'a mut W {
        self.variant(ABRRQ_A::REQUEST)
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
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline(always)]
    pub fn txfrq(&self) -> TXFRQ_R {
        TXFRQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline(always)]
    pub fn rxfrq(&self) -> RXFRQ_R {
        RXFRQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline(always)]
    pub fn mmrq(&self) -> MMRQ_R {
        MMRQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline(always)]
    pub fn sbkrq(&self) -> SBKRQ_R {
        SBKRQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline(always)]
    pub fn abrrq(&self) -> ABRRQ_R {
        ABRRQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline(always)]
    pub fn txfrq(&mut self) -> TXFRQ_W {
        TXFRQ_W { w: self }
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline(always)]
    pub fn rxfrq(&mut self) -> RXFRQ_W {
        RXFRQ_W { w: self }
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline(always)]
    pub fn mmrq(&mut self) -> MMRQ_W {
        MMRQ_W { w: self }
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline(always)]
    pub fn sbkrq(&mut self) -> SBKRQ_W {
        SBKRQ_W { w: self }
    }
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline(always)]
    pub fn abrrq(&mut self) -> ABRRQ_W {
        ABRRQ_W { w: self }
    }
}
