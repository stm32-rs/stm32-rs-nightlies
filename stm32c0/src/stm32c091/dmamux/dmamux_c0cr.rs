///Register `DMAMUX_C0CR` reader
pub type R = crate::R<DMAMUX_C0CRrs>;
///Register `DMAMUX_C0CR` writer
pub type W = crate::W<DMAMUX_C0CRrs>;
///Field `DMAREQ_ID` reader - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources.
pub type DMAREQ_ID_R = crate::FieldReader;
///Field `DMAREQ_ID` writer - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources.
pub type DMAREQ_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
/**Synchronization overrun interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOIE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<SOIE> for bool {
    #[inline(always)]
    fn from(variant: SOIE) -> Self {
        variant as u8 != 0
    }
}
///Field `SOIE` reader - Synchronization overrun interrupt enable
pub type SOIE_R = crate::BitReader<SOIE>;
impl SOIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SOIE {
        match self.bits {
            false => SOIE::B0x0,
            true => SOIE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SOIE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SOIE::B0x1
    }
}
///Field `SOIE` writer - Synchronization overrun interrupt enable
pub type SOIE_W<'a, REG> = crate::BitWriter<'a, REG, SOIE>;
impl<'a, REG> SOIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SOIE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SOIE::B0x1)
    }
}
/**Event generation enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EGE {
    ///0: Event generation disabled
    B0x0 = 0,
    ///1: Event generation enabled
    B0x1 = 1,
}
impl From<EGE> for bool {
    #[inline(always)]
    fn from(variant: EGE) -> Self {
        variant as u8 != 0
    }
}
///Field `EGE` reader - Event generation enable
pub type EGE_R = crate::BitReader<EGE>;
impl EGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EGE {
        match self.bits {
            false => EGE::B0x0,
            true => EGE::B0x1,
        }
    }
    ///Event generation disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EGE::B0x0
    }
    ///Event generation enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EGE::B0x1
    }
}
///Field `EGE` writer - Event generation enable
pub type EGE_W<'a, REG> = crate::BitWriter<'a, REG, EGE>;
impl<'a, REG> EGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Event generation disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EGE::B0x0)
    }
    ///Event generation enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EGE::B0x1)
    }
}
/**Synchronization enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SE {
    ///0: Synchronization disabled
    B0x0 = 0,
    ///1: Synchronization enabled
    B0x1 = 1,
}
impl From<SE> for bool {
    #[inline(always)]
    fn from(variant: SE) -> Self {
        variant as u8 != 0
    }
}
///Field `SE` reader - Synchronization enable
pub type SE_R = crate::BitReader<SE>;
impl SE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SE {
        match self.bits {
            false => SE::B0x0,
            true => SE::B0x1,
        }
    }
    ///Synchronization disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SE::B0x0
    }
    ///Synchronization enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SE::B0x1
    }
}
///Field `SE` writer - Synchronization enable
pub type SE_W<'a, REG> = crate::BitWriter<'a, REG, SE>;
impl<'a, REG> SE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Synchronization disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SE::B0x0)
    }
    ///Synchronization enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SE::B0x1)
    }
}
/**Synchronization polarity Defines the edge polarity of the selected synchronization input:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPOL {
    ///0: No event (no synchronization, no detection).
    B0x0 = 0,
    ///1: Rising edge
    B0x1 = 1,
    ///2: Falling edge
    B0x2 = 2,
    ///3: Rising and falling edges
    B0x3 = 3,
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
///Field `SPOL` reader - Synchronization polarity Defines the edge polarity of the selected synchronization input:
pub type SPOL_R = crate::FieldReader<SPOL>;
impl SPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPOL {
        match self.bits {
            0 => SPOL::B0x0,
            1 => SPOL::B0x1,
            2 => SPOL::B0x2,
            3 => SPOL::B0x3,
            _ => unreachable!(),
        }
    }
    ///No event (no synchronization, no detection).
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPOL::B0x0
    }
    ///Rising edge
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPOL::B0x1
    }
    ///Falling edge
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SPOL::B0x2
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SPOL::B0x3
    }
}
///Field `SPOL` writer - Synchronization polarity Defines the edge polarity of the selected synchronization input:
pub type SPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SPOL, crate::Safe>;
impl<'a, REG> SPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No event (no synchronization, no detection).
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::B0x0)
    }
    ///Rising edge
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::B0x1)
    }
    ///Falling edge
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::B0x2)
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::B0x3)
    }
}
///Field `NBREQ` reader - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field must only be written when both SE and EGE bits are low.
pub type NBREQ_R = crate::FieldReader;
///Field `NBREQ` writer - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field must only be written when both SE and EGE bits are low.
pub type NBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SYNC_ID` reader - Synchronization identification Selects the synchronization input (see Table 44: DMAMUX: assignment of synchronization inputs to resources).
pub type SYNC_ID_R = crate::FieldReader;
///Field `SYNC_ID` writer - Synchronization identification Selects the synchronization input (see Table 44: DMAMUX: assignment of synchronization inputs to resources).
pub type SYNC_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:5 - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources.
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 8 - Synchronization overrun interrupt enable
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Event generation enable
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - Synchronization enable
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - Synchronization polarity Defines the edge polarity of the selected synchronization input:
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field must only be written when both SE and EGE bits are low.
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 24:28 - Synchronization identification Selects the synchronization input (see Table 44: DMAMUX: assignment of synchronization inputs to resources).
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMUX_C0CR")
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
    ///Bits 0:5 - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources.
    #[inline(always)]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<'_, DMAMUX_C0CRrs> {
        DMAREQ_ID_W::new(self, 0)
    }
    ///Bit 8 - Synchronization overrun interrupt enable
    #[inline(always)]
    pub fn soie(&mut self) -> SOIE_W<'_, DMAMUX_C0CRrs> {
        SOIE_W::new(self, 8)
    }
    ///Bit 9 - Event generation enable
    #[inline(always)]
    pub fn ege(&mut self) -> EGE_W<'_, DMAMUX_C0CRrs> {
        EGE_W::new(self, 9)
    }
    ///Bit 16 - Synchronization enable
    #[inline(always)]
    pub fn se(&mut self) -> SE_W<'_, DMAMUX_C0CRrs> {
        SE_W::new(self, 16)
    }
    ///Bits 17:18 - Synchronization polarity Defines the edge polarity of the selected synchronization input:
    #[inline(always)]
    pub fn spol(&mut self) -> SPOL_W<'_, DMAMUX_C0CRrs> {
        SPOL_W::new(self, 17)
    }
    ///Bits 19:23 - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field must only be written when both SE and EGE bits are low.
    #[inline(always)]
    pub fn nbreq(&mut self) -> NBREQ_W<'_, DMAMUX_C0CRrs> {
        NBREQ_W::new(self, 19)
    }
    ///Bits 24:28 - Synchronization identification Selects the synchronization input (see Table 44: DMAMUX: assignment of synchronization inputs to resources).
    #[inline(always)]
    pub fn sync_id(&mut self) -> SYNC_ID_W<'_, DMAMUX_C0CRrs> {
        SYNC_ID_W::new(self, 24)
    }
}
/**DMAMUX request line multiplexer channel 0 configuration register

You can [`read`](crate::Reg::read) this register and get [`dmamux_c0cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_c0cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#DMAMUX:DMAMUX_C0CR)*/
pub struct DMAMUX_C0CRrs;
impl crate::RegisterSpec for DMAMUX_C0CRrs {
    type Ux = u32;
}
///`read()` method returns [`dmamux_c0cr::R`](R) reader structure
impl crate::Readable for DMAMUX_C0CRrs {}
///`write(|w| ..)` method takes [`dmamux_c0cr::W`](W) writer structure
impl crate::Writable for DMAMUX_C0CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAMUX_C0CR to value 0
impl crate::Resettable for DMAMUX_C0CRrs {}
