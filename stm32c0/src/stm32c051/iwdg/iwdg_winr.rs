///Register `IWDG_WINR` reader
pub type R = crate::R<IWDG_WINRrs>;
///Register `IWDG_WINR` writer
pub type W = crate::W<IWDG_WINRrs>;
///Field `WIN` reader - Watchdog counter window value These bits are write access protected, see Section 22.3.4, they contain the high limit of the window value to be compared with the downcounter. To prevent a reset, the downcounter must be reloaded when its value is lower than the window register value and greater than 0x0 The WVU bit in the IWDG status register (IWDG_SR) must be reset in order to be able to change the reload value. Note: Reading this register returns the reload value from the V<sub>DD</sub> voltage domain. This value may not be valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the WVU bit in the IWDG status register (IWDG_SR) is reset.
pub type WIN_R = crate::FieldReader<u16>;
///Field `WIN` writer - Watchdog counter window value These bits are write access protected, see Section 22.3.4, they contain the high limit of the window value to be compared with the downcounter. To prevent a reset, the downcounter must be reloaded when its value is lower than the window register value and greater than 0x0 The WVU bit in the IWDG status register (IWDG_SR) must be reset in order to be able to change the reload value. Note: Reading this register returns the reload value from the V<sub>DD</sub> voltage domain. This value may not be valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the WVU bit in the IWDG status register (IWDG_SR) is reset.
pub type WIN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Watchdog counter window value These bits are write access protected, see Section 22.3.4, they contain the high limit of the window value to be compared with the downcounter. To prevent a reset, the downcounter must be reloaded when its value is lower than the window register value and greater than 0x0 The WVU bit in the IWDG status register (IWDG_SR) must be reset in order to be able to change the reload value. Note: Reading this register returns the reload value from the V<sub>DD</sub> voltage domain. This value may not be valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the WVU bit in the IWDG status register (IWDG_SR) is reset.
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_WINR")
            .field("win", &self.win())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Watchdog counter window value These bits are write access protected, see Section 22.3.4, they contain the high limit of the window value to be compared with the downcounter. To prevent a reset, the downcounter must be reloaded when its value is lower than the window register value and greater than 0x0 The WVU bit in the IWDG status register (IWDG_SR) must be reset in order to be able to change the reload value. Note: Reading this register returns the reload value from the V<sub>DD</sub> voltage domain. This value may not be valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the WVU bit in the IWDG status register (IWDG_SR) is reset.
    #[inline(always)]
    pub fn win(&mut self) -> WIN_W<'_, IWDG_WINRrs> {
        WIN_W::new(self, 0)
    }
}
/**IWDG window register

You can [`read`](crate::Reg::read) this register and get [`iwdg_winr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_winr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#IWDG:IWDG_WINR)*/
pub struct IWDG_WINRrs;
impl crate::RegisterSpec for IWDG_WINRrs {
    type Ux = u32;
}
///`read()` method returns [`iwdg_winr::R`](R) reader structure
impl crate::Readable for IWDG_WINRrs {}
///`write(|w| ..)` method takes [`iwdg_winr::W`](W) writer structure
impl crate::Writable for IWDG_WINRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IWDG_WINR to value 0
impl crate::Resettable for IWDG_WINRrs {}
