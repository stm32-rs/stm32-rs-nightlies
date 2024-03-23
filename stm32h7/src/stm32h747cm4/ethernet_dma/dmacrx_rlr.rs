#[doc = "Register `DMACRxRLR` reader"]
pub type R = crate::R<DMACRX_RLRrs>;
#[doc = "Register `DMACRxRLR` writer"]
pub type W = crate::W<DMACRX_RLRrs>;
#[doc = "Field `RDRL` reader - Receive Descriptor Ring Length"]
pub type RDRL_R = crate::FieldReader<u16>;
#[doc = "Field `RDRL` writer - Receive Descriptor Ring Length"]
pub type RDRL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Receive Descriptor Ring Length"]
    #[inline(always)]
    pub fn rdrl(&self) -> RDRL_R {
        RDRL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Receive Descriptor Ring Length"]
    #[inline(always)]
    #[must_use]
    pub fn rdrl(&mut self) -> RDRL_W<DMACRX_RLRrs> {
        RDRL_W::new(self, 0)
    }
}
#[doc = "Channel Rx descriptor ring length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrx_rlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrx_rlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACRX_RLRrs;
impl crate::RegisterSpec for DMACRX_RLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrx_rlr::R`](R) reader structure"]
impl crate::Readable for DMACRX_RLRrs {}
#[doc = "`write(|w| ..)` method takes [`dmacrx_rlr::W`](W) writer structure"]
impl crate::Writable for DMACRX_RLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACRxRLR to value 0"]
impl crate::Resettable for DMACRX_RLRrs {
    const RESET_VALUE: u32 = 0;
}
