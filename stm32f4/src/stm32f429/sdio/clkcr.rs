///Register `CLKCR` reader
pub type R = crate::R<CLKCRrs>;
///Register `CLKCR` writer
pub type W = crate::W<CLKCRrs>;
///Field `CLKDIV` reader - Clock divide factor
pub type CLKDIV_R = crate::FieldReader;
///Field `CLKDIV` writer - Clock divide factor
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
/**Clock enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN {
    ///0: Disable clock
    Disabled = 0,
    ///1: Enable clock
    Enabled = 1,
}
impl From<CLKEN> for bool {
    #[inline(always)]
    fn from(variant: CLKEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CLKEN` reader - Clock enable bit
pub type CLKEN_R = crate::BitReader<CLKEN>;
impl CLKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CLKEN {
        match self.bits {
            false => CLKEN::Disabled,
            true => CLKEN::Enabled,
        }
    }
    ///Disable clock
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKEN::Disabled
    }
    ///Enable clock
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKEN::Enabled
    }
}
///Field `CLKEN` writer - Clock enable bit
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG, CLKEN>;
impl<'a, REG> CLKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable clock
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN::Disabled)
    }
    ///Enable clock
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN::Enabled)
    }
}
/**Power saving configuration bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWRSAV {
    ///0: SDIO_CK clock is always enabled
    Enabled = 0,
    ///1: SDIO_CK is only enabled when the bus is active
    Disabled = 1,
}
impl From<PWRSAV> for bool {
    #[inline(always)]
    fn from(variant: PWRSAV) -> Self {
        variant as u8 != 0
    }
}
///Field `PWRSAV` reader - Power saving configuration bit
pub type PWRSAV_R = crate::BitReader<PWRSAV>;
impl PWRSAV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PWRSAV {
        match self.bits {
            false => PWRSAV::Enabled,
            true => PWRSAV::Disabled,
        }
    }
    ///SDIO_CK clock is always enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWRSAV::Enabled
    }
    ///SDIO_CK is only enabled when the bus is active
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWRSAV::Disabled
    }
}
///Field `PWRSAV` writer - Power saving configuration bit
pub type PWRSAV_W<'a, REG> = crate::BitWriter<'a, REG, PWRSAV>;
impl<'a, REG> PWRSAV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SDIO_CK clock is always enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSAV::Enabled)
    }
    ///SDIO_CK is only enabled when the bus is active
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSAV::Disabled)
    }
}
/**Clock divider bypass enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS {
    ///0: SDIOCLK is divided according to the CLKDIV value before driving the SDIO_CK output signal.
    Disabled = 0,
    ///1: SDIOCLK directly drives the SDIO_CK output signal
    Enabled = 1,
}
impl From<BYPASS> for bool {
    #[inline(always)]
    fn from(variant: BYPASS) -> Self {
        variant as u8 != 0
    }
}
///Field `BYPASS` reader - Clock divider bypass enable bit
pub type BYPASS_R = crate::BitReader<BYPASS>;
impl BYPASS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BYPASS {
        match self.bits {
            false => BYPASS::Disabled,
            true => BYPASS::Enabled,
        }
    }
    ///SDIOCLK is divided according to the CLKDIV value before driving the SDIO_CK output signal.
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BYPASS::Disabled
    }
    ///SDIOCLK directly drives the SDIO_CK output signal
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BYPASS::Enabled
    }
}
///Field `BYPASS` writer - Clock divider bypass enable bit
pub type BYPASS_W<'a, REG> = crate::BitWriter<'a, REG, BYPASS>;
impl<'a, REG> BYPASS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SDIOCLK is divided according to the CLKDIV value before driving the SDIO_CK output signal.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS::Disabled)
    }
    ///SDIOCLK directly drives the SDIO_CK output signal
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS::Enabled)
    }
}
/**Wide bus mode enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WIDBUS {
    ///0: 1 lane wide bus
    BusWidth1 = 0,
    ///1: 4 lane wide bus
    BusWidth4 = 1,
    ///2: 8 lane wide bus
    BusWidth8 = 2,
}
impl From<WIDBUS> for u8 {
    #[inline(always)]
    fn from(variant: WIDBUS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WIDBUS {
    type Ux = u8;
}
impl crate::IsEnum for WIDBUS {}
///Field `WIDBUS` reader - Wide bus mode enable bit
pub type WIDBUS_R = crate::FieldReader<WIDBUS>;
impl WIDBUS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<WIDBUS> {
        match self.bits {
            0 => Some(WIDBUS::BusWidth1),
            1 => Some(WIDBUS::BusWidth4),
            2 => Some(WIDBUS::BusWidth8),
            _ => None,
        }
    }
    ///1 lane wide bus
    #[inline(always)]
    pub fn is_bus_width1(&self) -> bool {
        *self == WIDBUS::BusWidth1
    }
    ///4 lane wide bus
    #[inline(always)]
    pub fn is_bus_width4(&self) -> bool {
        *self == WIDBUS::BusWidth4
    }
    ///8 lane wide bus
    #[inline(always)]
    pub fn is_bus_width8(&self) -> bool {
        *self == WIDBUS::BusWidth8
    }
}
///Field `WIDBUS` writer - Wide bus mode enable bit
pub type WIDBUS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WIDBUS>;
impl<'a, REG> WIDBUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 lane wide bus
    #[inline(always)]
    pub fn bus_width1(self) -> &'a mut crate::W<REG> {
        self.variant(WIDBUS::BusWidth1)
    }
    ///4 lane wide bus
    #[inline(always)]
    pub fn bus_width4(self) -> &'a mut crate::W<REG> {
        self.variant(WIDBUS::BusWidth4)
    }
    ///8 lane wide bus
    #[inline(always)]
    pub fn bus_width8(self) -> &'a mut crate::W<REG> {
        self.variant(WIDBUS::BusWidth8)
    }
}
/**SDIO_CK dephasing selection bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NEGEDGE {
    ///0: SDIO_CK generated on the rising edge
    Rising = 0,
    ///1: SDIO_CK generated on the falling edge
    Falling = 1,
}
impl From<NEGEDGE> for bool {
    #[inline(always)]
    fn from(variant: NEGEDGE) -> Self {
        variant as u8 != 0
    }
}
///Field `NEGEDGE` reader - SDIO_CK dephasing selection bit
pub type NEGEDGE_R = crate::BitReader<NEGEDGE>;
impl NEGEDGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NEGEDGE {
        match self.bits {
            false => NEGEDGE::Rising,
            true => NEGEDGE::Falling,
        }
    }
    ///SDIO_CK generated on the rising edge
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == NEGEDGE::Rising
    }
    ///SDIO_CK generated on the falling edge
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == NEGEDGE::Falling
    }
}
///Field `NEGEDGE` writer - SDIO_CK dephasing selection bit
pub type NEGEDGE_W<'a, REG> = crate::BitWriter<'a, REG, NEGEDGE>;
impl<'a, REG> NEGEDGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SDIO_CK generated on the rising edge
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(NEGEDGE::Rising)
    }
    ///SDIO_CK generated on the falling edge
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(NEGEDGE::Falling)
    }
}
/**HW Flow Control enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HWFC_EN {
    ///0: HW Flow Control is disabled
    Disabled = 0,
    ///1: HW Flow Control is enabled
    Enabled = 1,
}
impl From<HWFC_EN> for bool {
    #[inline(always)]
    fn from(variant: HWFC_EN) -> Self {
        variant as u8 != 0
    }
}
///Field `HWFC_EN` reader - HW Flow Control enable
pub type HWFC_EN_R = crate::BitReader<HWFC_EN>;
impl HWFC_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HWFC_EN {
        match self.bits {
            false => HWFC_EN::Disabled,
            true => HWFC_EN::Enabled,
        }
    }
    ///HW Flow Control is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HWFC_EN::Disabled
    }
    ///HW Flow Control is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HWFC_EN::Enabled
    }
}
///Field `HWFC_EN` writer - HW Flow Control enable
pub type HWFC_EN_W<'a, REG> = crate::BitWriter<'a, REG, HWFC_EN>;
impl<'a, REG> HWFC_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HW Flow Control is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HWFC_EN::Disabled)
    }
    ///HW Flow Control is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HWFC_EN::Enabled)
    }
}
impl R {
    ///Bits 0:7 - Clock divide factor
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Clock enable bit
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Power saving configuration bit
    #[inline(always)]
    pub fn pwrsav(&self) -> PWRSAV_R {
        PWRSAV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock divider bypass enable bit
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:12 - Wide bus mode enable bit
    #[inline(always)]
    pub fn widbus(&self) -> WIDBUS_R {
        WIDBUS_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - SDIO_CK dephasing selection bit
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - HW Flow Control enable
    #[inline(always)]
    pub fn hwfc_en(&self) -> HWFC_EN_R {
        HWFC_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKCR")
            .field("hwfc_en", &self.hwfc_en())
            .field("negedge", &self.negedge())
            .field("widbus", &self.widbus())
            .field("bypass", &self.bypass())
            .field("pwrsav", &self.pwrsav())
            .field("clken", &self.clken())
            .field("clkdiv", &self.clkdiv())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Clock divide factor
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<'_, CLKCRrs> {
        CLKDIV_W::new(self, 0)
    }
    ///Bit 8 - Clock enable bit
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W<'_, CLKCRrs> {
        CLKEN_W::new(self, 8)
    }
    ///Bit 9 - Power saving configuration bit
    #[inline(always)]
    pub fn pwrsav(&mut self) -> PWRSAV_W<'_, CLKCRrs> {
        PWRSAV_W::new(self, 9)
    }
    ///Bit 10 - Clock divider bypass enable bit
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W<'_, CLKCRrs> {
        BYPASS_W::new(self, 10)
    }
    ///Bits 11:12 - Wide bus mode enable bit
    #[inline(always)]
    pub fn widbus(&mut self) -> WIDBUS_W<'_, CLKCRrs> {
        WIDBUS_W::new(self, 11)
    }
    ///Bit 13 - SDIO_CK dephasing selection bit
    #[inline(always)]
    pub fn negedge(&mut self) -> NEGEDGE_W<'_, CLKCRrs> {
        NEGEDGE_W::new(self, 13)
    }
    ///Bit 14 - HW Flow Control enable
    #[inline(always)]
    pub fn hwfc_en(&mut self) -> HWFC_EN_W<'_, CLKCRrs> {
        HWFC_EN_W::new(self, 14)
    }
}
/**SDI clock control register

You can [`read`](crate::Reg::read) this register and get [`clkcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F429.html#SDIO:CLKCR)*/
pub struct CLKCRrs;
impl crate::RegisterSpec for CLKCRrs {
    type Ux = u32;
}
///`read()` method returns [`clkcr::R`](R) reader structure
impl crate::Readable for CLKCRrs {}
///`write(|w| ..)` method takes [`clkcr::W`](W) writer structure
impl crate::Writable for CLKCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLKCR to value 0
impl crate::Resettable for CLKCRrs {}
