///Register `DMACRXDTPR` reader
pub type R = crate::R<DMACRXDTPRrs>;
///Register `DMACRXDTPR` writer
pub type W = crate::W<DMACRXDTPRrs>;
///Field `RDT` reader - Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring. The software writes the tail pointer to add more descriptors to the Rx channel. The hardware tries to write all received packets to the descriptors referenced between the head and the tail pointer registers.
pub type RDT_R = crate::FieldReader<u32>;
///Field `RDT` writer - Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring. The software writes the tail pointer to add more descriptors to the Rx channel. The hardware tries to write all received packets to the descriptors referenced between the head and the tail pointer registers.
pub type RDT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring. The software writes the tail pointer to add more descriptors to the Rx channel. The hardware tries to write all received packets to the descriptors referenced between the head and the tail pointer registers.
    #[inline(always)]
    pub fn rdt(&self) -> RDT_R {
        RDT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACRXDTPR")
            .field("rdt", &self.rdt())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring. The software writes the tail pointer to add more descriptors to the Rx channel. The hardware tries to write all received packets to the descriptors referenced between the head and the tail pointer registers.
    #[inline(always)]
    pub fn rdt(&mut self) -> RDT_W<'_, DMACRXDTPRrs> {
        RDT_W::new(self, 0)
    }
}
/**Channel Rx descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmacrxdtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrxdtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#ETH:DMACRXDTPR)*/
pub struct DMACRXDTPRrs;
impl crate::RegisterSpec for DMACRXDTPRrs {
    type Ux = u32;
}
///`read()` method returns [`dmacrxdtpr::R`](R) reader structure
impl crate::Readable for DMACRXDTPRrs {}
///`write(|w| ..)` method takes [`dmacrxdtpr::W`](W) writer structure
impl crate::Writable for DMACRXDTPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACRXDTPR to value 0
impl crate::Resettable for DMACRXDTPRrs {}
