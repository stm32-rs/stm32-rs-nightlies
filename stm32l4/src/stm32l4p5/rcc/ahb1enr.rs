///Register `AHB1ENR` reader
pub type R = crate::R<AHB1ENRrs>;
///Register `AHB1ENR` writer
pub type W = crate::W<AHB1ENRrs>;
/**DMA1 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1EN {
    ///0: DMAx clock disabled
    Disabled = 0,
    ///1: DMAx clock enabled
    Enabled = 1,
}
impl From<DMA1EN> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1EN` reader - DMA1 clock enable
pub type DMA1EN_R = crate::BitReader<DMA1EN>;
impl DMA1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMA1EN {
        match self.bits {
            false => DMA1EN::Disabled,
            true => DMA1EN::Enabled,
        }
    }
    ///DMAx clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1EN::Disabled
    }
    ///DMAx clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1EN::Enabled
    }
}
///Field `DMA1EN` writer - DMA1 clock enable
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1EN>;
impl<'a, REG> DMA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMAx clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Disabled)
    }
    ///DMAx clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Enabled)
    }
}
///Field `DMA2EN` reader - DMA2 clock enable
pub use DMA1EN_R as DMA2EN_R;
///Field `DMA2EN` writer - DMA2 clock enable
pub use DMA1EN_W as DMA2EN_W;
/**DMAMUX clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAMUX1EN {
    ///0: DMAMUX1 clock disabled
    Disabled = 0,
    ///1: DMAMUX1 clock enabled
    Enabled = 1,
}
impl From<DMAMUX1EN> for bool {
    #[inline(always)]
    fn from(variant: DMAMUX1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAMUX1EN` reader - DMAMUX clock enable
pub type DMAMUX1EN_R = crate::BitReader<DMAMUX1EN>;
impl DMAMUX1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAMUX1EN {
        match self.bits {
            false => DMAMUX1EN::Disabled,
            true => DMAMUX1EN::Enabled,
        }
    }
    ///DMAMUX1 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAMUX1EN::Disabled
    }
    ///DMAMUX1 clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAMUX1EN::Enabled
    }
}
///Field `DMAMUX1EN` writer - DMAMUX clock enable
pub type DMAMUX1EN_W<'a, REG> = crate::BitWriter<'a, REG, DMAMUX1EN>;
impl<'a, REG> DMAMUX1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMAMUX1 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAMUX1EN::Disabled)
    }
    ///DMAMUX1 clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAMUX1EN::Enabled)
    }
}
/**Flash memory interface clock enable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHEN {
    ///0: Flash memory interface clock disabled
    Disabled = 0,
    ///1: Flash memory interface clock enabled
    Enabled = 1,
}
impl From<FLASHEN> for bool {
    #[inline(always)]
    fn from(variant: FLASHEN) -> Self {
        variant as u8 != 0
    }
}
///Field `FLASHEN` reader - Flash memory interface clock enable
pub type FLASHEN_R = crate::BitReader<FLASHEN>;
impl FLASHEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLASHEN {
        match self.bits {
            false => FLASHEN::Disabled,
            true => FLASHEN::Enabled,
        }
    }
    ///Flash memory interface clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHEN::Disabled
    }
    ///Flash memory interface clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHEN::Enabled
    }
}
///Field `FLASHEN` writer - Flash memory interface clock enable
pub type FLASHEN_W<'a, REG> = crate::BitWriter<'a, REG, FLASHEN>;
impl<'a, REG> FLASHEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash memory interface clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHEN::Disabled)
    }
    ///Flash memory interface clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHEN::Enabled)
    }
}
/**CRC clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEN {
    ///0: CRC clock disabled
    Disabled = 0,
    ///1: CRC clock enabled
    Enabled = 1,
}
impl From<CRCEN> for bool {
    #[inline(always)]
    fn from(variant: CRCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCEN` reader - CRC clock enable
pub type CRCEN_R = crate::BitReader<CRCEN>;
impl CRCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCEN {
        match self.bits {
            false => CRCEN::Disabled,
            true => CRCEN::Enabled,
        }
    }
    ///CRC clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCEN::Disabled
    }
    ///CRC clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCEN::Enabled
    }
}
///Field `CRCEN` writer - CRC clock enable
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG, CRCEN>;
impl<'a, REG> CRCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CRC clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN::Disabled)
    }
    ///CRC clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN::Enabled)
    }
}
/**Touch Sensing Controller clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSCEN {
    ///0: TSC clock disabled
    Disabled = 0,
    ///1: TSC clock enabled
    Enabled = 1,
}
impl From<TSCEN> for bool {
    #[inline(always)]
    fn from(variant: TSCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TSCEN` reader - Touch Sensing Controller clock enable
pub type TSCEN_R = crate::BitReader<TSCEN>;
impl TSCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSCEN {
        match self.bits {
            false => TSCEN::Disabled,
            true => TSCEN::Enabled,
        }
    }
    ///TSC clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSCEN::Disabled
    }
    ///TSC clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSCEN::Enabled
    }
}
///Field `TSCEN` writer - Touch Sensing Controller clock enable
pub type TSCEN_W<'a, REG> = crate::BitWriter<'a, REG, TSCEN>;
impl<'a, REG> TSCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TSC clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSCEN::Disabled)
    }
    ///TSC clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSCEN::Enabled)
    }
}
/**DMA2D clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA2DEN {
    ///0: DMA2D clock disabled
    Disabled = 0,
    ///1: DMA2D clock enabled
    Enabled = 1,
}
impl From<DMA2DEN> for bool {
    #[inline(always)]
    fn from(variant: DMA2DEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA2DEN` reader - DMA2D clock enable
pub type DMA2DEN_R = crate::BitReader<DMA2DEN>;
impl DMA2DEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMA2DEN {
        match self.bits {
            false => DMA2DEN::Disabled,
            true => DMA2DEN::Enabled,
        }
    }
    ///DMA2D clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA2DEN::Disabled
    }
    ///DMA2D clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA2DEN::Enabled
    }
}
///Field `DMA2DEN` writer - DMA2D clock enable
pub type DMA2DEN_W<'a, REG> = crate::BitWriter<'a, REG, DMA2DEN>;
impl<'a, REG> DMA2DEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA2D clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA2DEN::Disabled)
    }
    ///DMA2D clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA2DEN::Enabled)
    }
}
/**Graphic MMU clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GFXMMUEN {
    ///0: GFXMMU clock disabled
    Disabled = 0,
    ///1: GFXMMU clock enabled
    Enabled = 1,
}
impl From<GFXMMUEN> for bool {
    #[inline(always)]
    fn from(variant: GFXMMUEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GFXMMUEN` reader - Graphic MMU clock enable
pub type GFXMMUEN_R = crate::BitReader<GFXMMUEN>;
impl GFXMMUEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GFXMMUEN {
        match self.bits {
            false => GFXMMUEN::Disabled,
            true => GFXMMUEN::Enabled,
        }
    }
    ///GFXMMU clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GFXMMUEN::Disabled
    }
    ///GFXMMU clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GFXMMUEN::Enabled
    }
}
///Field `GFXMMUEN` writer - Graphic MMU clock enable
pub type GFXMMUEN_W<'a, REG> = crate::BitWriter<'a, REG, GFXMMUEN>;
impl<'a, REG> GFXMMUEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GFXMMU clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GFXMMUEN::Disabled)
    }
    ///GFXMMU clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GFXMMUEN::Enabled)
    }
}
impl R {
    ///Bit 0 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMAMUX clock enable
    #[inline(always)]
    pub fn dmamux1en(&self) -> DMAMUX1EN_R {
        DMAMUX1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - Flash memory interface clock enable
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Touch Sensing Controller clock enable
    #[inline(always)]
    pub fn tscen(&self) -> TSCEN_R {
        TSCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DMA2D clock enable
    #[inline(always)]
    pub fn dma2den(&self) -> DMA2DEN_R {
        DMA2DEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Graphic MMU clock enable
    #[inline(always)]
    pub fn gfxmmuen(&self) -> GFXMMUEN_R {
        GFXMMUEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1ENR")
            .field("dma1en", &self.dma1en())
            .field("dma2en", &self.dma2en())
            .field("dmamux1en", &self.dmamux1en())
            .field("flashen", &self.flashen())
            .field("crcen", &self.crcen())
            .field("tscen", &self.tscen())
            .field("dma2den", &self.dma2den())
            .field("gfxmmuen", &self.gfxmmuen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<'_, AHB1ENRrs> {
        DMA1EN_W::new(self, 0)
    }
    ///Bit 1 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W<'_, AHB1ENRrs> {
        DMA2EN_W::new(self, 1)
    }
    ///Bit 2 - DMAMUX clock enable
    #[inline(always)]
    pub fn dmamux1en(&mut self) -> DMAMUX1EN_W<'_, AHB1ENRrs> {
        DMAMUX1EN_W::new(self, 2)
    }
    ///Bit 8 - Flash memory interface clock enable
    #[inline(always)]
    pub fn flashen(&mut self) -> FLASHEN_W<'_, AHB1ENRrs> {
        FLASHEN_W::new(self, 8)
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, AHB1ENRrs> {
        CRCEN_W::new(self, 12)
    }
    ///Bit 16 - Touch Sensing Controller clock enable
    #[inline(always)]
    pub fn tscen(&mut self) -> TSCEN_W<'_, AHB1ENRrs> {
        TSCEN_W::new(self, 16)
    }
    ///Bit 17 - DMA2D clock enable
    #[inline(always)]
    pub fn dma2den(&mut self) -> DMA2DEN_W<'_, AHB1ENRrs> {
        DMA2DEN_W::new(self, 17)
    }
    ///Bit 18 - Graphic MMU clock enable
    #[inline(always)]
    pub fn gfxmmuen(&mut self) -> GFXMMUEN_W<'_, AHB1ENRrs> {
        GFXMMUEN_W::new(self, 18)
    }
}
/**AHB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:AHB1ENR)*/
pub struct AHB1ENRrs;
impl crate::RegisterSpec for AHB1ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1enr::R`](R) reader structure
impl crate::Readable for AHB1ENRrs {}
///`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure
impl crate::Writable for AHB1ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1ENR to value 0x0100
impl crate::Resettable for AHB1ENRrs {
    const RESET_VALUE: u32 = 0x0100;
}
