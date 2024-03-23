#[doc = "Register `ADDR2_RX` reader"]
pub type R = crate::R<ADDR2_RXrs>;
#[doc = "Register `ADDR2_RX` writer"]
pub type W = crate::W<ADDR2_RXrs>;
#[doc = "Field `ADDR2_RX` reader - Reception buffer address"]
pub type ADDR2_RX_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR2_RX` writer - Reception buffer address"]
pub type ADDR2_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 1:15 - Reception buffer address"]
    #[inline(always)]
    pub fn addr2_rx(&self) -> ADDR2_RX_R {
        ADDR2_RX_R::new((self.bits >> 1) & 0x7fff)
    }
}
impl W {
    #[doc = "Bits 1:15 - Reception buffer address"]
    #[inline(always)]
    #[must_use]
    pub fn addr2_rx(&mut self) -> ADDR2_RX_W<ADDR2_RXrs> {
        ADDR2_RX_W::new(self, 1)
    }
}
#[doc = "Reception buffer address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr2_rx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr2_rx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR2_RXrs;
impl crate::RegisterSpec for ADDR2_RXrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`addr2_rx::R`](R) reader structure"]
impl crate::Readable for ADDR2_RXrs {}
#[doc = "`write(|w| ..)` method takes [`addr2_rx::W`](W) writer structure"]
impl crate::Writable for ADDR2_RXrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ADDR2_RX to value 0"]
impl crate::Resettable for ADDR2_RXrs {
    const RESET_VALUE: u16 = 0;
}
