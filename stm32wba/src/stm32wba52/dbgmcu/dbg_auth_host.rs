///Register `DBG_AUTH_HOST` writer
pub type W = crate::W<DBG_AUTH_HOSTrs>;
///Field `AUTH_KEY` writer - Device authentication key The device specific 64-bit authentication key (OEMn key) must be written to this register (in two successive 32-bit writes, least significant word first) to permit RDP regression. Writing a wrong key locks access to the device and prevent code execution from the Flash memory.
pub type AUTH_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<DBG_AUTH_HOSTrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Device authentication key The device specific 64-bit authentication key (OEMn key) must be written to this register (in two successive 32-bit writes, least significant word first) to permit RDP regression. Writing a wrong key locks access to the device and prevent code execution from the Flash memory.
    #[inline(always)]
    pub fn auth_key(&mut self) -> AUTH_KEY_W<'_, DBG_AUTH_HOSTrs> {
        AUTH_KEY_W::new(self, 0)
    }
}
/**DBGMCU debug host authentication register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_auth_host::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#DBGMCU:DBG_AUTH_HOST)*/
pub struct DBG_AUTH_HOSTrs;
impl crate::RegisterSpec for DBG_AUTH_HOSTrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`dbg_auth_host::W`](W) writer structure
impl crate::Writable for DBG_AUTH_HOSTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBG_AUTH_HOST to value 0
impl crate::Resettable for DBG_AUTH_HOSTrs {}
