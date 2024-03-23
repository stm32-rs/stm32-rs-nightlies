#[doc = "Register `FDCAN_TXBAR` reader"]
pub type R = crate::R<FDCAN_TXBARrs>;
#[doc = "Register `FDCAN_TXBAR` writer"]
pub type W = crate::W<FDCAN_TXBARrs>;
#[doc = "Field `AR` reader - Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
pub type AR_R = crate::FieldReader;
#[doc = "Field `AR` writer - Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
pub type AR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Add request Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request bit; writing a 0 has no impact. This enables the Host to set transmission requests for multiple Tx buffers with one write to TXBAR. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed."]
    #[inline(always)]
    #[must_use]
    pub fn ar(&mut self) -> AR_W<FDCAN_TXBARrs> {
        AR_W::new(self, 0)
    }
}
#[doc = "FDCAN Tx buffer add request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXBARrs;
impl crate::RegisterSpec for FDCAN_TXBARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbar::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXBARrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbar::W`](W) writer structure"]
impl crate::Writable for FDCAN_TXBARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBAR to value 0"]
impl crate::Resettable for FDCAN_TXBARrs {
    const RESET_VALUE: u32 = 0;
}
