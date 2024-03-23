#[doc = "Register `LPDMA_C1DAR` reader"]
pub type R = crate::R<LPDMA_C1DARrs>;
#[doc = "Register `LPDMA_C1DAR` writer"]
pub type W = crate::W<LPDMA_C1DARrs>;
#[doc = "Field `DA` reader - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (LPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (LPDMA_CxTR1.DDW_LOG2\\[21:0\\]) after each single destination data, reflecting the next address from which data is written. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by DMA from the memory, provided the LLI is set with LPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination single (DA\\[2:0\\]
versus LPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
pub type DA_R = crate::FieldReader<u32>;
#[doc = "Field `DA` writer - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (LPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (LPDMA_CxTR1.DDW_LOG2\\[21:0\\]) after each single destination data, reflecting the next address from which data is written. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by DMA from the memory, provided the LLI is set with LPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination single (DA\\[2:0\\]
versus LPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
pub type DA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (LPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (LPDMA_CxTR1.DDW_LOG2\\[21:0\\]) after each single destination data, reflecting the next address from which data is written. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by DMA from the memory, provided the LLI is set with LPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination single (DA\\[2:0\\]
versus LPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (LPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (LPDMA_CxTR1.DDW_LOG2\\[21:0\\]) after each single destination data, reflecting the next address from which data is written. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by DMA from the memory, provided the LLI is set with LPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination single (DA\\[2:0\\]
versus LPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<LPDMA_C1DARrs> {
        DA_W::new(self, 0)
    }
}
#[doc = "LPDMA channel 1 destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c1dar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c1dar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPDMA_C1DARrs;
impl crate::RegisterSpec for LPDMA_C1DARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpdma_c1dar::R`](R) reader structure"]
impl crate::Readable for LPDMA_C1DARrs {}
#[doc = "`write(|w| ..)` method takes [`lpdma_c1dar::W`](W) writer structure"]
impl crate::Writable for LPDMA_C1DARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPDMA_C1DAR to value 0"]
impl crate::Resettable for LPDMA_C1DARrs {
    const RESET_VALUE: u32 = 0;
}
