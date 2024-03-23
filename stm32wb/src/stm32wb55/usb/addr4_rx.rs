#[doc = "Register `ADDR4_RX` reader"]
pub type R = crate::R<ADDR4_RXrs>;
#[doc = "Register `ADDR4_RX` writer"]
pub type W = crate::W<ADDR4_RXrs>;
#[doc = "Field `ADDR4_RX` reader - Reception buffer address"]
pub type ADDR4_RX_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR4_RX` writer - Reception buffer address"]
pub type ADDR4_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 1:15 - Reception buffer address"]
    #[inline(always)]
    pub fn addr4_rx(&self) -> ADDR4_RX_R {
        ADDR4_RX_R::new((self.bits >> 1) & 0x7fff)
    }
}
impl W {
    #[doc = "Bits 1:15 - Reception buffer address"]
    #[inline(always)]
    #[must_use]
    pub fn addr4_rx(&mut self) -> ADDR4_RX_W<ADDR4_RXrs> {
        ADDR4_RX_W::new(self, 1)
    }
}
#[doc = "Reception buffer address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr4_rx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr4_rx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR4_RXrs;
impl crate::RegisterSpec for ADDR4_RXrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`addr4_rx::R`](R) reader structure"]
impl crate::Readable for ADDR4_RXrs {}
#[doc = "`write(|w| ..)` method takes [`addr4_rx::W`](W) writer structure"]
impl crate::Writable for ADDR4_RXrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ADDR4_RX to value 0"]
impl crate::Resettable for ADDR4_RXrs {
    const RESET_VALUE: u16 = 0;
}
