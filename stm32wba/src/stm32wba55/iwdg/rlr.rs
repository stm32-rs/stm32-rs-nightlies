///Register `RLR` reader
pub type R = crate::R<RLRrs>;
///Register `RLR` writer
pub type W = crate::W<RLRrs>;
///Field `RL` reader - Watchdog counter reload value These bits are write access protected see Register access protection. They are written by software to define the value to be loaded in the watchdog counter each time the value 0xAAAA is written in the IWDG key register (IWDG_KR). The watchdog counter counts down from this value. The timeout period is a function of this value and the prescaler.clock. It is not recommended to set RL\[11:0\] to a value lower than 2. The RVU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the reload value from the VsubDD/sub voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing on it. For this reason the value read from this register is valid only when the RVU bit in the IWDG status register (IWDG_SR) is reset.
pub type RL_R = crate::FieldReader<u16>;
///Field `RL` writer - Watchdog counter reload value These bits are write access protected see Register access protection. They are written by software to define the value to be loaded in the watchdog counter each time the value 0xAAAA is written in the IWDG key register (IWDG_KR). The watchdog counter counts down from this value. The timeout period is a function of this value and the prescaler.clock. It is not recommended to set RL\[11:0\] to a value lower than 2. The RVU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the reload value from the VsubDD/sub voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing on it. For this reason the value read from this register is valid only when the RVU bit in the IWDG status register (IWDG_SR) is reset.
pub type RL_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    ///Bits 0:11 - Watchdog counter reload value These bits are write access protected see Register access protection. They are written by software to define the value to be loaded in the watchdog counter each time the value 0xAAAA is written in the IWDG key register (IWDG_KR). The watchdog counter counts down from this value. The timeout period is a function of this value and the prescaler.clock. It is not recommended to set RL\[11:0\] to a value lower than 2. The RVU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the reload value from the VsubDD/sub voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing on it. For this reason the value read from this register is valid only when the RVU bit in the IWDG status register (IWDG_SR) is reset.
    #[inline(always)]
    pub fn rl(&self) -> RL_R {
        RL_R::new(self.bits & 0x0fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RLR").field("rl", &self.rl()).finish()
    }
}
impl W {
    ///Bits 0:11 - Watchdog counter reload value These bits are write access protected see Register access protection. They are written by software to define the value to be loaded in the watchdog counter each time the value 0xAAAA is written in the IWDG key register (IWDG_KR). The watchdog counter counts down from this value. The timeout period is a function of this value and the prescaler.clock. It is not recommended to set RL\[11:0\] to a value lower than 2. The RVU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the reload value. Note: Reading this register returns the reload value from the VsubDD/sub voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing on it. For this reason the value read from this register is valid only when the RVU bit in the IWDG status register (IWDG_SR) is reset.
    #[inline(always)]
    pub fn rl(&mut self) -> RL_W<'_, RLRrs> {
        RL_W::new(self, 0)
    }
}
/**IWDG reload register

You can [`read`](crate::Reg::read) this register and get [`rlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#IWDG:RLR)*/
pub struct RLRrs;
impl crate::RegisterSpec for RLRrs {
    type Ux = u16;
}
///`read()` method returns [`rlr::R`](R) reader structure
impl crate::Readable for RLRrs {}
///`write(|w| ..)` method takes [`rlr::W`](W) writer structure
impl crate::Writable for RLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RLR to value 0x0fff
impl crate::Resettable for RLRrs {
    const RESET_VALUE: u16 = 0x0fff;
}
