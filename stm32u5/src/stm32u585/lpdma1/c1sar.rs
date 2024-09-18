///Register `C1SAR` reader
pub type R = crate::R<C1SARrs>;
///Register `C1SAR` writer
pub type W = crate::W<C1SARrs>;
/**Field `SA` reader - source address This field is the pointer to the address from which the next data is read. During the channel activity, depending on the source addressing mode (LPDMA_CxTR1.SINC), this field is either kept fixed or incremented by the data width (LPDMA_CxTR1.SDW_LOG2\[1:0\]) after each single source data, reflecting the next address from which data is read. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by LPDMA from the memory, provided the LLI is set with LPDMA_CxLLR.USA = 1. Note: A source address must be aligned with the programmed data width of a source single (SA\[32:0\]
versus LPDMA_CxTR1.SDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.*/
pub type SA_R = crate::FieldReader<u32>;
/**Field `SA` writer - source address This field is the pointer to the address from which the next data is read. During the channel activity, depending on the source addressing mode (LPDMA_CxTR1.SINC), this field is either kept fixed or incremented by the data width (LPDMA_CxTR1.SDW_LOG2\[1:0\]) after each single source data, reflecting the next address from which data is read. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by LPDMA from the memory, provided the LLI is set with LPDMA_CxLLR.USA = 1. Note: A source address must be aligned with the programmed data width of a source single (SA\[32:0\]
versus LPDMA_CxTR1.SDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.*/
pub type SA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    /**Bits 0:31 - source address This field is the pointer to the address from which the next data is read. During the channel activity, depending on the source addressing mode (LPDMA_CxTR1.SINC), this field is either kept fixed or incremented by the data width (LPDMA_CxTR1.SDW_LOG2\[1:0\]) after each single source data, reflecting the next address from which data is read. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by LPDMA from the memory, provided the LLI is set with LPDMA_CxLLR.USA = 1. Note: A source address must be aligned with the programmed data width of a source single (SA\[32:0\]
    versus LPDMA_CxTR1.SDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.*/
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1SAR").field("sa", &self.sa()).finish()
    }
}
impl W {
    /**Bits 0:31 - source address This field is the pointer to the address from which the next data is read. During the channel activity, depending on the source addressing mode (LPDMA_CxTR1.SINC), this field is either kept fixed or incremented by the data width (LPDMA_CxTR1.SDW_LOG2\[1:0\]) after each single source data, reflecting the next address from which data is read. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by LPDMA from the memory, provided the LLI is set with LPDMA_CxLLR.USA = 1. Note: A source address must be aligned with the programmed data width of a source single (SA\[32:0\]
    versus LPDMA_CxTR1.SDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.*/
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<C1SARrs> {
        SA_W::new(self, 0)
    }
}
/**LPDMA channel 1 source address register

You can [`read`](crate::Reg::read) this register and get [`c1sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#LPDMA1:C1SAR)*/
pub struct C1SARrs;
impl crate::RegisterSpec for C1SARrs {
    type Ux = u32;
}
///`read()` method returns [`c1sar::R`](R) reader structure
impl crate::Readable for C1SARrs {}
///`write(|w| ..)` method takes [`c1sar::W`](W) writer structure
impl crate::Writable for C1SARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C1SAR to value 0
impl crate::Resettable for C1SARrs {
    const RESET_VALUE: u32 = 0;
}
