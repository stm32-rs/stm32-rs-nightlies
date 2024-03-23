#[doc = "Register `AHB1SMENR` reader"]
pub type R = crate::R<AHB1SMENRrs>;
#[doc = "Register `AHB1SMENR` writer"]
pub type W = crate::W<AHB1SMENRrs>;
#[doc = "DMA1 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1SMEN {
    #[doc = "0: DMAx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: DMAx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<DMA1SMEN> for bool {
    #[inline(always)]
    fn from(variant: DMA1SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1SMEN` reader - DMA1 clocks enable during Sleep and Stop modes"]
pub type DMA1SMEN_R = crate::BitReader<DMA1SMEN>;
impl DMA1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA1SMEN {
        match self.bits {
            false => DMA1SMEN::Disabled,
            true => DMA1SMEN::Enabled,
        }
    }
    #[doc = "DMAx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1SMEN::Disabled
    }
    #[doc = "DMAx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1SMEN::Enabled
    }
}
#[doc = "Field `DMA1SMEN` writer - DMA1 clocks enable during Sleep and Stop modes"]
pub type DMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1SMEN>;
impl<'a, REG> DMA1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMAx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1SMEN::Disabled)
    }
    #[doc = "DMAx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1SMEN::Enabled)
    }
}
#[doc = "Field `DMA2SMEN` reader - DMA2 clocks enable during Sleep and Stop modes"]
pub use DMA1SMEN_R as DMA2SMEN_R;
#[doc = "Field `DMA2SMEN` writer - DMA2 clocks enable during Sleep and Stop modes"]
pub use DMA1SMEN_W as DMA2SMEN_W;
#[doc = "DMAMUX clock enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAMUX1SMEN {
    #[doc = "0: DMAMUX1 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: DMAMUX1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<DMAMUX1SMEN> for bool {
    #[inline(always)]
    fn from(variant: DMAMUX1SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMUX1SMEN` reader - DMAMUX clock enable during Sleep and Stop modes"]
pub type DMAMUX1SMEN_R = crate::BitReader<DMAMUX1SMEN>;
impl DMAMUX1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAMUX1SMEN {
        match self.bits {
            false => DMAMUX1SMEN::Disabled,
            true => DMAMUX1SMEN::Enabled,
        }
    }
    #[doc = "DMAMUX1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAMUX1SMEN::Disabled
    }
    #[doc = "DMAMUX1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAMUX1SMEN::Enabled
    }
}
#[doc = "Field `DMAMUX1SMEN` writer - DMAMUX clock enable during Sleep and Stop modes"]
pub type DMAMUX1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAMUX1SMEN>;
impl<'a, REG> DMAMUX1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMAMUX1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAMUX1SMEN::Disabled)
    }
    #[doc = "DMAMUX1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAMUX1SMEN::Enabled)
    }
}
#[doc = "Flash memory interface clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHSMEN {
    #[doc = "0: Flash memory interface clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: Flash memory interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<FLASHSMEN> for bool {
    #[inline(always)]
    fn from(variant: FLASHSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHSMEN` reader - Flash memory interface clocks enable during Sleep and Stop modes"]
pub type FLASHSMEN_R = crate::BitReader<FLASHSMEN>;
impl FLASHSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLASHSMEN {
        match self.bits {
            false => FLASHSMEN::Disabled,
            true => FLASHSMEN::Enabled,
        }
    }
    #[doc = "Flash memory interface clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHSMEN::Disabled
    }
    #[doc = "Flash memory interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHSMEN::Enabled
    }
}
#[doc = "Field `FLASHSMEN` writer - Flash memory interface clocks enable during Sleep and Stop modes"]
pub type FLASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG, FLASHSMEN>;
impl<'a, REG> FLASHSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash memory interface clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHSMEN::Disabled)
    }
    #[doc = "Flash memory interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHSMEN::Enabled)
    }
}
#[doc = "SRAM1 interface clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM1SMEN {
    #[doc = "0: SRAM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: SRAM1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<SRAM1SMEN> for bool {
    #[inline(always)]
    fn from(variant: SRAM1SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM1SMEN` reader - SRAM1 interface clocks enable during Sleep and Stop modes"]
pub type SRAM1SMEN_R = crate::BitReader<SRAM1SMEN>;
impl SRAM1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM1SMEN {
        match self.bits {
            false => SRAM1SMEN::Disabled,
            true => SRAM1SMEN::Enabled,
        }
    }
    #[doc = "SRAM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM1SMEN::Disabled
    }
    #[doc = "SRAM1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM1SMEN::Enabled
    }
}
#[doc = "Field `SRAM1SMEN` writer - SRAM1 interface clocks enable during Sleep and Stop modes"]
pub type SRAM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, SRAM1SMEN>;
impl<'a, REG> SRAM1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1SMEN::Disabled)
    }
    #[doc = "SRAM1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM1SMEN::Enabled)
    }
}
#[doc = "CRCSMEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCSMEN {
    #[doc = "0: CRC clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: CRC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<CRCSMEN> for bool {
    #[inline(always)]
    fn from(variant: CRCSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCSMEN` reader - CRCSMEN"]
pub type CRCSMEN_R = crate::BitReader<CRCSMEN>;
impl CRCSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCSMEN {
        match self.bits {
            false => CRCSMEN::Disabled,
            true => CRCSMEN::Enabled,
        }
    }
    #[doc = "CRC clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCSMEN::Disabled
    }
    #[doc = "CRC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCSMEN::Enabled
    }
}
#[doc = "Field `CRCSMEN` writer - CRCSMEN"]
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG, CRCSMEN>;
impl<'a, REG> CRCSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSMEN::Disabled)
    }
    #[doc = "CRC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSMEN::Enabled)
    }
}
#[doc = "Touch Sensing Controller clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSCSMEN {
    #[doc = "0: TSC clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: TSC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<TSCSMEN> for bool {
    #[inline(always)]
    fn from(variant: TSCSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCSMEN` reader - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
pub type TSCSMEN_R = crate::BitReader<TSCSMEN>;
impl TSCSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSCSMEN {
        match self.bits {
            false => TSCSMEN::Disabled,
            true => TSCSMEN::Enabled,
        }
    }
    #[doc = "TSC clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSCSMEN::Disabled
    }
    #[doc = "TSC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSCSMEN::Enabled
    }
}
#[doc = "Field `TSCSMEN` writer - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
pub type TSCSMEN_W<'a, REG> = crate::BitWriter<'a, REG, TSCSMEN>;
impl<'a, REG> TSCSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TSC clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSCSMEN::Disabled)
    }
    #[doc = "TSC clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSCSMEN::Enabled)
    }
}
#[doc = "DMA2D clock enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA2DSMEN {
    #[doc = "0: DMA2D clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: DMA2D clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<DMA2DSMEN> for bool {
    #[inline(always)]
    fn from(variant: DMA2DSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA2DSMEN` reader - DMA2D clock enable during Sleep and Stop modes"]
pub type DMA2DSMEN_R = crate::BitReader<DMA2DSMEN>;
impl DMA2DSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA2DSMEN {
        match self.bits {
            false => DMA2DSMEN::Disabled,
            true => DMA2DSMEN::Enabled,
        }
    }
    #[doc = "DMA2D clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA2DSMEN::Disabled
    }
    #[doc = "DMA2D clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA2DSMEN::Enabled
    }
}
#[doc = "Field `DMA2DSMEN` writer - DMA2D clock enable during Sleep and Stop modes"]
pub type DMA2DSMEN_W<'a, REG> = crate::BitWriter<'a, REG, DMA2DSMEN>;
impl<'a, REG> DMA2DSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA2D clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA2DSMEN::Disabled)
    }
    #[doc = "DMA2D clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA2DSMEN::Enabled)
    }
}
#[doc = "GFXMMU clock enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GFXMMUSMEN {
    #[doc = "0: GFXMMU clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: GFXMMU clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<GFXMMUSMEN> for bool {
    #[inline(always)]
    fn from(variant: GFXMMUSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GFXMMUSMEN` reader - GFXMMU clock enable during Sleep and Stop modes"]
pub type GFXMMUSMEN_R = crate::BitReader<GFXMMUSMEN>;
impl GFXMMUSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GFXMMUSMEN {
        match self.bits {
            false => GFXMMUSMEN::Disabled,
            true => GFXMMUSMEN::Enabled,
        }
    }
    #[doc = "GFXMMU clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GFXMMUSMEN::Disabled
    }
    #[doc = "GFXMMU clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GFXMMUSMEN::Enabled
    }
}
#[doc = "Field `GFXMMUSMEN` writer - GFXMMU clock enable during Sleep and Stop modes"]
pub type GFXMMUSMEN_W<'a, REG> = crate::BitWriter<'a, REG, GFXMMUSMEN>;
impl<'a, REG> GFXMMUSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GFXMMU clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GFXMMUSMEN::Disabled)
    }
    #[doc = "GFXMMU clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GFXMMUSMEN::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUX clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dmamux1smen(&self) -> DMAMUX1SMEN_R {
        DMAMUX1SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CRCSMEN"]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tscsmen(&self) -> TSCSMEN_R {
        TSCSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA2D clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma2dsmen(&self) -> DMA2DSMEN_R {
        DMA2DSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GFXMMU clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gfxmmusmen(&self) -> GFXMMUSMEN_R {
        GFXMMUSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<AHB1SMENRrs> {
        DMA1SMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W<AHB1SMENRrs> {
        DMA2SMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMAMUX clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1smen(&mut self) -> DMAMUX1SMEN_W<AHB1SMENRrs> {
        DMAMUX1SMEN_W::new(self, 2)
    }
    #[doc = "Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<AHB1SMENRrs> {
        FLASHSMEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W<AHB1SMENRrs> {
        SRAM1SMEN_W::new(self, 9)
    }
    #[doc = "Bit 12 - CRCSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<AHB1SMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
    #[doc = "Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tscsmen(&mut self) -> TSCSMEN_W<AHB1SMENRrs> {
        TSCSMEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - DMA2D clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dma2dsmen(&mut self) -> DMA2DSMEN_W<AHB1SMENRrs> {
        DMA2DSMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - GFXMMU clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn gfxmmusmen(&mut self) -> GFXMMUSMEN_W<AHB1SMENRrs> {
        GFXMMUSMEN_W::new(self, 18)
    }
}
#[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1SMENRrs;
impl crate::RegisterSpec for AHB1SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1smenr::R`](R) reader structure"]
impl crate::Readable for AHB1SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb1smenr::W`](W) writer structure"]
impl crate::Writable for AHB1SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1SMENR to value 0x0007_1307"]
impl crate::Resettable for AHB1SMENRrs {
    const RESET_VALUE: u32 = 0x0007_1307;
}
