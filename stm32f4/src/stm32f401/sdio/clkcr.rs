#[doc = "Register `CLKCR` reader"]
pub type R = crate::R<CLKCRrs>;
#[doc = "Register `CLKCR` writer"]
pub type W = crate::W<CLKCRrs>;
#[doc = "Field `CLKDIV` reader - Clock divide factor"]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock divide factor"]
pub type CLKDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Clock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN {
    #[doc = "0: Disable clock"]
    Disabled = 0,
    #[doc = "1: Enable clock"]
    Enabled = 1,
}
impl From<CLKEN> for bool {
    #[inline(always)]
    fn from(variant: CLKEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` reader - Clock enable bit"]
pub type CLKEN_R = crate::BitReader<CLKEN>;
impl CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKEN {
        match self.bits {
            false => CLKEN::Disabled,
            true => CLKEN::Enabled,
        }
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKEN::Disabled
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKEN::Enabled
    }
}
#[doc = "Field `CLKEN` writer - Clock enable bit"]
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG, CLKEN>;
impl<'a, REG> CLKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN::Disabled)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN::Enabled)
    }
}
#[doc = "Power saving configuration bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWRSAV {
    #[doc = "0: SDIO_CK clock is always enabled"]
    Enabled = 0,
    #[doc = "1: SDIO_CK is only enabled when the bus is active"]
    Disabled = 1,
}
impl From<PWRSAV> for bool {
    #[inline(always)]
    fn from(variant: PWRSAV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSAV` reader - Power saving configuration bit"]
pub type PWRSAV_R = crate::BitReader<PWRSAV>;
impl PWRSAV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWRSAV {
        match self.bits {
            false => PWRSAV::Enabled,
            true => PWRSAV::Disabled,
        }
    }
    #[doc = "SDIO_CK clock is always enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWRSAV::Enabled
    }
    #[doc = "SDIO_CK is only enabled when the bus is active"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWRSAV::Disabled
    }
}
#[doc = "Field `PWRSAV` writer - Power saving configuration bit"]
pub type PWRSAV_W<'a, REG> = crate::BitWriter<'a, REG, PWRSAV>;
impl<'a, REG> PWRSAV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDIO_CK clock is always enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSAV::Enabled)
    }
    #[doc = "SDIO_CK is only enabled when the bus is active"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSAV::Disabled)
    }
}
#[doc = "Clock divider bypass enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS {
    #[doc = "0: SDIOCLK is divided according to the CLKDIV value before driving the SDIO_CK output signal."]
    Disabled = 0,
    #[doc = "1: SDIOCLK directly drives the SDIO_CK output signal"]
    Enabled = 1,
}
impl From<BYPASS> for bool {
    #[inline(always)]
    fn from(variant: BYPASS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - Clock divider bypass enable bit"]
pub type BYPASS_R = crate::BitReader<BYPASS>;
impl BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BYPASS {
        match self.bits {
            false => BYPASS::Disabled,
            true => BYPASS::Enabled,
        }
    }
    #[doc = "SDIOCLK is divided according to the CLKDIV value before driving the SDIO_CK output signal."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BYPASS::Disabled
    }
    #[doc = "SDIOCLK directly drives the SDIO_CK output signal"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BYPASS::Enabled
    }
}
#[doc = "Field `BYPASS` writer - Clock divider bypass enable bit"]
pub type BYPASS_W<'a, REG> = crate::BitWriter<'a, REG, BYPASS>;
impl<'a, REG> BYPASS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDIOCLK is divided according to the CLKDIV value before driving the SDIO_CK output signal."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS::Disabled)
    }
    #[doc = "SDIOCLK directly drives the SDIO_CK output signal"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BYPASS::Enabled)
    }
}
#[doc = "Wide bus mode enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WIDBUS {
    #[doc = "0: 1 lane wide bus"]
    BusWidth1 = 0,
    #[doc = "1: 4 lane wide bus"]
    BusWidth4 = 1,
    #[doc = "2: 8 lane wide bus"]
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
#[doc = "Field `WIDBUS` reader - Wide bus mode enable bit"]
pub type WIDBUS_R = crate::FieldReader<WIDBUS>;
impl WIDBUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WIDBUS> {
        match self.bits {
            0 => Some(WIDBUS::BusWidth1),
            1 => Some(WIDBUS::BusWidth4),
            2 => Some(WIDBUS::BusWidth8),
            _ => None,
        }
    }
    #[doc = "1 lane wide bus"]
    #[inline(always)]
    pub fn is_bus_width1(&self) -> bool {
        *self == WIDBUS::BusWidth1
    }
    #[doc = "4 lane wide bus"]
    #[inline(always)]
    pub fn is_bus_width4(&self) -> bool {
        *self == WIDBUS::BusWidth4
    }
    #[doc = "8 lane wide bus"]
    #[inline(always)]
    pub fn is_bus_width8(&self) -> bool {
        *self == WIDBUS::BusWidth8
    }
}
#[doc = "Field `WIDBUS` writer - Wide bus mode enable bit"]
pub type WIDBUS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WIDBUS>;
impl<'a, REG> WIDBUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 lane wide bus"]
    #[inline(always)]
    pub fn bus_width1(self) -> &'a mut crate::W<REG> {
        self.variant(WIDBUS::BusWidth1)
    }
    #[doc = "4 lane wide bus"]
    #[inline(always)]
    pub fn bus_width4(self) -> &'a mut crate::W<REG> {
        self.variant(WIDBUS::BusWidth4)
    }
    #[doc = "8 lane wide bus"]
    #[inline(always)]
    pub fn bus_width8(self) -> &'a mut crate::W<REG> {
        self.variant(WIDBUS::BusWidth8)
    }
}
#[doc = "SDIO_CK dephasing selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NEGEDGE {
    #[doc = "0: SDIO_CK generated on the rising edge"]
    Rising = 0,
    #[doc = "1: SDIO_CK generated on the falling edge"]
    Falling = 1,
}
impl From<NEGEDGE> for bool {
    #[inline(always)]
    fn from(variant: NEGEDGE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEGEDGE` reader - SDIO_CK dephasing selection bit"]
pub type NEGEDGE_R = crate::BitReader<NEGEDGE>;
impl NEGEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NEGEDGE {
        match self.bits {
            false => NEGEDGE::Rising,
            true => NEGEDGE::Falling,
        }
    }
    #[doc = "SDIO_CK generated on the rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == NEGEDGE::Rising
    }
    #[doc = "SDIO_CK generated on the falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == NEGEDGE::Falling
    }
}
#[doc = "Field `NEGEDGE` writer - SDIO_CK dephasing selection bit"]
pub type NEGEDGE_W<'a, REG> = crate::BitWriter<'a, REG, NEGEDGE>;
impl<'a, REG> NEGEDGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDIO_CK generated on the rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(NEGEDGE::Rising)
    }
    #[doc = "SDIO_CK generated on the falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(NEGEDGE::Falling)
    }
}
#[doc = "HW Flow Control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HWFC_EN {
    #[doc = "0: HW Flow Control is disabled"]
    Disabled = 0,
    #[doc = "1: HW Flow Control is enabled"]
    Enabled = 1,
}
impl From<HWFC_EN> for bool {
    #[inline(always)]
    fn from(variant: HWFC_EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWFC_EN` reader - HW Flow Control enable"]
pub type HWFC_EN_R = crate::BitReader<HWFC_EN>;
impl HWFC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HWFC_EN {
        match self.bits {
            false => HWFC_EN::Disabled,
            true => HWFC_EN::Enabled,
        }
    }
    #[doc = "HW Flow Control is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HWFC_EN::Disabled
    }
    #[doc = "HW Flow Control is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HWFC_EN::Enabled
    }
}
#[doc = "Field `HWFC_EN` writer - HW Flow Control enable"]
pub type HWFC_EN_W<'a, REG> = crate::BitWriter<'a, REG, HWFC_EN>;
impl<'a, REG> HWFC_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HW Flow Control is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HWFC_EN::Disabled)
    }
    #[doc = "HW Flow Control is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HWFC_EN::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock divide factor"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Clock enable bit"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Power saving configuration bit"]
    #[inline(always)]
    pub fn pwrsav(&self) -> PWRSAV_R {
        PWRSAV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Wide bus mode enable bit"]
    #[inline(always)]
    pub fn widbus(&self) -> WIDBUS_R {
        WIDBUS_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - SDIO_CK dephasing selection bit"]
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HW Flow Control enable"]
    #[inline(always)]
    pub fn hwfc_en(&self) -> HWFC_EN_R {
        HWFC_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divide factor"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CLKCRrs> {
        CLKDIV_W::new(self, 0)
    }
    #[doc = "Bit 8 - Clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<CLKCRrs> {
        CLKEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Power saving configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsav(&mut self) -> PWRSAV_W<CLKCRrs> {
        PWRSAV_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<CLKCRrs> {
        BYPASS_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - Wide bus mode enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn widbus(&mut self) -> WIDBUS_W<CLKCRrs> {
        WIDBUS_W::new(self, 11)
    }
    #[doc = "Bit 13 - SDIO_CK dephasing selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn negedge(&mut self) -> NEGEDGE_W<CLKCRrs> {
        NEGEDGE_W::new(self, 13)
    }
    #[doc = "Bit 14 - HW Flow Control enable"]
    #[inline(always)]
    #[must_use]
    pub fn hwfc_en(&mut self) -> HWFC_EN_W<CLKCRrs> {
        HWFC_EN_W::new(self, 14)
    }
}
#[doc = "SDI clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCRrs;
impl crate::RegisterSpec for CLKCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcr::R`](R) reader structure"]
impl crate::Readable for CLKCRrs {}
#[doc = "`write(|w| ..)` method takes [`clkcr::W`](W) writer structure"]
impl crate::Writable for CLKCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCR to value 0"]
impl crate::Resettable for CLKCRrs {
    const RESET_VALUE: u32 = 0;
}
