#[doc = "Register `FDCAN_TXBTIE` reader"]
pub type R = crate::R<FDCAN_TXBTIErs>;
#[doc = "Register `FDCAN_TXBTIE` writer"]
pub type W = crate::W<FDCAN_TXBTIErs>;
#[doc = "Field `TIE` reader - TIE"]
pub type TIE_R = crate::FieldReader<u32>;
#[doc = "Field `TIE` writer - TIE"]
pub type TIE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TIE"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TIE"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<FDCAN_TXBTIErs> {
        TIE_W::new(self, 0)
    }
}
#[doc = "FDCAN Tx buffer transmission interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbtie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbtie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXBTIErs;
impl crate::RegisterSpec for FDCAN_TXBTIErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbtie::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXBTIErs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbtie::W`](W) writer structure"]
impl crate::Writable for FDCAN_TXBTIErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBTIE to value 0"]
impl crate::Resettable for FDCAN_TXBTIErs {
    const RESET_VALUE: u32 = 0;
}
