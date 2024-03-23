#[doc = "Register `C2CR` reader"]
pub type R = crate::R<C2CRrs>;
#[doc = "Register `C2CR` writer"]
pub type W = crate::W<C2CRrs>;
#[doc = "enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    #[doc = "0: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Channel enabled"]
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::Disabled,
            true => EN::Enabled,
        }
    }
    #[doc = "Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    #[doc = "Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
#[doc = "Field `EN` writer - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    #[doc = "Channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
#[doc = "reset This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the FIFO, the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (GPDMA_CxSR.SUSPF = 1 and GPDMA_CxSR.IDLEF = GPDMA_CxCR.EN = 1) - channel in disabled state (GPDMA_CxSR.IDLEF = 1 and GPDMA_CxCR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (GPDMA_CxBR1, GPDMA_CxSAR and GPDMA_CxDAR) before enabling again the channel (see the programming sequence in ).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETW {
    #[doc = "1: Reset channel"]
    Reset = 1,
}
impl From<RESETW> for bool {
    #[inline(always)]
    fn from(variant: RESETW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` writer - reset This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the FIFO, the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (GPDMA_CxSR.SUSPF = 1 and GPDMA_CxSR.IDLEF = GPDMA_CxCR.EN = 1) - channel in disabled state (GPDMA_CxSR.IDLEF = 1 and GPDMA_CxCR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (GPDMA_CxBR1, GPDMA_CxSAR and GPDMA_CxDAR) before enabling again the channel (see the programming sequence in )."]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG, RESETW>;
impl<'a, REG> RESET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset channel"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RESETW::Reset)
    }
}
#[doc = "suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an ongoing GPDMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSP {
    #[doc = "0: Channel operation not suspended"]
    NotSuspended = 0,
    #[doc = "1: Channel operation suspended"]
    Suspended = 1,
}
impl From<SUSP> for bool {
    #[inline(always)]
    fn from(variant: SUSP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSP` reader - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an ongoing GPDMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in ."]
pub type SUSP_R = crate::BitReader<SUSP>;
impl SUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSP {
        match self.bits {
            false => SUSP::NotSuspended,
            true => SUSP::Suspended,
        }
    }
    #[doc = "Channel operation not suspended"]
    #[inline(always)]
    pub fn is_not_suspended(&self) -> bool {
        *self == SUSP::NotSuspended
    }
    #[doc = "Channel operation suspended"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == SUSP::Suspended
    }
}
#[doc = "Field `SUSP` writer - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an ongoing GPDMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in ."]
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG, SUSP>;
impl<'a, REG> SUSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel operation not suspended"]
    #[inline(always)]
    pub fn not_suspended(self) -> &'a mut crate::W<REG> {
        self.variant(SUSP::NotSuspended)
    }
    #[doc = "Channel operation suspended"]
    #[inline(always)]
    pub fn suspended(self) -> &'a mut crate::W<REG> {
        self.variant(SUSP::Suspended)
    }
}
#[doc = "transfer complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<TCIE> for bool {
    #[inline(always)]
    fn from(variant: TCIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - transfer complete interrupt enable"]
pub type TCIE_R = crate::BitReader<TCIE>;
impl TCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIE {
        match self.bits {
            false => TCIE::Disabled,
            true => TCIE::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE::Enabled
    }
}
#[doc = "Field `TCIE` writer - transfer complete interrupt enable"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Enabled)
    }
}
#[doc = "Field `HTIE` reader - half transfer complete interrupt enable"]
pub use TCIE_R as HTIE_R;
#[doc = "Field `DTEIE` reader - data transfer error interrupt enable"]
pub use TCIE_R as DTEIE_R;
#[doc = "Field `ULEIE` reader - update link transfer error interrupt enable"]
pub use TCIE_R as ULEIE_R;
#[doc = "Field `USEIE` reader - user setting error interrupt enable"]
pub use TCIE_R as USEIE_R;
#[doc = "Field `SUSPIE` reader - completed suspension interrupt enable"]
pub use TCIE_R as SUSPIE_R;
#[doc = "Field `TOIE` reader - trigger overrun interrupt enable"]
pub use TCIE_R as TOIE_R;
#[doc = "Field `HTIE` writer - half transfer complete interrupt enable"]
pub use TCIE_W as HTIE_W;
#[doc = "Field `DTEIE` writer - data transfer error interrupt enable"]
pub use TCIE_W as DTEIE_W;
#[doc = "Field `ULEIE` writer - update link transfer error interrupt enable"]
pub use TCIE_W as ULEIE_W;
#[doc = "Field `USEIE` writer - user setting error interrupt enable"]
pub use TCIE_W as USEIE_W;
#[doc = "Field `SUSPIE` writer - completed suspension interrupt enable"]
pub use TCIE_W as SUSPIE_W;
#[doc = "Field `TOIE` writer - trigger overrun interrupt enable"]
pub use TCIE_W as TOIE_W;
#[doc = "Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\\[15:0\\]
= 0 and GPDMA_CxBR1.BRC\\[10:0\\]
= 0 if present. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSM {
    #[doc = "0: Channel executed for full linked list"]
    FullLinkedList = 0,
    #[doc = "1: Channel executed once for current linked list"]
    Once = 1,
}
impl From<LSM> for bool {
    #[inline(always)]
    fn from(variant: LSM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSM` reader - Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\\[15:0\\]
= 0 and GPDMA_CxBR1.BRC\\[10:0\\]
= 0 if present. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
pub type LSM_R = crate::BitReader<LSM>;
impl LSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSM {
        match self.bits {
            false => LSM::FullLinkedList,
            true => LSM::Once,
        }
    }
    #[doc = "Channel executed for full linked list"]
    #[inline(always)]
    pub fn is_full_linked_list(&self) -> bool {
        *self == LSM::FullLinkedList
    }
    #[doc = "Channel executed once for current linked list"]
    #[inline(always)]
    pub fn is_once(&self) -> bool {
        *self == LSM::Once
    }
}
#[doc = "Field `LSM` writer - Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\\[15:0\\]
= 0 and GPDMA_CxBR1.BRC\\[10:0\\]
= 0 if present. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
pub type LSM_W<'a, REG> = crate::BitWriter<'a, REG, LSM>;
impl<'a, REG> LSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel executed for full linked list"]
    #[inline(always)]
    pub fn full_linked_list(self) -> &'a mut crate::W<REG> {
        self.variant(LSM::FullLinkedList)
    }
    #[doc = "Channel executed once for current linked list"]
    #[inline(always)]
    pub fn once(self) -> &'a mut crate::W<REG> {
        self.variant(LSM::Once)
    }
}
#[doc = "linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LAP {
    #[doc = "0: Port 0 (AHB) allocated"]
    Port0 = 0,
    #[doc = "1: Port 1 (AHB) allocated"]
    Port1 = 1,
}
impl From<LAP> for bool {
    #[inline(always)]
    fn from(variant: LAP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LAP` reader - linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
pub type LAP_R = crate::BitReader<LAP>;
impl LAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LAP {
        match self.bits {
            false => LAP::Port0,
            true => LAP::Port1,
        }
    }
    #[doc = "Port 0 (AHB) allocated"]
    #[inline(always)]
    pub fn is_port0(&self) -> bool {
        *self == LAP::Port0
    }
    #[doc = "Port 1 (AHB) allocated"]
    #[inline(always)]
    pub fn is_port1(&self) -> bool {
        *self == LAP::Port1
    }
}
#[doc = "Field `LAP` writer - linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
pub type LAP_W<'a, REG> = crate::BitWriter<'a, REG, LAP>;
impl<'a, REG> LAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port 0 (AHB) allocated"]
    #[inline(always)]
    pub fn port0(self) -> &'a mut crate::W<REG> {
        self.variant(LAP::Port0)
    }
    #[doc = "Port 1 (AHB) allocated"]
    #[inline(always)]
    pub fn port1(self) -> &'a mut crate::W<REG> {
        self.variant(LAP::Port1)
    }
}
#[doc = "priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRIO {
    #[doc = "0: Low priority, low weight"]
    LowPrioLowWeight = 0,
    #[doc = "1: Low priority, mid weight"]
    LowPrioMidWeight = 1,
    #[doc = "2: Low priority, high weight"]
    LowPrioHighWeight = 2,
    #[doc = "3: High priority"]
    HighPrio = 3,
}
impl From<PRIO> for u8 {
    #[inline(always)]
    fn from(variant: PRIO) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRIO {
    type Ux = u8;
}
#[doc = "Field `PRIO` reader - priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type PRIO_R = crate::FieldReader<PRIO>;
impl PRIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIO {
        match self.bits {
            0 => PRIO::LowPrioLowWeight,
            1 => PRIO::LowPrioMidWeight,
            2 => PRIO::LowPrioHighWeight,
            3 => PRIO::HighPrio,
            _ => unreachable!(),
        }
    }
    #[doc = "Low priority, low weight"]
    #[inline(always)]
    pub fn is_low_prio_low_weight(&self) -> bool {
        *self == PRIO::LowPrioLowWeight
    }
    #[doc = "Low priority, mid weight"]
    #[inline(always)]
    pub fn is_low_prio_mid_weight(&self) -> bool {
        *self == PRIO::LowPrioMidWeight
    }
    #[doc = "Low priority, high weight"]
    #[inline(always)]
    pub fn is_low_prio_high_weight(&self) -> bool {
        *self == PRIO::LowPrioHighWeight
    }
    #[doc = "High priority"]
    #[inline(always)]
    pub fn is_high_prio(&self) -> bool {
        *self == PRIO::HighPrio
    }
}
#[doc = "Field `PRIO` writer - priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type PRIO_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PRIO>;
impl<'a, REG> PRIO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low priority, low weight"]
    #[inline(always)]
    pub fn low_prio_low_weight(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO::LowPrioLowWeight)
    }
    #[doc = "Low priority, mid weight"]
    #[inline(always)]
    pub fn low_prio_mid_weight(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO::LowPrioMidWeight)
    }
    #[doc = "Low priority, high weight"]
    #[inline(always)]
    pub fn low_prio_high_weight(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO::LowPrioHighWeight)
    }
    #[doc = "High priority"]
    #[inline(always)]
    pub fn high_prio(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO::HighPrio)
    }
}
impl R {
    #[doc = "Bit 0 - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an ongoing GPDMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in ."]
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
    #[doc = "Bit 16 - Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\\[15:0\\]
= 0 and GPDMA_CxBR1.BRC\\[10:0\\]
= 0 if present. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
    #[inline(always)]
    pub fn lap(&self) -> LAP_R {
        LAP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 22:23 - priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<C2CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - reset This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the FIFO, the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (GPDMA_CxSR.SUSPF = 1 and GPDMA_CxSR.IDLEF = GPDMA_CxCR.EN = 1) - channel in disabled state (GPDMA_CxSR.IDLEF = 1 and GPDMA_CxCR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (GPDMA_CxBR1, GPDMA_CxSAR and GPDMA_CxDAR) before enabling again the channel (see the programming sequence in )."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<C2CRrs> {
        RESET_W::new(self, 1)
    }
    #[doc = "Bit 2 - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an ongoing GPDMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in ."]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<C2CRrs> {
        SUSP_W::new(self, 2)
    }
    #[doc = "Bit 8 - transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<C2CRrs> {
        TCIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - half transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HTIE_W<C2CRrs> {
        HTIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - data transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dteie(&mut self) -> DTEIE_W<C2CRrs> {
        DTEIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - update link transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uleie(&mut self) -> ULEIE_W<C2CRrs> {
        ULEIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - user setting error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn useie(&mut self) -> USEIE_W<C2CRrs> {
        USEIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - completed suspension interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn suspie(&mut self) -> SUSPIE_W<C2CRrs> {
        SUSPIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - trigger overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie(&mut self) -> TOIE_W<C2CRrs> {
        TOIE_W::new(self, 14)
    }
    #[doc = "Bit 16 - Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\\[15:0\\]
= 0 and GPDMA_CxBR1.BRC\\[10:0\\]
= 0 if present. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
    #[inline(always)]
    #[must_use]
    pub fn lsm(&mut self) -> LSM_W<C2CRrs> {
        LSM_W::new(self, 16)
    }
    #[doc = "Bit 17 - linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
    #[inline(always)]
    #[must_use]
    pub fn lap(&mut self) -> LAP_W<C2CRrs> {
        LAP_W::new(self, 17)
    }
    #[doc = "Bits 22:23 - priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<C2CRrs> {
        PRIO_W::new(self, 22)
    }
}
#[doc = "GPDMA channel 2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2CRrs;
impl crate::RegisterSpec for C2CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2cr::R`](R) reader structure"]
impl crate::Readable for C2CRrs {}
#[doc = "`write(|w| ..)` method takes [`c2cr::W`](W) writer structure"]
impl crate::Writable for C2CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2CR to value 0"]
impl crate::Resettable for C2CRrs {
    const RESET_VALUE: u32 = 0;
}
