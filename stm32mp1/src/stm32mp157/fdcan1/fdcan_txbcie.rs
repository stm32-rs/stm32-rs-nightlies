#[doc = "Register `FDCAN_TXBCIE` reader"]
pub type R = crate::R<FDCAN_TXBCIErs>;
#[doc = "Register `FDCAN_TXBCIE` writer"]
pub type W = crate::W<FDCAN_TXBCIErs>;
#[doc = "Field `CFIE` reader - CFIE"]
pub type CFIE_R = crate::FieldReader<u32>;
#[doc = "Field `CFIE` writer - CFIE"]
pub type CFIE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CFIE"]
    #[inline(always)]
    pub fn cfie(&self) -> CFIE_R {
        CFIE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CFIE"]
    #[inline(always)]
    #[must_use]
    pub fn cfie(&mut self) -> CFIE_W<FDCAN_TXBCIErs> {
        CFIE_W::new(self, 0)
    }
}
#[doc = "FDCAN Tx buffer cancellation finished interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbcie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbcie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXBCIErs;
impl crate::RegisterSpec for FDCAN_TXBCIErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbcie::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXBCIErs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbcie::W`](W) writer structure"]
impl crate::Writable for FDCAN_TXBCIErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBCIE to value 0"]
impl crate::Resettable for FDCAN_TXBCIErs {
    const RESET_VALUE: u32 = 0;
}
