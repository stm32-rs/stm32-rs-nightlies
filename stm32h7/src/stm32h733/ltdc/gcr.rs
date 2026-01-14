///Register `GCR` reader
pub type R = crate::R<GCRrs>;
///Register `GCR` writer
pub type W = crate::W<GCRrs>;
/**LCD-TFT controller enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCEN {
    ///0: LCD-TFT controller disabled
    Disabled = 0,
    ///1: LCD-TFT controller enabled
    Enabled = 1,
}
impl From<LTDCEN> for bool {
    #[inline(always)]
    fn from(variant: LTDCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LTDCEN` reader - LCD-TFT controller enable bit
pub type LTDCEN_R = crate::BitReader<LTDCEN>;
impl LTDCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LTDCEN {
        match self.bits {
            false => LTDCEN::Disabled,
            true => LTDCEN::Enabled,
        }
    }
    ///LCD-TFT controller disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LTDCEN::Disabled
    }
    ///LCD-TFT controller enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LTDCEN::Enabled
    }
}
///Field `LTDCEN` writer - LCD-TFT controller enable bit
pub type LTDCEN_W<'a, REG> = crate::BitWriter<'a, REG, LTDCEN>;
impl<'a, REG> LTDCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LCD-TFT controller disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCEN::Disabled)
    }
    ///LCD-TFT controller enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCEN::Enabled)
    }
}
///Field `DBW` reader - Dither Blue Width
pub type DBW_R = crate::FieldReader;
///Field `DGW` reader - Dither Green Width
pub type DGW_R = crate::FieldReader;
///Field `DRW` reader - Dither Red Width
pub type DRW_R = crate::FieldReader;
/**Dither Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEN {
    ///0: Dither disabled
    Disabled = 0,
    ///1: Dither enabled
    Enabled = 1,
}
impl From<DEN> for bool {
    #[inline(always)]
    fn from(variant: DEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DEN` reader - Dither Enable
pub type DEN_R = crate::BitReader<DEN>;
impl DEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEN {
        match self.bits {
            false => DEN::Disabled,
            true => DEN::Enabled,
        }
    }
    ///Dither disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEN::Disabled
    }
    ///Dither enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEN::Enabled
    }
}
///Field `DEN` writer - Dither Enable
pub type DEN_W<'a, REG> = crate::BitWriter<'a, REG, DEN>;
impl<'a, REG> DEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Dither disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEN::Disabled)
    }
    ///Dither enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEN::Enabled)
    }
}
/**Pixel Clock Polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCPOL {
    ///0: Pixel clock on rising edge
    RisingEdge = 0,
    ///1: Pixel clock on falling edge
    FallingEdge = 1,
}
impl From<PCPOL> for bool {
    #[inline(always)]
    fn from(variant: PCPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `PCPOL` reader - Pixel Clock Polarity
pub type PCPOL_R = crate::BitReader<PCPOL>;
impl PCPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCPOL {
        match self.bits {
            false => PCPOL::RisingEdge,
            true => PCPOL::FallingEdge,
        }
    }
    ///Pixel clock on rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == PCPOL::RisingEdge
    }
    ///Pixel clock on falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == PCPOL::FallingEdge
    }
}
///Field `PCPOL` writer - Pixel Clock Polarity
pub type PCPOL_W<'a, REG> = crate::BitWriter<'a, REG, PCPOL>;
impl<'a, REG> PCPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pixel clock on rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PCPOL::RisingEdge)
    }
    ///Pixel clock on falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PCPOL::FallingEdge)
    }
}
/**Data Enable Polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEPOL {
    ///0: Data enable polarity is active low
    ActiveLow = 0,
    ///1: Data enable polarity is active high
    ActiveHigh = 1,
}
impl From<DEPOL> for bool {
    #[inline(always)]
    fn from(variant: DEPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `DEPOL` reader - Data Enable Polarity
pub type DEPOL_R = crate::BitReader<DEPOL>;
impl DEPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEPOL {
        match self.bits {
            false => DEPOL::ActiveLow,
            true => DEPOL::ActiveHigh,
        }
    }
    ///Data enable polarity is active low
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == DEPOL::ActiveLow
    }
    ///Data enable polarity is active high
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == DEPOL::ActiveHigh
    }
}
///Field `DEPOL` writer - Data Enable Polarity
pub type DEPOL_W<'a, REG> = crate::BitWriter<'a, REG, DEPOL>;
impl<'a, REG> DEPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data enable polarity is active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(DEPOL::ActiveLow)
    }
    ///Data enable polarity is active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(DEPOL::ActiveHigh)
    }
}
/**Vertical Synchronization Polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSPOL {
    ///0: Vertical synchronization polarity is active low
    ActiveLow = 0,
    ///1: Vertical synchronization polarity is active high
    ActiveHigh = 1,
}
impl From<VSPOL> for bool {
    #[inline(always)]
    fn from(variant: VSPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `VSPOL` reader - Vertical Synchronization Polarity
pub type VSPOL_R = crate::BitReader<VSPOL>;
impl VSPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VSPOL {
        match self.bits {
            false => VSPOL::ActiveLow,
            true => VSPOL::ActiveHigh,
        }
    }
    ///Vertical synchronization polarity is active low
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == VSPOL::ActiveLow
    }
    ///Vertical synchronization polarity is active high
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == VSPOL::ActiveHigh
    }
}
///Field `VSPOL` writer - Vertical Synchronization Polarity
pub type VSPOL_W<'a, REG> = crate::BitWriter<'a, REG, VSPOL>;
impl<'a, REG> VSPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Vertical synchronization polarity is active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(VSPOL::ActiveLow)
    }
    ///Vertical synchronization polarity is active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(VSPOL::ActiveHigh)
    }
}
/**Horizontal Synchronization Polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSPOL {
    ///0: Horizontal synchronization polarity is active low
    ActiveLow = 0,
    ///1: Horizontal synchronization polarity is active high
    ActiveHigh = 1,
}
impl From<HSPOL> for bool {
    #[inline(always)]
    fn from(variant: HSPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `HSPOL` reader - Horizontal Synchronization Polarity
pub type HSPOL_R = crate::BitReader<HSPOL>;
impl HSPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSPOL {
        match self.bits {
            false => HSPOL::ActiveLow,
            true => HSPOL::ActiveHigh,
        }
    }
    ///Horizontal synchronization polarity is active low
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == HSPOL::ActiveLow
    }
    ///Horizontal synchronization polarity is active high
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == HSPOL::ActiveHigh
    }
}
///Field `HSPOL` writer - Horizontal Synchronization Polarity
pub type HSPOL_W<'a, REG> = crate::BitWriter<'a, REG, HSPOL>;
impl<'a, REG> HSPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Horizontal synchronization polarity is active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(HSPOL::ActiveLow)
    }
    ///Horizontal synchronization polarity is active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(HSPOL::ActiveHigh)
    }
}
impl R {
    ///Bit 0 - LCD-TFT controller enable bit
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:6 - Dither Blue Width
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - Dither Green Width
    #[inline(always)]
    pub fn dgw(&self) -> DGW_R {
        DGW_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - Dither Red Width
    #[inline(always)]
    pub fn drw(&self) -> DRW_R {
        DRW_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 16 - Dither Enable
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 28 - Pixel Clock Polarity
    #[inline(always)]
    pub fn pcpol(&self) -> PCPOL_R {
        PCPOL_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Data Enable Polarity
    #[inline(always)]
    pub fn depol(&self) -> DEPOL_R {
        DEPOL_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Vertical Synchronization Polarity
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Horizontal Synchronization Polarity
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCR")
            .field("hspol", &self.hspol())
            .field("vspol", &self.vspol())
            .field("depol", &self.depol())
            .field("pcpol", &self.pcpol())
            .field("den", &self.den())
            .field("drw", &self.drw())
            .field("dgw", &self.dgw())
            .field("dbw", &self.dbw())
            .field("ltdcen", &self.ltdcen())
            .finish()
    }
}
impl W {
    ///Bit 0 - LCD-TFT controller enable bit
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W<'_, GCRrs> {
        LTDCEN_W::new(self, 0)
    }
    ///Bit 16 - Dither Enable
    #[inline(always)]
    pub fn den(&mut self) -> DEN_W<'_, GCRrs> {
        DEN_W::new(self, 16)
    }
    ///Bit 28 - Pixel Clock Polarity
    #[inline(always)]
    pub fn pcpol(&mut self) -> PCPOL_W<'_, GCRrs> {
        PCPOL_W::new(self, 28)
    }
    ///Bit 29 - Data Enable Polarity
    #[inline(always)]
    pub fn depol(&mut self) -> DEPOL_W<'_, GCRrs> {
        DEPOL_W::new(self, 29)
    }
    ///Bit 30 - Vertical Synchronization Polarity
    #[inline(always)]
    pub fn vspol(&mut self) -> VSPOL_W<'_, GCRrs> {
        VSPOL_W::new(self, 30)
    }
    ///Bit 31 - Horizontal Synchronization Polarity
    #[inline(always)]
    pub fn hspol(&mut self) -> HSPOL_W<'_, GCRrs> {
        HSPOL_W::new(self, 31)
    }
}
/**Global Control Register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#LTDC:GCR)*/
pub struct GCRrs;
impl crate::RegisterSpec for GCRrs {
    type Ux = u32;
}
///`read()` method returns [`gcr::R`](R) reader structure
impl crate::Readable for GCRrs {}
///`write(|w| ..)` method takes [`gcr::W`](W) writer structure
impl crate::Writable for GCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GCR to value 0x2220
impl crate::Resettable for GCRrs {
    const RESET_VALUE: u32 = 0x2220;
}
