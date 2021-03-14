#[doc = "Reader of register HUFFSYMB64"]
pub type R = crate::R<u32, super::HUFFSYMB64>;
#[doc = "Writer for register HUFFSYMB64"]
pub type W = crate::W<u32, super::HUFFSYMB64>;
#[doc = "Register HUFFSYMB64 `reset()`'s with value 0"]
impl crate::ResetValue for super::HUFFSYMB64 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HuffSymb_RAM`"]
pub type HUFFSYMB_RAM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HuffSymb_RAM`"]
pub struct HUFFSYMB_RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> HUFFSYMB_RAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DHTSymb RAM"]
    #[inline(always)]
    pub fn huff_symb_ram(&self) -> HUFFSYMB_RAM_R {
        HUFFSYMB_RAM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DHTSymb RAM"]
    #[inline(always)]
    pub fn huff_symb_ram(&mut self) -> HUFFSYMB_RAM_W {
        HUFFSYMB_RAM_W { w: self }
    }
}
