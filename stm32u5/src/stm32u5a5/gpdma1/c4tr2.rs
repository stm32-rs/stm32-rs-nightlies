///Register `C4TR2` reader
pub type R = crate::R<C4TR2rs>;
///Register `C4TR2` writer
pub type W = crate::W<C4TR2rs>;
/**Field `REQSEL` reader - DMA hardware request selection If the channel x is activated (i.e. GPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else, the selected hardware request as per Table 12 is internally taken into account. Note: The user must not assign a same input hardware request (i.e. a same REQSEL\[6:0\]
value) to different active DMA channels (i.e. if GPDMA_CxCR.EN=1 and GPDMA_CxTR2.SWREQ=0 for the related x channels). In other words, DMA is not intended to hardware support the case of simultaneous enabled channels having been -incorrectly- configured with a same hardware peripheral request signal, and there is no user setting error reporting.*/
pub type REQSEL_R = crate::FieldReader;
/**Field `REQSEL` writer - DMA hardware request selection If the channel x is activated (i.e. GPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else, the selected hardware request as per Table 12 is internally taken into account. Note: The user must not assign a same input hardware request (i.e. a same REQSEL\[6:0\]
value) to different active DMA channels (i.e. if GPDMA_CxCR.EN=1 and GPDMA_CxTR2.SWREQ=0 for the related x channels). In other words, DMA is not intended to hardware support the case of simultaneous enabled channels having been -incorrectly- configured with a same hardware peripheral request signal, and there is no user setting error reporting.*/
pub type REQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**Field `SWREQ` reader - Software request When GPDMA_CxCR.EN is asserted, this field is internally taken into account: - 0: no software request. The selected hardware request REQSEL\[6:0\]
is taken into account. - 1: software request (for a memory-to-memory transfer). And the default selected hardware request as per REQSEL\[6:0\]
is ignored.*/
pub type SWREQ_R = crate::BitReader;
/**Field `SWREQ` writer - Software request When GPDMA_CxCR.EN is asserted, this field is internally taken into account: - 0: no software request. The selected hardware request REQSEL\[6:0\]
is taken into account. - 1: software request (for a memory-to-memory transfer). And the default selected hardware request as per REQSEL\[6:0\]
is ignored.*/
pub type SWREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DREQ` reader - Destination hardware request If the channel x is activated (i.e. GPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else: - 0: the selected hardware request is driven by a source peripheral (i.e. this request signal is taken into account by the DMA transfer scheduler over the source/read port) - 1: the selected hardware request is driven by a destination peripheral (.e. this request signal is taken into account by the DMA transfer scheduler over the destination/write port)
pub type DREQ_R = crate::BitReader;
///Field `DREQ` writer - Destination hardware request If the channel x is activated (i.e. GPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else: - 0: the selected hardware request is driven by a source peripheral (i.e. this request signal is taken into account by the DMA transfer scheduler over the source/read port) - 1: the selected hardware request is driven by a destination peripheral (.e. this request signal is taken into account by the DMA transfer scheduler over the destination/write port)
pub type DREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BREQ` reader - BREQ
pub type BREQ_R = crate::BitReader;
///Field `BREQ` writer - BREQ
pub type BREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIGM` reader - Trigger mode: rst read of a/each block transfer is conditioned by one hit trigger.
pub type TRIGM_R = crate::FieldReader;
///Field `TRIGM` writer - Trigger mode: rst read of a/each block transfer is conditioned by one hit trigger.
pub type TRIGM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Field `TRIGSEL` reader - Trigger event input selection Note: Selects the trigger event input as per Table 13 of the DMA transfer, with an active trigger event if TRIGPOL\[1:0\]
!=00.*/
pub type TRIGSEL_R = crate::FieldReader;
/**Field `TRIGSEL` writer - Trigger event input selection Note: Selects the trigger event input as per Table 13 of the DMA transfer, with an active trigger event if TRIGPOL\[1:0\]
!=00.*/
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `TRIGPOL` reader - Trigger event polarity Defines the polarity of the selected trigger event input defined by TRIGSEL\[5:0\]. - 00: no trigger. Masked trigger event. - 01: trigger on the rising edge - 10: trigger on the falling edge - 11: same as 00
pub type TRIGPOL_R = crate::FieldReader;
///Field `TRIGPOL` writer - Trigger event polarity Defines the polarity of the selected trigger event input defined by TRIGSEL\[5:0\]. - 00: no trigger. Masked trigger event. - 01: trigger on the rising edge - 10: trigger on the falling edge - 11: same as 00
pub type TRIGPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TCEM` reader - Transfer complete event mode
pub type TCEM_R = crate::FieldReader;
///Field `TCEM` writer - Transfer complete event mode
pub type TCEM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    /**Bits 0:6 - DMA hardware request selection If the channel x is activated (i.e. GPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else, the selected hardware request as per Table 12 is internally taken into account. Note: The user must not assign a same input hardware request (i.e. a same REQSEL\[6:0\]
    value) to different active DMA channels (i.e. if GPDMA_CxCR.EN=1 and GPDMA_CxTR2.SWREQ=0 for the related x channels). In other words, DMA is not intended to hardware support the case of simultaneous enabled channels having been -incorrectly- configured with a same hardware peripheral request signal, and there is no user setting error reporting.*/
    #[inline(always)]
    pub fn reqsel(&self) -> REQSEL_R {
        REQSEL_R::new((self.bits & 0x7f) as u8)
    }
    /**Bit 9 - Software request When GPDMA_CxCR.EN is asserted, this field is internally taken into account: - 0: no software request. The selected hardware request REQSEL\[6:0\]
    is taken into account. - 1: software request (for a memory-to-memory transfer). And the default selected hardware request as per REQSEL\[6:0\]
    is ignored.*/
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Destination hardware request If the channel x is activated (i.e. GPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else: - 0: the selected hardware request is driven by a source peripheral (i.e. this request signal is taken into account by the DMA transfer scheduler over the source/read port) - 1: the selected hardware request is driven by a destination peripheral (.e. this request signal is taken into account by the DMA transfer scheduler over the destination/write port)
    #[inline(always)]
    pub fn dreq(&self) -> DREQ_R {
        DREQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - BREQ
    #[inline(always)]
    pub fn breq(&self) -> BREQ_R {
        BREQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 14:15 - Trigger mode: rst read of a/each block transfer is conditioned by one hit trigger.
    #[inline(always)]
    pub fn trigm(&self) -> TRIGM_R {
        TRIGM_R::new(((self.bits >> 14) & 3) as u8)
    }
    /**Bits 16:21 - Trigger event input selection Note: Selects the trigger event input as per Table 13 of the DMA transfer, with an active trigger event if TRIGPOL\[1:0\]
    !=00.*/
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:25 - Trigger event polarity Defines the polarity of the selected trigger event input defined by TRIGSEL\[5:0\]. - 00: no trigger. Masked trigger event. - 01: trigger on the rising edge - 10: trigger on the falling edge - 11: same as 00
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 30:31 - Transfer complete event mode
    #[inline(always)]
    pub fn tcem(&self) -> TCEM_R {
        TCEM_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C4TR2")
            .field("reqsel", &self.reqsel())
            .field("swreq", &self.swreq())
            .field("dreq", &self.dreq())
            .field("breq", &self.breq())
            .field("trigm", &self.trigm())
            .field("trigsel", &self.trigsel())
            .field("trigpol", &self.trigpol())
            .field("tcem", &self.tcem())
            .finish()
    }
}
impl W {
    /**Bits 0:6 - DMA hardware request selection If the channel x is activated (i.e. GPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else, the selected hardware request as per Table 12 is internally taken into account. Note: The user must not assign a same input hardware request (i.e. a same REQSEL\[6:0\]
    value) to different active DMA channels (i.e. if GPDMA_CxCR.EN=1 and GPDMA_CxTR2.SWREQ=0 for the related x channels). In other words, DMA is not intended to hardware support the case of simultaneous enabled channels having been -incorrectly- configured with a same hardware peripheral request signal, and there is no user setting error reporting.*/
    #[inline(always)]
    pub fn reqsel(&mut self) -> REQSEL_W<C4TR2rs> {
        REQSEL_W::new(self, 0)
    }
    /**Bit 9 - Software request When GPDMA_CxCR.EN is asserted, this field is internally taken into account: - 0: no software request. The selected hardware request REQSEL\[6:0\]
    is taken into account. - 1: software request (for a memory-to-memory transfer). And the default selected hardware request as per REQSEL\[6:0\]
    is ignored.*/
    #[inline(always)]
    pub fn swreq(&mut self) -> SWREQ_W<C4TR2rs> {
        SWREQ_W::new(self, 9)
    }
    ///Bit 10 - Destination hardware request If the channel x is activated (i.e. GPDMA_CxCR.EN is asserted) with SWREQ=1 (i.e. software request for a memory-to-memory transfer), this bit is ignored. Else: - 0: the selected hardware request is driven by a source peripheral (i.e. this request signal is taken into account by the DMA transfer scheduler over the source/read port) - 1: the selected hardware request is driven by a destination peripheral (.e. this request signal is taken into account by the DMA transfer scheduler over the destination/write port)
    #[inline(always)]
    pub fn dreq(&mut self) -> DREQ_W<C4TR2rs> {
        DREQ_W::new(self, 10)
    }
    ///Bit 11 - BREQ
    #[inline(always)]
    pub fn breq(&mut self) -> BREQ_W<C4TR2rs> {
        BREQ_W::new(self, 11)
    }
    ///Bits 14:15 - Trigger mode: rst read of a/each block transfer is conditioned by one hit trigger.
    #[inline(always)]
    pub fn trigm(&mut self) -> TRIGM_W<C4TR2rs> {
        TRIGM_W::new(self, 14)
    }
    /**Bits 16:21 - Trigger event input selection Note: Selects the trigger event input as per Table 13 of the DMA transfer, with an active trigger event if TRIGPOL\[1:0\]
    !=00.*/
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W<C4TR2rs> {
        TRIGSEL_W::new(self, 16)
    }
    ///Bits 24:25 - Trigger event polarity Defines the polarity of the selected trigger event input defined by TRIGSEL\[5:0\]. - 00: no trigger. Masked trigger event. - 01: trigger on the rising edge - 10: trigger on the falling edge - 11: same as 00
    #[inline(always)]
    pub fn trigpol(&mut self) -> TRIGPOL_W<C4TR2rs> {
        TRIGPOL_W::new(self, 24)
    }
    ///Bits 30:31 - Transfer complete event mode
    #[inline(always)]
    pub fn tcem(&mut self) -> TCEM_W<C4TR2rs> {
        TCEM_W::new(self, 30)
    }
}
/**GPDMA channel x transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c4tr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4tr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#GPDMA1:C4TR2)*/
pub struct C4TR2rs;
impl crate::RegisterSpec for C4TR2rs {
    type Ux = u32;
}
///`read()` method returns [`c4tr2::R`](R) reader structure
impl crate::Readable for C4TR2rs {}
///`write(|w| ..)` method takes [`c4tr2::W`](W) writer structure
impl crate::Writable for C4TR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C4TR2 to value 0
impl crate::Resettable for C4TR2rs {
    const RESET_VALUE: u32 = 0;
}
