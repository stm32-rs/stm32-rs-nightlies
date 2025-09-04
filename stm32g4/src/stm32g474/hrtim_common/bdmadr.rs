///Register `BDMADR` writer
pub type W = crate::W<BDMADRrs>;
///Field `BDMADR` writer - Burst DMA Data register
pub type BDMADR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<BDMADRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Burst DMA Data register
    #[inline(always)]
    pub fn bdmadr(&mut self) -> BDMADR_W<BDMADRrs> {
        BDMADR_W::new(self, 0)
    }
}
/**Burst DMA Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdmadr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:BDMADR)*/
pub struct BDMADRrs;
impl crate::RegisterSpec for BDMADRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`bdmadr::W`](W) writer structure
impl crate::Writable for BDMADRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets BDMADR to value 0
impl crate::Resettable for BDMADRrs {}
