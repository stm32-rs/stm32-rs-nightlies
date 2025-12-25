///Register `DMAC1TXRLR` reader
pub type R = crate::R<DMAC1TXRLRrs>;
///Register `DMAC1TXRLR` writer
pub type W = crate::W<DMAC1TXRLRrs>;
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
        f.debug_struct("DMAC1TXRLR")
            .field("tdrl", &self.tdrl())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Transmit Descriptor Ring Length
    #[inline(always)]
    pub fn tdrl(&mut self) -> TDRL_W<'_, DMAC1TXRLRrs> {
        TDRL_W::new(self, 0)
    }
}
/**Channel 1 T1 descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmac1txrlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1txrlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1TXRLR)*/
pub struct DMAC1TXRLRrs;
impl crate::RegisterSpec for DMAC1TXRLRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac1txrlr::R`](R) reader structure
impl crate::Readable for DMAC1TXRLRrs {}
///`write(|w| ..)` method takes [`dmac1txrlr::W`](W) writer structure
impl crate::Writable for DMAC1TXRLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC1TXRLR to value 0
impl crate::Resettable for DMAC1TXRLRrs {}
