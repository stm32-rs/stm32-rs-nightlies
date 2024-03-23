#[doc = "Register `AHB1RSTR` reader"]
pub type R = crate::R<AHB1RSTRrs>;
#[doc = "Register `AHB1RSTR` writer"]
pub type W = crate::W<AHB1RSTRrs>;
#[doc = "DMA1 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1RST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset DMA1"]
    Reset = 1,
}
impl From<DMA1RST> for bool {
    #[inline(always)]
    fn from(variant: DMA1RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1RST` reader - DMA1 reset"]
pub type DMA1RST_R = crate::BitReader<DMA1RST>;
impl DMA1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA1RST {
        match self.bits {
            false => DMA1RST::NoEffect,
            true => DMA1RST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMA1RST::NoEffect
    }
    #[doc = "Reset DMA1"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMA1RST::Reset
    }
}
#[doc = "Field `DMA1RST` writer - DMA1 reset"]
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG, DMA1RST>;
impl<'a, REG> DMA1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RST::NoEffect)
    }
    #[doc = "Reset DMA1"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RST::Reset)
    }
}
#[doc = "DMA2 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA2RST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset DMA2"]
    Reset = 1,
}
impl From<DMA2RST> for bool {
    #[inline(always)]
    fn from(variant: DMA2RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA2RST` reader - DMA2 reset"]
pub type DMA2RST_R = crate::BitReader<DMA2RST>;
impl DMA2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA2RST {
        match self.bits {
            false => DMA2RST::NoEffect,
            true => DMA2RST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMA2RST::NoEffect
    }
    #[doc = "Reset DMA2"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMA2RST::Reset
    }
}
#[doc = "Field `DMA2RST` writer - DMA2 reset"]
pub type DMA2RST_W<'a, REG> = crate::BitWriter<'a, REG, DMA2RST>;
impl<'a, REG> DMA2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DMA2RST::NoEffect)
    }
    #[doc = "Reset DMA2"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DMA2RST::Reset)
    }
}
#[doc = "DMAMUXRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAMUX1RST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset DMAMUX1"]
    Reset = 1,
}
impl From<DMAMUX1RST> for bool {
    #[inline(always)]
    fn from(variant: DMAMUX1RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMUX1RST` reader - DMAMUXRST"]
pub type DMAMUX1RST_R = crate::BitReader<DMAMUX1RST>;
impl DMAMUX1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAMUX1RST {
        match self.bits {
            false => DMAMUX1RST::NoEffect,
            true => DMAMUX1RST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMAMUX1RST::NoEffect
    }
    #[doc = "Reset DMAMUX1"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMAMUX1RST::Reset
    }
}
#[doc = "Field `DMAMUX1RST` writer - DMAMUXRST"]
pub type DMAMUX1RST_W<'a, REG> = crate::BitWriter<'a, REG, DMAMUX1RST>;
impl<'a, REG> DMAMUX1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DMAMUX1RST::NoEffect)
    }
    #[doc = "Reset DMAMUX1"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DMAMUX1RST::Reset)
    }
}
#[doc = "Flash memory interface reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHRST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset Flash memory interface"]
    Reset = 1,
}
impl From<FLASHRST> for bool {
    #[inline(always)]
    fn from(variant: FLASHRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHRST` reader - Flash memory interface reset"]
pub type FLASHRST_R = crate::BitReader<FLASHRST>;
impl FLASHRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLASHRST {
        match self.bits {
            false => FLASHRST::NoEffect,
            true => FLASHRST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == FLASHRST::NoEffect
    }
    #[doc = "Reset Flash memory interface"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FLASHRST::Reset
    }
}
#[doc = "Field `FLASHRST` writer - Flash memory interface reset"]
pub type FLASHRST_W<'a, REG> = crate::BitWriter<'a, REG, FLASHRST>;
impl<'a, REG> FLASHRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHRST::NoEffect)
    }
    #[doc = "Reset Flash memory interface"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHRST::Reset)
    }
}
#[doc = "CRC reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCRST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset CRC"]
    Reset = 1,
}
impl From<CRCRST> for bool {
    #[inline(always)]
    fn from(variant: CRCRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCRST` reader - CRC reset"]
pub type CRCRST_R = crate::BitReader<CRCRST>;
impl CRCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCRST {
        match self.bits {
            false => CRCRST::NoEffect,
            true => CRCRST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CRCRST::NoEffect
    }
    #[doc = "Reset CRC"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CRCRST::Reset
    }
}
#[doc = "Field `CRCRST` writer - CRC reset"]
pub type CRCRST_W<'a, REG> = crate::BitWriter<'a, REG, CRCRST>;
impl<'a, REG> CRCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(CRCRST::NoEffect)
    }
    #[doc = "Reset CRC"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(CRCRST::Reset)
    }
}
#[doc = "Touch Sensing Controller reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSCRST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset TSC"]
    Reset = 1,
}
impl From<TSCRST> for bool {
    #[inline(always)]
    fn from(variant: TSCRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCRST` reader - Touch Sensing Controller reset"]
pub type TSCRST_R = crate::BitReader<TSCRST>;
impl TSCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSCRST {
        match self.bits {
            false => TSCRST::NoEffect,
            true => TSCRST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TSCRST::NoEffect
    }
    #[doc = "Reset TSC"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TSCRST::Reset
    }
}
#[doc = "Field `TSCRST` writer - Touch Sensing Controller reset"]
pub type TSCRST_W<'a, REG> = crate::BitWriter<'a, REG, TSCRST>;
impl<'a, REG> TSCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TSCRST::NoEffect)
    }
    #[doc = "Reset TSC"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TSCRST::Reset)
    }
}
#[doc = "DMA2D reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA2DRST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset DMA2D"]
    Reset = 1,
}
impl From<DMA2DRST> for bool {
    #[inline(always)]
    fn from(variant: DMA2DRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA2DRST` reader - DMA2D reset"]
pub type DMA2DRST_R = crate::BitReader<DMA2DRST>;
impl DMA2DRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA2DRST {
        match self.bits {
            false => DMA2DRST::NoEffect,
            true => DMA2DRST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMA2DRST::NoEffect
    }
    #[doc = "Reset DMA2D"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMA2DRST::Reset
    }
}
#[doc = "Field `DMA2DRST` writer - DMA2D reset"]
pub type DMA2DRST_W<'a, REG> = crate::BitWriter<'a, REG, DMA2DRST>;
impl<'a, REG> DMA2DRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DMA2DRST::NoEffect)
    }
    #[doc = "Reset DMA2D"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DMA2DRST::Reset)
    }
}
#[doc = "GFXMMU reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GFXMMURST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset GFXMMU"]
    Reset = 1,
}
impl From<GFXMMURST> for bool {
    #[inline(always)]
    fn from(variant: GFXMMURST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GFXMMURST` reader - GFXMMU reset"]
pub type GFXMMURST_R = crate::BitReader<GFXMMURST>;
impl GFXMMURST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GFXMMURST {
        match self.bits {
            false => GFXMMURST::NoEffect,
            true => GFXMMURST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == GFXMMURST::NoEffect
    }
    #[doc = "Reset GFXMMU"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GFXMMURST::Reset
    }
}
#[doc = "Field `GFXMMURST` writer - GFXMMU reset"]
pub type GFXMMURST_W<'a, REG> = crate::BitWriter<'a, REG, GFXMMURST>;
impl<'a, REG> GFXMMURST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(GFXMMURST::NoEffect)
    }
    #[doc = "Reset GFXMMU"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(GFXMMURST::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUXRST"]
    #[inline(always)]
    pub fn dmamux1rst(&self) -> DMAMUX1RST_R {
        DMAMUX1RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface reset"]
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Touch Sensing Controller reset"]
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA2D reset"]
    #[inline(always)]
    pub fn dma2drst(&self) -> DMA2DRST_R {
        DMA2DRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GFXMMU reset"]
    #[inline(always)]
    pub fn gfxmmurst(&self) -> GFXMMURST_R {
        GFXMMURST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> DMA1RST_W<AHB1RSTRrs> {
        DMA1RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> DMA2RST_W<AHB1RSTRrs> {
        DMA2RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMAMUXRST"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1rst(&mut self) -> DMAMUX1RST_W<AHB1RSTRrs> {
        DMAMUX1RST_W::new(self, 2)
    }
    #[doc = "Bit 8 - Flash memory interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn flashrst(&mut self) -> FLASHRST_W<AHB1RSTRrs> {
        FLASHRST_W::new(self, 8)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<AHB1RSTRrs> {
        CRCRST_W::new(self, 12)
    }
    #[doc = "Bit 16 - Touch Sensing Controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn tscrst(&mut self) -> TSCRST_W<AHB1RSTRrs> {
        TSCRST_W::new(self, 16)
    }
    #[doc = "Bit 17 - DMA2D reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2drst(&mut self) -> DMA2DRST_W<AHB1RSTRrs> {
        DMA2DRST_W::new(self, 17)
    }
    #[doc = "Bit 18 - GFXMMU reset"]
    #[inline(always)]
    #[must_use]
    pub fn gfxmmurst(&mut self) -> GFXMMURST_W<AHB1RSTRrs> {
        GFXMMURST_W::new(self, 18)
    }
}
#[doc = "AHB1 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1RSTRrs;
impl crate::RegisterSpec for AHB1RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1rstr::R`](R) reader structure"]
impl crate::Readable for AHB1RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb1rstr::W`](W) writer structure"]
impl crate::Writable for AHB1RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1RSTR to value 0"]
impl crate::Resettable for AHB1RSTRrs {
    const RESET_VALUE: u32 = 0;
}
