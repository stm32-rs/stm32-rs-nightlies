#[doc = "Register `RX_ORDEXT2` reader"]
pub type R = crate::R<RX_ORDEXT2rs>;
#[doc = "Register `RX_ORDEXT2` writer"]
pub type W = crate::W<RX_ORDEXT2rs>;
#[doc = "Field `RXSOPX2` reader - RXSOPX2"]
pub type RXSOPX2_R = crate::FieldReader<u32>;
#[doc = "Field `RXSOPX2` writer - RXSOPX2"]
pub type RXSOPX2_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - RXSOPX2"]
    #[inline(always)]
    pub fn rxsopx2(&self) -> RXSOPX2_R {
        RXSOPX2_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - RXSOPX2"]
    #[inline(always)]
    #[must_use]
    pub fn rxsopx2(&mut self) -> RXSOPX2_W<RX_ORDEXT2rs> {
        RXSOPX2_W::new(self, 0)
    }
}
#[doc = "UCPD Rx Ordered Set Extension Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ordext2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ordext2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_ORDEXT2rs;
impl crate::RegisterSpec for RX_ORDEXT2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ordext2::R`](R) reader structure"]
impl crate::Readable for RX_ORDEXT2rs {}
#[doc = "`write(|w| ..)` method takes [`rx_ordext2::W`](W) writer structure"]
impl crate::Writable for RX_ORDEXT2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_ORDEXT2 to value 0"]
impl crate::Resettable for RX_ORDEXT2rs {
    const RESET_VALUE: u32 = 0;
}
