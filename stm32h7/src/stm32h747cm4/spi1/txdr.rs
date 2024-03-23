#[doc = "Register `TXDR` writer"]
pub type W = crate::W<TXDRrs>;
#[doc = "Field `TXDR` writer - Transmit data register"]
pub type TXDR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    #[must_use]
    pub fn txdr(&mut self) -> TXDR_W<TXDRrs> {
        TXDR_W::new(self, 0)
    }
}
#[doc = "Transmit Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDRrs;
impl crate::RegisterSpec for TXDRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txdr::W`](W) writer structure"]
impl crate::Writable for TXDRrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDR to value 0"]
impl crate::Resettable for TXDRrs {
    const RESET_VALUE: u32 = 0;
}
