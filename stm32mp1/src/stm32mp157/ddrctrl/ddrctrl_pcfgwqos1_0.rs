#[doc = "Reader of register DDRCTRL_PCFGWQOS1_0"]
pub type R = crate::R<u32, super::DDRCTRL_PCFGWQOS1_0>;
#[doc = "Writer for register DDRCTRL_PCFGWQOS1_0"]
pub type W = crate::W<u32, super::DDRCTRL_PCFGWQOS1_0>;
#[doc = "Register DDRCTRL_PCFGWQOS1_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_PCFGWQOS1_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WQOS_MAP_TIMEOUT1`"]
pub type WQOS_MAP_TIMEOUT1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WQOS_MAP_TIMEOUT1`"]
pub struct WQOS_MAP_TIMEOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> WQOS_MAP_TIMEOUT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `WQOS_MAP_TIMEOUT2`"]
pub type WQOS_MAP_TIMEOUT2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WQOS_MAP_TIMEOUT2`"]
pub struct WQOS_MAP_TIMEOUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> WQOS_MAP_TIMEOUT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - WQOS_MAP_TIMEOUT1"]
    #[inline(always)]
    pub fn wqos_map_timeout1(&self) -> WQOS_MAP_TIMEOUT1_R {
        WQOS_MAP_TIMEOUT1_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - WQOS_MAP_TIMEOUT2"]
    #[inline(always)]
    pub fn wqos_map_timeout2(&self) -> WQOS_MAP_TIMEOUT2_R {
        WQOS_MAP_TIMEOUT2_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - WQOS_MAP_TIMEOUT1"]
    #[inline(always)]
    pub fn wqos_map_timeout1(&mut self) -> WQOS_MAP_TIMEOUT1_W {
        WQOS_MAP_TIMEOUT1_W { w: self }
    }
    #[doc = "Bits 16:26 - WQOS_MAP_TIMEOUT2"]
    #[inline(always)]
    pub fn wqos_map_timeout2(&mut self) -> WQOS_MAP_TIMEOUT2_W {
        WQOS_MAP_TIMEOUT2_W { w: self }
    }
}
