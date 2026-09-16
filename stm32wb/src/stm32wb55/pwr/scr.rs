///Register `SCR` writer
pub type W = crate::W<SCRrs>;
///Field `CWUF1` writer - Clear wakeup flag 1
pub type CWUF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF2` writer - Clear wakeup flag 2
pub type CWUF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF3` writer - Clear wakeup flag 3
pub type CWUF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF4` writer - Clear wakeup flag 4
pub type CWUF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF5` writer - Clear wakeup flag 5
pub type CWUF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSMPSFBF` writer - Clear SMPS Step Down converter forced in Bypass interrupt flag
pub type CSMPSFBF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CBORHF` writer - Clear BORH interrupt flag
pub type CBORHF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CBLEWUF` writer - Clear BLE wakeup interrupt flag
pub type CBLEWUF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `C802WUF` writer - Clear 802.15.4 wakeup interrupt flag
pub type C802WUF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCRPEF` writer - Clear critical radio phase end of activity interrupt flag
pub type CCRPEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CBLEAF` writer - Clear BLE end of activity interrupt flag
pub type CBLEAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `C802AF` writer - Clear 802.15.4 end of activity interrupt flag
pub type C802AF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2HF` writer - Clear CPU2 Hold interrupt flag
pub type CC2HF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear wakeup flag 1
    #[inline(always)]
    pub fn cwuf1(&mut self) -> CWUF1_W<'_, SCRrs> {
        CWUF1_W::new(self, 0)
    }
    ///Bit 1 - Clear wakeup flag 2
    #[inline(always)]
    pub fn cwuf2(&mut self) -> CWUF2_W<'_, SCRrs> {
        CWUF2_W::new(self, 1)
    }
    ///Bit 2 - Clear wakeup flag 3
    #[inline(always)]
    pub fn cwuf3(&mut self) -> CWUF3_W<'_, SCRrs> {
        CWUF3_W::new(self, 2)
    }
    ///Bit 3 - Clear wakeup flag 4
    #[inline(always)]
    pub fn cwuf4(&mut self) -> CWUF4_W<'_, SCRrs> {
        CWUF4_W::new(self, 3)
    }
    ///Bit 4 - Clear wakeup flag 5
    #[inline(always)]
    pub fn cwuf5(&mut self) -> CWUF5_W<'_, SCRrs> {
        CWUF5_W::new(self, 4)
    }
    ///Bit 7 - Clear SMPS Step Down converter forced in Bypass interrupt flag
    #[inline(always)]
    pub fn csmpsfbf(&mut self) -> CSMPSFBF_W<'_, SCRrs> {
        CSMPSFBF_W::new(self, 7)
    }
    ///Bit 8 - Clear BORH interrupt flag
    #[inline(always)]
    pub fn cborhf(&mut self) -> CBORHF_W<'_, SCRrs> {
        CBORHF_W::new(self, 8)
    }
    ///Bit 9 - Clear BLE wakeup interrupt flag
    #[inline(always)]
    pub fn cblewuf(&mut self) -> CBLEWUF_W<'_, SCRrs> {
        CBLEWUF_W::new(self, 9)
    }
    ///Bit 10 - Clear 802.15.4 wakeup interrupt flag
    #[inline(always)]
    pub fn c802wuf(&mut self) -> C802WUF_W<'_, SCRrs> {
        C802WUF_W::new(self, 10)
    }
    ///Bit 11 - Clear critical radio phase end of activity interrupt flag
    #[inline(always)]
    pub fn ccrpef(&mut self) -> CCRPEF_W<'_, SCRrs> {
        CCRPEF_W::new(self, 11)
    }
    ///Bit 12 - Clear BLE end of activity interrupt flag
    #[inline(always)]
    pub fn cbleaf(&mut self) -> CBLEAF_W<'_, SCRrs> {
        CBLEAF_W::new(self, 12)
    }
    ///Bit 13 - Clear 802.15.4 end of activity interrupt flag
    #[inline(always)]
    pub fn c802af(&mut self) -> C802AF_W<'_, SCRrs> {
        C802AF_W::new(self, 13)
    }
    ///Bit 14 - Clear CPU2 Hold interrupt flag
    #[inline(always)]
    pub fn cc2hf(&mut self) -> CC2HF_W<'_, SCRrs> {
        CC2HF_W::new(self, 14)
    }
}
/**Power status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#PWR:SCR)*/
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`scr::W`](W) writer structure
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCRrs {}
