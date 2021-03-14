#[doc = "Reader of register C2CR"]
pub type R = crate::R<u32, super::C2CR>;
#[doc = "Writer for register C2CR"]
pub type W = crate::W<u32, super::C2CR>;
#[doc = "Register C2CR `reset()`'s with value 0"]
impl crate::ResetValue for super::C2CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXFIE`"]
pub type TXFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFIE`"]
pub struct TXFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `RXOIE`"]
pub type RXOIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOIE`"]
pub struct RXOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOIE_W<'a> {
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
    #[doc = "Bit 16 - processor 2 Transmit channel free interrupt enable"]
    #[inline(always)]
    pub fn txfie(&self) -> TXFIE_R {
        TXFIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 0 - processor 2 Receive channel occupied interrupt enable"]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - processor 2 Transmit channel free interrupt enable"]
    #[inline(always)]
    pub fn txfie(&mut self) -> TXFIE_W {
        TXFIE_W { w: self }
    }
    #[doc = "Bit 0 - processor 2 Receive channel occupied interrupt enable"]
    #[inline(always)]
    pub fn rxoie(&mut self) -> RXOIE_W {
        RXOIE_W { w: self }
    }
}
