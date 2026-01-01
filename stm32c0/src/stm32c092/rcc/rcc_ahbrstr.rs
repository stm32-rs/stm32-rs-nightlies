///Register `RCC_AHBRSTR` reader
pub type R = crate::R<RCC_AHBRSTRrs>;
///Register `RCC_AHBRSTR` writer
pub type W = crate::W<RCC_AHBRSTRrs>;
/**DMA1 and DMAMUX reset Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1RST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset DMA1 and DMAMUX
    B0x1 = 1,
}
impl From<DMA1RST> for bool {
    #[inline(always)]
    fn from(variant: DMA1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1RST` reader - DMA1 and DMAMUX reset Set and cleared by software.
pub type DMA1RST_R = crate::BitReader<DMA1RST>;
impl DMA1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMA1RST {
        match self.bits {
            false => DMA1RST::B0x0,
            true => DMA1RST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMA1RST::B0x0
    }
    ///Reset DMA1 and DMAMUX
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMA1RST::B0x1
    }
}
///Field `DMA1RST` writer - DMA1 and DMAMUX reset Set and cleared by software.
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG, DMA1RST>;
impl<'a, REG> DMA1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RST::B0x0)
    }
    ///Reset DMA1 and DMAMUX
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RST::B0x1)
    }
}
/**Flash memory interface reset Set and cleared by software. This bit can only be set when the Flash memory is in power down mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHRST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset Flash memory interface
    B0x1 = 1,
}
impl From<FLASHRST> for bool {
    #[inline(always)]
    fn from(variant: FLASHRST) -> Self {
        variant as u8 != 0
    }
}
///Field `FLASHRST` reader - Flash memory interface reset Set and cleared by software. This bit can only be set when the Flash memory is in power down mode.
pub type FLASHRST_R = crate::BitReader<FLASHRST>;
impl FLASHRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLASHRST {
        match self.bits {
            false => FLASHRST::B0x0,
            true => FLASHRST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FLASHRST::B0x0
    }
    ///Reset Flash memory interface
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FLASHRST::B0x1
    }
}
///Field `FLASHRST` writer - Flash memory interface reset Set and cleared by software. This bit can only be set when the Flash memory is in power down mode.
pub type FLASHRST_W<'a, REG> = crate::BitWriter<'a, REG, FLASHRST>;
impl<'a, REG> FLASHRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHRST::B0x0)
    }
    ///Reset Flash memory interface
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHRST::B0x1)
    }
}
/**CRC reset Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCRST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset CRC
    B0x1 = 1,
}
impl From<CRCRST> for bool {
    #[inline(always)]
    fn from(variant: CRCRST) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCRST` reader - CRC reset Set and cleared by software.
pub type CRCRST_R = crate::BitReader<CRCRST>;
impl CRCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCRST {
        match self.bits {
            false => CRCRST::B0x0,
            true => CRCRST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRCRST::B0x0
    }
    ///Reset CRC
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRCRST::B0x1
    }
}
///Field `CRCRST` writer - CRC reset Set and cleared by software.
pub type CRCRST_W<'a, REG> = crate::BitWriter<'a, REG, CRCRST>;
impl<'a, REG> CRCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCRST::B0x0)
    }
    ///Reset CRC
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCRST::B0x1)
    }
}
impl R {
    ///Bit 0 - DMA1 and DMAMUX reset Set and cleared by software.
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Flash memory interface reset Set and cleared by software. This bit can only be set when the Flash memory is in power down mode.
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC reset Set and cleared by software.
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_AHBRSTR")
            .field("dma1rst", &self.dma1rst())
            .field("flashrst", &self.flashrst())
            .field("crcrst", &self.crcrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 and DMAMUX reset Set and cleared by software.
    #[inline(always)]
    pub fn dma1rst(&mut self) -> DMA1RST_W<'_, RCC_AHBRSTRrs> {
        DMA1RST_W::new(self, 0)
    }
    ///Bit 8 - Flash memory interface reset Set and cleared by software. This bit can only be set when the Flash memory is in power down mode.
    #[inline(always)]
    pub fn flashrst(&mut self) -> FLASHRST_W<'_, RCC_AHBRSTRrs> {
        FLASHRST_W::new(self, 8)
    }
    ///Bit 12 - CRC reset Set and cleared by software.
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W<'_, RCC_AHBRSTRrs> {
        CRCRST_W::new(self, 12)
    }
}
/**RCC AHB peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#RCC:RCC_AHBRSTR)*/
pub struct RCC_AHBRSTRrs;
impl crate::RegisterSpec for RCC_AHBRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_ahbrstr::R`](R) reader structure
impl crate::Readable for RCC_AHBRSTRrs {}
///`write(|w| ..)` method takes [`rcc_ahbrstr::W`](W) writer structure
impl crate::Writable for RCC_AHBRSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_AHBRSTR to value 0
impl crate::Resettable for RCC_AHBRSTRrs {}
