#[doc = "Register `LPDMA_C2SAR` reader"]
pub type R = crate::R<LPDMA_C2SARrs>;
#[doc = "Register `LPDMA_C2SAR` writer"]
pub type W = crate::W<LPDMA_C2SARrs>;
#[doc = "Field `SA` reader - source address This field is the pointer to the address from which the next data is read. During the channel activity, depending on the source addressing mode (LPDMA_CxTR1.SINC), this field is either kept fixed or incremented by the data width (LPDMA_CxTR1.SDW_LOG2\\[1:0\\]) after each single source data, reflecting the next address from which data is read. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by LPDMA from the memory, provided the LLI is set with LPDMA_CxLLR.USA = 1. Note: A source address must be aligned with the programmed data width of a source single (SA\\[32:0\\]
versus LPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
pub type SA_R = crate::FieldReader<u32>;
#[doc = "Field `SA` writer - source address This field is the pointer to the address from which the next data is read. During the channel activity, depending on the source addressing mode (LPDMA_CxTR1.SINC), this field is either kept fixed or incremented by the data width (LPDMA_CxTR1.SDW_LOG2\\[1:0\\]) after each single source data, reflecting the next address from which data is read. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by LPDMA from the memory, provided the LLI is set with LPDMA_CxLLR.USA = 1. Note: A source address must be aligned with the programmed data width of a source single (SA\\[32:0\\]
versus LPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
pub type SA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - source address This field is the pointer to the address from which the next data is read. During the channel activity, depending on the source addressing mode (LPDMA_CxTR1.SINC), this field is either kept fixed or incremented by the data width (LPDMA_CxTR1.SDW_LOG2\\[1:0\\]) after each single source data, reflecting the next address from which data is read. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by LPDMA from the memory, provided the LLI is set with LPDMA_CxLLR.USA = 1. Note: A source address must be aligned with the programmed data width of a source single (SA\\[32:0\\]
versus LPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - source address This field is the pointer to the address from which the next data is read. During the channel activity, depending on the source addressing mode (LPDMA_CxTR1.SINC), this field is either kept fixed or incremented by the data width (LPDMA_CxTR1.SDW_LOG2\\[1:0\\]) after each single source data, reflecting the next address from which data is read. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by LPDMA from the memory, provided the LLI is set with LPDMA_CxLLR.USA = 1. Note: A source address must be aligned with the programmed data width of a source single (SA\\[32:0\\]
versus LPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<LPDMA_C2SARrs> {
        SA_W::new(self, 0)
    }
}
#[doc = "LPDMA channel 2 source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c2sar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c2sar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPDMA_C2SARrs;
impl crate::RegisterSpec for LPDMA_C2SARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpdma_c2sar::R`](R) reader structure"]
impl crate::Readable for LPDMA_C2SARrs {}
#[doc = "`write(|w| ..)` method takes [`lpdma_c2sar::W`](W) writer structure"]
impl crate::Writable for LPDMA_C2SARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPDMA_C2SAR to value 0"]
impl crate::Resettable for LPDMA_C2SARrs {
    const RESET_VALUE: u32 = 0;
}
