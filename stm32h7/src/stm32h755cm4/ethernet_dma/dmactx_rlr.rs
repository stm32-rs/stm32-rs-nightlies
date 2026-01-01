///Register `DMACTxRLR` reader
pub type R = crate::R<DMACTX_RLRrs>;
///Register `DMACTxRLR` writer
pub type W = crate::W<DMACTX_RLRrs>;
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
        f.debug_struct("DMACTxRLR")
            .field("tdrl", &self.tdrl())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Transmit Descriptor Ring Length
    #[inline(always)]
    pub fn tdrl(&mut self) -> TDRL_W<'_, DMACTX_RLRrs> {
        TDRL_W::new(self, 0)
    }
}
/**Channel Tx descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmactx_rlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactx_rlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#Ethernet_DMA:DMACTxRLR)*/
pub struct DMACTX_RLRrs;
impl crate::RegisterSpec for DMACTX_RLRrs {
    type Ux = u32;
}
///`read()` method returns [`dmactx_rlr::R`](R) reader structure
impl crate::Readable for DMACTX_RLRrs {}
///`write(|w| ..)` method takes [`dmactx_rlr::W`](W) writer structure
impl crate::Writable for DMACTX_RLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACTxRLR to value 0
impl crate::Resettable for DMACTX_RLRrs {}
