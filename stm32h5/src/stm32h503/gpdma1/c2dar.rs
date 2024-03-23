#[doc = "Register `C2DAR` reader"]
pub type R = crate::R<C2DARrs>;
#[doc = "Register `C2DAR` writer"]
pub type W = crate::W<C2DARrs>;
#[doc = "Field `DA` reader - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (GPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (GPDMA_CxTR1.DDW_LOG2\\[21:0\\]) after each burst destination data, reflecting the next address from which data is written. During the channel activity, this address is updated after each completed destination burst, consequently to: the programmed destination burst; either in fixed addressing mode or in contiguous-data incremented mode. If contiguously incremented (GPDMA_CxTR1.DINC = 1), then the additional address offset value is the programmed burst size, as defined by GPDMA_CxTR1.DBL_1\\[5:0\\]
and GPDMA_CxTR1.DDW_LOG2\\[1:0\\]
the additional destination incremented/decremented offset value as programmed by GPDMA_CxBR1.DDEC and GPDMA_CxTR3.DAO\\[12:0\\]. once/if completed destination block transfer, for a channel x with 2D addressing capability (x = 12 to 15), the additional block repeat destination incremented/decremented offset value as programmed by GPDMA_CxBR1.BRDDEC and GPDMA_CxBR2.BRDAO\\[15:0\\]
In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by the GPDMA from the memory, provided the LLI is set with GPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination burst (DA\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
pub type DA_R = crate::FieldReader<u32>;
#[doc = "Field `DA` writer - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (GPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (GPDMA_CxTR1.DDW_LOG2\\[21:0\\]) after each burst destination data, reflecting the next address from which data is written. During the channel activity, this address is updated after each completed destination burst, consequently to: the programmed destination burst; either in fixed addressing mode or in contiguous-data incremented mode. If contiguously incremented (GPDMA_CxTR1.DINC = 1), then the additional address offset value is the programmed burst size, as defined by GPDMA_CxTR1.DBL_1\\[5:0\\]
and GPDMA_CxTR1.DDW_LOG2\\[1:0\\]
the additional destination incremented/decremented offset value as programmed by GPDMA_CxBR1.DDEC and GPDMA_CxTR3.DAO\\[12:0\\]. once/if completed destination block transfer, for a channel x with 2D addressing capability (x = 12 to 15), the additional block repeat destination incremented/decremented offset value as programmed by GPDMA_CxBR1.BRDDEC and GPDMA_CxBR2.BRDAO\\[15:0\\]
In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by the GPDMA from the memory, provided the LLI is set with GPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination burst (DA\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
pub type DA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (GPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (GPDMA_CxTR1.DDW_LOG2\\[21:0\\]) after each burst destination data, reflecting the next address from which data is written. During the channel activity, this address is updated after each completed destination burst, consequently to: the programmed destination burst; either in fixed addressing mode or in contiguous-data incremented mode. If contiguously incremented (GPDMA_CxTR1.DINC = 1), then the additional address offset value is the programmed burst size, as defined by GPDMA_CxTR1.DBL_1\\[5:0\\]
and GPDMA_CxTR1.DDW_LOG2\\[1:0\\]
the additional destination incremented/decremented offset value as programmed by GPDMA_CxBR1.DDEC and GPDMA_CxTR3.DAO\\[12:0\\]. once/if completed destination block transfer, for a channel x with 2D addressing capability (x = 12 to 15), the additional block repeat destination incremented/decremented offset value as programmed by GPDMA_CxBR1.BRDDEC and GPDMA_CxBR2.BRDAO\\[15:0\\]
In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by the GPDMA from the memory, provided the LLI is set with GPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination burst (DA\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (GPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (GPDMA_CxTR1.DDW_LOG2\\[21:0\\]) after each burst destination data, reflecting the next address from which data is written. During the channel activity, this address is updated after each completed destination burst, consequently to: the programmed destination burst; either in fixed addressing mode or in contiguous-data incremented mode. If contiguously incremented (GPDMA_CxTR1.DINC = 1), then the additional address offset value is the programmed burst size, as defined by GPDMA_CxTR1.DBL_1\\[5:0\\]
and GPDMA_CxTR1.DDW_LOG2\\[1:0\\]
the additional destination incremented/decremented offset value as programmed by GPDMA_CxBR1.DDEC and GPDMA_CxTR3.DAO\\[12:0\\]. once/if completed destination block transfer, for a channel x with 2D addressing capability (x = 12 to 15), the additional block repeat destination incremented/decremented offset value as programmed by GPDMA_CxBR1.BRDDEC and GPDMA_CxBR2.BRDAO\\[15:0\\]
In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by the GPDMA from the memory, provided the LLI is set with GPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination burst (DA\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<C2DARrs> {
        DA_W::new(self, 0)
    }
}
#[doc = "GPDMA channel 2 destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2dar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2dar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2DARrs;
impl crate::RegisterSpec for C2DARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2dar::R`](R) reader structure"]
impl crate::Readable for C2DARrs {}
#[doc = "`write(|w| ..)` method takes [`c2dar::W`](W) writer structure"]
impl crate::Writable for C2DARrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2DAR to value 0"]
impl crate::Resettable for C2DARrs {
    const RESET_VALUE: u32 = 0;
}
