///Register `MTLTxQ1QWR` reader
pub type R = crate::R<MTLTX_Q1QWRrs>;
///Register `MTLTxQ1QWR` writer
pub type W = crate::W<MTLTX_Q1QWRrs>;
///Field `ISCQW` reader - ISCQW
pub type ISCQW_R = crate::FieldReader<u32>;
///Field `ISCQW` writer - ISCQW
pub type ISCQW_W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    ///Bits 0:20 - ISCQW
    #[inline(always)]
    pub fn iscqw(&self) -> ISCQW_R {
        ISCQW_R::new(self.bits & 0x001f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTxQ1QWR")
            .field("iscqw", &self.iscqw())
            .finish()
    }
}
impl W {
    ///Bits 0:20 - ISCQW
    #[inline(always)]
    pub fn iscqw(&mut self) -> ISCQW_W<'_, MTLTX_Q1QWRrs> {
        ISCQW_W::new(self, 0)
    }
}
/**This register provides the average traffic transmitted on queue 1.

You can [`read`](crate::Reg::read) this register and get [`mtltx_q1qwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_q1qwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLTxQ1QWR)*/
pub struct MTLTX_Q1QWRrs;
impl crate::RegisterSpec for MTLTX_Q1QWRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltx_q1qwr::R`](R) reader structure
impl crate::Readable for MTLTX_Q1QWRrs {}
///`write(|w| ..)` method takes [`mtltx_q1qwr::W`](W) writer structure
impl crate::Writable for MTLTX_Q1QWRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTxQ1QWR to value 0
impl crate::Resettable for MTLTX_Q1QWRrs {}
