#[doc = "Register `COUNT7_TX` reader"]
pub type R = crate::R<COUNT7_TXrs>;
#[doc = "Register `COUNT7_TX` writer"]
pub type W = crate::W<COUNT7_TXrs>;
#[doc = "Field `COUNT7_TX` reader - Transmission byte count"]
pub type COUNT7_TX_R = crate::FieldReader<u16>;
#[doc = "Field `COUNT7_TX` writer - Transmission byte count"]
pub type COUNT7_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Transmission byte count"]
    #[inline(always)]
    pub fn count7_tx(&self) -> COUNT7_TX_R {
        COUNT7_TX_R::new(self.bits & 0x03ff)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transmission byte count"]
    #[inline(always)]
    #[must_use]
    pub fn count7_tx(&mut self) -> COUNT7_TX_W<COUNT7_TXrs> {
        COUNT7_TX_W::new(self, 0)
    }
}
#[doc = "Transmission byte count 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count7_tx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count7_tx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COUNT7_TXrs;
impl crate::RegisterSpec for COUNT7_TXrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`count7_tx::R`](R) reader structure"]
impl crate::Readable for COUNT7_TXrs {}
#[doc = "`write(|w| ..)` method takes [`count7_tx::W`](W) writer structure"]
impl crate::Writable for COUNT7_TXrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets COUNT7_TX to value 0"]
impl crate::Resettable for COUNT7_TXrs {
    const RESET_VALUE: u16 = 0;
}
