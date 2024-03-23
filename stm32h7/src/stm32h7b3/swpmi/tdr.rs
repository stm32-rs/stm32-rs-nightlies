#[doc = "Register `TDR` writer"]
pub type W = crate::W<TDRrs>;
#[doc = "Field `TD` writer - Transmit data"]
pub type TD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Transmit data"]
    #[inline(always)]
    #[must_use]
    pub fn td(&mut self) -> TD_W<TDRrs> {
        TD_W::new(self, 0)
    }
}
#[doc = "SWPMI Transmit data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDRrs;
impl crate::RegisterSpec for TDRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tdr::W`](W) writer structure"]
impl crate::Writable for TDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDR to value 0"]
impl crate::Resettable for TDRrs {
    const RESET_VALUE: u32 = 0;
}
