#[doc = "Register `UCPD_TXDR` reader"]
pub type R = crate::R<UCPD_TXDRrs>;
#[doc = "Register `UCPD_TXDR` writer"]
pub type W = crate::W<UCPD_TXDRrs>;
#[doc = "Field `TXDATA` reader - Data byte to transmit"]
pub type TXDATA_R = crate::FieldReader;
#[doc = "Field `TXDATA` writer - Data byte to transmit"]
pub type TXDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data byte to transmit"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte to transmit"]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TXDATA_W<UCPD_TXDRrs> {
        TXDATA_W::new(self, 0)
    }
}
#[doc = "UCPD Tx data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucpd_txdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucpd_txdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCPD_TXDRrs;
impl crate::RegisterSpec for UCPD_TXDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ucpd_txdr::R`](R) reader structure"]
impl crate::Readable for UCPD_TXDRrs {}
#[doc = "`write(|w| ..)` method takes [`ucpd_txdr::W`](W) writer structure"]
impl crate::Writable for UCPD_TXDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UCPD_TXDR to value 0"]
impl crate::Resettable for UCPD_TXDRrs {
    const RESET_VALUE: u32 = 0;
}
