///Register `PWR_SCR` writer
pub type W = crate::W<PWR_SCRrs>;
///Field `CWUF1` writer - Clear wake-up flag 1 Setting this bit clears the WUF1 flag in the PWR_SR1 register.
pub type CWUF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF2` writer - Clear wake-up flag 2 Setting this bit clears the WUF2 flag in the PWR_SR1 register.
pub type CWUF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF3` writer - Clear wake-up flag 3 Setting this bit clears the WUF3 flag in the PWR_SR1 register.
pub type CWUF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF4` writer - Clear wake-up flag 4 Setting this bit clears the WUF4 flag in the PWR_SR1 register.
pub type CWUF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF5` writer - Clear wake-up flag 5 Setting this bit clears the WUF5 flag in the PWR_SR1 register.
pub type CWUF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF7` writer - Clear wake-up flag 7 Setting this bit clears the WUF7 flag in the PWR_SR1 register.
pub type CWUF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSBF` writer - Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register.
pub type CSBF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PWR_SCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear wake-up flag 1 Setting this bit clears the WUF1 flag in the PWR_SR1 register.
    #[inline(always)]
    pub fn cwuf1(&mut self) -> CWUF1_W<PWR_SCRrs> {
        CWUF1_W::new(self, 0)
    }
    ///Bit 1 - Clear wake-up flag 2 Setting this bit clears the WUF2 flag in the PWR_SR1 register.
    #[inline(always)]
    pub fn cwuf2(&mut self) -> CWUF2_W<PWR_SCRrs> {
        CWUF2_W::new(self, 1)
    }
    ///Bit 2 - Clear wake-up flag 3 Setting this bit clears the WUF3 flag in the PWR_SR1 register.
    #[inline(always)]
    pub fn cwuf3(&mut self) -> CWUF3_W<PWR_SCRrs> {
        CWUF3_W::new(self, 2)
    }
    ///Bit 3 - Clear wake-up flag 4 Setting this bit clears the WUF4 flag in the PWR_SR1 register.
    #[inline(always)]
    pub fn cwuf4(&mut self) -> CWUF4_W<PWR_SCRrs> {
        CWUF4_W::new(self, 3)
    }
    ///Bit 4 - Clear wake-up flag 5 Setting this bit clears the WUF5 flag in the PWR_SR1 register.
    #[inline(always)]
    pub fn cwuf5(&mut self) -> CWUF5_W<PWR_SCRrs> {
        CWUF5_W::new(self, 4)
    }
    ///Bit 6 - Clear wake-up flag 7 Setting this bit clears the WUF7 flag in the PWR_SR1 register.
    #[inline(always)]
    pub fn cwuf7(&mut self) -> CWUF7_W<PWR_SCRrs> {
        CWUF7_W::new(self, 6)
    }
    ///Bit 8 - Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register.
    #[inline(always)]
    pub fn csbf(&mut self) -> CSBF_W<PWR_SCRrs> {
        CSBF_W::new(self, 8)
    }
}
/**Power status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#PWR:PWR_SCR)*/
pub struct PWR_SCRrs;
impl crate::RegisterSpec for PWR_SCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pwr_scr::W`](W) writer structure
impl crate::Writable for PWR_SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_SCR to value 0
impl crate::Resettable for PWR_SCRrs {
    const RESET_VALUE: u32 = 0;
}