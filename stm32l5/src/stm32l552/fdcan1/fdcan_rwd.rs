#[doc = "Reader of register FDCAN_RWD"]
pub type R = crate::R<u32, super::FDCAN_RWD>;
#[doc = "Writer for register FDCAN_RWD"]
pub type W = crate::W<u32, super::FDCAN_RWD>;
#[doc = "Register FDCAN_RWD `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_RWD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDV`"]
pub type WDV_R = crate::R<u8, u8>;
#[doc = "Reader of field `WDC`"]
pub type WDC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WDC`"]
pub struct WDC_W<'a> {
    w: &'a mut W,
}
impl<'a> WDC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Watchdog value"]
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Watchdog configuration"]
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog configuration"]
    #[inline(always)]
    pub fn wdc(&mut self) -> WDC_W {
        WDC_W { w: self }
    }
}
