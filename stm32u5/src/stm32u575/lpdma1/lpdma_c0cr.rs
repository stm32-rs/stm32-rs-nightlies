#[doc = "Register `LPDMA_C0CR` reader"]
pub type R = crate::R<LPDMA_C0CRrs>;
#[doc = "Register `LPDMA_C0CR` writer"]
pub type W = crate::W<LPDMA_C0CRrs>;
#[doc = "Field `EN` reader - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` writer - reset This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (LPDMA_CxSR.SUSPF = 1 and LPDMA_CxSR.IDLEF = LPDMA_CxCR.EN = 1) - channel in disabled state (LPDMA_CxSR.IDLEF = 1 and LPDMA_CxCR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (LPDMA_CxBR1, LPDMA_CxSAR and LPDMA_CxDAR) before enabling again the channel (see the programming sequence in )."]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP` reader - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an on-going DMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in sequence."]
pub type SUSP_R = crate::BitReader;
#[doc = "Field `SUSP` writer - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an on-going DMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in sequence."]
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - transfer complete interrupt enable"]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - transfer complete interrupt enable"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTIE` reader - half transfer complete interrupt enable"]
pub type HTIE_R = crate::BitReader;
#[doc = "Field `HTIE` writer - half transfer complete interrupt enable"]
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTEIE` reader - data transfer error interrupt enable"]
pub type DTEIE_R = crate::BitReader;
#[doc = "Field `DTEIE` writer - data transfer error interrupt enable"]
pub type DTEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULEIE` reader - update link transfer error interrupt enable"]
pub type ULEIE_R = crate::BitReader;
#[doc = "Field `ULEIE` writer - update link transfer error interrupt enable"]
pub type ULEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USEIE` reader - user setting error interrupt enable"]
pub type USEIE_R = crate::BitReader;
#[doc = "Field `USEIE` writer - user setting error interrupt enable"]
pub type USEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPIE` reader - completed suspension interrupt enable"]
pub type SUSPIE_R = crate::BitReader;
#[doc = "Field `SUSPIE` writer - completed suspension interrupt enable"]
pub type SUSPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOIE` reader - trigger overrun interrupt enable"]
pub type TOIE_R = crate::BitReader;
#[doc = "Field `TOIE` writer - trigger overrun interrupt enable"]
pub type TOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSM` reader - Link step mode First the block transfer is executed as defined by the current internal register file until LPDMA_CxBR1.BNDT\\[15:0 \\]
=0). Secondly the next linked-list data structure is conditionally uploaded from memory as defined by LPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
pub type LSM_R = crate::BitReader;
#[doc = "Field `LSM` writer - Link step mode First the block transfer is executed as defined by the current internal register file until LPDMA_CxBR1.BNDT\\[15:0 \\]
=0). Secondly the next linked-list data structure is conditionally uploaded from memory as defined by LPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
pub type LSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIO` reader - priority level of the channel x LPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type PRIO_R = crate::FieldReader;
#[doc = "Field `PRIO` writer - priority level of the channel x LPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type PRIO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an on-going DMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in sequence."]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - half transfer complete interrupt enable"]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - data transfer error interrupt enable"]
    #[inline(always)]
    pub fn dteie(&self) -> DTEIE_R {
        DTEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - update link transfer error interrupt enable"]
    #[inline(always)]
    pub fn uleie(&self) -> ULEIE_R {
        ULEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - user setting error interrupt enable"]
    #[inline(always)]
    pub fn useie(&self) -> USEIE_R {
        USEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - completed suspension interrupt enable"]
    #[inline(always)]
    pub fn suspie(&self) -> SUSPIE_R {
        SUSPIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Link step mode First the block transfer is executed as defined by the current internal register file until LPDMA_CxBR1.BNDT\\[15:0 \\]
=0). Secondly the next linked-list data structure is conditionally uploaded from memory as defined by LPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 22:23 - priority level of the channel x LPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<LPDMA_C0CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - reset This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (LPDMA_CxSR.SUSPF = 1 and LPDMA_CxSR.IDLEF = LPDMA_CxCR.EN = 1) - channel in disabled state (LPDMA_CxSR.IDLEF = 1 and LPDMA_CxCR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (LPDMA_CxBR1, LPDMA_CxSAR and LPDMA_CxDAR) before enabling again the channel (see the programming sequence in )."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<LPDMA_C0CRrs> {
        RESET_W::new(self, 1)
    }
    #[doc = "Bit 2 - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an on-going DMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in sequence."]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<LPDMA_C0CRrs> {
        SUSP_W::new(self, 2)
    }
    #[doc = "Bit 8 - transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<LPDMA_C0CRrs> {
        TCIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - half transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HTIE_W<LPDMA_C0CRrs> {
        HTIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - data transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dteie(&mut self) -> DTEIE_W<LPDMA_C0CRrs> {
        DTEIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - update link transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uleie(&mut self) -> ULEIE_W<LPDMA_C0CRrs> {
        ULEIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - user setting error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn useie(&mut self) -> USEIE_W<LPDMA_C0CRrs> {
        USEIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - completed suspension interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn suspie(&mut self) -> SUSPIE_W<LPDMA_C0CRrs> {
        SUSPIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - trigger overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie(&mut self) -> TOIE_W<LPDMA_C0CRrs> {
        TOIE_W::new(self, 14)
    }
    #[doc = "Bit 16 - Link step mode First the block transfer is executed as defined by the current internal register file until LPDMA_CxBR1.BNDT\\[15:0 \\]
=0). Secondly the next linked-list data structure is conditionally uploaded from memory as defined by LPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
    #[inline(always)]
    #[must_use]
    pub fn lsm(&mut self) -> LSM_W<LPDMA_C0CRrs> {
        LSM_W::new(self, 16)
    }
    #[doc = "Bits 22:23 - priority level of the channel x LPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<LPDMA_C0CRrs> {
        PRIO_W::new(self, 22)
    }
}
#[doc = "LPDMA channel 0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdma_c0cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdma_c0cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPDMA_C0CRrs;
impl crate::RegisterSpec for LPDMA_C0CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpdma_c0cr::R`](R) reader structure"]
impl crate::Readable for LPDMA_C0CRrs {}
#[doc = "`write(|w| ..)` method takes [`lpdma_c0cr::W`](W) writer structure"]
impl crate::Writable for LPDMA_C0CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPDMA_C0CR to value 0"]
impl crate::Resettable for LPDMA_C0CRrs {
    const RESET_VALUE: u32 = 0;
}
