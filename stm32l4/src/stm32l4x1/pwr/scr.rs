///Register `SCR` writer
pub type W = crate::W<SCRrs>;
///Field `WUF1` writer - Clear wakeup flag 1
pub type WUF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF2` writer - Clear wakeup flag 2
pub type WUF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF3` writer - Clear wakeup flag 3
pub type WUF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF4` writer - Clear wakeup flag 4
pub type WUF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUF5` writer - Clear wakeup flag 5
pub type WUF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBF` writer - Clear standby flag
pub type SBF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear wakeup flag 1
    #[inline(always)]
    #[must_use]
    pub fn wuf1(&mut self) -> WUF1_W<SCRrs> {
        WUF1_W::new(self, 0)
    }
    ///Bit 1 - Clear wakeup flag 2
    #[inline(always)]
    #[must_use]
    pub fn wuf2(&mut self) -> WUF2_W<SCRrs> {
        WUF2_W::new(self, 1)
    }
    ///Bit 2 - Clear wakeup flag 3
    #[inline(always)]
    #[must_use]
    pub fn wuf3(&mut self) -> WUF3_W<SCRrs> {
        WUF3_W::new(self, 2)
    }
    ///Bit 3 - Clear wakeup flag 4
    #[inline(always)]
    #[must_use]
    pub fn wuf4(&mut self) -> WUF4_W<SCRrs> {
        WUF4_W::new(self, 3)
    }
    ///Bit 4 - Clear wakeup flag 5
    #[inline(always)]
    #[must_use]
    pub fn wuf5(&mut self) -> WUF5_W<SCRrs> {
        WUF5_W::new(self, 4)
    }
    ///Bit 8 - Clear standby flag
    #[inline(always)]
    #[must_use]
    pub fn sbf(&mut self) -> SBF_W<SCRrs> {
        SBF_W::new(self, 8)
    }
}
/**Power status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x1.html#PWR:SCR)*/
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`scr::W`](W) writer structure
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCRrs {
    const RESET_VALUE: u32 = 0;
}
