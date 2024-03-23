#[doc = "Register `FDCAN_TXBCR` reader"]
pub type R = crate::R<FDCAN_TXBCRrs>;
#[doc = "Register `FDCAN_TXBCR` writer"]
pub type W = crate::W<FDCAN_TXBCRrs>;
#[doc = "Field `CR` reader - Cancellation Request"]
pub type CR_R = crate::FieldReader;
#[doc = "Field `CR` writer - Cancellation Request"]
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Cancellation Request"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Cancellation Request"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<FDCAN_TXBCRrs> {
        CR_W::new(self, 0)
    }
}
#[doc = "FDCAN Tx Buffer Cancellation Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXBCRrs;
impl crate::RegisterSpec for FDCAN_TXBCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbcr::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXBCRrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbcr::W`](W) writer structure"]
impl crate::Writable for FDCAN_TXBCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBCR to value 0"]
impl crate::Resettable for FDCAN_TXBCRrs {
    const RESET_VALUE: u32 = 0;
}
