#[doc = "Register `AHB1ENR` reader"]
pub type R = crate::R<AHB1ENRrs>;
#[doc = "Register `AHB1ENR` writer"]
pub type W = crate::W<AHB1ENRrs>;
#[doc = "DMA1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1EN {
    #[doc = "0: DMAx clock disabled"]
    Disabled = 0,
    #[doc = "1: DMAx clock enabled"]
    Enabled = 1,
}
impl From<DMA1EN> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type DMA1EN_R = crate::BitReader<DMA1EN>;
impl DMA1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA1EN {
        match self.bits {
            false => DMA1EN::Disabled,
            true => DMA1EN::Enabled,
        }
    }
    #[doc = "DMAx clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1EN::Disabled
    }
    #[doc = "DMAx clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1EN::Enabled
    }
}
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1EN>;
impl<'a, REG> DMA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMAx clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Disabled)
    }
    #[doc = "DMAx clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Enabled)
    }
}
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub use DMA1EN_R as DMA2EN_R;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub use DMA1EN_W as DMA2EN_W;
#[doc = "DMAMUX clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAMUX1EN {
    #[doc = "0: DMAMUX1 clock disabled"]
    Disabled = 0,
    #[doc = "1: DMAMUX1 clock enabled"]
    Enabled = 1,
}
impl From<DMAMUX1EN> for bool {
    #[inline(always)]
    fn from(variant: DMAMUX1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMUX1EN` reader - DMAMUX clock enable"]
pub type DMAMUX1EN_R = crate::BitReader<DMAMUX1EN>;
impl DMAMUX1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAMUX1EN {
        match self.bits {
            false => DMAMUX1EN::Disabled,
            true => DMAMUX1EN::Enabled,
        }
    }
    #[doc = "DMAMUX1 clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAMUX1EN::Disabled
    }
    #[doc = "DMAMUX1 clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAMUX1EN::Enabled
    }
}
#[doc = "Field `DMAMUX1EN` writer - DMAMUX clock enable"]
pub type DMAMUX1EN_W<'a, REG> = crate::BitWriter<'a, REG, DMAMUX1EN>;
impl<'a, REG> DMAMUX1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMAMUX1 clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAMUX1EN::Disabled)
    }
    #[doc = "DMAMUX1 clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAMUX1EN::Enabled)
    }
}
#[doc = "Flash memory interface clock enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHEN {
    #[doc = "0: Flash memory interface clock disabled"]
    Disabled = 0,
    #[doc = "1: Flash memory interface clock enabled"]
    Enabled = 1,
}
impl From<FLASHEN> for bool {
    #[inline(always)]
    fn from(variant: FLASHEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHEN` reader - Flash memory interface clock enable"]
pub type FLASHEN_R = crate::BitReader<FLASHEN>;
impl FLASHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLASHEN {
        match self.bits {
            false => FLASHEN::Disabled,
            true => FLASHEN::Enabled,
        }
    }
    #[doc = "Flash memory interface clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHEN::Disabled
    }
    #[doc = "Flash memory interface clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHEN::Enabled
    }
}
#[doc = "Field `FLASHEN` writer - Flash memory interface clock enable"]
pub type FLASHEN_W<'a, REG> = crate::BitWriter<'a, REG, FLASHEN>;
impl<'a, REG> FLASHEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash memory interface clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHEN::Disabled)
    }
    #[doc = "Flash memory interface clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHEN::Enabled)
    }
}
#[doc = "CRC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEN {
    #[doc = "0: CRC clock disabled"]
    Disabled = 0,
    #[doc = "1: CRC clock enabled"]
    Enabled = 1,
}
impl From<CRCEN> for bool {
    #[inline(always)]
    fn from(variant: CRCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CRCEN_R = crate::BitReader<CRCEN>;
impl CRCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCEN {
        match self.bits {
            false => CRCEN::Disabled,
            true => CRCEN::Enabled,
        }
    }
    #[doc = "CRC clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCEN::Disabled
    }
    #[doc = "CRC clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCEN::Enabled
    }
}
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG, CRCEN>;
impl<'a, REG> CRCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN::Disabled)
    }
    #[doc = "CRC clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN::Enabled)
    }
}
#[doc = "Touch Sensing Controller clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSCEN {
    #[doc = "0: TSC clock disabled"]
    Disabled = 0,
    #[doc = "1: TSC clock enabled"]
    Enabled = 1,
}
impl From<TSCEN> for bool {
    #[inline(always)]
    fn from(variant: TSCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCEN` reader - Touch Sensing Controller clock enable"]
pub type TSCEN_R = crate::BitReader<TSCEN>;
impl TSCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSCEN {
        match self.bits {
            false => TSCEN::Disabled,
            true => TSCEN::Enabled,
        }
    }
    #[doc = "TSC clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSCEN::Disabled
    }
    #[doc = "TSC clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSCEN::Enabled
    }
}
#[doc = "Field `TSCEN` writer - Touch Sensing Controller clock enable"]
pub type TSCEN_W<'a, REG> = crate::BitWriter<'a, REG, TSCEN>;
impl<'a, REG> TSCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TSC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSCEN::Disabled)
    }
    #[doc = "TSC clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSCEN::Enabled)
    }
}
#[doc = "DMA2D clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA2DEN {
    #[doc = "0: DMA2D clock disabled"]
    Disabled = 0,
    #[doc = "1: DMA2D clock enabled"]
    Enabled = 1,
}
impl From<DMA2DEN> for bool {
    #[inline(always)]
    fn from(variant: DMA2DEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA2DEN` reader - DMA2D clock enable"]
pub type DMA2DEN_R = crate::BitReader<DMA2DEN>;
impl DMA2DEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA2DEN {
        match self.bits {
            false => DMA2DEN::Disabled,
            true => DMA2DEN::Enabled,
        }
    }
    #[doc = "DMA2D clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA2DEN::Disabled
    }
    #[doc = "DMA2D clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA2DEN::Enabled
    }
}
#[doc = "Field `DMA2DEN` writer - DMA2D clock enable"]
pub type DMA2DEN_W<'a, REG> = crate::BitWriter<'a, REG, DMA2DEN>;
impl<'a, REG> DMA2DEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA2D clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA2DEN::Disabled)
    }
    #[doc = "DMA2D clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA2DEN::Enabled)
    }
}
#[doc = "Graphic MMU clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GFXMMUEN {
    #[doc = "0: GFXMMU clock disabled"]
    Disabled = 0,
    #[doc = "1: GFXMMU clock enabled"]
    Enabled = 1,
}
impl From<GFXMMUEN> for bool {
    #[inline(always)]
    fn from(variant: GFXMMUEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GFXMMUEN` reader - Graphic MMU clock enable"]
pub type GFXMMUEN_R = crate::BitReader<GFXMMUEN>;
impl GFXMMUEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GFXMMUEN {
        match self.bits {
            false => GFXMMUEN::Disabled,
            true => GFXMMUEN::Enabled,
        }
    }
    #[doc = "GFXMMU clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GFXMMUEN::Disabled
    }
    #[doc = "GFXMMU clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GFXMMUEN::Enabled
    }
}
#[doc = "Field `GFXMMUEN` writer - Graphic MMU clock enable"]
pub type GFXMMUEN_W<'a, REG> = crate::BitWriter<'a, REG, GFXMMUEN>;
impl<'a, REG> GFXMMUEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GFXMMU clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GFXMMUEN::Disabled)
    }
    #[doc = "GFXMMU clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GFXMMUEN::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUX clock enable"]
    #[inline(always)]
    pub fn dmamux1en(&self) -> DMAMUX1EN_R {
        DMAMUX1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable"]
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Touch Sensing Controller clock enable"]
    #[inline(always)]
    pub fn tscen(&self) -> TSCEN_R {
        TSCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA2D clock enable"]
    #[inline(always)]
    pub fn dma2den(&self) -> DMA2DEN_R {
        DMA2DEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Graphic MMU clock enable"]
    #[inline(always)]
    pub fn gfxmmuen(&self) -> GFXMMUEN_R {
        GFXMMUEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<AHB1ENRrs> {
        DMA1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<AHB1ENRrs> {
        DMA2EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMAMUX clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1en(&mut self) -> DMAMUX1EN_W<AHB1ENRrs> {
        DMAMUX1EN_W::new(self, 2)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn flashen(&mut self) -> FLASHEN_W<AHB1ENRrs> {
        FLASHEN_W::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHB1ENRrs> {
        CRCEN_W::new(self, 12)
    }
    #[doc = "Bit 16 - Touch Sensing Controller clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tscen(&mut self) -> TSCEN_W<AHB1ENRrs> {
        TSCEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - DMA2D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2den(&mut self) -> DMA2DEN_W<AHB1ENRrs> {
        DMA2DEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Graphic MMU clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gfxmmuen(&mut self) -> GFXMMUEN_W<AHB1ENRrs> {
        GFXMMUEN_W::new(self, 18)
    }
}
#[doc = "AHB1 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1ENRrs;
impl crate::RegisterSpec for AHB1ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1enr::R`](R) reader structure"]
impl crate::Readable for AHB1ENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure"]
impl crate::Writable for AHB1ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1ENR to value 0x0100"]
impl crate::Resettable for AHB1ENRrs {
    const RESET_VALUE: u32 = 0x0100;
}
