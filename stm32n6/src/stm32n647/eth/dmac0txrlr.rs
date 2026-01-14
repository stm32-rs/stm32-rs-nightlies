///Register `DMAC0TXRLR` reader
pub type R = crate::R<DMAC0TXRLRrs>;
///Register `DMAC0TXRLR` writer
pub type W = crate::W<DMAC0TXRLRrs>;
///Field `TDRL` reader - Transmit Descriptor Ring Length
pub type TDRL_R = crate::FieldReader<u16>;
///Field `TDRL` writer - Transmit Descriptor Ring Length
pub type TDRL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Transmit Descriptor Ring Length
    #[inline(always)]
    pub fn tdrl(&self) -> TDRL_R {
        TDRL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC0TXRLR")
            .field("tdrl", &self.tdrl())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Transmit Descriptor Ring Length
    #[inline(always)]
    pub fn tdrl(&mut self) -> TDRL_W<'_, DMAC0TXRLRrs> {
        TDRL_W::new(self, 0)
    }
}
/**Channel 0 T0 descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmac0txrlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0txrlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:DMAC0TXRLR)*/
pub struct DMAC0TXRLRrs;
impl crate::RegisterSpec for DMAC0TXRLRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac0txrlr::R`](R) reader structure
impl crate::Readable for DMAC0TXRLRrs {}
///`write(|w| ..)` method takes [`dmac0txrlr::W`](W) writer structure
impl crate::Writable for DMAC0TXRLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC0TXRLR to value 0
impl crate::Resettable for DMAC0TXRLRrs {}
