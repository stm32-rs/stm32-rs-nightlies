#[doc = "Writer for register PEKEYR"]
pub type W = crate::W<u32, super::PEKEYR>;
#[doc = "Register PEKEYR `reset()`'s with value 0"]
impl crate::ResetValue for super::PEKEYR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PEKEYR`"]
pub struct PEKEYR_W<'a> {
    w: &'a mut W,
}
impl<'a> PEKEYR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - FLASH_PEC and data EEPROM key"]
    #[inline(always)]
    pub fn pekeyr(&mut self) -> PEKEYR_W {
        PEKEYR_W { w: self }
    }
}
