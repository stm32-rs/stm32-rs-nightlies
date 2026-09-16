///Register `MTLTxQ1SSCR` reader
pub type R = crate::R<MTLTX_Q1SSCRrs>;
///Register `MTLTxQ1SSCR` writer
pub type W = crate::W<MTLTX_Q1SSCRrs>;
///Field `SSC` reader - SSC
pub type SSC_R = crate::FieldReader<u16>;
///Field `SSC` writer - SSC
pub type SSC_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 0:13 - SSC
    #[inline(always)]
    pub fn ssc(&self) -> SSC_R {
        SSC_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTxQ1SSCR")
            .field("ssc", &self.ssc())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - SSC
    #[inline(always)]
    pub fn ssc(&mut self) -> SSC_W<'_, MTLTX_Q1SSCRrs> {
        SSC_W::new(self, 0)
    }
}
/**The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue.

You can [`read`](crate::Reg::read) this register and get [`mtltx_q1sscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_q1sscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:MTLTxQ1SSCR)*/
pub struct MTLTX_Q1SSCRrs;
impl crate::RegisterSpec for MTLTX_Q1SSCRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltx_q1sscr::R`](R) reader structure
impl crate::Readable for MTLTX_Q1SSCRrs {}
///`write(|w| ..)` method takes [`mtltx_q1sscr::W`](W) writer structure
impl crate::Writable for MTLTX_Q1SSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTxQ1SSCR to value 0
impl crate::Resettable for MTLTX_Q1SSCRrs {}
