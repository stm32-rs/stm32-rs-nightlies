///Register `DMACTXDTPR` reader
pub type R = crate::R<DMACTXDTPRrs>;
///Register `DMACTXDTPR` writer
pub type W = crate::W<DMACTXDTPRrs>;
///Field `TDT` reader - Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring. The software writes the tail pointer to add more descriptors to the Tx channel. The hardware tries to transmit all packets referenced by the descriptors between the head and the tail pointer registers.
pub type TDT_R = crate::FieldReader<u32>;
///Field `TDT` writer - Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring. The software writes the tail pointer to add more descriptors to the Tx channel. The hardware tries to transmit all packets referenced by the descriptors between the head and the tail pointer registers.
pub type TDT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring. The software writes the tail pointer to add more descriptors to the Tx channel. The hardware tries to transmit all packets referenced by the descriptors between the head and the tail pointer registers.
    #[inline(always)]
    pub fn tdt(&self) -> TDT_R {
        TDT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACTXDTPR")
            .field("tdt", &self.tdt())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Transmit Descriptor Tail Pointer This field contains the tail pointer for the Tx descriptor ring. The software writes the tail pointer to add more descriptors to the Tx channel. The hardware tries to transmit all packets referenced by the descriptors between the head and the tail pointer registers.
    #[inline(always)]
    pub fn tdt(&mut self) -> TDT_W<'_, DMACTXDTPRrs> {
        TDT_W::new(self, 0)
    }
}
/**Channel Tx descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmactxdtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactxdtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#ETH:DMACTXDTPR)*/
pub struct DMACTXDTPRrs;
impl crate::RegisterSpec for DMACTXDTPRrs {
    type Ux = u32;
}
///`read()` method returns [`dmactxdtpr::R`](R) reader structure
impl crate::Readable for DMACTXDTPRrs {}
///`write(|w| ..)` method takes [`dmactxdtpr::W`](W) writer structure
impl crate::Writable for DMACTXDTPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACTXDTPR to value 0
impl crate::Resettable for DMACTXDTPRrs {}
