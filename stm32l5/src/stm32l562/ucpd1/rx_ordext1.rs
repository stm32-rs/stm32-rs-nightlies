#[doc = "Register `RX_ORDEXT1` reader"]
pub type R = crate::R<RX_ORDEXT1rs>;
#[doc = "Register `RX_ORDEXT1` writer"]
pub type W = crate::W<RX_ORDEXT1rs>;
#[doc = "Field `RXSOPX1` reader - RXSOPX1"]
pub type RXSOPX1_R = crate::FieldReader<u32>;
#[doc = "Field `RXSOPX1` writer - RXSOPX1"]
pub type RXSOPX1_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - RXSOPX1"]
    #[inline(always)]
    pub fn rxsopx1(&self) -> RXSOPX1_R {
        RXSOPX1_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - RXSOPX1"]
    #[inline(always)]
    #[must_use]
    pub fn rxsopx1(&mut self) -> RXSOPX1_W<RX_ORDEXT1rs> {
        RXSOPX1_W::new(self, 0)
    }
}
#[doc = "UCPD Rx Ordered Set Extension Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ordext1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ordext1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_ORDEXT1rs;
impl crate::RegisterSpec for RX_ORDEXT1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ordext1::R`](R) reader structure"]
impl crate::Readable for RX_ORDEXT1rs {}
#[doc = "`write(|w| ..)` method takes [`rx_ordext1::W`](W) writer structure"]
impl crate::Writable for RX_ORDEXT1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_ORDEXT1 to value 0"]
impl crate::Resettable for RX_ORDEXT1rs {
    const RESET_VALUE: u32 = 0;
}
