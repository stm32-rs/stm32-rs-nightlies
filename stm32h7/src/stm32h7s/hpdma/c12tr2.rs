///Register `C12TR2` reader
pub type R = crate::R<C12TR2rs>;
///Register `C12TR2` writer
pub type W = crate::W<C12TR2rs>;
/**Field `REQSEL` reader - hardware request selection These bits are ignored if channel x is activated (HPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per Section 27.3.3. The user must not assign a same input hardware request (same REQSEL\[4:0\]
value) to different active HPDMA channels (HPDMA_CxCR.EN = 1 and HPDMA_CxTR2.SWREQ = 0 for these channels). The HPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.*/
pub type REQSEL_R = crate::FieldReader;
/**Field `REQSEL` writer - hardware request selection These bits are ignored if channel x is activated (HPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per Section 27.3.3. The user must not assign a same input hardware request (same REQSEL\[4:0\]
value) to different active HPDMA channels (HPDMA_CxCR.EN = 1 and HPDMA_CxTR2.SWREQ = 0 for these channels). The HPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.*/
pub type REQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SWREQ` reader - software request This bit is internally taken into account when HPDMA_CxCR.EN is asserted.
pub type SWREQ_R = crate::BitReader;
///Field `SWREQ` writer - software request This bit is internally taken into account when HPDMA_CxCR.EN is asserted.
pub type SWREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DREQ` reader - destination hardware request This bit is ignored if channel x is activated (HPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (HPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported.
pub type DREQ_R = crate::BitReader;
///Field `DREQ` writer - destination hardware request This bit is ignored if channel x is activated (HPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (HPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported.
pub type DREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BREQ` reader - Block hardware request If the channel x is activated (HPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:
pub type BREQ_R = crate::BitReader;
///Field `BREQ` writer - Block hardware request If the channel x is activated (HPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:
pub type BREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PFREQ` reader - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see Section 27.3.5 for the list of the implemented channels with this feature. If the channel x is activated (HPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: Note: - no 2D/repeated block support (HPDMA_CxBR1.BRC\[10:0\]
must be set to 0 if present) Note: - the peripheral must be set as the source of the transfer (DREQ = 0). Note: - data packing to a wider destination width is not supported (if destination width > source data width, HPDMA_CxTR1.PAM\[1\]
must be set to 0). Note: - HPDMA_CxBR1.BNDT\[15:0\]
must be set as a multiple of the source (peripheral) burst size.*/
pub type PFREQ_R = crate::BitReader;
/**Field `PFREQ` writer - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see Section 27.3.5 for the list of the implemented channels with this feature. If the channel x is activated (HPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: Note: - no 2D/repeated block support (HPDMA_CxBR1.BRC\[10:0\]
must be set to 0 if present) Note: - the peripheral must be set as the source of the transfer (DREQ = 0). Note: - data packing to a wider destination width is not supported (if destination width > source data width, HPDMA_CxTR1.PAM\[1\]
must be set to 0). Note: - HPDMA_CxBR1.BNDT\[15:0\]
must be set as a multiple of the source (peripheral) burst size.*/
pub type PFREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `TRIGM` reader - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (HPDMA_CxCR.EN asserted) with TRIGPOL\[1:0\]
= 00 or 11, these TRIGM\[1:0\]
bits are ignored. Else, an HPDMA transfer is conditioned by at least one trigger hit: first burst read of a 2D/repeated block transfer is conditioned by one hit trigger. If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each programmed burst read is conditioned. If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each programmed burst write is conditioned. The first memory burst read of a (possibly 2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the occurrence of both the hardware request and the first trigger hit. The HPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\[1:0\]
= 01 or respectively TRIGPOL\[1:0\]
= 10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\[5:0\]
is not modified, and the channel is enabled. Transferring a next LLI<sub>n+1</sub> that updates the HPDMA_CxTR2 with a new value for any of TRIGSEL\[5:0\]
or TRIGPOL\[1:0\], resets the monitoring, trashing the memorized hit of the formerly defined LLI<sub>n </sub>trigger. After a first new trigger hit<sub>n+1</sub> is memorized, if another second trigger hit<sub>n+2</sub> is detected and if the hit<sub>n</sub> triggered transfer is still not completed, hit<sub>n+2 </sub>is lost and not memorized.memorized. A trigger overrun flag is reported (HPDMA_CxSR.TOF =1 ), and an interrupt is generated if enabled (HPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\[1:0\]
= 11 and (SWREQ =1 or (SWREQ = 0 and DREQ =0 )), the shortened burst transfer (by singles or/and by bursts of lower length) is conditioned once by the trigger. Note: When the programmed destination burst is internally shortened by singles or/and by bursts of lower length (versus FIFO size, versus block size, 1/4-Kbyte boundary address crossing maximum burst length vs AHB/AXI protocol): if the trigger is conditioning the programmed destination burst (if TRIGM\[1:0\]
= 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst transfer is conditioned once by the trigger.*/
pub type TRIGM_R = crate::FieldReader;
/**Field `TRIGM` writer - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (HPDMA_CxCR.EN asserted) with TRIGPOL\[1:0\]
= 00 or 11, these TRIGM\[1:0\]
bits are ignored. Else, an HPDMA transfer is conditioned by at least one trigger hit: first burst read of a 2D/repeated block transfer is conditioned by one hit trigger. If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each programmed burst read is conditioned. If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each programmed burst write is conditioned. The first memory burst read of a (possibly 2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the occurrence of both the hardware request and the first trigger hit. The HPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\[1:0\]
= 01 or respectively TRIGPOL\[1:0\]
= 10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\[5:0\]
is not modified, and the channel is enabled. Transferring a next LLI<sub>n+1</sub> that updates the HPDMA_CxTR2 with a new value for any of TRIGSEL\[5:0\]
or TRIGPOL\[1:0\], resets the monitoring, trashing the memorized hit of the formerly defined LLI<sub>n </sub>trigger. After a first new trigger hit<sub>n+1</sub> is memorized, if another second trigger hit<sub>n+2</sub> is detected and if the hit<sub>n</sub> triggered transfer is still not completed, hit<sub>n+2 </sub>is lost and not memorized.memorized. A trigger overrun flag is reported (HPDMA_CxSR.TOF =1 ), and an interrupt is generated if enabled (HPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\[1:0\]
= 11 and (SWREQ =1 or (SWREQ = 0 and DREQ =0 )), the shortened burst transfer (by singles or/and by bursts of lower length) is conditioned once by the trigger. Note: When the programmed destination burst is internally shortened by singles or/and by bursts of lower length (versus FIFO size, versus block size, 1/4-Kbyte boundary address crossing maximum burst length vs AHB/AXI protocol): if the trigger is conditioning the programmed destination burst (if TRIGM\[1:0\]
= 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst transfer is conditioned once by the trigger.*/
pub type TRIGM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Field `TRIGSEL` reader - trigger event input selection These bits select the trigger event input of the HPDMA transfer (as per Section 27.3.6), with an active trigger event if TRIGPOL\[1:0\]
different from 00.*/
pub type TRIGSEL_R = crate::FieldReader;
/**Field `TRIGSEL` writer - trigger event input selection These bits select the trigger event input of the HPDMA transfer (as per Section 27.3.6), with an active trigger event if TRIGPOL\[1:0\]
different from 00.*/
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `TRIGPOL` reader - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[5:0\].
pub type TRIGPOL_R = crate::FieldReader;
///Field `TRIGPOL` writer - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[5:0\].
pub type TRIGPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Field `TCEM` reader - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI<sub>0 </sub>data transfer is null/void (directly programmed by the internal register file with HPDMA_CxBR1.BNDT\[15:0\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI<sub>0 </sub>data transfer is null/void (directly programmed by the internal register file with HPDMA_CxBR1.BNDT\[15:0\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI<sub>0 </sub>data transfer is null/void (directly programmed by the internal register file with HPDMA_CxBR1.BNDT\[15:0\]
= 0), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI<sub>1</sub>.*/
pub type TCEM_R = crate::FieldReader;
/**Field `TCEM` writer - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI<sub>0 </sub>data transfer is null/void (directly programmed by the internal register file with HPDMA_CxBR1.BNDT\[15:0\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI<sub>0 </sub>data transfer is null/void (directly programmed by the internal register file with HPDMA_CxBR1.BNDT\[15:0\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI<sub>0 </sub>data transfer is null/void (directly programmed by the internal register file with HPDMA_CxBR1.BNDT\[15:0\]
= 0), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI<sub>1</sub>.*/
pub type TCEM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    /**Bits 0:4 - hardware request selection These bits are ignored if channel x is activated (HPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per Section 27.3.3. The user must not assign a same input hardware request (same REQSEL\[4:0\]
    value) to different active HPDMA channels (HPDMA_CxCR.EN = 1 and HPDMA_CxTR2.SWREQ = 0 for these channels). The HPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.*/
    #[inline(always)]
    pub fn reqsel(&self) -> REQSEL_R {
        REQSEL_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 9 - software request This bit is internally taken into account when HPDMA_CxCR.EN is asserted.
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - destination hardware request This bit is ignored if channel x is activated (HPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (HPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported.
    #[inline(always)]
    pub fn dreq(&self) -> DREQ_R {
        DREQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Block hardware request If the channel x is activated (HPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:
    #[inline(always)]
    pub fn breq(&self) -> BREQ_R {
        BREQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    /**Bit 12 - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see Section 27.3.5 for the list of the implemented channels with this feature. If the channel x is activated (HPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: Note: - no 2D/repeated block support (HPDMA_CxBR1.BRC\[10:0\]
    must be set to 0 if present) Note: - the peripheral must be set as the source of the transfer (DREQ = 0). Note: - data packing to a wider destination width is not supported (if destination width > source data width, HPDMA_CxTR1.PAM\[1\]
    must be set to 0). Note: - HPDMA_CxBR1.BNDT\[15:0\]
    must be set as a multiple of the source (peripheral) burst size.*/
    #[inline(always)]
    pub fn pfreq(&self) -> PFREQ_R {
        PFREQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    /**Bits 14:15 - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (HPDMA_CxCR.EN asserted) with TRIGPOL\[1:0\]
    = 00 or 11, these TRIGM\[1:0\]
    bits are ignored. Else, an HPDMA transfer is conditioned by at least one trigger hit: first burst read of a 2D/repeated block transfer is conditioned by one hit trigger. If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each programmed burst read is conditioned. If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each programmed burst write is conditioned. The first memory burst read of a (possibly 2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the occurrence of both the hardware request and the first trigger hit. The HPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\[1:0\]
    = 01 or respectively TRIGPOL\[1:0\]
    = 10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\[5:0\]
    is not modified, and the channel is enabled. Transferring a next LLI<sub>n+1</sub> that updates the HPDMA_CxTR2 with a new value for any of TRIGSEL\[5:0\]
    or TRIGPOL\[1:0\], resets the monitoring, trashing the memorized hit of the formerly defined LLI<sub>n </sub>trigger. After a first new trigger hit<sub>n+1</sub> is memorized, if another second trigger hit<sub>n+2</sub> is detected and if the hit<sub>n</sub> triggered transfer is still not completed, hit<sub>n+2 </sub>is lost and not memorized.memorized. A trigger overrun flag is reported (HPDMA_CxSR.TOF =1 ), and an interrupt is generated if enabled (HPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\[1:0\]
    = 11 and (SWREQ =1 or (SWREQ = 0 and DREQ =0 )), the shortened burst transfer (by singles or/and by bursts of lower length) is conditioned once by the trigger. Note: When the programmed destination burst is internally shortened by singles or/and by bursts of lower length (versus FIFO size, versus block size, 1/4-Kbyte boundary address crossing maximum burst length vs AHB/AXI protocol): if the trigger is conditioning the programmed destination burst (if TRIGM\[1:0\]
    = 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst transfer is conditioned once by the trigger.*/
    #[inline(always)]
    pub fn trigm(&self) -> TRIGM_R {
        TRIGM_R::new(((self.bits >> 14) & 3) as u8)
    }
    /**Bits 16:21 - trigger event input selection These bits select the trigger event input of the HPDMA transfer (as per Section 27.3.6), with an active trigger event if TRIGPOL\[1:0\]
    different from 00.*/
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:25 - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[5:0\].
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 24) & 3) as u8)
    }
    /**Bits 30:31 - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI<sub>0 </sub>data transfer is null/void (directly programmed by the internal register file with HPDMA_CxBR1.BNDT\[15:0\]
    = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI<sub>0 </sub>data transfer is null/void (directly programmed by the internal register file with HPDMA_CxBR1.BNDT\[15:0\]
    = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI<sub>0 </sub>data transfer is null/void (directly programmed by the internal register file with HPDMA_CxBR1.BNDT\[15:0\]
    = 0), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI<sub>1</sub>.*/
    #[inline(always)]
    pub fn tcem(&self) -> TCEM_R {
        TCEM_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C12TR2")
            .field("reqsel", &self.reqsel())
            .field("swreq", &self.swreq())
            .field("dreq", &self.dreq())
            .field("breq", &self.breq())
            .field("pfreq", &self.pfreq())
            .field("trigm", &self.trigm())
            .field("trigsel", &self.trigsel())
            .field("trigpol", &self.trigpol())
            .field("tcem", &self.tcem())
            .finish()
    }
}
impl W {
    /**Bits 0:4 - hardware request selection These bits are ignored if channel x is activated (HPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per Section 27.3.3. The user must not assign a same input hardware request (same REQSEL\[4:0\]
    value) to different active HPDMA channels (HPDMA_CxCR.EN = 1 and HPDMA_CxTR2.SWREQ = 0 for these channels). The HPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.*/
    #[inline(always)]
    pub fn reqsel(&mut self) -> REQSEL_W<C12TR2rs> {
        REQSEL_W::new(self, 0)
    }
    ///Bit 9 - software request This bit is internally taken into account when HPDMA_CxCR.EN is asserted.
    #[inline(always)]
    pub fn swreq(&mut self) -> SWREQ_W<C12TR2rs> {
        SWREQ_W::new(self, 9)
    }
    ///Bit 10 - destination hardware request This bit is ignored if channel x is activated (HPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (HPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported.
    #[inline(always)]
    pub fn dreq(&mut self) -> DREQ_W<C12TR2rs> {
        DREQ_W::new(self, 10)
    }
    ///Bit 11 - Block hardware request If the channel x is activated (HPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:
    #[inline(always)]
    pub fn breq(&mut self) -> BREQ_W<C12TR2rs> {
        BREQ_W::new(self, 11)
    }
    /**Bit 12 - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see Section 27.3.5 for the list of the implemented channels with this feature. If the channel x is activated (HPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: Note: - no 2D/repeated block support (HPDMA_CxBR1.BRC\[10:0\]
    must be set to 0 if present) Note: - the peripheral must be set as the source of the transfer (DREQ = 0). Note: - data packing to a wider destination width is not supported (if destination width > source data width, HPDMA_CxTR1.PAM\[1\]
    must be set to 0). Note: - HPDMA_CxBR1.BNDT\[15:0\]
    must be set as a multiple of the source (peripheral) burst size.*/
    #[inline(always)]
    pub fn pfreq(&mut self) -> PFREQ_W<C12TR2rs> {
        PFREQ_W::new(self, 12)
    }
    /**Bits 14:15 - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (HPDMA_CxCR.EN asserted) with TRIGPOL\[1:0\]
    = 00 or 11, these TRIGM\[1:0\]
    bits are ignored. Else, an HPDMA transfer is conditioned by at least one trigger hit: first burst read of a 2D/repeated block transfer is conditioned by one hit trigger. If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each programmed burst read is conditioned. If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each programmed burst write is conditioned. The first memory burst read of a (possibly 2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the occurrence of both the hardware request and the first trigger hit. The HPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\[1:0\]
    = 01 or respectively TRIGPOL\[1:0\]
    = 10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\[5:0\]
    is not modified, and the channel is enabled. Transferring a next LLI<sub>n+1</sub> that updates the HPDMA_CxTR2 with a new value for any of TRIGSEL\[5:0\]
    or TRIGPOL\[1:0\], resets the monitoring, trashing the memorized hit of the formerly defined LLI<sub>n </sub>trigger. After a first new trigger hit<sub>n+1</sub> is memorized, if another second trigger hit<sub>n+2</sub> is detected and if the hit<sub>n</sub> triggered transfer is still not completed, hit<sub>n+2 </sub>is lost and not memorized.memorized. A trigger overrun flag is reported (HPDMA_CxSR.TOF =1 ), and an interrupt is generated if enabled (HPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\[1:0\]
    = 11 and (SWREQ =1 or (SWREQ = 0 and DREQ =0 )), the shortened burst transfer (by singles or/and by bursts of lower length) is conditioned once by the trigger. Note: When the programmed destination burst is internally shortened by singles or/and by bursts of lower length (versus FIFO size, versus block size, 1/4-Kbyte boundary address crossing maximum burst length vs AHB/AXI protocol): if the trigger is conditioning the programmed destination burst (if TRIGM\[1:0\]
    = 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst transfer is conditioned once by the trigger.*/
    #[inline(always)]
    pub fn trigm(&mut self) -> TRIGM_W<C12TR2rs> {
        TRIGM_W::new(self, 14)
    }
    /**Bits 16:21 - trigger event input selection These bits select the trigger event input of the HPDMA transfer (as per Section 27.3.6), with an active trigger event if TRIGPOL\[1:0\]
    different from 00.*/
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W<C12TR2rs> {
        TRIGSEL_W::new(self, 16)
    }
    ///Bits 24:25 - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[5:0\].
    #[inline(always)]
    pub fn trigpol(&mut self) -> TRIGPOL_W<C12TR2rs> {
        TRIGPOL_W::new(self, 24)
    }
    /**Bits 30:31 - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI<sub>0 </sub>data transfer is null/void (directly programmed by the internal register file with HPDMA_CxBR1.BNDT\[15:0\]
    = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI<sub>0 </sub>data transfer is null/void (directly programmed by the internal register file with HPDMA_CxBR1.BNDT\[15:0\]
    = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI<sub>0 </sub>data transfer is null/void (directly programmed by the internal register file with HPDMA_CxBR1.BNDT\[15:0\]
    = 0), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI<sub>1</sub>.*/
    #[inline(always)]
    pub fn tcem(&mut self) -> TCEM_W<C12TR2rs> {
        TCEM_W::new(self, 30)
    }
}
/**HPDMA channel 12 transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c12tr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12tr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#HPDMA:C12TR2)*/
pub struct C12TR2rs;
impl crate::RegisterSpec for C12TR2rs {
    type Ux = u32;
}
///`read()` method returns [`c12tr2::R`](R) reader structure
impl crate::Readable for C12TR2rs {}
///`write(|w| ..)` method takes [`c12tr2::W`](W) writer structure
impl crate::Writable for C12TR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C12TR2 to value 0
impl crate::Resettable for C12TR2rs {
    const RESET_VALUE: u32 = 0;
}