///Register `AHBSMENR` reader
pub type R = crate::R<AHBSMENRrs>;
///Register `AHBSMENR` writer
pub type W = crate::W<AHBSMENRrs>;
/**DMA1 clock enable during Sleep mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1SMEN {
    ///0: Peripheral disabled (typically saves power)
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<DMA1SMEN> for bool {
    #[inline(always)]
    fn from(variant: DMA1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1SMEN` reader - DMA1 clock enable during Sleep mode
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
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1SMEN::Disabled
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1SMEN::Enabled
    }
}
///Field `DMA1SMEN` writer - DMA1 clock enable during Sleep mode
pub type DMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1SMEN>;
impl<'a, REG> DMA1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1SMEN::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1SMEN::Enabled)
    }
}
///Field `DMA2SMEN` reader - DMA2 clock enable during Sleep mode
pub use DMA1SMEN_R as DMA2SMEN_R;
///Field `FLASHSMEN` reader - Flash memory interface clock enable during Sleep mode
pub use DMA1SMEN_R as FLASHSMEN_R;
///Field `SRAMSMEN` reader - SRAM clock enable during Sleep mode
pub use DMA1SMEN_R as SRAMSMEN_R;
///Field `CRCSMEN` reader - CRC clock enable during Sleep mode
pub use DMA1SMEN_R as CRCSMEN_R;
///Field `DMA2SMEN` writer - DMA2 clock enable during Sleep mode
pub use DMA1SMEN_W as DMA2SMEN_W;
///Field `FLASHSMEN` writer - Flash memory interface clock enable during Sleep mode
pub use DMA1SMEN_W as FLASHSMEN_W;
///Field `SRAMSMEN` writer - SRAM clock enable during Sleep mode
pub use DMA1SMEN_W as SRAMSMEN_W;
///Field `CRCSMEN` writer - CRC clock enable during Sleep mode
pub use DMA1SMEN_W as CRCSMEN_W;
impl R {
    ///Bit 0 - DMA1 clock enable during Sleep mode
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 clock enable during Sleep mode
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Flash memory interface clock enable during Sleep mode
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM clock enable during Sleep mode
    #[inline(always)]
    pub fn sramsmen(&self) -> SRAMSMEN_R {
        SRAMSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable during Sleep mode
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBSMENR")
            .field("dma1smen", &self.dma1smen())
            .field("dma2smen", &self.dma2smen())
            .field("flashsmen", &self.flashsmen())
            .field("sramsmen", &self.sramsmen())
            .field("crcsmen", &self.crcsmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 clock enable during Sleep mode
    #[inline(always)]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<'_, AHBSMENRrs> {
        DMA1SMEN_W::new(self, 0)
    }
    ///Bit 1 - DMA2 clock enable during Sleep mode
    #[inline(always)]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W<'_, AHBSMENRrs> {
        DMA2SMEN_W::new(self, 1)
    }
    ///Bit 8 - Flash memory interface clock enable during Sleep mode
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<'_, AHBSMENRrs> {
        FLASHSMEN_W::new(self, 8)
    }
    ///Bit 9 - SRAM clock enable during Sleep mode
    #[inline(always)]
    pub fn sramsmen(&mut self) -> SRAMSMEN_W<'_, AHBSMENRrs> {
        SRAMSMEN_W::new(self, 9)
    }
    ///Bit 12 - CRC clock enable during Sleep mode
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<'_, AHBSMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
}
/**AHB peripheral clock enable in Sleep mode register

You can [`read`](crate::Reg::read) this register and get [`ahbsmenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsmenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#RCC:AHBSMENR)*/
pub struct AHBSMENRrs;
impl crate::RegisterSpec for AHBSMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbsmenr::R`](R) reader structure
impl crate::Readable for AHBSMENRrs {}
///`write(|w| ..)` method takes [`ahbsmenr::W`](W) writer structure
impl crate::Writable for AHBSMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHBSMENR to value 0x0005_1303
impl crate::Resettable for AHBSMENRrs {
    const RESET_VALUE: u32 = 0x0005_1303;
}
