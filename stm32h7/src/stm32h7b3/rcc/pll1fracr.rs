#[doc = "Reader of register PLL1FRACR"]
pub type R = crate::R<u32, super::PLL1FRACR>;
#[doc = "Writer for register PLL1FRACR"]
pub type W = crate::W<u32, super::PLL1FRACR>;
#[doc = "Register PLL1FRACR `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL1FRACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRACN1`"]
pub type FRACN1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRACN1`"]
pub struct FRACN1_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 3)) | (((value as u32) & 0x1fff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn fracn1(&self) -> FRACN1_R {
        FRACN1_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn fracn1(&mut self) -> FRACN1_W {
        FRACN1_W { w: self }
    }
}
