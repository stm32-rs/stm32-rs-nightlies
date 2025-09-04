///Register `DINR` writer
pub type W = crate::W<DINRrs>;
///Field `DIN` writer - Data input
pub type DIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<DINRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Data input
    #[inline(always)]
    pub fn din(&mut self) -> DIN_W<DINRrs> {
        DIN_W::new(self, 0)
    }
}
/**AES data input register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#AES:DINR)*/
pub struct DINRrs;
impl crate::RegisterSpec for DINRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`dinr::W`](W) writer structure
impl crate::Writable for DINRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DINR to value 0
impl crate::Resettable for DINRrs {}
