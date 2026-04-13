///Register `RGCR%s` reader
pub type R = crate::R<RGCRrs>;
///Register `RGCR%s` writer
pub type W = crate::W<RGCRrs>;
///Field `SIG_ID` reader - Signal ID
pub type SIG_ID_R = crate::FieldReader;
///Field `SIG_ID` writer - Signal ID
pub type SIG_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Overrun Interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIE {
    ///0: Trigger overrun interrupt disabled
    Disabled = 0,
    ///1: Trigger overrun interrupt enabled
    Enabled = 1,
}
impl From<OIE> for bool {
    #[inline(always)]
    fn from(variant: OIE) -> Self {
        variant as u8 != 0
    }
}
///Field `OIE` reader - Overrun Interrupt Enable
pub type OIE_R = crate::BitReader<OIE>;
impl OIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OIE {
        match self.bits {
            false => OIE::Disabled,
            true => OIE::Enabled,
        }
    }
    ///Trigger overrun interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OIE::Disabled
    }
    ///Trigger overrun interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OIE::Enabled
    }
}
///Field `OIE` writer - Overrun Interrupt Enable
pub type OIE_W<'a, REG> = crate::BitWriter<'a, REG, OIE>;
impl<'a, REG> OIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger overrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OIE::Disabled)
    }
    ///Trigger overrun interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OIE::Enabled)
    }
}
/**Generation Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GE {
    ///0: DMA request generation disabled
    Disabled = 0,
    ///1: DMA request enabled
    Enabled = 1,
}
impl From<GE> for bool {
    #[inline(always)]
    fn from(variant: GE) -> Self {
        variant as u8 != 0
    }
}
///Field `GE` reader - Generation Enable
pub type GE_R = crate::BitReader<GE>;
impl GE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GE {
        match self.bits {
            false => GE::Disabled,
            true => GE::Enabled,
        }
    }
    ///DMA request generation disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GE::Disabled
    }
    ///DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GE::Enabled
    }
}
///Field `GE` writer - Generation Enable
pub type GE_W<'a, REG> = crate::BitWriter<'a, REG, GE>;
impl<'a, REG> GE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA request generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GE::Disabled)
    }
    ///DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GE::Enabled)
    }
}
/**Generation Polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPOL {
    ///0: No event, i.e. no detection nor generation
    NoEdge = 0,
    ///1: Rising edge
    RisingEdge = 1,
    ///2: Falling edge
    FallingEdge = 2,
    ///3: Rising and falling edges
    BothEdges = 3,
}
impl From<GPOL> for u8 {
    #[inline(always)]
    fn from(variant: GPOL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPOL {
    type Ux = u8;
}
impl crate::IsEnum for GPOL {}
///Field `GPOL` reader - Generation Polarity
pub type GPOL_R = crate::FieldReader<GPOL>;
impl GPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPOL {
        match self.bits {
            0 => GPOL::NoEdge,
            1 => GPOL::RisingEdge,
            2 => GPOL::FallingEdge,
            3 => GPOL::BothEdges,
            _ => unreachable!(),
        }
    }
    ///No event, i.e. no detection nor generation
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == GPOL::NoEdge
    }
    ///Rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == GPOL::RisingEdge
    }
    ///Falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == GPOL::FallingEdge
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == GPOL::BothEdges
    }
}
///Field `GPOL` writer - Generation Polarity
pub type GPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, GPOL, crate::Safe>;
impl<'a, REG> GPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No event, i.e. no detection nor generation
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::NoEdge)
    }
    ///Rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::RisingEdge)
    }
    ///Falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::FallingEdge)
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::BothEdges)
    }
}
///Field `GNBREQ` reader - Number of Request
pub type GNBREQ_R = crate::FieldReader;
///Field `GNBREQ` writer - Number of Request
pub type GNBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    ///Bits 0:4 - Signal ID
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 8 - Overrun Interrupt Enable
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - Generation Enable
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - Generation Polarity
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of Request
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGCR")
            .field("gnbreq", &self.gnbreq())
            .field("gpol", &self.gpol())
            .field("ge", &self.ge())
            .field("oie", &self.oie())
            .field("sig_id", &self.sig_id())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Signal ID
    #[inline(always)]
    pub fn sig_id(&mut self) -> SIG_ID_W<'_, RGCRrs> {
        SIG_ID_W::new(self, 0)
    }
    ///Bit 8 - Overrun Interrupt Enable
    #[inline(always)]
    pub fn oie(&mut self) -> OIE_W<'_, RGCRrs> {
        OIE_W::new(self, 8)
    }
    ///Bit 16 - Generation Enable
    #[inline(always)]
    pub fn ge(&mut self) -> GE_W<'_, RGCRrs> {
        GE_W::new(self, 16)
    }
    ///Bits 17:18 - Generation Polarity
    #[inline(always)]
    pub fn gpol(&mut self) -> GPOL_W<'_, RGCRrs> {
        GPOL_W::new(self, 17)
    }
    ///Bits 19:23 - Number of Request
    #[inline(always)]
    pub fn gnbreq(&mut self) -> GNBREQ_W<'_, RGCRrs> {
        GNBREQ_W::new(self, 19)
    }
}
/**DMA Request Generator %s Control Register

You can [`read`](crate::Reg::read) this register and get [`rgcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#DMAMUX1:RGCR[0])*/
pub struct RGCRrs;
impl crate::RegisterSpec for RGCRrs {
    type Ux = u32;
}
///`read()` method returns [`rgcr::R`](R) reader structure
impl crate::Readable for RGCRrs {}
///`write(|w| ..)` method takes [`rgcr::W`](W) writer structure
impl crate::Writable for RGCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RGCR%s to value 0
impl crate::Resettable for RGCRrs {}
