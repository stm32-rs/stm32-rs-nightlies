///Register `PWR_WUSCR` reader
pub type R = crate::R<PWR_WUSCRrs>;
///Register `PWR_WUSCR` writer
pub type W = crate::W<PWR_WUSCRrs>;
///Field `CWUF1` writer - Wakeup flag 1 Writing 1 to this bit clears the WUF1 flag in PWR_WUSR.
pub type CWUF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF2` writer - Wakeup flag 2 Writing 1 to this bit clears the WUF2 flag in PWR_WUSR.
pub type CWUF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF3` writer - Wakeup flag 3 Writing 1 to this bit clears the WUF3 flag in PWR_WUSR.
pub type CWUF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF4` writer - Wakeup flag 4 Writing 1 to this bit clears the WUF4 flag in PWR_WUSR.
pub type CWUF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF5` writer - Wakeup flag 5 Writing 1 to this bit clears the WUF5 flag in PWR_WUSR.
pub type CWUF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF6` writer - Wakeup flag 6 Writing 1 to this bit clears the WUF6 flag in PWR_WUSR.
pub type CWUF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF7` writer - Wakeup flag 7 Writing 1 to this bit clears the WUF7 flag in PWR_WUSR.
pub type CWUF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF8` writer - Wakeup flag 8 Writing 1 to this bit clears the WUF8 flag in PWR_WUSR.
pub type CWUF8_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_WUSCR").finish()
    }
}
impl W {
    ///Bit 0 - Wakeup flag 1 Writing 1 to this bit clears the WUF1 flag in PWR_WUSR.
    #[inline(always)]
    #[must_use]
    pub fn cwuf1(&mut self) -> CWUF1_W<PWR_WUSCRrs> {
        CWUF1_W::new(self, 0)
    }
    ///Bit 1 - Wakeup flag 2 Writing 1 to this bit clears the WUF2 flag in PWR_WUSR.
    #[inline(always)]
    #[must_use]
    pub fn cwuf2(&mut self) -> CWUF2_W<PWR_WUSCRrs> {
        CWUF2_W::new(self, 1)
    }
    ///Bit 2 - Wakeup flag 3 Writing 1 to this bit clears the WUF3 flag in PWR_WUSR.
    #[inline(always)]
    #[must_use]
    pub fn cwuf3(&mut self) -> CWUF3_W<PWR_WUSCRrs> {
        CWUF3_W::new(self, 2)
    }
    ///Bit 3 - Wakeup flag 4 Writing 1 to this bit clears the WUF4 flag in PWR_WUSR.
    #[inline(always)]
    #[must_use]
    pub fn cwuf4(&mut self) -> CWUF4_W<PWR_WUSCRrs> {
        CWUF4_W::new(self, 3)
    }
    ///Bit 4 - Wakeup flag 5 Writing 1 to this bit clears the WUF5 flag in PWR_WUSR.
    #[inline(always)]
    #[must_use]
    pub fn cwuf5(&mut self) -> CWUF5_W<PWR_WUSCRrs> {
        CWUF5_W::new(self, 4)
    }
    ///Bit 5 - Wakeup flag 6 Writing 1 to this bit clears the WUF6 flag in PWR_WUSR.
    #[inline(always)]
    #[must_use]
    pub fn cwuf6(&mut self) -> CWUF6_W<PWR_WUSCRrs> {
        CWUF6_W::new(self, 5)
    }
    ///Bit 6 - Wakeup flag 7 Writing 1 to this bit clears the WUF7 flag in PWR_WUSR.
    #[inline(always)]
    #[must_use]
    pub fn cwuf7(&mut self) -> CWUF7_W<PWR_WUSCRrs> {
        CWUF7_W::new(self, 6)
    }
    ///Bit 7 - Wakeup flag 8 Writing 1 to this bit clears the WUF8 flag in PWR_WUSR.
    #[inline(always)]
    #[must_use]
    pub fn cwuf8(&mut self) -> CWUF8_W<PWR_WUSCRrs> {
        CWUF8_W::new(self, 7)
    }
}
/**PWR wakeup status clear register

You can [`read`](crate::Reg::read) this register and get [`pwr_wuscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_wuscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#PWR:PWR_WUSCR)*/
pub struct PWR_WUSCRrs;
impl crate::RegisterSpec for PWR_WUSCRrs {
    type Ux = u32;
}
///`read()` method returns [`pwr_wuscr::R`](R) reader structure
impl crate::Readable for PWR_WUSCRrs {}
///`write(|w| ..)` method takes [`pwr_wuscr::W`](W) writer structure
impl crate::Writable for PWR_WUSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_WUSCR to value 0
impl crate::Resettable for PWR_WUSCRrs {
    const RESET_VALUE: u32 = 0;
}
