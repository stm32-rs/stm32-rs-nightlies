#[doc = "Reader of register DDRCTRL_ADDRMAP9"]
pub type R = crate::R<u32, super::DDRCTRL_ADDRMAP9>;
#[doc = "Writer for register DDRCTRL_ADDRMAP9"]
pub type W = crate::W<u32, super::DDRCTRL_ADDRMAP9>;
#[doc = "Register DDRCTRL_ADDRMAP9 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_ADDRMAP9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDRMAP_ROW_B2`"]
pub type ADDRMAP_ROW_B2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_ROW_B2`"]
pub struct ADDRMAP_ROW_B2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `ADDRMAP_ROW_B3`"]
pub type ADDRMAP_ROW_B3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_ROW_B3`"]
pub struct ADDRMAP_ROW_B3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADDRMAP_ROW_B4`"]
pub type ADDRMAP_ROW_B4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_ROW_B4`"]
pub struct ADDRMAP_ROW_B4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADDRMAP_ROW_B5`"]
pub type ADDRMAP_ROW_B5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_ROW_B5`"]
pub struct ADDRMAP_ROW_B5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B2"]
    #[inline(always)]
    pub fn addrmap_row_b2(&self) -> ADDRMAP_ROW_B2_R {
        ADDRMAP_ROW_B2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B3"]
    #[inline(always)]
    pub fn addrmap_row_b3(&self) -> ADDRMAP_ROW_B3_R {
        ADDRMAP_ROW_B3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B4"]
    #[inline(always)]
    pub fn addrmap_row_b4(&self) -> ADDRMAP_ROW_B4_R {
        ADDRMAP_ROW_B4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B5"]
    #[inline(always)]
    pub fn addrmap_row_b5(&self) -> ADDRMAP_ROW_B5_R {
        ADDRMAP_ROW_B5_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B2"]
    #[inline(always)]
    pub fn addrmap_row_b2(&mut self) -> ADDRMAP_ROW_B2_W {
        ADDRMAP_ROW_B2_W { w: self }
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B3"]
    #[inline(always)]
    pub fn addrmap_row_b3(&mut self) -> ADDRMAP_ROW_B3_W {
        ADDRMAP_ROW_B3_W { w: self }
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B4"]
    #[inline(always)]
    pub fn addrmap_row_b4(&mut self) -> ADDRMAP_ROW_B4_W {
        ADDRMAP_ROW_B4_W { w: self }
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B5"]
    #[inline(always)]
    pub fn addrmap_row_b5(&mut self) -> ADDRMAP_ROW_B5_W {
        ADDRMAP_ROW_B5_W { w: self }
    }
}
