///Register `MTLTxQ1HCR` reader
pub type R = crate::R<MTLTX_Q1HCRrs>;
///Register `MTLTxQ1HCR` writer
pub type W = crate::W<MTLTX_Q1HCRrs>;
///Field `HC` reader - HC
pub type HC_R = crate::FieldReader<u32>;
///Field `HC` writer - HC
pub type HC_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bits 0:28 - HC
    #[inline(always)]
    pub fn hc(&self) -> HC_R {
        HC_R::new(self.bits & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTxQ1HCR")
            .field("hc", &self.hc())
            .finish()
    }
}
impl W {
    ///Bits 0:28 - HC
    #[inline(always)]
    pub fn hc(&mut self) -> HC_W<'_, MTLTX_Q1HCRrs> {
        HC_W::new(self, 0)
    }
}
/**The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue.

You can [`read`](crate::Reg::read) this register and get [`mtltx_q1hcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_q1hcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:MTLTxQ1HCR)*/
pub struct MTLTX_Q1HCRrs;
impl crate::RegisterSpec for MTLTX_Q1HCRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltx_q1hcr::R`](R) reader structure
impl crate::Readable for MTLTX_Q1HCRrs {}
///`write(|w| ..)` method takes [`mtltx_q1hcr::W`](W) writer structure
impl crate::Writable for MTLTX_Q1HCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTxQ1HCR to value 0
impl crate::Resettable for MTLTX_Q1HCRrs {}
