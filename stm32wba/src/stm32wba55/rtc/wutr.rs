///Register `WUTR` reader
pub type R = crate::R<WUTRrs>;
///Register `WUTR` writer
pub type W = crate::W<WUTRrs>;
///Field `WUT` reader - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\[15:0\]+1) ck_wut cycles. The ck_wut period is selected through WUCKSEL\[2:0\] bits of the RTC_CR register. When WUCKSEL\[2\] = 1, the wakeup timer becomes 17-bits and WUCKSEL\[1\] effectively becomes WUT\[16\] the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs between WUT and (WUT + 2) ck_wut cycles after WUTE is set. Setting WUT\[15:0\] to 0x0000 with WUCKSEL\[2:0\] = 011 (RTCCLK/2) is forbidden.
pub type WUT_R = crate::FieldReader<u16>;
///Field `WUT` writer - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\[15:0\]+1) ck_wut cycles. The ck_wut period is selected through WUCKSEL\[2:0\] bits of the RTC_CR register. When WUCKSEL\[2\] = 1, the wakeup timer becomes 17-bits and WUCKSEL\[1\] effectively becomes WUT\[16\] the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs between WUT and (WUT + 2) ck_wut cycles after WUTE is set. Setting WUT\[15:0\] to 0x0000 with WUCKSEL\[2:0\] = 011 (RTCCLK/2) is forbidden.
pub type WUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
///Field `WUTOCLR` reader - Wakeup auto-reload output clear value When WUTOCLR\[15:0\] is different from 0x0000, WUTF is set by hardware when the auto-reload down-counter reaches 0 and is cleared by hardware when the auto-reload downcounter reaches WUTOCLR\[15:0\]. When WUTOCLR\[15:0\] = 0x0000, WUTF is set by hardware when the WUT down-counter reaches 0 and is cleared by software.
pub type WUTOCLR_R = crate::FieldReader<u16>;
///Field `WUTOCLR` writer - Wakeup auto-reload output clear value When WUTOCLR\[15:0\] is different from 0x0000, WUTF is set by hardware when the auto-reload down-counter reaches 0 and is cleared by hardware when the auto-reload downcounter reaches WUTOCLR\[15:0\]. When WUTOCLR\[15:0\] = 0x0000, WUTF is set by hardware when the WUT down-counter reaches 0 and is cleared by software.
pub type WUTOCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\[15:0\]+1) ck_wut cycles. The ck_wut period is selected through WUCKSEL\[2:0\] bits of the RTC_CR register. When WUCKSEL\[2\] = 1, the wakeup timer becomes 17-bits and WUCKSEL\[1\] effectively becomes WUT\[16\] the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs between WUT and (WUT + 2) ck_wut cycles after WUTE is set. Setting WUT\[15:0\] to 0x0000 with WUCKSEL\[2:0\] = 011 (RTCCLK/2) is forbidden.
    #[inline(always)]
    pub fn wut(&self) -> WUT_R {
        WUT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Wakeup auto-reload output clear value When WUTOCLR\[15:0\] is different from 0x0000, WUTF is set by hardware when the auto-reload down-counter reaches 0 and is cleared by hardware when the auto-reload downcounter reaches WUTOCLR\[15:0\]. When WUTOCLR\[15:0\] = 0x0000, WUTF is set by hardware when the WUT down-counter reaches 0 and is cleared by software.
    #[inline(always)]
    pub fn wutoclr(&self) -> WUTOCLR_R {
        WUTOCLR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUTR")
            .field("wut", &self.wut())
            .field("wutoclr", &self.wutoclr())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE set to 1), the WUTF flag is set every (WUT\[15:0\]+1) ck_wut cycles. The ck_wut period is selected through WUCKSEL\[2:0\] bits of the RTC_CR register. When WUCKSEL\[2\] = 1, the wakeup timer becomes 17-bits and WUCKSEL\[1\] effectively becomes WUT\[16\] the most-significant bit to be reloaded into the timer. The first assertion of WUTF occurs between WUT and (WUT + 2) ck_wut cycles after WUTE is set. Setting WUT\[15:0\] to 0x0000 with WUCKSEL\[2:0\] = 011 (RTCCLK/2) is forbidden.
    #[inline(always)]
    pub fn wut(&mut self) -> WUT_W<WUTRrs> {
        WUT_W::new(self, 0)
    }
    ///Bits 16:31 - Wakeup auto-reload output clear value When WUTOCLR\[15:0\] is different from 0x0000, WUTF is set by hardware when the auto-reload down-counter reaches 0 and is cleared by hardware when the auto-reload downcounter reaches WUTOCLR\[15:0\]. When WUTOCLR\[15:0\] = 0x0000, WUTF is set by hardware when the WUT down-counter reaches 0 and is cleared by software.
    #[inline(always)]
    pub fn wutoclr(&mut self) -> WUTOCLR_W<WUTRrs> {
        WUTOCLR_W::new(self, 16)
    }
}
/**RTC wakeup timer register

You can [`read`](crate::Reg::read) this register and get [`wutr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wutr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#RTC:WUTR)*/
pub struct WUTRrs;
impl crate::RegisterSpec for WUTRrs {
    type Ux = u32;
}
///`read()` method returns [`wutr::R`](R) reader structure
impl crate::Readable for WUTRrs {}
///`write(|w| ..)` method takes [`wutr::W`](W) writer structure
impl crate::Writable for WUTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WUTR to value 0xffff
impl crate::Resettable for WUTRrs {
    const RESET_VALUE: u32 = 0xffff;
}
