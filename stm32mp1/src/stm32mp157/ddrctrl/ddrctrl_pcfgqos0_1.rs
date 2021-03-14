#[doc = "Reader of register DDRCTRL_PCFGQOS0_1"]
pub type R = crate::R<u32, super::DDRCTRL_PCFGQOS0_1>;
#[doc = "Writer for register DDRCTRL_PCFGQOS0_1"]
pub type W = crate::W<u32, super::DDRCTRL_PCFGQOS0_1>;
#[doc = "Register DDRCTRL_PCFGQOS0_1 `reset()`'s with value 0x0200_0e00"]
impl crate::ResetValue for super::DDRCTRL_PCFGQOS0_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0e00
    }
}
#[doc = "Reader of field `RQOS_MAP_LEVEL1`"]
pub type RQOS_MAP_LEVEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RQOS_MAP_LEVEL1`"]
pub struct RQOS_MAP_LEVEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> RQOS_MAP_LEVEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `RQOS_MAP_LEVEL2`"]
pub type RQOS_MAP_LEVEL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RQOS_MAP_LEVEL2`"]
pub struct RQOS_MAP_LEVEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> RQOS_MAP_LEVEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RQOS_MAP_REGION0`"]
pub type RQOS_MAP_REGION0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RQOS_MAP_REGION0`"]
pub struct RQOS_MAP_REGION0_W<'a> {
    w: &'a mut W,
}
impl<'a> RQOS_MAP_REGION0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `RQOS_MAP_REGION1`"]
pub type RQOS_MAP_REGION1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RQOS_MAP_REGION1`"]
pub struct RQOS_MAP_REGION1_W<'a> {
    w: &'a mut W,
}
impl<'a> RQOS_MAP_REGION1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `RQOS_MAP_REGION2`"]
pub type RQOS_MAP_REGION2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RQOS_MAP_REGION2`"]
pub struct RQOS_MAP_REGION2_W<'a> {
    w: &'a mut W,
}
impl<'a> RQOS_MAP_REGION2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - RQOS_MAP_LEVEL1"]
    #[inline(always)]
    pub fn rqos_map_level1(&self) -> RQOS_MAP_LEVEL1_R {
        RQOS_MAP_LEVEL1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - RQOS_MAP_LEVEL2"]
    #[inline(always)]
    pub fn rqos_map_level2(&self) -> RQOS_MAP_LEVEL2_R {
        RQOS_MAP_LEVEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - RQOS_MAP_REGION0"]
    #[inline(always)]
    pub fn rqos_map_region0(&self) -> RQOS_MAP_REGION0_R {
        RQOS_MAP_REGION0_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - RQOS_MAP_REGION1"]
    #[inline(always)]
    pub fn rqos_map_region1(&self) -> RQOS_MAP_REGION1_R {
        RQOS_MAP_REGION1_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - RQOS_MAP_REGION2"]
    #[inline(always)]
    pub fn rqos_map_region2(&self) -> RQOS_MAP_REGION2_R {
        RQOS_MAP_REGION2_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RQOS_MAP_LEVEL1"]
    #[inline(always)]
    pub fn rqos_map_level1(&mut self) -> RQOS_MAP_LEVEL1_W {
        RQOS_MAP_LEVEL1_W { w: self }
    }
    #[doc = "Bits 8:11 - RQOS_MAP_LEVEL2"]
    #[inline(always)]
    pub fn rqos_map_level2(&mut self) -> RQOS_MAP_LEVEL2_W {
        RQOS_MAP_LEVEL2_W { w: self }
    }
    #[doc = "Bits 16:17 - RQOS_MAP_REGION0"]
    #[inline(always)]
    pub fn rqos_map_region0(&mut self) -> RQOS_MAP_REGION0_W {
        RQOS_MAP_REGION0_W { w: self }
    }
    #[doc = "Bits 20:21 - RQOS_MAP_REGION1"]
    #[inline(always)]
    pub fn rqos_map_region1(&mut self) -> RQOS_MAP_REGION1_W {
        RQOS_MAP_REGION1_W { w: self }
    }
    #[doc = "Bits 24:25 - RQOS_MAP_REGION2"]
    #[inline(always)]
    pub fn rqos_map_region2(&mut self) -> RQOS_MAP_REGION2_W {
        RQOS_MAP_REGION2_W { w: self }
    }
}
