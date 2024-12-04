///Register `C3DAR` reader
pub type R = crate::R<C3DARrs>;
///Register `C3DAR` writer
pub type W = crate::W<C3DARrs>;
/**Field `DA` reader - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (GPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (GPDMA_CxTR1.DDW_LOG2\[21:0\]) after each burst destination data, reflecting the next address from which data is written. During the channel activity, this address is updated after each completed destination burst, consequently to: the programmed destination burst; either in fixed addressing mode or in contiguous-data incremented mode. If contiguously incremented (GPDMA_CxTR1.DINC = 1), then the additional address offset value is the programmed burst size, as defined by GPDMA_CxTR1.DBL_1\[5:0\]
and GPDMA_CxTR1.DDW_LOG2\[1:0\]
the additional destination incremented/decremented offset value as programmed by GPDMA_CxBR1.DDEC and GPDMA_CxTR3.DAO\[12:0\]
once/if completed destination block transfer, for a channel x with 2D addressing capability (x = 12 to 15), the additional block repeat destination incremented/decremented offset value as programmed by GPDMA_CxBR1.BRDDEC and GPDMA_CxBR2.BRDAO\[15:0\]
In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by the GPDMA from the memory, provided the LLI is set with GPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination burst (DA\[2:0\]
versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.*/
pub type DA_R = crate::FieldReader<u32>;
/**Field `DA` writer - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (GPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (GPDMA_CxTR1.DDW_LOG2\[21:0\]) after each burst destination data, reflecting the next address from which data is written. During the channel activity, this address is updated after each completed destination burst, consequently to: the programmed destination burst; either in fixed addressing mode or in contiguous-data incremented mode. If contiguously incremented (GPDMA_CxTR1.DINC = 1), then the additional address offset value is the programmed burst size, as defined by GPDMA_CxTR1.DBL_1\[5:0\]
and GPDMA_CxTR1.DDW_LOG2\[1:0\]
the additional destination incremented/decremented offset value as programmed by GPDMA_CxBR1.DDEC and GPDMA_CxTR3.DAO\[12:0\]
once/if completed destination block transfer, for a channel x with 2D addressing capability (x = 12 to 15), the additional block repeat destination incremented/decremented offset value as programmed by GPDMA_CxBR1.BRDDEC and GPDMA_CxBR2.BRDAO\[15:0\]
In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by the GPDMA from the memory, provided the LLI is set with GPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination burst (DA\[2:0\]
versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.*/
pub type DA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    /**Bits 0:31 - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (GPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (GPDMA_CxTR1.DDW_LOG2\[21:0\]) after each burst destination data, reflecting the next address from which data is written. During the channel activity, this address is updated after each completed destination burst, consequently to: the programmed destination burst; either in fixed addressing mode or in contiguous-data incremented mode. If contiguously incremented (GPDMA_CxTR1.DINC = 1), then the additional address offset value is the programmed burst size, as defined by GPDMA_CxTR1.DBL_1\[5:0\]
    and GPDMA_CxTR1.DDW_LOG2\[1:0\]
    the additional destination incremented/decremented offset value as programmed by GPDMA_CxBR1.DDEC and GPDMA_CxTR3.DAO\[12:0\]
    once/if completed destination block transfer, for a channel x with 2D addressing capability (x = 12 to 15), the additional block repeat destination incremented/decremented offset value as programmed by GPDMA_CxBR1.BRDDEC and GPDMA_CxBR2.BRDAO\[15:0\]
    In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by the GPDMA from the memory, provided the LLI is set with GPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination burst (DA\[2:0\]
    versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.*/
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C3DAR").field("da", &self.da()).finish()
    }
}
impl W {
    /**Bits 0:31 - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (GPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (GPDMA_CxTR1.DDW_LOG2\[21:0\]) after each burst destination data, reflecting the next address from which data is written. During the channel activity, this address is updated after each completed destination burst, consequently to: the programmed destination burst; either in fixed addressing mode or in contiguous-data incremented mode. If contiguously incremented (GPDMA_CxTR1.DINC = 1), then the additional address offset value is the programmed burst size, as defined by GPDMA_CxTR1.DBL_1\[5:0\]
    and GPDMA_CxTR1.DDW_LOG2\[1:0\]
    the additional destination incremented/decremented offset value as programmed by GPDMA_CxBR1.DDEC and GPDMA_CxTR3.DAO\[12:0\]
    once/if completed destination block transfer, for a channel x with 2D addressing capability (x = 12 to 15), the additional block repeat destination incremented/decremented offset value as programmed by GPDMA_CxBR1.BRDDEC and GPDMA_CxBR2.BRDAO\[15:0\]
    In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by the GPDMA from the memory, provided the LLI is set with GPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination burst (DA\[2:0\]
    versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.*/
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<C3DARrs> {
        DA_W::new(self, 0)
    }
}
/**GPDMA channel 3 destination address register

You can [`read`](crate::Reg::read) this register and get [`c3dar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3dar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#GPDMA1:C3DAR)*/
pub struct C3DARrs;
impl crate::RegisterSpec for C3DARrs {
    type Ux = u32;
}
///`read()` method returns [`c3dar::R`](R) reader structure
impl crate::Readable for C3DARrs {}
///`write(|w| ..)` method takes [`c3dar::W`](W) writer structure
impl crate::Writable for C3DARrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C3DAR to value 0
impl crate::Resettable for C3DARrs {
    const RESET_VALUE: u32 = 0;
}
