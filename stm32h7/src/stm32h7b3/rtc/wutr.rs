#[doc = "Register `WUTR` reader"]
pub type R = crate::R<WUTRrs>;
#[doc = "Register `WUTR` writer"]
pub type W = crate::W<WUTRrs>;
#[doc = "Field `WUT` reader - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\\[15:0\\]
+ 1) ck_wut cycles. The ck_wut period is selected through WUCKSEL\\[2:0\\]
bits of the RTC_CR register When WUCKSEL\\[2\\]
= 1, the wakeup timer becomes 17-bits and WUCKSEL\\[1\\]
effectively becomes WUT\\[16\\]
the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs (WUT+1) ck_wut cycles after WUTE is set. Setting WUT\\[15:0\\]
to 0x0000 with WUCKSEL\\[2:0\\]
=011 (RTCCLK/2) is forbidden."]
pub type WUT_R = crate::FieldReader<u16>;
#[doc = "Field `WUT` writer - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\\[15:0\\]
+ 1) ck_wut cycles. The ck_wut period is selected through WUCKSEL\\[2:0\\]
bits of the RTC_CR register When WUCKSEL\\[2\\]
= 1, the wakeup timer becomes 17-bits and WUCKSEL\\[1\\]
effectively becomes WUT\\[16\\]
the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs (WUT+1) ck_wut cycles after WUTE is set. Setting WUT\\[15:0\\]
to 0x0000 with WUCKSEL\\[2:0\\]
=011 (RTCCLK/2) is forbidden."]
pub type WUT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
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
    #[must_use]
    pub fn wut(&mut self) -> WUT_W<WUTRrs> {
        WUT_W::new(self, 0)
    }
}
#[doc = "This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wutr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wutr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WUTRrs;
impl crate::RegisterSpec for WUTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wutr::R`](R) reader structure"]
impl crate::Readable for WUTRrs {}
#[doc = "`write(|w| ..)` method takes [`wutr::W`](W) writer structure"]
impl crate::Writable for WUTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUTR to value 0xffff"]
impl crate::Resettable for WUTRrs {
    const RESET_VALUE: u32 = 0xffff;
}
