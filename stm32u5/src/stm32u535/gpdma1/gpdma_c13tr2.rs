#[doc = "Register `GPDMA_C13TR2` reader"]
pub type R = crate::R<GPDMA_C13TR2rs>;
#[doc = "Register `GPDMA_C13TR2` writer"]
pub type W = crate::W<GPDMA_C13TR2rs>;
#[doc = "Field `REQSEL` reader - DMA hardware request selection If the channel x is activated (i.e. GPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else, the selected hardware request as per Table 12 is internally taken into account. Note: The user must not assign a same input hardware request (i.e. a same REQSEL\\[6:0\\]
value) to different active DMA channels (i.e. if GPDMA_CxCR.EN=1 and GPDMA_CxTR2.SWREQ=0 for the related x channels). In other words, DMA is not intended to hardware support the case of simultaneous enabled channels having been -incorrectly- configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
pub type REQSEL_R = crate::FieldReader;
#[doc = "Field `REQSEL` writer - DMA hardware request selection If the channel x is activated (i.e. GPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else, the selected hardware request as per Table 12 is internally taken into account. Note: The user must not assign a same input hardware request (i.e. a same REQSEL\\[6:0\\]
value) to different active DMA channels (i.e. if GPDMA_CxCR.EN=1 and GPDMA_CxTR2.SWREQ=0 for the related x channels). In other words, DMA is not intended to hardware support the case of simultaneous enabled channels having been -incorrectly- configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
pub type REQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SWREQ` reader - Software request When GPDMA_CxCR.EN is asserted, this field is internally taken into account: - 0: no software request. The selected hardware request REQSEL\\[6:0\\]
is taken into account. - 1: software request (for a memory-to-memory transfer). And the default selected hardware request as per REQSEL\\[6:0\\]
is ignored."]
pub type SWREQ_R = crate::BitReader;
#[doc = "Field `SWREQ` writer - Software request When GPDMA_CxCR.EN is asserted, this field is internally taken into account: - 0: no software request. The selected hardware request REQSEL\\[6:0\\]
is taken into account. - 1: software request (for a memory-to-memory transfer). And the default selected hardware request as per REQSEL\\[6:0\\]
is ignored."]
pub type SWREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DREQ` reader - Destination hardware request If the channel x is activated (i.e. GPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else: - 0: the selected hardware request is driven by a source peripheral (i.e. this request signal is taken into account by the DMA transfer scheduler over the source/read port) - 1: the selected hardware request is driven by a destination peripheral (.e. this request signal is taken into account by the DMA transfer scheduler over the destination/write port)"]
pub type DREQ_R = crate::BitReader;
#[doc = "Field `DREQ` writer - Destination hardware request If the channel x is activated (i.e. GPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else: - 0: the selected hardware request is driven by a source peripheral (i.e. this request signal is taken into account by the DMA transfer scheduler over the source/read port) - 1: the selected hardware request is driven by a destination peripheral (.e. this request signal is taken into account by the DMA transfer scheduler over the destination/write port)"]
pub type DREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREQ` reader - BREQ"]
pub type BREQ_R = crate::BitReader;
#[doc = "Field `BREQ` writer - BREQ"]
pub type BREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGM` reader - Trigger mode: enabled. Transferring a next LLIn+1 which updates the GPDMA_CxTR2 with a new value for any of TRIGSEL\\[5:0\\]
or TRIGPOL\\[1:0\\]
resets the monitoring, trashing the (possible) memorized hit of the formerly defined LLIn trigger. After that a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, this new second trigger hitn+2 is lost and not memorized. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\\[1:0\\]=11 and (SWREQ=1 or (SWREQ=0 and DREQ=0)), the shortened burst transfer (by single(s) or/and by burst(s) of lower length) is conditioned once by the trigger. Note: When the programmed destination burst is internally shortened by single(s) or/and by burst(s) of lower length (e.g. vs FIFO size, vs block size, 1kB/4kB boundary address crossing): if the trigger is conditioning the programmed destination burst (if TRIGM\\[1:0\\]=11 and SWREQ=0 and DREQ=1), this shortened destination burst transfer is conditioned once by the trigger."]
pub type TRIGM_R = crate::FieldReader;
#[doc = "Field `TRIGM` writer - Trigger mode: enabled. Transferring a next LLIn+1 which updates the GPDMA_CxTR2 with a new value for any of TRIGSEL\\[5:0\\]
or TRIGPOL\\[1:0\\]
resets the monitoring, trashing the (possible) memorized hit of the formerly defined LLIn trigger. After that a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, this new second trigger hitn+2 is lost and not memorized. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\\[1:0\\]=11 and (SWREQ=1 or (SWREQ=0 and DREQ=0)), the shortened burst transfer (by single(s) or/and by burst(s) of lower length) is conditioned once by the trigger. Note: When the programmed destination burst is internally shortened by single(s) or/and by burst(s) of lower length (e.g. vs FIFO size, vs block size, 1kB/4kB boundary address crossing): if the trigger is conditioning the programmed destination burst (if TRIGM\\[1:0\\]=11 and SWREQ=0 and DREQ=1), this shortened destination burst transfer is conditioned once by the trigger."]
pub type TRIGM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRIGSEL` reader - Trigger event input selection Note: Selects the trigger event input as per Table 13 of the DMA transfer, with an active trigger event if TRIGPOL\\[1:0\\]
!=00."]
pub type TRIGSEL_R = crate::FieldReader;
#[doc = "Field `TRIGSEL` writer - Trigger event input selection Note: Selects the trigger event input as per Table 13 of the DMA transfer, with an active trigger event if TRIGPOL\\[1:0\\]
!=00."]
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TRIGPOL` reader - Trigger event polarity Defines the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]. - 00: no trigger. Masked trigger event. - 01: trigger on the rising edge - 10: trigger on the falling edge - 11: same as 00"]
pub type TRIGPOL_R = crate::FieldReader;
#[doc = "Field `TRIGPOL` writer - Trigger event polarity Defines the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]. - 00: no trigger. Masked trigger event. - 01: trigger on the rising edge - 10: trigger on the falling edge - 11: same as 00"]
pub type TRIGPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TCEM` reader - Transfer complete event mode Defines the transfer granularity for the transfer complete (and half transfer complete) event generation. - 00: at block level (i.e. when GPDMA_CxBR1.BNDT\\[15:0\\]= 0): the complete (and the half) transfer event is generated at the (respectively half of the) end of a block. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]=0), then neither the complete transfer event nor the half transfer event is generated. - 01: channel x=0 to 11: same as 00 ;channel x=12 to 15: at 2D/repeated block level (i.e. when GPDMA_CxBR1.BRC\\[10:0\\]= 0 and GPDMA_CxBR1.BNDT\\[15:0\\]= 0): the complete (and the half) transfer event is generated at the end (respectively half of the end) of the 2D/repeated block Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]=0), then neither the complete transfer event nor the half transfer event is generated. - 10: at LLI level: the complete transfer event is generated at the end of the LLI transfer, including the update of the LLI if any. The half transfer event is generated at the half of the LLI data transfer (the LLI data transfer being a block or a 2D/repeated block transfer), if any data transfer. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]=0), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1. - 11: at channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI. The last LLI is the one that updates the link address GPDMA_CxLLR.LA\\[15:2\\]
to zero and that clears all the update bits - UT1, UT2, UB1, USA, UDA, if present UT3, UB2 and ULL - of the GPDMA_CxLLR register. If the channel transfer is continuous/infinite, no event is generated."]
pub type TCEM_R = crate::FieldReader;
#[doc = "Field `TCEM` writer - Transfer complete event mode Defines the transfer granularity for the transfer complete (and half transfer complete) event generation. - 00: at block level (i.e. when GPDMA_CxBR1.BNDT\\[15:0\\]= 0): the complete (and the half) transfer event is generated at the (respectively half of the) end of a block. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]=0), then neither the complete transfer event nor the half transfer event is generated. - 01: channel x=0 to 11: same as 00 ;channel x=12 to 15: at 2D/repeated block level (i.e. when GPDMA_CxBR1.BRC\\[10:0\\]= 0 and GPDMA_CxBR1.BNDT\\[15:0\\]= 0): the complete (and the half) transfer event is generated at the end (respectively half of the end) of the 2D/repeated block Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]=0), then neither the complete transfer event nor the half transfer event is generated. - 10: at LLI level: the complete transfer event is generated at the end of the LLI transfer, including the update of the LLI if any. The half transfer event is generated at the half of the LLI data transfer (the LLI data transfer being a block or a 2D/repeated block transfer), if any data transfer. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]=0), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1. - 11: at channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI. The last LLI is the one that updates the link address GPDMA_CxLLR.LA\\[15:2\\]
to zero and that clears all the update bits - UT1, UT2, UB1, USA, UDA, if present UT3, UB2 and ULL - of the GPDMA_CxLLR register. If the channel transfer is continuous/infinite, no event is generated."]
pub type TCEM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - DMA hardware request selection If the channel x is activated (i.e. GPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else, the selected hardware request as per Table 12 is internally taken into account. Note: The user must not assign a same input hardware request (i.e. a same REQSEL\\[6:0\\]
value) to different active DMA channels (i.e. if GPDMA_CxCR.EN=1 and GPDMA_CxTR2.SWREQ=0 for the related x channels). In other words, DMA is not intended to hardware support the case of simultaneous enabled channels having been -incorrectly- configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
    #[inline(always)]
    pub fn reqsel(&self) -> REQSEL_R {
        REQSEL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 9 - Software request When GPDMA_CxCR.EN is asserted, this field is internally taken into account: - 0: no software request. The selected hardware request REQSEL\\[6:0\\]
is taken into account. - 1: software request (for a memory-to-memory transfer). And the default selected hardware request as per REQSEL\\[6:0\\]
is ignored."]
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Destination hardware request If the channel x is activated (i.e. GPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else: - 0: the selected hardware request is driven by a source peripheral (i.e. this request signal is taken into account by the DMA transfer scheduler over the source/read port) - 1: the selected hardware request is driven by a destination peripheral (.e. this request signal is taken into account by the DMA transfer scheduler over the destination/write port)"]
    #[inline(always)]
    pub fn dreq(&self) -> DREQ_R {
        DREQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BREQ"]
    #[inline(always)]
    pub fn breq(&self) -> BREQ_R {
        BREQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Trigger mode: enabled. Transferring a next LLIn+1 which updates the GPDMA_CxTR2 with a new value for any of TRIGSEL\\[5:0\\]
or TRIGPOL\\[1:0\\]
resets the monitoring, trashing the (possible) memorized hit of the formerly defined LLIn trigger. After that a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, this new second trigger hitn+2 is lost and not memorized. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\\[1:0\\]=11 and (SWREQ=1 or (SWREQ=0 and DREQ=0)), the shortened burst transfer (by single(s) or/and by burst(s) of lower length) is conditioned once by the trigger. Note: When the programmed destination burst is internally shortened by single(s) or/and by burst(s) of lower length (e.g. vs FIFO size, vs block size, 1kB/4kB boundary address crossing): if the trigger is conditioning the programmed destination burst (if TRIGM\\[1:0\\]=11 and SWREQ=0 and DREQ=1), this shortened destination burst transfer is conditioned once by the trigger."]
    #[inline(always)]
    pub fn trigm(&self) -> TRIGM_R {
        TRIGM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21 - Trigger event input selection Note: Selects the trigger event input as per Table 13 of the DMA transfer, with an active trigger event if TRIGPOL\\[1:0\\]
!=00."]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - Trigger event polarity Defines the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]. - 00: no trigger. Masked trigger event. - 01: trigger on the rising edge - 10: trigger on the falling edge - 11: same as 00"]
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Transfer complete event mode Defines the transfer granularity for the transfer complete (and half transfer complete) event generation. - 00: at block level (i.e. when GPDMA_CxBR1.BNDT\\[15:0\\]= 0): the complete (and the half) transfer event is generated at the (respectively half of the) end of a block. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]=0), then neither the complete transfer event nor the half transfer event is generated. - 01: channel x=0 to 11: same as 00 ;channel x=12 to 15: at 2D/repeated block level (i.e. when GPDMA_CxBR1.BRC\\[10:0\\]= 0 and GPDMA_CxBR1.BNDT\\[15:0\\]= 0): the complete (and the half) transfer event is generated at the end (respectively half of the end) of the 2D/repeated block Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]=0), then neither the complete transfer event nor the half transfer event is generated. - 10: at LLI level: the complete transfer event is generated at the end of the LLI transfer, including the update of the LLI if any. The half transfer event is generated at the half of the LLI data transfer (the LLI data transfer being a block or a 2D/repeated block transfer), if any data transfer. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]=0), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1. - 11: at channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI. The last LLI is the one that updates the link address GPDMA_CxLLR.LA\\[15:2\\]
to zero and that clears all the update bits - UT1, UT2, UB1, USA, UDA, if present UT3, UB2 and ULL - of the GPDMA_CxLLR register. If the channel transfer is continuous/infinite, no event is generated."]
    #[inline(always)]
    pub fn tcem(&self) -> TCEM_R {
        TCEM_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DMA hardware request selection If the channel x is activated (i.e. GPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else, the selected hardware request as per Table 12 is internally taken into account. Note: The user must not assign a same input hardware request (i.e. a same REQSEL\\[6:0\\]
value) to different active DMA channels (i.e. if GPDMA_CxCR.EN=1 and GPDMA_CxTR2.SWREQ=0 for the related x channels). In other words, DMA is not intended to hardware support the case of simultaneous enabled channels having been -incorrectly- configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
    #[inline(always)]
    #[must_use]
    pub fn reqsel(&mut self) -> REQSEL_W<GPDMA_C13TR2rs> {
        REQSEL_W::new(self, 0)
    }
    #[doc = "Bit 9 - Software request When GPDMA_CxCR.EN is asserted, this field is internally taken into account: - 0: no software request. The selected hardware request REQSEL\\[6:0\\]
is taken into account. - 1: software request (for a memory-to-memory transfer). And the default selected hardware request as per REQSEL\\[6:0\\]
is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn swreq(&mut self) -> SWREQ_W<GPDMA_C13TR2rs> {
        SWREQ_W::new(self, 9)
    }
    #[doc = "Bit 10 - Destination hardware request If the channel x is activated (i.e. GPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else: - 0: the selected hardware request is driven by a source peripheral (i.e. this request signal is taken into account by the DMA transfer scheduler over the source/read port) - 1: the selected hardware request is driven by a destination peripheral (.e. this request signal is taken into account by the DMA transfer scheduler over the destination/write port)"]
    #[inline(always)]
    #[must_use]
    pub fn dreq(&mut self) -> DREQ_W<GPDMA_C13TR2rs> {
        DREQ_W::new(self, 10)
    }
    #[doc = "Bit 11 - BREQ"]
    #[inline(always)]
    #[must_use]
    pub fn breq(&mut self) -> BREQ_W<GPDMA_C13TR2rs> {
        BREQ_W::new(self, 11)
    }
    #[doc = "Bits 14:15 - Trigger mode: enabled. Transferring a next LLIn+1 which updates the GPDMA_CxTR2 with a new value for any of TRIGSEL\\[5:0\\]
or TRIGPOL\\[1:0\\]
resets the monitoring, trashing the (possible) memorized hit of the formerly defined LLIn trigger. After that a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, this new second trigger hitn+2 is lost and not memorized. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\\[1:0\\]=11 and (SWREQ=1 or (SWREQ=0 and DREQ=0)), the shortened burst transfer (by single(s) or/and by burst(s) of lower length) is conditioned once by the trigger. Note: When the programmed destination burst is internally shortened by single(s) or/and by burst(s) of lower length (e.g. vs FIFO size, vs block size, 1kB/4kB boundary address crossing): if the trigger is conditioning the programmed destination burst (if TRIGM\\[1:0\\]=11 and SWREQ=0 and DREQ=1), this shortened destination burst transfer is conditioned once by the trigger."]
    #[inline(always)]
    #[must_use]
    pub fn trigm(&mut self) -> TRIGM_W<GPDMA_C13TR2rs> {
        TRIGM_W::new(self, 14)
    }
    #[doc = "Bits 16:21 - Trigger event input selection Note: Selects the trigger event input as per Table 13 of the DMA transfer, with an active trigger event if TRIGPOL\\[1:0\\]
!=00."]
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<GPDMA_C13TR2rs> {
        TRIGSEL_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - Trigger event polarity Defines the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]. - 00: no trigger. Masked trigger event. - 01: trigger on the rising edge - 10: trigger on the falling edge - 11: same as 00"]
    #[inline(always)]
    #[must_use]
    pub fn trigpol(&mut self) -> TRIGPOL_W<GPDMA_C13TR2rs> {
        TRIGPOL_W::new(self, 24)
    }
    #[doc = "Bits 30:31 - Transfer complete event mode Defines the transfer granularity for the transfer complete (and half transfer complete) event generation. - 00: at block level (i.e. when GPDMA_CxBR1.BNDT\\[15:0\\]= 0): the complete (and the half) transfer event is generated at the (respectively half of the) end of a block. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]=0), then neither the complete transfer event nor the half transfer event is generated. - 01: channel x=0 to 11: same as 00 ;channel x=12 to 15: at 2D/repeated block level (i.e. when GPDMA_CxBR1.BRC\\[10:0\\]= 0 and GPDMA_CxBR1.BNDT\\[15:0\\]= 0): the complete (and the half) transfer event is generated at the end (respectively half of the end) of the 2D/repeated block Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]=0), then neither the complete transfer event nor the half transfer event is generated. - 10: at LLI level: the complete transfer event is generated at the end of the LLI transfer, including the update of the LLI if any. The half transfer event is generated at the half of the LLI data transfer (the LLI data transfer being a block or a 2D/repeated block transfer), if any data transfer. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]=0), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1. - 11: at channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI. The last LLI is the one that updates the link address GPDMA_CxLLR.LA\\[15:2\\]
to zero and that clears all the update bits - UT1, UT2, UB1, USA, UDA, if present UT3, UB2 and ULL - of the GPDMA_CxLLR register. If the channel transfer is continuous/infinite, no event is generated."]
    #[inline(always)]
    #[must_use]
    pub fn tcem(&mut self) -> TCEM_W<GPDMA_C13TR2rs> {
        TCEM_W::new(self, 30)
    }
}
#[doc = "GPDMA channel x transfer register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdma_c13tr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdma_c13tr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPDMA_C13TR2rs;
impl crate::RegisterSpec for GPDMA_C13TR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdma_c13tr2::R`](R) reader structure"]
impl crate::Readable for GPDMA_C13TR2rs {}
#[doc = "`write(|w| ..)` method takes [`gpdma_c13tr2::W`](W) writer structure"]
impl crate::Writable for GPDMA_C13TR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPDMA_C13TR2 to value 0"]
impl crate::Resettable for GPDMA_C13TR2rs {
    const RESET_VALUE: u32 = 0;
}
