#[doc = "Register `FDCAN_TXBCIE` reader"]
pub type R = crate::R<FDCAN_TXBCIErs>;
#[doc = "Register `FDCAN_TXBCIE` writer"]
pub type W = crate::W<FDCAN_TXBCIErs>;
#[doc = "Field `CF` reader - Cancellation Finished Interrupt Enable"]
pub type CF_R = crate::FieldReader;
#[doc = "Field `CF` writer - Cancellation Finished Interrupt Enable"]
pub type CF_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf(&mut self) -> CF_W<FDCAN_TXBCIErs> {
        CF_W::new(self, 0)
    }
}
#[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbcie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbcie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
