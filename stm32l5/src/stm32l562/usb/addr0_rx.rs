#[doc = "Reader of register ADDR0_RX"]
pub type R = crate::R<u16, super::ADDR0_RX>;
#[doc = "Writer for register ADDR0_RX"]
pub type W = crate::W<u16, super::ADDR0_RX>;
#[doc = "Register ADDR0_RX `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDR0_RX {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR0_RX`"]
pub type ADDR0_RX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDR0_RX`"]
pub struct ADDR0_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR0_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 1)) | (((value as u16) & 0x7fff) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:15 - Reception buffer address"]
    #[inline(always)]
    pub fn addr0_rx(&self) -> ADDR0_RX_R {
        ADDR0_RX_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:15 - Reception buffer address"]
    #[inline(always)]
    pub fn addr0_rx(&mut self) -> ADDR0_RX_W {
        ADDR0_RX_W { w: self }
    }
}
