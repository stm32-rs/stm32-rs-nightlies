#[doc = "Reader of register WUTR"]
pub type R = crate::R<u32, super::WUTR>;
#[doc = "Writer for register WUTR"]
pub type W = crate::W<u32, super::WUTR>;
#[doc = "Register WUTR `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::WUTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `WUT`"]
pub type WUT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WUT`"]
pub struct WUT_W<'a> {
    w: &'a mut W,
}
impl<'a> WUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\\[15:0\\]
+ 1) ck_wut cycles. The ck_wut period is selected through WUCKSEL\\[2:0\\]
bits of the RTC_CR register When WUCKSEL\\[2\\]
= 1, the wakeup timer becomes 17-bits and WUCKSEL\\[1\\]
effectively becomes WUT\\[16\\]
the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs (WUT+1) ck_wut cycles after WUTE is set. Setting WUT\\[15:0\\]
to 0x0000 with WUCKSEL\\[2:0\\]
=011 (RTCCLK/2) is forbidden."]
    #[inline(always)]
    pub fn wut(&self) -> WUT_R {
        WUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\\[15:0\\]
+ 1) ck_wut cycles. The ck_wut period is selected through WUCKSEL\\[2:0\\]
bits of the RTC_CR register When WUCKSEL\\[2\\]
= 1, the wakeup timer becomes 17-bits and WUCKSEL\\[1\\]
effectively becomes WUT\\[16\\]
the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs (WUT+1) ck_wut cycles after WUTE is set. Setting WUT\\[15:0\\]
to 0x0000 with WUCKSEL\\[2:0\\]
=011 (RTCCLK/2) is forbidden."]
    #[inline(always)]
    pub fn wut(&mut self) -> WUT_W {
        WUT_W { w: self }
    }
}
