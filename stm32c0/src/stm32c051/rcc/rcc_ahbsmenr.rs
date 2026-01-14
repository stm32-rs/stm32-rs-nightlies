///Register `RCC_AHBSMENR` reader
pub type R = crate::R<RCC_AHBSMENRrs>;
///Register `RCC_AHBSMENR` writer
pub type W = crate::W<RCC_AHBSMENRrs>;
/**DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1SMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<DMA1SMEN> for bool {
    #[inline(always)]
    fn from(variant: DMA1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1SMEN` reader - DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.
pub type DMA1SMEN_R = crate::BitReader<DMA1SMEN>;
impl DMA1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMA1SMEN {
        match self.bits {
            false => DMA1SMEN::B0x0,
            true => DMA1SMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMA1SMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMA1SMEN::B0x1
    }
}
///Field `DMA1SMEN` writer - DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.
pub type DMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1SMEN>;
impl<'a, REG> DMA1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1SMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1SMEN::B0x1)
    }
}
/**Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHSMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<FLASHSMEN> for bool {
    #[inline(always)]
    fn from(variant: FLASHSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `FLASHSMEN` reader - Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode.
pub type FLASHSMEN_R = crate::BitReader<FLASHSMEN>;
impl FLASHSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLASHSMEN {
        match self.bits {
            false => FLASHSMEN::B0x0,
            true => FLASHSMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FLASHSMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FLASHSMEN::B0x1
    }
}
///Field `FLASHSMEN` writer - Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode.
pub type FLASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG, FLASHSMEN>;
impl<'a, REG> FLASHSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHSMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHSMEN::B0x1)
    }
}
/**SRAM clock enable during Sleep mode Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAMSMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<SRAMSMEN> for bool {
    #[inline(always)]
    fn from(variant: SRAMSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAMSMEN` reader - SRAM clock enable during Sleep mode Set and cleared by software.
pub type SRAMSMEN_R = crate::BitReader<SRAMSMEN>;
impl SRAMSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAMSMEN {
        match self.bits {
            false => SRAMSMEN::B0x0,
            true => SRAMSMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SRAMSMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SRAMSMEN::B0x1
    }
}
///Field `SRAMSMEN` writer - SRAM clock enable during Sleep mode Set and cleared by software.
pub type SRAMSMEN_W<'a, REG> = crate::BitWriter<'a, REG, SRAMSMEN>;
impl<'a, REG> SRAMSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SRAMSMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SRAMSMEN::B0x1)
    }
}
/**CRC clock enable during Sleep mode Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCSMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<CRCSMEN> for bool {
    #[inline(always)]
    fn from(variant: CRCSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCSMEN` reader - CRC clock enable during Sleep mode Set and cleared by software.
pub type CRCSMEN_R = crate::BitReader<CRCSMEN>;
impl CRCSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCSMEN {
        match self.bits {
            false => CRCSMEN::B0x0,
            true => CRCSMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRCSMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRCSMEN::B0x1
    }
}
///Field `CRCSMEN` writer - CRC clock enable during Sleep mode Set and cleared by software.
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG, CRCSMEN>;
impl<'a, REG> CRCSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSMEN::B0x1)
    }
}
impl R {
    ///Bit 0 - DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode.
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn sramsmen(&self) -> SRAMSMEN_R {
        SRAMSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_AHBSMENR")
            .field("dma1smen", &self.dma1smen())
            .field("flashsmen", &self.flashsmen())
            .field("sramsmen", &self.sramsmen())
            .field("crcsmen", &self.crcsmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.
    #[inline(always)]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<'_, RCC_AHBSMENRrs> {
        DMA1SMEN_W::new(self, 0)
    }
    ///Bit 8 - Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode.
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<'_, RCC_AHBSMENRrs> {
        FLASHSMEN_W::new(self, 8)
    }
    ///Bit 9 - SRAM clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn sramsmen(&mut self) -> SRAMSMEN_W<'_, RCC_AHBSMENRrs> {
        SRAMSMEN_W::new(self, 9)
    }
    ///Bit 12 - CRC clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<'_, RCC_AHBSMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
}
/**RCC AHB peripheral clock enable in Sleep/Stop mode register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahbsmenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahbsmenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_AHBSMENR)*/
pub struct RCC_AHBSMENRrs;
impl crate::RegisterSpec for RCC_AHBSMENRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_ahbsmenr::R`](R) reader structure
impl crate::Readable for RCC_AHBSMENRrs {}
///`write(|w| ..)` method takes [`rcc_ahbsmenr::W`](W) writer structure
impl crate::Writable for RCC_AHBSMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_AHBSMENR to value 0x1301
impl crate::Resettable for RCC_AHBSMENRrs {
    const RESET_VALUE: u32 = 0x1301;
}
