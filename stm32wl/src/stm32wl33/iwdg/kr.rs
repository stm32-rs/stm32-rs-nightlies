///Register `KR` reader
pub type R = crate::R<KRrs>;
///Register `KR` writer
pub type W = crate::W<KRrs>;
///Field `KEY` writer - Key value. Software can only write these bits. Reading returns the reset value. These bits must be written by software at regular intervals with the key value 0xAAAA, otherwise the watchdog generates a reset when the counter reaches 0. Writing the key value 0x5555 to enables access to the IWDG_PR, IWDG_RLR and IWDG_WINR registers. Writing the key value CCCCh starts the watchdog
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KR").finish()
    }
}
impl W {
    ///Bits 0:15 - Key value. Software can only write these bits. Reading returns the reset value. These bits must be written by software at regular intervals with the key value 0xAAAA, otherwise the watchdog generates a reset when the counter reaches 0. Writing the key value 0x5555 to enables access to the IWDG_PR, IWDG_RLR and IWDG_WINR registers. Writing the key value CCCCh starts the watchdog
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, KRrs> {
        KEY_W::new(self, 0)
    }
}
/**IWDG_KR register

You can [`read`](crate::Reg::read) this register and get [`kr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#IWDG:KR)*/
pub struct KRrs;
impl crate::RegisterSpec for KRrs {
    type Ux = u32;
}
///`read()` method returns [`kr::R`](R) reader structure
impl crate::Readable for KRrs {}
///`write(|w| ..)` method takes [`kr::W`](W) writer structure
impl crate::Writable for KRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KR to value 0
impl crate::Resettable for KRrs {}
