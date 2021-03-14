#[doc = "Reader of register DDRCTRL_ADDRMAP3"]
pub type R = crate::R<u32, super::DDRCTRL_ADDRMAP3>;
#[doc = "Writer for register DDRCTRL_ADDRMAP3"]
pub type W = crate::W<u32, super::DDRCTRL_ADDRMAP3>;
#[doc = "Register DDRCTRL_ADDRMAP3 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_ADDRMAP3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDRMAP_COL_B6`"]
pub type ADDRMAP_COL_B6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_COL_B6`"]
pub struct ADDRMAP_COL_B6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_COL_B6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `ADDRMAP_COL_B7`"]
pub type ADDRMAP_COL_B7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_COL_B7`"]
pub struct ADDRMAP_COL_B7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_COL_B7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADDRMAP_COL_B8`"]
pub type ADDRMAP_COL_B8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_COL_B8`"]
pub struct ADDRMAP_COL_B8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_COL_B8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADDRMAP_COL_B9`"]
pub type ADDRMAP_COL_B9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_COL_B9`"]
pub struct ADDRMAP_COL_B9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_COL_B9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ADDRMAP_COL_B6"]
    #[inline(always)]
    pub fn addrmap_col_b6(&self) -> ADDRMAP_COL_B6_R {
        ADDRMAP_COL_B6_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - ADDRMAP_COL_B7"]
    #[inline(always)]
    pub fn addrmap_col_b7(&self) -> ADDRMAP_COL_B7_R {
        ADDRMAP_COL_B7_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADDRMAP_COL_B8"]
    #[inline(always)]
    pub fn addrmap_col_b8(&self) -> ADDRMAP_COL_B8_R {
        ADDRMAP_COL_B8_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADDRMAP_COL_B9"]
    #[inline(always)]
    pub fn addrmap_col_b9(&self) -> ADDRMAP_COL_B9_R {
        ADDRMAP_COL_B9_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRMAP_COL_B6"]
    #[inline(always)]
    pub fn addrmap_col_b6(&mut self) -> ADDRMAP_COL_B6_W {
        ADDRMAP_COL_B6_W { w: self }
    }
    #[doc = "Bits 8:12 - ADDRMAP_COL_B7"]
    #[inline(always)]
    pub fn addrmap_col_b7(&mut self) -> ADDRMAP_COL_B7_W {
        ADDRMAP_COL_B7_W { w: self }
    }
    #[doc = "Bits 16:20 - ADDRMAP_COL_B8"]
    #[inline(always)]
    pub fn addrmap_col_b8(&mut self) -> ADDRMAP_COL_B8_W {
        ADDRMAP_COL_B8_W { w: self }
    }
    #[doc = "Bits 24:28 - ADDRMAP_COL_B9"]
    #[inline(always)]
    pub fn addrmap_col_b9(&mut self) -> ADDRMAP_COL_B9_W {
        ADDRMAP_COL_B9_W { w: self }
    }
}
