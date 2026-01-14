///Register `MACRWUFFR` reader
pub type R = crate::R<MACRWUFFRrs>;
///Register `MACRWUFFR` writer
pub type W = crate::W<MACRWUFFRrs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)

You can [`read`](crate::Reg::read) this register and get [`macrwuffr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrwuffr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#Ethernet_MAC:MACRWUFFR)*/
pub struct MACRWUFFRrs;
impl crate::RegisterSpec for MACRWUFFRrs {
    type Ux = u32;
}
///`read()` method returns [`macrwuffr::R`](R) reader structure
impl crate::Readable for MACRWUFFRrs {}
///`write(|w| ..)` method takes [`macrwuffr::W`](W) writer structure
impl crate::Writable for MACRWUFFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACRWUFFR to value 0
impl crate::Resettable for MACRWUFFRrs {}
