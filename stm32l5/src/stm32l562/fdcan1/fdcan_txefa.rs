#[doc = "Reader of register FDCAN_TXEFA"]
pub type R = crate::R<u32, super::FDCAN_TXEFA>;
#[doc = "Writer for register FDCAN_TXEFA"]
pub type W = crate::W<u32, super::FDCAN_TXEFA>;
#[doc = "Register FDCAN_TXEFA `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TXEFA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFAI`"]
pub type EFAI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFAI`"]
pub struct EFAI_W<'a> {
    w: &'a mut W,
}
impl<'a> EFAI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Event FIFO Acknowledge Index"]
    #[inline(always)]
    pub fn efai(&self) -> EFAI_R {
        EFAI_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Event FIFO Acknowledge Index"]
    #[inline(always)]
    pub fn efai(&mut self) -> EFAI_W {
        EFAI_W { w: self }
    }
}
