///Register `DMAMUX_RG3CR` reader
pub type R = crate::R<DMAMUX_RG3CRrs>;
///Register `DMAMUX_RG3CR` writer
pub type W = crate::W<DMAMUX_RG3CRrs>;
///Field `SIG_ID` reader - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator
pub type SIG_ID_R = crate::FieldReader;
///Field `SIG_ID` writer - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator
pub type SIG_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Trigger overrun interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIE {
    ///0: Interrupt on a trigger overrun event occurrence is disabled
    B0x0 = 0,
    ///1: Interrupt on a trigger overrun event occurrence is enabled
    B0x1 = 1,
}
impl From<OIE> for bool {
    #[inline(always)]
    fn from(variant: OIE) -> Self {
        variant as u8 != 0
    }
}
///Field `OIE` reader - Trigger overrun interrupt enable
pub type OIE_R = crate::BitReader<OIE>;
impl OIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OIE {
        match self.bits {
            false => OIE::B0x0,
            true => OIE::B0x1,
        }
    }
    ///Interrupt on a trigger overrun event occurrence is disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OIE::B0x0
    }
    ///Interrupt on a trigger overrun event occurrence is enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OIE::B0x1
    }
}
///Field `OIE` writer - Trigger overrun interrupt enable
pub type OIE_W<'a, REG> = crate::BitWriter<'a, REG, OIE>;
impl<'a, REG> OIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt on a trigger overrun event occurrence is disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OIE::B0x0)
    }
    ///Interrupt on a trigger overrun event occurrence is enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OIE::B0x1)
    }
}
/**DMA request generator channel x enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GE {
    ///0: DMA request generator channel x disabled
    B0x0 = 0,
    ///1: DMA request generator channel x enabled
    B0x1 = 1,
}
impl From<GE> for bool {
    #[inline(always)]
    fn from(variant: GE) -> Self {
        variant as u8 != 0
    }
}
///Field `GE` reader - DMA request generator channel x enable
pub type GE_R = crate::BitReader<GE>;
impl GE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GE {
        match self.bits {
            false => GE::B0x0,
            true => GE::B0x1,
        }
    }
    ///DMA request generator channel x disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GE::B0x0
    }
    ///DMA request generator channel x enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GE::B0x1
    }
}
///Field `GE` writer - DMA request generator channel x enable
pub type GE_W<'a, REG> = crate::BitWriter<'a, REG, GE>;
impl<'a, REG> GE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA request generator channel x disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GE::B0x0)
    }
    ///DMA request generator channel x enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GE::B0x1)
    }
}
/**DMA request generator trigger polarity Defines the edge polarity of the selected trigger input

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPOL {
    ///0: No event, i.e. no trigger detection nor generation.
    B0x0 = 0,
    ///1: Rising edge
    B0x1 = 1,
    ///2: Falling edge
    B0x2 = 2,
    ///3: Rising and falling edges
    B0x3 = 3,
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
///Field `GPOL` reader - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input
pub type GPOL_R = crate::FieldReader<GPOL>;
impl GPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPOL {
        match self.bits {
            0 => GPOL::B0x0,
            1 => GPOL::B0x1,
            2 => GPOL::B0x2,
            3 => GPOL::B0x3,
            _ => unreachable!(),
        }
    }
    ///No event, i.e. no trigger detection nor generation.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPOL::B0x0
    }
    ///Rising edge
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPOL::B0x1
    }
    ///Falling edge
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == GPOL::B0x2
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == GPOL::B0x3
    }
}
///Field `GPOL` writer - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input
pub type GPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, GPOL, crate::Safe>;
impl<'a, REG> GPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No event, i.e. no trigger detection nor generation.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::B0x0)
    }
    ///Rising edge
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::B0x1)
    }
    ///Falling edge
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::B0x2)
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::B0x3)
    }
}
///Field `GNBREQ` reader - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field must be written only when GE bit is disabled.
pub type GNBREQ_R = crate::FieldReader;
///Field `GNBREQ` writer - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field must be written only when GE bit is disabled.
pub type GNBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 8 - Trigger overrun interrupt enable
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - DMA request generator channel x enable
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field must be written only when GE bit is disabled.
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMUX_RG3CR")
            .field("sig_id", &self.sig_id())
            .field("oie", &self.oie())
            .field("ge", &self.ge())
            .field("gpol", &self.gpol())
            .field("gnbreq", &self.gnbreq())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator
    #[inline(always)]
    pub fn sig_id(&mut self) -> SIG_ID_W<'_, DMAMUX_RG3CRrs> {
        SIG_ID_W::new(self, 0)
    }
    ///Bit 8 - Trigger overrun interrupt enable
    #[inline(always)]
    pub fn oie(&mut self) -> OIE_W<'_, DMAMUX_RG3CRrs> {
        OIE_W::new(self, 8)
    }
    ///Bit 16 - DMA request generator channel x enable
    #[inline(always)]
    pub fn ge(&mut self) -> GE_W<'_, DMAMUX_RG3CRrs> {
        GE_W::new(self, 16)
    }
    ///Bits 17:18 - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input
    #[inline(always)]
    pub fn gpol(&mut self) -> GPOL_W<'_, DMAMUX_RG3CRrs> {
        GPOL_W::new(self, 17)
    }
    ///Bits 19:23 - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field must be written only when GE bit is disabled.
    #[inline(always)]
    pub fn gnbreq(&mut self) -> GNBREQ_W<'_, DMAMUX_RG3CRrs> {
        GNBREQ_W::new(self, 19)
    }
}
/**DMAMUX request generator channel 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`dmamux_rg3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_rg3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#DMAMUX:DMAMUX_RG3CR)*/
pub struct DMAMUX_RG3CRrs;
impl crate::RegisterSpec for DMAMUX_RG3CRrs {
    type Ux = u32;
}
///`read()` method returns [`dmamux_rg3cr::R`](R) reader structure
impl crate::Readable for DMAMUX_RG3CRrs {}
///`write(|w| ..)` method takes [`dmamux_rg3cr::W`](W) writer structure
impl crate::Writable for DMAMUX_RG3CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAMUX_RG3CR to value 0
impl crate::Resettable for DMAMUX_RG3CRrs {}
