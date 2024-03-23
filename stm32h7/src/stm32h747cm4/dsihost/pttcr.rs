#[doc = "Register `PTTCR` reader"]
pub type R = crate::R<PTTCRrs>;
#[doc = "Register `PTTCR` writer"]
pub type W = crate::W<PTTCRrs>;
#[doc = "Field `TX_TRIG` reader - Transmission trigger"]
pub type TX_TRIG_R = crate::FieldReader;
#[doc = "Field `TX_TRIG` writer - Transmission trigger"]
pub type TX_TRIG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Transmission trigger"]
    #[inline(always)]
    pub fn tx_trig(&self) -> TX_TRIG_R {
        TX_TRIG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmission trigger"]
    #[inline(always)]
    #[must_use]
    pub fn tx_trig(&mut self) -> TX_TRIG_W<PTTCRrs> {
        TX_TRIG_W::new(self, 0)
    }
}
#[doc = "DSI Host PHY TX triggers configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pttcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pttcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTTCRrs;
impl crate::RegisterSpec for PTTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pttcr::R`](R) reader structure"]
impl crate::Readable for PTTCRrs {}
#[doc = "`write(|w| ..)` method takes [`pttcr::W`](W) writer structure"]
impl crate::Writable for PTTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTTCR to value 0"]
impl crate::Resettable for PTTCRrs {
    const RESET_VALUE: u32 = 0;
}
