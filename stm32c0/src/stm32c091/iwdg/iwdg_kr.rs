///Register `IWDG_KR` writer
pub type W = crate::W<IWDG_KRrs>;
///Field `KEY` writer - Key value (write only, read 0x0000) These bits must be written by software at regular intervals with the key value 0xAAAA, otherwise the watchdog generates a reset when the counter reaches 0. Writing the key value 0x5555 to enable access to the IWDG_PR, IWDG_RLR and IWDG_WINR registers (see Section 22.3.4: Register access protection) Writing the key value 0xCCCC starts the watchdog (except if the hardware watchdog option is selected)
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<IWDG_KRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:15 - Key value (write only, read 0x0000) These bits must be written by software at regular intervals with the key value 0xAAAA, otherwise the watchdog generates a reset when the counter reaches 0. Writing the key value 0x5555 to enable access to the IWDG_PR, IWDG_RLR and IWDG_WINR registers (see Section 22.3.4: Register access protection) Writing the key value 0xCCCC starts the watchdog (except if the hardware watchdog option is selected)
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, IWDG_KRrs> {
        KEY_W::new(self, 0)
    }
}
/**IWDG key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_kr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#IWDG:IWDG_KR)*/
pub struct IWDG_KRrs;
impl crate::RegisterSpec for IWDG_KRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`iwdg_kr::W`](W) writer structure
impl crate::Writable for IWDG_KRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IWDG_KR to value 0
impl crate::Resettable for IWDG_KRrs {}
