///Register `GPDMA_C12SAR` reader
pub type R = crate::R<GPDMA_C12SARrs>;
///Register `GPDMA_C12SAR` writer
pub type W = crate::W<GPDMA_C12SARrs>;
///Field `SA` reader - source address
pub type SA_R = crate::FieldReader<u32>;
///Field `SA` writer - source address
pub type SA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - source address
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPDMA_C12SAR")
            .field("sa", &self.sa())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - source address
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<GPDMA_C12SARrs> {
        SA_W::new(self, 0)
    }
}
/**GPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`gpdma_c12sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdma_c12sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GPDMA1:GPDMA_C12SAR)*/
pub struct GPDMA_C12SARrs;
impl crate::RegisterSpec for GPDMA_C12SARrs {
    type Ux = u32;
}
///`read()` method returns [`gpdma_c12sar::R`](R) reader structure
impl crate::Readable for GPDMA_C12SARrs {}
///`write(|w| ..)` method takes [`gpdma_c12sar::W`](W) writer structure
impl crate::Writable for GPDMA_C12SARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GPDMA_C12SAR to value 0
impl crate::Resettable for GPDMA_C12SARrs {
    const RESET_VALUE: u32 = 0;
}
