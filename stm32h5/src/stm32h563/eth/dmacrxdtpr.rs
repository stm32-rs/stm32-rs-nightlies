#[doc = "Register `DMACRXDTPR` reader"]
pub type R = crate::R<DMACRXDTPRrs>;
#[doc = "Register `DMACRXDTPR` writer"]
pub type W = crate::W<DMACRXDTPRrs>;
#[doc = "Field `RDT` reader - Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring. The software writes the tail pointer to add more descriptors to the Rx channel. The hardware tries to write all received packets to the descriptors referenced between the head and the tail pointer registers."]
pub type RDT_R = crate::FieldReader<u32>;
#[doc = "Field `RDT` writer - Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring. The software writes the tail pointer to add more descriptors to the Rx channel. The hardware tries to write all received packets to the descriptors referenced between the head and the tail pointer registers."]
pub type RDT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring. The software writes the tail pointer to add more descriptors to the Rx channel. The hardware tries to write all received packets to the descriptors referenced between the head and the tail pointer registers."]
    #[inline(always)]
    pub fn rdt(&self) -> RDT_R {
        RDT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Descriptor Tail Pointer This field contains the tail pointer for the Rx descriptor ring. The software writes the tail pointer to add more descriptors to the Rx channel. The hardware tries to write all received packets to the descriptors referenced between the head and the tail pointer registers."]
    #[inline(always)]
    #[must_use]
    pub fn rdt(&mut self) -> RDT_W<DMACRXDTPRrs> {
        RDT_W::new(self, 0)
    }
}
#[doc = "Channel Rx descriptor tail pointer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrxdtpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrxdtpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACRXDTPRrs;
impl crate::RegisterSpec for DMACRXDTPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrxdtpr::R`](R) reader structure"]
impl crate::Readable for DMACRXDTPRrs {}
#[doc = "`write(|w| ..)` method takes [`dmacrxdtpr::W`](W) writer structure"]
impl crate::Writable for DMACRXDTPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACRXDTPR to value 0"]
impl crate::Resettable for DMACRXDTPRrs {
    const RESET_VALUE: u32 = 0;
}
