///Register `C2DAR` reader
pub type R = crate::R<C2DARrs>;
///Register `C2DAR` writer
pub type W = crate::W<C2DARrs>;
/**Field `DA` reader - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (LPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (LPDMA_CxTR1.DDW_LOG2\[21:0\]) after each single destination data, reflecting the next address from which data is written. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by DMA from the memory, provided the LLI is set with LPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination single (DA\[2:0\]
versus LPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.*/
pub type DA_R = crate::FieldReader<u32>;
/**Field `DA` writer - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (LPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (LPDMA_CxTR1.DDW_LOG2\[21:0\]) after each single destination data, reflecting the next address from which data is written. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by DMA from the memory, provided the LLI is set with LPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination single (DA\[2:0\]
versus LPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.*/
pub type DA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    /**Bits 0:31 - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (LPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (LPDMA_CxTR1.DDW_LOG2\[21:0\]) after each single destination data, reflecting the next address from which data is written. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by DMA from the memory, provided the LLI is set with LPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination single (DA\[2:0\]
    versus LPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.*/
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2DAR").field("da", &self.da()).finish()
    }
}
impl W {
    /**Bits 0:31 - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (LPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (LPDMA_CxTR1.DDW_LOG2\[21:0\]) after each single destination data, reflecting the next address from which data is written. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by DMA from the memory, provided the LLI is set with LPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination single (DA\[2:0\]
    versus LPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.*/
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<C2DARrs> {
        DA_W::new(self, 0)
    }
}
/**LPDMA channel 2 destination address register

You can [`read`](crate::Reg::read) this register and get [`c2dar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2dar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#LPDMA1:C2DAR)*/
pub struct C2DARrs;
impl crate::RegisterSpec for C2DARrs {
    type Ux = u32;
}
///`read()` method returns [`c2dar::R`](R) reader structure
impl crate::Readable for C2DARrs {}
///`write(|w| ..)` method takes [`c2dar::W`](W) writer structure
impl crate::Writable for C2DARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C2DAR to value 0
impl crate::Resettable for C2DARrs {
    const RESET_VALUE: u32 = 0;
}
