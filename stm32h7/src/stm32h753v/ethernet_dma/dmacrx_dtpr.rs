#[doc = "Register `DMACRxDTPR` reader"]
pub type R = crate::R<DMACRX_DTPRrs>;
#[doc = "Register `DMACRxDTPR` writer"]
pub type W = crate::W<DMACRX_DTPRrs>;
#[doc = "Field `RDT` reader - Receive Descriptor Tail Pointer"]
pub type RDT_R = crate::FieldReader<u32>;
#[doc = "Field `RDT` writer - Receive Descriptor Tail Pointer"]
pub type RDT_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Receive Descriptor Tail Pointer"]
    #[inline(always)]
    pub fn rdt(&self) -> RDT_R {
        RDT_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive Descriptor Tail Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn rdt(&mut self) -> RDT_W<DMACRX_DTPRrs> {
        RDT_W::new(self, 2)
    }
}
#[doc = "Channel Rx descriptor tail pointer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrx_dtpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrx_dtpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACRX_DTPRrs;
impl crate::RegisterSpec for DMACRX_DTPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrx_dtpr::R`](R) reader structure"]
impl crate::Readable for DMACRX_DTPRrs {}
#[doc = "`write(|w| ..)` method takes [`dmacrx_dtpr::W`](W) writer structure"]
impl crate::Writable for DMACRX_DTPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACRxDTPR to value 0"]
impl crate::Resettable for DMACRX_DTPRrs {
    const RESET_VALUE: u32 = 0;
}
