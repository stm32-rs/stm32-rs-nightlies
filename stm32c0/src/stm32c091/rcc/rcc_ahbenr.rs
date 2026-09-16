///Register `RCC_AHBENR` reader
pub type R = crate::R<RCC_AHBENRrs>;
///Register `RCC_AHBENR` writer
pub type W = crate::W<RCC_AHBENRrs>;
/**DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1EN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<DMA1EN> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1EN` reader - DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.
pub type DMA1EN_R = crate::BitReader<DMA1EN>;
impl DMA1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMA1EN {
        match self.bits {
            false => DMA1EN::B0x0,
            true => DMA1EN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMA1EN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMA1EN::B0x1
    }
}
///Field `DMA1EN` writer - DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1EN>;
impl<'a, REG> DMA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::B0x1)
    }
}
/**Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the Flash memory is in power down mode.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<FLASHEN> for bool {
    #[inline(always)]
    fn from(variant: FLASHEN) -> Self {
        variant as u8 != 0
    }
}
///Field `FLASHEN` reader - Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the Flash memory is in power down mode.
pub type FLASHEN_R = crate::BitReader<FLASHEN>;
impl FLASHEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLASHEN {
        match self.bits {
            false => FLASHEN::B0x0,
            true => FLASHEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FLASHEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FLASHEN::B0x1
    }
}
///Field `FLASHEN` writer - Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the Flash memory is in power down mode.
pub type FLASHEN_W<'a, REG> = crate::BitWriter<'a, REG, FLASHEN>;
impl<'a, REG> FLASHEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHEN::B0x1)
    }
}
/**CRC clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<CRCEN> for bool {
    #[inline(always)]
    fn from(variant: CRCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCEN` reader - CRC clock enable Set and cleared by software.
pub type CRCEN_R = crate::BitReader<CRCEN>;
impl CRCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCEN {
        match self.bits {
            false => CRCEN::B0x0,
            true => CRCEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRCEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRCEN::B0x1
    }
}
///Field `CRCEN` writer - CRC clock enable Set and cleared by software.
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG, CRCEN>;
impl<'a, REG> CRCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN::B0x1)
    }
}
impl R {
    ///Bit 0 - DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the Flash memory is in power down mode.
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable Set and cleared by software.
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_AHBENR")
            .field("dma1en", &self.dma1en())
            .field("flashen", &self.flashen())
            .field("crcen", &self.crcen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 and DMAMUX clock enable Set and cleared by software. DMAMUX is enabled as long as at least one DMA peripheral is enabled.
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<'_, RCC_AHBENRrs> {
        DMA1EN_W::new(self, 0)
    }
    ///Bit 8 - Flash memory interface clock enable Set and cleared by software. This bit can only be cleared when the Flash memory is in power down mode.
    #[inline(always)]
    pub fn flashen(&mut self) -> FLASHEN_W<'_, RCC_AHBENRrs> {
        FLASHEN_W::new(self, 8)
    }
    ///Bit 12 - CRC clock enable Set and cleared by software.
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, RCC_AHBENRrs> {
        CRCEN_W::new(self, 12)
    }
}
/**RCC AHB peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahbenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahbenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#RCC:RCC_AHBENR)*/
pub struct RCC_AHBENRrs;
impl crate::RegisterSpec for RCC_AHBENRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_ahbenr::R`](R) reader structure
impl crate::Readable for RCC_AHBENRrs {}
///`write(|w| ..)` method takes [`rcc_ahbenr::W`](W) writer structure
impl crate::Writable for RCC_AHBENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_AHBENR to value 0x0100
impl crate::Resettable for RCC_AHBENRrs {
    const RESET_VALUE: u32 = 0x0100;
}
