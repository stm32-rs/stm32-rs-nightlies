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
///Field `CWUF6` writer - Clear wakeup flag 6
pub type CWUF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSBF` writer - Clear standby flag
pub type CSBF_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 5 - Clear wakeup flag 6
    #[inline(always)]
    pub fn cwuf6(&mut self) -> CWUF6_W<'_, SCRrs> {
        CWUF6_W::new(self, 5)
    }
    ///Bit 8 - Clear standby flag
    #[inline(always)]
    pub fn csbf(&mut self) -> CSBF_W<'_, SCRrs> {
        CSBF_W::new(self, 8)
    }
}
/**Power status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#PWR:SCR)*/
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
