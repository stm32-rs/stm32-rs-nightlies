#[doc = "Register `C1BR1` reader"]
pub type R = crate::R<C1BR1rs>;
#[doc = "Register `C1BR1` writer"]
pub type W = crate::W<C1BR1rs>;
#[doc = "Field `BNDT` reader - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\\[15:0\\]
is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\\[15:0\\]
= 0): - if GPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if GPDMA_CxLLR.UB1 = 0 and if there is at least one non null Uxx update bit, this field is internally restored to the programmed value. - if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\\[15:0\\]
= 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. When configured in packing mode (GPDMA_CxTR1.PAM\\[1\\]
= 1 and destination data width different from source data width), a non-null source block size must be a multiple of the destination data width (BNDT\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued."]
pub type BNDT_R = crate::FieldReader<u16>;
#[doc = "Field `BNDT` writer - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\\[15:0\\]
is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\\[15:0\\]
= 0): - if GPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if GPDMA_CxLLR.UB1 = 0 and if there is at least one non null Uxx update bit, this field is internally restored to the programmed value. - if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\\[15:0\\]
= 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. When configured in packing mode (GPDMA_CxTR1.PAM\\[1\\]
= 1 and destination data width different from source data width), a non-null source block size must be a multiple of the destination data width (BNDT\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued."]
pub type BNDT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\\[15:0\\]
is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\\[15:0\\]
= 0): - if GPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if GPDMA_CxLLR.UB1 = 0 and if there is at least one non null Uxx update bit, this field is internally restored to the programmed value. - if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\\[15:0\\]
= 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. When configured in packing mode (GPDMA_CxTR1.PAM\\[1\\]
= 1 and destination data width different from source data width), a non-null source block size must be a multiple of the destination data width (BNDT\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\\[15:0\\]
is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\\[15:0\\]
= 0): - if GPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if GPDMA_CxLLR.UB1 = 0 and if there is at least one non null Uxx update bit, this field is internally restored to the programmed value. - if all GPDMA_CxLLR.Uxx = 0 and if GPDMA_CxLLR.LA\\[15:0\\]
= 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if GPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\\[2:0\\]
versus GPDMA_CxTR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. When configured in packing mode (GPDMA_CxTR1.PAM\\[1\\]
= 1 and destination data width different from source data width), a non-null source block size must be a multiple of the destination data width (BNDT\\[2:0\\]
versus GPDMA_CxTR1.DDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    #[must_use]
    pub fn bndt(&mut self) -> BNDT_W<C1BR1rs> {
        BNDT_W::new(self, 0)
    }
}
#[doc = "GPDMA channel 1 block register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1br1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1br1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1BR1rs;
impl crate::RegisterSpec for C1BR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1br1::R`](R) reader structure"]
impl crate::Readable for C1BR1rs {}
#[doc = "`write(|w| ..)` method takes [`c1br1::W`](W) writer structure"]
impl crate::Writable for C1BR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1BR1 to value 0"]
impl crate::Resettable for C1BR1rs {
    const RESET_VALUE: u32 = 0;
}
