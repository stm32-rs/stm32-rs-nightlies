///Register `MACRWUFFER` reader
pub type R = crate::R<MACRWUFFERrs>;
///Register `MACRWUFFER` writer
pub type W = crate::W<MACRWUFFERrs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Ethernet MAC remote wakeup frame filter register

You can [`read`](crate::Reg::read) this register and get [`macrwuffer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrwuffer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#Ethernet_MAC:MACRWUFFER)*/
pub struct MACRWUFFERrs;
impl crate::RegisterSpec for MACRWUFFERrs {
    type Ux = u32;
}
///`read()` method returns [`macrwuffer::R`](R) reader structure
impl crate::Readable for MACRWUFFERrs {}
///`write(|w| ..)` method takes [`macrwuffer::W`](W) writer structure
impl crate::Writable for MACRWUFFERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACRWUFFER to value 0xffff_ffff
impl crate::Resettable for MACRWUFFERrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
