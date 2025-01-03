///Register `DBGMCU_DBG_AUTH_HOST` reader
pub type R = crate::R<DBGMCU_DBG_AUTH_HOSTrs>;
///Register `DBGMCU_DBG_AUTH_HOST` writer
pub type W = crate::W<DBGMCU_DBG_AUTH_HOSTrs>;
///Field `MESSAGE` reader - Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register.
pub type MESSAGE_R = crate::FieldReader<u32>;
///Field `MESSAGE` writer - Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register.
pub type MESSAGE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register.
    #[inline(always)]
    pub fn message(&self) -> MESSAGE_R {
        MESSAGE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGMCU_DBG_AUTH_HOST")
            .field("message", &self.message())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Debug host to device mailbox message. During debug authentication the debug host communicates with the device via this register.
    #[inline(always)]
    pub fn message(&mut self) -> MESSAGE_W<DBGMCU_DBG_AUTH_HOSTrs> {
        MESSAGE_W::new(self, 0)
    }
}
/**DBGMCU debug authentication mailbox host register

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_dbg_auth_host::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgmcu_dbg_auth_host::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DBGMCU:DBGMCU_DBG_AUTH_HOST)*/
pub struct DBGMCU_DBG_AUTH_HOSTrs;
impl crate::RegisterSpec for DBGMCU_DBG_AUTH_HOSTrs {
    type Ux = u32;
}
///`read()` method returns [`dbgmcu_dbg_auth_host::R`](R) reader structure
impl crate::Readable for DBGMCU_DBG_AUTH_HOSTrs {}
///`write(|w| ..)` method takes [`dbgmcu_dbg_auth_host::W`](W) writer structure
impl crate::Writable for DBGMCU_DBG_AUTH_HOSTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DBGMCU_DBG_AUTH_HOST to value 0
impl crate::Resettable for DBGMCU_DBG_AUTH_HOSTrs {
    const RESET_VALUE: u32 = 0;
}
