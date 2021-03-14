#[doc = "Reader of register PLL3FRACR"]
pub type R = crate::R<u32, super::PLL3FRACR>;
#[doc = "Writer for register PLL3FRACR"]
pub type W = crate::W<u32, super::PLL3FRACR>;
#[doc = "Register PLL3FRACR `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL3FRACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRACN3`"]
pub type FRACN3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRACN3`"]
pub struct FRACN3_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACN3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 3)) | (((value as u32) & 0x1fff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL3 VCO"]
    #[inline(always)]
    pub fn fracn3(&self) -> FRACN3_R {
        FRACN3_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL3 VCO"]
    #[inline(always)]
    pub fn fracn3(&mut self) -> FRACN3_W {
        FRACN3_W { w: self }
    }
}
