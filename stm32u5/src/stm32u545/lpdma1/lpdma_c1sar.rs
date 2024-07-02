///Register `LPDMA_C1SAR` reader
pub type R = crate::R<LPDMA_C1SARrs>;
///Register `LPDMA_C1SAR` writer
pub type W = crate::W<LPDMA_C1SARrs>;
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
        f.debug_struct("LPDMA_C1SAR")
            .field("sa", &self.sa())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - source address
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<LPDMA_C1SARrs> {
        SA_W::new(self, 0)
    }
}
/**LPDMA channel x source address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c1sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c1sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#LPDMA1:LPDMA_C1SAR)*/
pub struct LPDMA_C1SARrs;
impl crate::RegisterSpec for LPDMA_C1SARrs {
    type Ux = u32;
}
///`read()` method returns [`lpdma_c1sar::R`](R) reader structure
impl crate::Readable for LPDMA_C1SARrs {}
///`write(|w| ..)` method takes [`lpdma_c1sar::W`](W) writer structure
impl crate::Writable for LPDMA_C1SARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPDMA_C1SAR to value 0
impl crate::Resettable for LPDMA_C1SARrs {
    const RESET_VALUE: u32 = 0;
}
