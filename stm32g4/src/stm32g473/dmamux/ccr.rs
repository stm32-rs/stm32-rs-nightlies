///Register `CCR%s` reader
pub type R = crate::R<CCRrs>;
///Register `CCR%s` writer
pub type W = crate::W<CCRrs>;
///Field `DMAREQ_ID` reader - Input DMA request line selected
pub type DMAREQ_ID_R = crate::FieldReader;
///Field `DMAREQ_ID` writer - Input DMA request line selected
pub type DMAREQ_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**Interrupt enable at synchronization event overrun

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOIE {
    ///0: Synchronization overrun interrupt disabled
    Disabled = 0,
    ///1: Synchronization overrun interrupt enabled
    Enabled = 1,
}
impl From<SOIE> for bool {
    #[inline(always)]
    fn from(variant: SOIE) -> Self {
        variant as u8 != 0
    }
}
///Field `SOIE` reader - Interrupt enable at synchronization event overrun
pub type SOIE_R = crate::BitReader<SOIE>;
impl SOIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SOIE {
        match self.bits {
            false => SOIE::Disabled,
            true => SOIE::Enabled,
        }
    }
    ///Synchronization overrun interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SOIE::Disabled
    }
    ///Synchronization overrun interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SOIE::Enabled
    }
}
///Field `SOIE` writer - Interrupt enable at synchronization event overrun
pub type SOIE_W<'a, REG> = crate::BitWriter<'a, REG, SOIE>;
impl<'a, REG> SOIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Synchronization overrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SOIE::Disabled)
    }
    ///Synchronization overrun interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SOIE::Enabled)
    }
}
/**Event generation enable/disable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EGE {
    ///0: Event generation disabled
    Disabled = 0,
    ///1: Event generation enabled
    Enabled = 1,
}
impl From<EGE> for bool {
    #[inline(always)]
    fn from(variant: EGE) -> Self {
        variant as u8 != 0
    }
}
///Field `EGE` reader - Event generation enable/disable
pub type EGE_R = crate::BitReader<EGE>;
impl EGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EGE {
        match self.bits {
            false => EGE::Disabled,
            true => EGE::Enabled,
        }
    }
    ///Event generation disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EGE::Disabled
    }
    ///Event generation enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EGE::Enabled
    }
}
///Field `EGE` writer - Event generation enable/disable
pub type EGE_W<'a, REG> = crate::BitWriter<'a, REG, EGE>;
impl<'a, REG> EGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Event generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EGE::Disabled)
    }
    ///Event generation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EGE::Enabled)
    }
}
/**Synchronous operating mode enable/disable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SE {
    ///0: Synchronization disabled
    Disabled = 0,
    ///1: Synchronization enabled
    Enabled = 1,
}
impl From<SE> for bool {
    #[inline(always)]
    fn from(variant: SE) -> Self {
        variant as u8 != 0
    }
}
///Field `SE` reader - Synchronous operating mode enable/disable
pub type SE_R = crate::BitReader<SE>;
impl SE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SE {
        match self.bits {
            false => SE::Disabled,
            true => SE::Enabled,
        }
    }
    ///Synchronization disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SE::Disabled
    }
    ///Synchronization enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SE::Enabled
    }
}
///Field `SE` writer - Synchronous operating mode enable/disable
pub type SE_W<'a, REG> = crate::BitWriter<'a, REG, SE>;
impl<'a, REG> SE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Synchronization disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SE::Disabled)
    }
    ///Synchronization enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SE::Enabled)
    }
}
/**Synchronization event type selector Defines the synchronization event on the selected synchronization input:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPOL {
    ///0: No event, i.e. no synchronization nor detection
    NoEdge = 0,
    ///1: Rising edge
    RisingEdge = 1,
    ///2: Falling edge
    FallingEdge = 2,
    ///3: Rising and falling edges
    BothEdges = 3,
}
impl From<SPOL> for u8 {
    #[inline(always)]
    fn from(variant: SPOL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPOL {
    type Ux = u8;
}
impl crate::IsEnum for SPOL {}
///Field `SPOL` reader - Synchronization event type selector Defines the synchronization event on the selected synchronization input:
pub type SPOL_R = crate::FieldReader<SPOL>;
impl SPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPOL {
        match self.bits {
            0 => SPOL::NoEdge,
            1 => SPOL::RisingEdge,
            2 => SPOL::FallingEdge,
            3 => SPOL::BothEdges,
            _ => unreachable!(),
        }
    }
    ///No event, i.e. no synchronization nor detection
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == SPOL::NoEdge
    }
    ///Rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SPOL::RisingEdge
    }
    ///Falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SPOL::FallingEdge
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == SPOL::BothEdges
    }
}
///Field `SPOL` writer - Synchronization event type selector Defines the synchronization event on the selected synchronization input:
pub type SPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SPOL, crate::Safe>;
impl<'a, REG> SPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No event, i.e. no synchronization nor detection
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::NoEdge)
    }
    ///Rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::RisingEdge)
    }
    ///Falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::FallingEdge)
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::BothEdges)
    }
}
///Field `NBREQ` reader - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset.
pub type NBREQ_R = crate::FieldReader;
///Field `NBREQ` writer - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset.
pub type NBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
///Field `SYNC_ID` reader - Synchronization input selected
pub type SYNC_ID_R = crate::FieldReader;
///Field `SYNC_ID` writer - Synchronization input selected
pub type SYNC_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:6 - Input DMA request line selected
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 8 - Interrupt enable at synchronization event overrun
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Event generation enable/disable
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - Synchronous operating mode enable/disable
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset.
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 24:28 - Synchronization input selected
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("dmareq_id", &self.dmareq_id())
            .field("soie", &self.soie())
            .field("ege", &self.ege())
            .field("se", &self.se())
            .field("spol", &self.spol())
            .field("nbreq", &self.nbreq())
            .field("sync_id", &self.sync_id())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Input DMA request line selected
    #[inline(always)]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<'_, CCRrs> {
        DMAREQ_ID_W::new(self, 0)
    }
    ///Bit 8 - Interrupt enable at synchronization event overrun
    #[inline(always)]
    pub fn soie(&mut self) -> SOIE_W<'_, CCRrs> {
        SOIE_W::new(self, 8)
    }
    ///Bit 9 - Event generation enable/disable
    #[inline(always)]
    pub fn ege(&mut self) -> EGE_W<'_, CCRrs> {
        EGE_W::new(self, 9)
    }
    ///Bit 16 - Synchronous operating mode enable/disable
    #[inline(always)]
    pub fn se(&mut self) -> SE_W<'_, CCRrs> {
        SE_W::new(self, 16)
    }
    ///Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:
    #[inline(always)]
    pub fn spol(&mut self) -> SPOL_W<'_, CCRrs> {
        SPOL_W::new(self, 17)
    }
    ///Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset.
    #[inline(always)]
    pub fn nbreq(&mut self) -> NBREQ_W<'_, CCRrs> {
        NBREQ_W::new(self, 19)
    }
    ///Bits 24:28 - Synchronization input selected
    #[inline(always)]
    pub fn sync_id(&mut self) -> SYNC_ID_W<'_, CCRrs> {
        SYNC_ID_W::new(self, 24)
    }
}
/**DMA Multiplexer Channel %s Control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#DMAMUX:CCR[0])*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR%s to value 0
impl crate::Resettable for CCRrs {}
