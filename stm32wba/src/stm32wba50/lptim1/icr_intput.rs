///Register `ICR_intput` writer
pub type W = crate::W<ICR_INTPUTrs>;
///Field `CC1CF` writer - Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register.
pub type CC1CF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARRMCF` writer - Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register
pub type ARRMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTTRIGCF` writer - External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register
pub type EXTTRIGCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARROKCF` writer - Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register
pub type ARROKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UPCF` writer - Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section69.3.
pub type UPCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOWNCF` writer - Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section69.3.
pub type DOWNCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UECF` writer - Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register.
pub type UECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REPOKCF` writer - Repetition register update OK clear flag Writing 1 to this bit clears the REPOK flag in the LPTIM_ISR register.
pub type REPOKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2CF` writer - Capture/compare 2 clear flag Writing 1 to this bit clears the CC2IF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section69.3.
pub type CC2CF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1OCF` writer - Capture/compare 1 over-capture clear flag Writing 1 to this bit clears the CC1OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section69.3.
pub type CC1OCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2OCF` writer - Capture/compare 2 over-capture clear flag Writing 1 to this bit clears the CC2OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section69.3.
pub type CC2OCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIEROKCF` writer - Interrupt enable register update OK clear flag Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register.
pub type DIEROKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICR_INTPUTrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register.
    #[inline(always)]
    pub fn cc1cf(&mut self) -> CC1CF_W<'_, ICR_INTPUTrs> {
        CC1CF_W::new(self, 0)
    }
    ///Bit 1 - Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register
    #[inline(always)]
    pub fn arrmcf(&mut self) -> ARRMCF_W<'_, ICR_INTPUTrs> {
        ARRMCF_W::new(self, 1)
    }
    ///Bit 2 - External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register
    #[inline(always)]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W<'_, ICR_INTPUTrs> {
        EXTTRIGCF_W::new(self, 2)
    }
    ///Bit 4 - Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register
    #[inline(always)]
    pub fn arrokcf(&mut self) -> ARROKCF_W<'_, ICR_INTPUTrs> {
        ARROKCF_W::new(self, 4)
    }
    ///Bit 5 - Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section69.3.
    #[inline(always)]
    pub fn upcf(&mut self) -> UPCF_W<'_, ICR_INTPUTrs> {
        UPCF_W::new(self, 5)
    }
    ///Bit 6 - Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section69.3.
    #[inline(always)]
    pub fn downcf(&mut self) -> DOWNCF_W<'_, ICR_INTPUTrs> {
        DOWNCF_W::new(self, 6)
    }
    ///Bit 7 - Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register.
    #[inline(always)]
    pub fn uecf(&mut self) -> UECF_W<'_, ICR_INTPUTrs> {
        UECF_W::new(self, 7)
    }
    ///Bit 8 - Repetition register update OK clear flag Writing 1 to this bit clears the REPOK flag in the LPTIM_ISR register.
    #[inline(always)]
    pub fn repokcf(&mut self) -> REPOKCF_W<'_, ICR_INTPUTrs> {
        REPOKCF_W::new(self, 8)
    }
    ///Bit 9 - Capture/compare 2 clear flag Writing 1 to this bit clears the CC2IF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section69.3.
    #[inline(always)]
    pub fn cc2cf(&mut self) -> CC2CF_W<'_, ICR_INTPUTrs> {
        CC2CF_W::new(self, 9)
    }
    ///Bit 12 - Capture/compare 1 over-capture clear flag Writing 1 to this bit clears the CC1OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section69.3.
    #[inline(always)]
    pub fn cc1ocf(&mut self) -> CC1OCF_W<'_, ICR_INTPUTrs> {
        CC1OCF_W::new(self, 12)
    }
    ///Bit 13 - Capture/compare 2 over-capture clear flag Writing 1 to this bit clears the CC2OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section69.3.
    #[inline(always)]
    pub fn cc2ocf(&mut self) -> CC2OCF_W<'_, ICR_INTPUTrs> {
        CC2OCF_W::new(self, 13)
    }
    ///Bit 24 - Interrupt enable register update OK clear flag Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register.
    #[inline(always)]
    pub fn dierokcf(&mut self) -> DIEROKCF_W<'_, ICR_INTPUTrs> {
        DIEROKCF_W::new(self, 24)
    }
}
/**Interrupt Clear Register (intput mode)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr_intput::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#LPTIM1:ICR_intput)*/
pub struct ICR_INTPUTrs;
impl crate::RegisterSpec for ICR_INTPUTrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr_intput::W`](W) writer structure
impl crate::Writable for ICR_INTPUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR_intput to value 0
impl crate::Resettable for ICR_INTPUTrs {}
