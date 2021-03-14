#[doc = "Reader of register DDRCTRL_PCFGQOS1_1"]
pub type R = crate::R<u32, super::DDRCTRL_PCFGQOS1_1>;
#[doc = "Writer for register DDRCTRL_PCFGQOS1_1"]
pub type W = crate::W<u32, super::DDRCTRL_PCFGQOS1_1>;
#[doc = "Register DDRCTRL_PCFGQOS1_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_PCFGQOS1_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RQOS_MAP_TIMEOUTB`"]
pub type RQOS_MAP_TIMEOUTB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RQOS_MAP_TIMEOUTB`"]
pub struct RQOS_MAP_TIMEOUTB_W<'a> {
    w: &'a mut W,
}
impl<'a> RQOS_MAP_TIMEOUTB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `RQOS_MAP_TIMEOUTR`"]
pub type RQOS_MAP_TIMEOUTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RQOS_MAP_TIMEOUTR`"]
pub struct RQOS_MAP_TIMEOUTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RQOS_MAP_TIMEOUTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - RQOS_MAP_TIMEOUTB"]
    #[inline(always)]
    pub fn rqos_map_timeoutb(&self) -> RQOS_MAP_TIMEOUTB_R {
        RQOS_MAP_TIMEOUTB_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - RQOS_MAP_TIMEOUTR"]
    #[inline(always)]
    pub fn rqos_map_timeoutr(&self) -> RQOS_MAP_TIMEOUTR_R {
        RQOS_MAP_TIMEOUTR_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - RQOS_MAP_TIMEOUTB"]
    #[inline(always)]
    pub fn rqos_map_timeoutb(&mut self) -> RQOS_MAP_TIMEOUTB_W {
        RQOS_MAP_TIMEOUTB_W { w: self }
    }
    #[doc = "Bits 16:26 - RQOS_MAP_TIMEOUTR"]
    #[inline(always)]
    pub fn rqos_map_timeoutr(&mut self) -> RQOS_MAP_TIMEOUTR_W {
        RQOS_MAP_TIMEOUTR_W { w: self }
    }
}
