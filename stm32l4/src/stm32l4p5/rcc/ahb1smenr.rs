///Register `AHB1SMENR` reader
pub type R = crate::R<AHB1SMENRrs>;
///Register `AHB1SMENR` writer
pub type W = crate::W<AHB1SMENRrs>;
/**DMA1 clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1SMEN {
    ///0: DMAx clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: DMAx clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<DMA1SMEN> for bool {
    #[inline(always)]
    fn from(variant: DMA1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1SMEN` reader - DMA1 clocks enable during Sleep and Stop modes
pub type DMA1SMEN_R = crate::BitReader<DMA1SMEN>;
impl DMA1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMA1SMEN {
        match self.bits {
            false => DMA1SMEN::Disabled,
            true => DMA1SMEN::Enabled,
        }
    }
    ///DMAx clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1SMEN::Disabled
    }
    ///DMAx clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1SMEN::Enabled
    }
}
///Field `DMA1SMEN` writer - DMA1 clocks enable during Sleep and Stop modes
pub type DMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1SMEN>;
impl<'a, REG> DMA1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMAx clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1SMEN::Disabled)
    }
    ///DMAx clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1SMEN::Enabled)
    }
}
///Field `DMA2SMEN` reader - DMA2 clocks enable during Sleep and Stop modes
pub use DMA1SMEN_R as DMA2SMEN_R;
///Field `DMA2SMEN` writer - DMA2 clocks enable during Sleep and Stop modes
pub use DMA1SMEN_W as DMA2SMEN_W;
/**DMAMUX clock enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAMUX1SMEN {
    ///0: DMAMUX1 clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: DMAMUX1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<DMAMUX1SMEN> for bool {
    #[inline(always)]
    fn from(variant: DMAMUX1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAMUX1SMEN` reader - DMAMUX clock enable during Sleep and Stop modes
pub type DMAMUX1SMEN_R = crate::BitReader<DMAMUX1SMEN>;
impl DMAMUX1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAMUX1SMEN {
        match self.bits {
            false => DMAMUX1SMEN::Disabled,
            true => DMAMUX1SMEN::Enabled,
        }
    }
    ///DMAMUX1 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAMUX1SMEN::Disabled
    }
    ///DMAMUX1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAMUX1SMEN::Enabled
    }
}
///Field `DMAMUX1SMEN` writer - DMAMUX clock enable during Sleep and Stop modes
pub type DMAMUX1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAMUX1SMEN>;
impl<'a, REG> DMAMUX1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMAMUX1 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAMUX1SMEN::Disabled)
    }
    ///DMAMUX1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAMUX1SMEN::Enabled)
    }
}
/**Flash memory interface clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHSMEN {
    ///0: Flash memory interface clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: Flash memory interface clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<FLASHSMEN> for bool {
    #[inline(always)]
    fn from(variant: FLASHSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `FLASHSMEN` reader - Flash memory interface clocks enable during Sleep and Stop modes
pub type FLASHSMEN_R = crate::BitReader<FLASHSMEN>;
impl FLASHSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLASHSMEN {
        match self.bits {
            false => FLASHSMEN::Disabled,
            true => FLASHSMEN::Enabled,
        }
    }
    ///Flash memory interface clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHSMEN::Disabled
    }
    ///Flash memory interface clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHSMEN::Enabled
    }
}
///Field `FLASHSMEN` writer - Flash memory interface clocks enable during Sleep and Stop modes
pub type FLASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG, FLASHSMEN>;
impl<'a, REG> FLASHSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash memory interface clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHSMEN::Disabled)
    }
    ///Flash memory interface clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHSMEN::Enabled)
    }
}
/**SRAM1 interface clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1SMEN {
    ///0: SRAM1 clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: SRAM1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<SRAM1SMEN> for bool {
    #[inline(always)]
    fn from(variant: SRAM1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM1SMEN` reader - SRAM1 interface clocks enable during Sleep and Stop modes
pub type SRAM1SMEN_R = crate::BitReader<SRAM1SMEN>;
impl SRAM1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM1SMEN {
        match self.bits {
            false => SRAM1SMEN::Disabled,
            true => SRAM1SMEN::Enabled,
        }
    }
    ///SRAM1 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM1SMEN::Disabled
    }
    ///SRAM1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM1SMEN::Enabled
    }
}
///Field `SRAM1SMEN` writer - SRAM1 interface clocks enable during Sleep and Stop modes
pub type SRAM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, SRAM1SMEN>;
impl<'a, REG> SRAM1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM1 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1SMEN::Disabled)
    }
    ///SRAM1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1SMEN::Enabled)
    }
}
/**CRCSMEN

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCSMEN {
    ///0: CRC clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: CRC clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<CRCSMEN> for bool {
    #[inline(always)]
    fn from(variant: CRCSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCSMEN` reader - CRCSMEN
pub type CRCSMEN_R = crate::BitReader<CRCSMEN>;
impl CRCSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCSMEN {
        match self.bits {
            false => CRCSMEN::Disabled,
            true => CRCSMEN::Enabled,
        }
    }
    ///CRC clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCSMEN::Disabled
    }
    ///CRC clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCSMEN::Enabled
    }
}
///Field `CRCSMEN` writer - CRCSMEN
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG, CRCSMEN>;
impl<'a, REG> CRCSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CRC clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSMEN::Disabled)
    }
    ///CRC clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSMEN::Enabled)
    }
}
/**Touch Sensing Controller clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSCSMEN {
    ///0: TSC clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: TSC clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<TSCSMEN> for bool {
    #[inline(always)]
    fn from(variant: TSCSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TSCSMEN` reader - Touch Sensing Controller clocks enable during Sleep and Stop modes
pub type TSCSMEN_R = crate::BitReader<TSCSMEN>;
impl TSCSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSCSMEN {
        match self.bits {
            false => TSCSMEN::Disabled,
            true => TSCSMEN::Enabled,
        }
    }
    ///TSC clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSCSMEN::Disabled
    }
    ///TSC clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSCSMEN::Enabled
    }
}
///Field `TSCSMEN` writer - Touch Sensing Controller clocks enable during Sleep and Stop modes
pub type TSCSMEN_W<'a, REG> = crate::BitWriter<'a, REG, TSCSMEN>;
impl<'a, REG> TSCSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TSC clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSCSMEN::Disabled)
    }
    ///TSC clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSCSMEN::Enabled)
    }
}
/**DMA2D clock enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA2DSMEN {
    ///0: DMA2D clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: DMA2D clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<DMA2DSMEN> for bool {
    #[inline(always)]
    fn from(variant: DMA2DSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA2DSMEN` reader - DMA2D clock enable during Sleep and Stop modes
pub type DMA2DSMEN_R = crate::BitReader<DMA2DSMEN>;
impl DMA2DSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMA2DSMEN {
        match self.bits {
            false => DMA2DSMEN::Disabled,
            true => DMA2DSMEN::Enabled,
        }
    }
    ///DMA2D clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA2DSMEN::Disabled
    }
    ///DMA2D clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA2DSMEN::Enabled
    }
}
///Field `DMA2DSMEN` writer - DMA2D clock enable during Sleep and Stop modes
pub type DMA2DSMEN_W<'a, REG> = crate::BitWriter<'a, REG, DMA2DSMEN>;
impl<'a, REG> DMA2DSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA2D clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA2DSMEN::Disabled)
    }
    ///DMA2D clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA2DSMEN::Enabled)
    }
}
/**GFXMMU clock enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GFXMMUSMEN {
    ///0: GFXMMU clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: GFXMMU clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<GFXMMUSMEN> for bool {
    #[inline(always)]
    fn from(variant: GFXMMUSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GFXMMUSMEN` reader - GFXMMU clock enable during Sleep and Stop modes
pub type GFXMMUSMEN_R = crate::BitReader<GFXMMUSMEN>;
impl GFXMMUSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GFXMMUSMEN {
        match self.bits {
            false => GFXMMUSMEN::Disabled,
            true => GFXMMUSMEN::Enabled,
        }
    }
    ///GFXMMU clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GFXMMUSMEN::Disabled
    }
    ///GFXMMU clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GFXMMUSMEN::Enabled
    }
}
///Field `GFXMMUSMEN` writer - GFXMMU clock enable during Sleep and Stop modes
pub type GFXMMUSMEN_W<'a, REG> = crate::BitWriter<'a, REG, GFXMMUSMEN>;
impl<'a, REG> GFXMMUSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///GFXMMU clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GFXMMUSMEN::Disabled)
    }
    ///GFXMMU clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GFXMMUSMEN::Enabled)
    }
}
impl R {
    ///Bit 0 - DMA1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMAMUX clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dmamux1smen(&self) -> DMAMUX1SMEN_R {
        DMAMUX1SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - CRCSMEN
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tscsmen(&self) -> TSCSMEN_R {
        TSCSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DMA2D clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dma2dsmen(&self) -> DMA2DSMEN_R {
        DMA2DSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - GFXMMU clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gfxmmusmen(&self) -> GFXMMUSMEN_R {
        GFXMMUSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1SMENR")
            .field("dma1smen", &self.dma1smen())
            .field("dma2smen", &self.dma2smen())
            .field("dmamux1smen", &self.dmamux1smen())
            .field("flashsmen", &self.flashsmen())
            .field("sram1smen", &self.sram1smen())
            .field("crcsmen", &self.crcsmen())
            .field("tscsmen", &self.tscsmen())
            .field("dma2dsmen", &self.dma2dsmen())
            .field("gfxmmusmen", &self.gfxmmusmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<'_, AHB1SMENRrs> {
        DMA1SMEN_W::new(self, 0)
    }
    ///Bit 1 - DMA2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W<'_, AHB1SMENRrs> {
        DMA2SMEN_W::new(self, 1)
    }
    ///Bit 2 - DMAMUX clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dmamux1smen(&mut self) -> DMAMUX1SMEN_W<'_, AHB1SMENRrs> {
        DMAMUX1SMEN_W::new(self, 2)
    }
    ///Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<'_, AHB1SMENRrs> {
        FLASHSMEN_W::new(self, 8)
    }
    ///Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W<'_, AHB1SMENRrs> {
        SRAM1SMEN_W::new(self, 9)
    }
    ///Bit 12 - CRCSMEN
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<'_, AHB1SMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
    ///Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tscsmen(&mut self) -> TSCSMEN_W<'_, AHB1SMENRrs> {
        TSCSMEN_W::new(self, 16)
    }
    ///Bit 17 - DMA2D clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dma2dsmen(&mut self) -> DMA2DSMEN_W<'_, AHB1SMENRrs> {
        DMA2DSMEN_W::new(self, 17)
    }
    ///Bit 18 - GFXMMU clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gfxmmusmen(&mut self) -> GFXMMUSMEN_W<'_, AHB1SMENRrs> {
        GFXMMUSMEN_W::new(self, 18)
    }
}
/**AHB1 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb1smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:AHB1SMENR)*/
pub struct AHB1SMENRrs;
impl crate::RegisterSpec for AHB1SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1smenr::R`](R) reader structure
impl crate::Readable for AHB1SMENRrs {}
///`write(|w| ..)` method takes [`ahb1smenr::W`](W) writer structure
impl crate::Writable for AHB1SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1SMENR to value 0x0007_1307
impl crate::Resettable for AHB1SMENRrs {
    const RESET_VALUE: u32 = 0x0007_1307;
}
