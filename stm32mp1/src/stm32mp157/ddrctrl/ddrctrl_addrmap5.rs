#[doc = "Reader of register DDRCTRL_ADDRMAP5"]
pub type R = crate::R<u32, super::DDRCTRL_ADDRMAP5>;
#[doc = "Writer for register DDRCTRL_ADDRMAP5"]
pub type W = crate::W<u32, super::DDRCTRL_ADDRMAP5>;
#[doc = "Register DDRCTRL_ADDRMAP5 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_ADDRMAP5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDRMAP_ROW_B0`"]
pub type ADDRMAP_ROW_B0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_ROW_B0`"]
pub struct ADDRMAP_ROW_B0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `ADDRMAP_ROW_B1`"]
pub type ADDRMAP_ROW_B1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_ROW_B1`"]
pub struct ADDRMAP_ROW_B1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADDRMAP_ROW_B2_10`"]
pub type ADDRMAP_ROW_B2_10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_ROW_B2_10`"]
pub struct ADDRMAP_ROW_B2_10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B2_10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADDRMAP_ROW_B11`"]
pub type ADDRMAP_ROW_B11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_ROW_B11`"]
pub struct ADDRMAP_ROW_B11_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B0"]
    #[inline(always)]
    pub fn addrmap_row_b0(&self) -> ADDRMAP_ROW_B0_R {
        ADDRMAP_ROW_B0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B1"]
    #[inline(always)]
    pub fn addrmap_row_b1(&self) -> ADDRMAP_ROW_B1_R {
        ADDRMAP_ROW_B1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B2_10"]
    #[inline(always)]
    pub fn addrmap_row_b2_10(&self) -> ADDRMAP_ROW_B2_10_R {
        ADDRMAP_ROW_B2_10_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B11"]
    #[inline(always)]
    pub fn addrmap_row_b11(&self) -> ADDRMAP_ROW_B11_R {
        ADDRMAP_ROW_B11_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B0"]
    #[inline(always)]
    pub fn addrmap_row_b0(&mut self) -> ADDRMAP_ROW_B0_W {
        ADDRMAP_ROW_B0_W { w: self }
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B1"]
    #[inline(always)]
    pub fn addrmap_row_b1(&mut self) -> ADDRMAP_ROW_B1_W {
        ADDRMAP_ROW_B1_W { w: self }
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B2_10"]
    #[inline(always)]
    pub fn addrmap_row_b2_10(&mut self) -> ADDRMAP_ROW_B2_10_W {
        ADDRMAP_ROW_B2_10_W { w: self }
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B11"]
    #[inline(always)]
    pub fn addrmap_row_b11(&mut self) -> ADDRMAP_ROW_B11_W {
        ADDRMAP_ROW_B11_W { w: self }
    }
}
