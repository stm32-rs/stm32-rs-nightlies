#[doc = "Reader of register DDRCTRL_ADDRMAP4"]
pub type R = crate::R<u32, super::DDRCTRL_ADDRMAP4>;
#[doc = "Writer for register DDRCTRL_ADDRMAP4"]
pub type W = crate::W<u32, super::DDRCTRL_ADDRMAP4>;
#[doc = "Register DDRCTRL_ADDRMAP4 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_ADDRMAP4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDRMAP_COL_B10`"]
pub type ADDRMAP_COL_B10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_COL_B10`"]
pub struct ADDRMAP_COL_B10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_COL_B10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `ADDRMAP_COL_B11`"]
pub type ADDRMAP_COL_B11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_COL_B11`"]
pub struct ADDRMAP_COL_B11_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_COL_B11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - ADDRMAP_COL_B10"]
    #[inline(always)]
    pub fn addrmap_col_b10(&self) -> ADDRMAP_COL_B10_R {
        ADDRMAP_COL_B10_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - ADDRMAP_COL_B11"]
    #[inline(always)]
    pub fn addrmap_col_b11(&self) -> ADDRMAP_COL_B11_R {
        ADDRMAP_COL_B11_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADDRMAP_COL_B10"]
    #[inline(always)]
    pub fn addrmap_col_b10(&mut self) -> ADDRMAP_COL_B10_W {
        ADDRMAP_COL_B10_W { w: self }
    }
    #[doc = "Bits 8:12 - ADDRMAP_COL_B11"]
    #[inline(always)]
    pub fn addrmap_col_b11(&mut self) -> ADDRMAP_COL_B11_W {
        ADDRMAP_COL_B11_W { w: self }
    }
}
