///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `CC1CF` writer - Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register.
pub type CC1CF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARRMCF` writer - Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register
pub type ARRMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTTRIGCF` writer - External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register
pub type EXTTRIGCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMP1OKCF` writer - Compare register 1 update OK clear flag Writing 1 to this bit clears the CMP1OK flag in the LPTIM_ISR register.
pub type CMP1OKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARROKCF` writer - Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register
pub type ARROKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UPCF` writer - Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 26.3.
pub type UPCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOWNCF` writer - Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 26.3.
pub type DOWNCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UECF` writer - Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register.
pub type UECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REPOKCF` writer - Repetition register update OK clear flag Writing 1 to this bit clears the REPOK flag in the LPTIM_ISR register.
pub type REPOKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIEROKCF` writer - Interrupt enable register update OK clear flag Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register.
pub type DIEROKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register.
    #[inline(always)]
    pub fn cc1cf(&mut self) -> CC1CF_W<ICRrs> {
        CC1CF_W::new(self, 0)
    }
    ///Bit 1 - Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register
    #[inline(always)]
    pub fn arrmcf(&mut self) -> ARRMCF_W<ICRrs> {
        ARRMCF_W::new(self, 1)
    }
    ///Bit 2 - External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register
    #[inline(always)]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W<ICRrs> {
        EXTTRIGCF_W::new(self, 2)
    }
    ///Bit 3 - Compare register 1 update OK clear flag Writing 1 to this bit clears the CMP1OK flag in the LPTIM_ISR register.
    #[inline(always)]
    pub fn cmp1okcf(&mut self) -> CMP1OKCF_W<ICRrs> {
        CMP1OKCF_W::new(self, 3)
    }
    ///Bit 4 - Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register
    #[inline(always)]
    pub fn arrokcf(&mut self) -> ARROKCF_W<ICRrs> {
        ARROKCF_W::new(self, 4)
    }
    ///Bit 5 - Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 26.3.
    #[inline(always)]
    pub fn upcf(&mut self) -> UPCF_W<ICRrs> {
        UPCF_W::new(self, 5)
    }
    ///Bit 6 - Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section 26.3.
    #[inline(always)]
    pub fn downcf(&mut self) -> DOWNCF_W<ICRrs> {
        DOWNCF_W::new(self, 6)
    }
    ///Bit 7 - Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register.
    #[inline(always)]
    pub fn uecf(&mut self) -> UECF_W<ICRrs> {
        UECF_W::new(self, 7)
    }
    ///Bit 8 - Repetition register update OK clear flag Writing 1 to this bit clears the REPOK flag in the LPTIM_ISR register.
    #[inline(always)]
    pub fn repokcf(&mut self) -> REPOKCF_W<ICRrs> {
        REPOKCF_W::new(self, 8)
    }
    ///Bit 24 - Interrupt enable register update OK clear flag Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register.
    #[inline(always)]
    pub fn dierokcf(&mut self) -> DIEROKCF_W<ICRrs> {
        DIEROKCF_W::new(self, 24)
    }
}
/**LPTIM4 interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#LPTIM4:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
