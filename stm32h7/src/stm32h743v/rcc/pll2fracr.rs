#[doc = "Reader of register PLL2FRACR"]
pub type R = crate::R<u32, super::PLL2FRACR>;
#[doc = "Writer for register PLL2FRACR"]
pub type W = crate::W<u32, super::PLL2FRACR>;
#[doc = "Register PLL2FRACR `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL2FRACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRACN2`"]
pub type FRACN2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRACN2`"]
pub struct FRACN2_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 3)) | (((value as u32) & 0x1fff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL VCO"]
    #[inline(always)]
    pub fn fracn2(&self) -> FRACN2_R {
        FRACN2_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL VCO"]
    #[inline(always)]
    pub fn fracn2(&mut self) -> FRACN2_W {
        FRACN2_W { w: self }
    }
}
