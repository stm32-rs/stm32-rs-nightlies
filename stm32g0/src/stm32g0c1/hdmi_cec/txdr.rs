#[doc = "Register `TXDR` writer"]
pub type W = crate::W<TXDRrs>;
#[doc = "Field `TXD` writer - Tx Data register. TXD is a write-only register containing the data byte to be transmitted. Note: TXD must be written when TXSTART=1"]
pub type TXD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Tx Data register. TXD is a write-only register containing the data byte to be transmitted. Note: TXD must be written when TXSTART=1"]
    #[inline(always)]
    #[must_use]
    pub fn txd(&mut self) -> TXD_W<TXDRrs> {
        TXD_W::new(self, 0)
    }
}
#[doc = "CEC Tx data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDRrs;
impl crate::RegisterSpec for TXDRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txdr::W`](W) writer structure"]
impl crate::Writable for TXDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDR to value 0"]
impl crate::Resettable for TXDRrs {
    const RESET_VALUE: u32 = 0;
}
