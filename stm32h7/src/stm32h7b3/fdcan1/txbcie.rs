#[doc = "Register `TXBCIE` reader"]
pub type R = crate::R<TXBCIErs>;
#[doc = "Register `TXBCIE` writer"]
pub type W = crate::W<TXBCIErs>;
#[doc = "Field `CF` reader - Cancellation Finished Interrupt Enable"]
pub type CF_R = crate::FieldReader<u32>;
#[doc = "Field `CF` writer - Cancellation Finished Interrupt Enable"]
pub type CF_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf(&mut self) -> CF_W<TXBCIErs> {
        CF_W::new(self, 0)
    }
}
#[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbcie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBCIErs;
impl crate::RegisterSpec for TXBCIErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcie::R`](R) reader structure"]
impl crate::Readable for TXBCIErs {}
#[doc = "`write(|w| ..)` method takes [`txbcie::W`](W) writer structure"]
impl crate::Writable for TXBCIErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBCIE to value 0"]
impl crate::Resettable for TXBCIErs {
    const RESET_VALUE: u32 = 0;
}
