///Register `MTLTxQ1LCR` reader
pub type R = crate::R<MTLTX_Q1LCRrs>;
///Register `MTLTxQ1LCR` writer
pub type W = crate::W<MTLTX_Q1LCRrs>;
///Field `LC` reader - LC
pub type LC_R = crate::FieldReader<u32>;
///Field `LC` writer - LC
pub type LC_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bits 0:28 - LC
    #[inline(always)]
    pub fn lc(&self) -> LC_R {
        LC_R::new(self.bits & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTxQ1LCR")
            .field("lc", &self.lc())
            .finish()
    }
}
impl W {
    ///Bits 0:28 - LC
    #[inline(always)]
    pub fn lc(&mut self) -> LC_W<'_, MTLTX_Q1LCRrs> {
        LC_W::new(self, 0)
    }
}
/**The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue.

You can [`read`](crate::Reg::read) this register and get [`mtltx_q1lcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_q1lcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:MTLTxQ1LCR)*/
pub struct MTLTX_Q1LCRrs;
impl crate::RegisterSpec for MTLTX_Q1LCRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltx_q1lcr::R`](R) reader structure
impl crate::Readable for MTLTX_Q1LCRrs {}
///`write(|w| ..)` method takes [`mtltx_q1lcr::W`](W) writer structure
impl crate::Writable for MTLTX_Q1LCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTxQ1LCR to value 0
impl crate::Resettable for MTLTX_Q1LCRrs {}
