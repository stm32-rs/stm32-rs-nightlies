#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCRrs>;
#[doc = "Field `CWUF1` writer - Clear wakeup flag 1"]
pub type CWUF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF2` writer - Clear wakeup flag 2"]
pub type CWUF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF3` writer - Clear wakeup flag 3"]
pub type CWUF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF4` writer - Clear wakeup flag 4"]
pub type CWUF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF5` writer - Clear wakeup flag 5"]
pub type CWUF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMPSFBF` writer - Clear SMPS Step Down converter forced in Bypass interrupt flag"]
pub type CSMPSFBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBORHF` writer - Clear BORH interrupt flag"]
pub type CBORHF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBLEWUF` writer - Clear BLE wakeup interrupt flag"]
pub type CBLEWUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C802WUF` writer - Clear 802.15.4 wakeup interrupt flag"]
pub type C802WUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCRPEF` writer - Clear critical radio phase end of activity interrupt flag"]
pub type CCRPEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBLEAF` writer - Clear BLE end of activity interrupt flag"]
pub type CBLEAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C802AF` writer - Clear 802.15.4 end of activity interrupt flag"]
pub type C802AF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2HF` writer - Clear CPU2 Hold interrupt flag"]
pub type CC2HF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear wakeup flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf1(&mut self) -> CWUF1_W<SCRrs> {
        CWUF1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear wakeup flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf2(&mut self) -> CWUF2_W<SCRrs> {
        CWUF2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wakeup flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf3(&mut self) -> CWUF3_W<SCRrs> {
        CWUF3_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear wakeup flag 4"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf4(&mut self) -> CWUF4_W<SCRrs> {
        CWUF4_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear wakeup flag 5"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf5(&mut self) -> CWUF5_W<SCRrs> {
        CWUF5_W::new(self, 4)
    }
    #[doc = "Bit 7 - Clear SMPS Step Down converter forced in Bypass interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn csmpsfbf(&mut self) -> CSMPSFBF_W<SCRrs> {
        CSMPSFBF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear BORH interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cborhf(&mut self) -> CBORHF_W<SCRrs> {
        CBORHF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear BLE wakeup interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cblewuf(&mut self) -> CBLEWUF_W<SCRrs> {
        CBLEWUF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear 802.15.4 wakeup interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn c802wuf(&mut self) -> C802WUF_W<SCRrs> {
        C802WUF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear critical radio phase end of activity interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ccrpef(&mut self) -> CCRPEF_W<SCRrs> {
        CCRPEF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear BLE end of activity interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cbleaf(&mut self) -> CBLEAF_W<SCRrs> {
        CBLEAF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear 802.15.4 end of activity interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn c802af(&mut self) -> C802AF_W<SCRrs> {
        C802AF_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear CPU2 Hold interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc2hf(&mut self) -> CC2HF_W<SCRrs> {
        CC2HF_W::new(self, 14)
    }
}
#[doc = "Power status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCRrs {
    const RESET_VALUE: u32 = 0;
}
