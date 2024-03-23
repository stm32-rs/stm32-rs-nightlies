#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START {
    #[doc = "1: Launch the DMA2D"]
    Start = 1,
}
impl From<START> for bool {
    #[inline(always)]
    fn from(variant: START) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers"]
pub type START_R = crate::BitReader<START>;
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<START> {
        match self.bits {
            true => Some(START::Start),
            _ => None,
        }
    }
    #[doc = "Launch the DMA2D"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START::Start
    }
}
#[doc = "Field `START` writer - Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG, START>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Launch the DMA2D"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(START::Start)
    }
}
#[doc = "Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSP {
    #[doc = "0: Transfer not suspended"]
    NotSuspended = 0,
    #[doc = "1: Transfer suspended"]
    Suspended = 1,
}
impl From<SUSP> for bool {
    #[inline(always)]
    fn from(variant: SUSP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSP` reader - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset."]
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
    #[doc = "Transfer not suspended"]
    #[inline(always)]
    pub fn is_not_suspended(&self) -> bool {
        *self == SUSP::NotSuspended
    }
    #[doc = "Transfer suspended"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == SUSP::Suspended
    }
}
#[doc = "Field `SUSP` writer - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset."]
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG, SUSP>;
impl<'a, REG> SUSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer not suspended"]
    #[inline(always)]
    pub fn not_suspended(self) -> &'a mut crate::W<REG> {
        self.variant(SUSP::NotSuspended)
    }
    #[doc = "Transfer suspended"]
    #[inline(always)]
    pub fn suspended(self) -> &'a mut crate::W<REG> {
        self.variant(SUSP::Suspended)
    }
}
#[doc = "Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABORT {
    #[doc = "1: Transfer abort requested"]
    AbortRequest = 1,
}
impl From<ABORT> for bool {
    #[inline(always)]
    fn from(variant: ABORT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT` reader - Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset."]
pub type ABORT_R = crate::BitReader<ABORT>;
impl ABORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ABORT> {
        match self.bits {
            true => Some(ABORT::AbortRequest),
            _ => None,
        }
    }
    #[doc = "Transfer abort requested"]
    #[inline(always)]
    pub fn is_abort_request(&self) -> bool {
        *self == ABORT::AbortRequest
    }
}
#[doc = "Field `ABORT` writer - Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset."]
pub type ABORT_W<'a, REG> = crate::BitWriter<'a, REG, ABORT>;
impl<'a, REG> ABORT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer abort requested"]
    #[inline(always)]
    pub fn abort_request(self) -> &'a mut crate::W<REG> {
        self.variant(ABORT::AbortRequest)
    }
}
#[doc = "Transfer error interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE {
    #[doc = "0: TE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: TE interrupt enabled"]
    Enabled = 1,
}
impl From<TEIE> for bool {
    #[inline(always)]
    fn from(variant: TEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIE` reader - Transfer error interrupt enable This bit is set and cleared by software."]
pub type TEIE_R = crate::BitReader<TEIE>;
impl TEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEIE {
        match self.bits {
            false => TEIE::Disabled,
            true => TEIE::Enabled,
        }
    }
    #[doc = "TE interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEIE::Disabled
    }
    #[doc = "TE interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEIE::Enabled
    }
}
#[doc = "Field `TEIE` writer - Transfer error interrupt enable This bit is set and cleared by software."]
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG, TEIE>;
impl<'a, REG> TEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Disabled)
    }
    #[doc = "TE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Enabled)
    }
}
#[doc = "Transfer complete interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE {
    #[doc = "0: TC interrupt disabled"]
    Disabled = 0,
    #[doc = "1: TC interrupt enabled"]
    Enabled = 1,
}
impl From<TCIE> for bool {
    #[inline(always)]
    fn from(variant: TCIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable This bit is set and cleared by software."]
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
    #[doc = "TC interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE::Disabled
    }
    #[doc = "TC interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE::Enabled
    }
}
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable This bit is set and cleared by software."]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Disabled)
    }
    #[doc = "TC interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Enabled)
    }
}
#[doc = "Transfer watermark interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWIE {
    #[doc = "0: TW interrupt disabled"]
    Disabled = 0,
    #[doc = "1: TW interrupt enabled"]
    Enabled = 1,
}
impl From<TWIE> for bool {
    #[inline(always)]
    fn from(variant: TWIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TWIE` reader - Transfer watermark interrupt enable This bit is set and cleared by software."]
pub type TWIE_R = crate::BitReader<TWIE>;
impl TWIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TWIE {
        match self.bits {
            false => TWIE::Disabled,
            true => TWIE::Enabled,
        }
    }
    #[doc = "TW interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TWIE::Disabled
    }
    #[doc = "TW interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TWIE::Enabled
    }
}
#[doc = "Field `TWIE` writer - Transfer watermark interrupt enable This bit is set and cleared by software."]
pub type TWIE_W<'a, REG> = crate::BitWriter<'a, REG, TWIE>;
impl<'a, REG> TWIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TW interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TWIE::Disabled)
    }
    #[doc = "TW interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TWIE::Enabled)
    }
}
#[doc = "CLUT access error interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAEIE {
    #[doc = "0: CAE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: CAE interrupt enabled"]
    Enabled = 1,
}
impl From<CAEIE> for bool {
    #[inline(always)]
    fn from(variant: CAEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAEIE` reader - CLUT access error interrupt enable This bit is set and cleared by software."]
pub type CAEIE_R = crate::BitReader<CAEIE>;
impl CAEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAEIE {
        match self.bits {
            false => CAEIE::Disabled,
            true => CAEIE::Enabled,
        }
    }
    #[doc = "CAE interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAEIE::Disabled
    }
    #[doc = "CAE interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAEIE::Enabled
    }
}
#[doc = "Field `CAEIE` writer - CLUT access error interrupt enable This bit is set and cleared by software."]
pub type CAEIE_W<'a, REG> = crate::BitWriter<'a, REG, CAEIE>;
impl<'a, REG> CAEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CAE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAEIE::Disabled)
    }
    #[doc = "CAE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAEIE::Enabled)
    }
}
#[doc = "CLUT transfer complete interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCIE {
    #[doc = "0: CTC interrupt disabled"]
    Disabled = 0,
    #[doc = "1: CTC interrupt enabled"]
    Enabled = 1,
}
impl From<CTCIE> for bool {
    #[inline(always)]
    fn from(variant: CTCIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCIE` reader - CLUT transfer complete interrupt enable This bit is set and cleared by software."]
pub type CTCIE_R = crate::BitReader<CTCIE>;
impl CTCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTCIE {
        match self.bits {
            false => CTCIE::Disabled,
            true => CTCIE::Enabled,
        }
    }
    #[doc = "CTC interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTCIE::Disabled
    }
    #[doc = "CTC interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTCIE::Enabled
    }
}
#[doc = "Field `CTCIE` writer - CLUT transfer complete interrupt enable This bit is set and cleared by software."]
pub type CTCIE_W<'a, REG> = crate::BitWriter<'a, REG, CTCIE>;
impl<'a, REG> CTCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTCIE::Disabled)
    }
    #[doc = "CTC interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTCIE::Enabled)
    }
}
#[doc = "Configuration Error Interrupt Enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEIE {
    #[doc = "0: CE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: CE interrupt enabled"]
    Enabled = 1,
}
impl From<CEIE> for bool {
    #[inline(always)]
    fn from(variant: CEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIE` reader - Configuration Error Interrupt Enable This bit is set and cleared by software."]
pub type CEIE_R = crate::BitReader<CEIE>;
impl CEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEIE {
        match self.bits {
            false => CEIE::Disabled,
            true => CEIE::Enabled,
        }
    }
    #[doc = "CE interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CEIE::Disabled
    }
    #[doc = "CE interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CEIE::Enabled
    }
}
#[doc = "Field `CEIE` writer - Configuration Error Interrupt Enable This bit is set and cleared by software."]
pub type CEIE_W<'a, REG> = crate::BitWriter<'a, REG, CEIE>;
impl<'a, REG> CEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CEIE::Disabled)
    }
    #[doc = "CE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CEIE::Enabled)
    }
}
#[doc = "DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    #[doc = "0: Memory-to-memory (FG fetch only)"]
    MemoryToMemory = 0,
    #[doc = "1: Memory-to-memory with PFC (FG fetch only with FG PFC active)"]
    MemoryToMemoryPfc = 1,
    #[doc = "2: Memory-to-memory with blending (FG and BG fetch with PFC and blending)"]
    MemoryToMemoryPfcblending = 2,
    #[doc = "3: Register-to-memory"]
    RegisterToMemory = 3,
}
impl From<MODE> for u8 {
    #[inline(always)]
    fn from(variant: MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE {
    type Ux = u8;
}
#[doc = "Field `MODE` reader - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing."]
pub type MODE_R = crate::FieldReader<MODE>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE {
        match self.bits {
            0 => MODE::MemoryToMemory,
            1 => MODE::MemoryToMemoryPfc,
            2 => MODE::MemoryToMemoryPfcblending,
            3 => MODE::RegisterToMemory,
            _ => unreachable!(),
        }
    }
    #[doc = "Memory-to-memory (FG fetch only)"]
    #[inline(always)]
    pub fn is_memory_to_memory(&self) -> bool {
        *self == MODE::MemoryToMemory
    }
    #[doc = "Memory-to-memory with PFC (FG fetch only with FG PFC active)"]
    #[inline(always)]
    pub fn is_memory_to_memory_pfc(&self) -> bool {
        *self == MODE::MemoryToMemoryPfc
    }
    #[doc = "Memory-to-memory with blending (FG and BG fetch with PFC and blending)"]
    #[inline(always)]
    pub fn is_memory_to_memory_pfcblending(&self) -> bool {
        *self == MODE::MemoryToMemoryPfcblending
    }
    #[doc = "Register-to-memory"]
    #[inline(always)]
    pub fn is_register_to_memory(&self) -> bool {
        *self == MODE::RegisterToMemory
    }
}
#[doc = "Field `MODE` writer - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing."]
pub type MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODE>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Memory-to-memory (FG fetch only)"]
    #[inline(always)]
    pub fn memory_to_memory(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MemoryToMemory)
    }
    #[doc = "Memory-to-memory with PFC (FG fetch only with FG PFC active)"]
    #[inline(always)]
    pub fn memory_to_memory_pfc(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MemoryToMemoryPfc)
    }
    #[doc = "Memory-to-memory with blending (FG and BG fetch with PFC and blending)"]
    #[inline(always)]
    pub fn memory_to_memory_pfcblending(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MemoryToMemoryPfcblending)
    }
    #[doc = "Register-to-memory"]
    #[inline(always)]
    pub fn register_to_memory(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::RegisterToMemory)
    }
}
impl R {
    #[doc = "Bit 0 - Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset."]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Transfer error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transfer watermark interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn twie(&self) -> TWIE_R {
        TWIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CLUT access error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn caeie(&self) -> CAEIE_R {
        CAEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CLUT transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ctcie(&self) -> CTCIE_R {
        CTCIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configuration Error Interrupt Enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ceie(&self) -> CEIE_R {
        CEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CRrs> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<CRrs> {
        SUSP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<CRrs> {
        ABORT_W::new(self, 2)
    }
    #[doc = "Bit 8 - Transfer error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<CRrs> {
        TEIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CRrs> {
        TCIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Transfer watermark interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn twie(&mut self) -> TWIE_W<CRrs> {
        TWIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - CLUT access error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn caeie(&mut self) -> CAEIE_W<CRrs> {
        CAEIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - CLUT transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn ctcie(&mut self) -> CTCIE_W<CRrs> {
        CTCIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configuration Error Interrupt Enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn ceie(&mut self) -> CEIE_W<CRrs> {
        CEIE_W::new(self, 13)
    }
    #[doc = "Bits 16:17 - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CRrs> {
        MODE_W::new(self, 16)
    }
}
#[doc = "DMA2D control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
