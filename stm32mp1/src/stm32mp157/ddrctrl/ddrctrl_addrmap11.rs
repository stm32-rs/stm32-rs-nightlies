#[doc = "Reader of register DDRCTRL_ADDRMAP11"]
pub type R = crate::R<u32, super::DDRCTRL_ADDRMAP11>;
#[doc = "Writer for register DDRCTRL_ADDRMAP11"]
pub type W = crate::W<u32, super::DDRCTRL_ADDRMAP11>;
#[doc = "Register DDRCTRL_ADDRMAP11 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_ADDRMAP11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDRMAP_ROW_B10`"]
pub type ADDRMAP_ROW_B10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_ROW_B10`"]
pub struct ADDRMAP_ROW_B10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B10"]
    #[inline(always)]
    pub fn addrmap_row_b10(&self) -> ADDRMAP_ROW_B10_R {
        ADDRMAP_ROW_B10_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B10"]
    #[inline(always)]
    pub fn addrmap_row_b10(&mut self) -> ADDRMAP_ROW_B10_W {
        ADDRMAP_ROW_B10_W { w: self }
    }
}
