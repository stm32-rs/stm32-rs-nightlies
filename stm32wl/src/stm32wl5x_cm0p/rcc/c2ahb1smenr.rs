///Register `C2AHB1SMENR` reader
pub type R = crate::R<C2AHB1SMENRrs>;
///Register `C2AHB1SMENR` writer
pub type W = crate::W<C2AHB1SMENRrs>;
/**DMA1 clock enable during CPU2 CSleep mode.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1SMEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<DMA1SMEN> for bool {
    #[inline(always)]
    fn from(variant: DMA1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1SMEN` reader - DMA1 clock enable during CPU2 CSleep mode.
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
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1SMEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1SMEN::Enabled
    }
}
///Field `DMA1SMEN` writer - DMA1 clock enable during CPU2 CSleep mode.
pub type DMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1SMEN>;
impl<'a, REG> DMA1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1SMEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1SMEN::Enabled)
    }
}
///Field `DMA2SMEN` reader - DMA2 clock enable during CPU2 CSleep mode.
pub use DMA1SMEN_R as DMA2SMEN_R;
///Field `DMAMUX1SMEN` reader - DMAMUX1 clock enable during CPU2 CSleep mode.
pub use DMA1SMEN_R as DMAMUX1SMEN_R;
///Field `CRCSMEN` reader - CRC clock enable during CPU2 CSleep mode.
pub use DMA1SMEN_R as CRCSMEN_R;
///Field `DMA2SMEN` writer - DMA2 clock enable during CPU2 CSleep mode.
pub use DMA1SMEN_W as DMA2SMEN_W;
///Field `DMAMUX1SMEN` writer - DMAMUX1 clock enable during CPU2 CSleep mode.
pub use DMA1SMEN_W as DMAMUX1SMEN_W;
///Field `CRCSMEN` writer - CRC clock enable during CPU2 CSleep mode.
pub use DMA1SMEN_W as CRCSMEN_W;
impl R {
    ///Bit 0 - DMA1 clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMAMUX1 clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn dmamux1smen(&self) -> DMAMUX1SMEN_R {
        DMAMUX1SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2AHB1SMENR")
            .field("dma1smen", &self.dma1smen())
            .field("crcsmen", &self.crcsmen())
            .field("dmamux1smen", &self.dmamux1smen())
            .field("dma2smen", &self.dma2smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<'_, C2AHB1SMENRrs> {
        DMA1SMEN_W::new(self, 0)
    }
    ///Bit 1 - DMA2 clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W<'_, C2AHB1SMENRrs> {
        DMA2SMEN_W::new(self, 1)
    }
    ///Bit 2 - DMAMUX1 clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn dmamux1smen(&mut self) -> DMAMUX1SMEN_W<'_, C2AHB1SMENRrs> {
        DMAMUX1SMEN_W::new(self, 2)
    }
    ///Bit 12 - CRC clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<'_, C2AHB1SMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
}
/**CPU2 AHB1 peripheral clocks enable in Sleep modes register \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2ahb1smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ahb1smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2AHB1SMENR)*/
pub struct C2AHB1SMENRrs;
impl crate::RegisterSpec for C2AHB1SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`c2ahb1smenr::R`](R) reader structure
impl crate::Readable for C2AHB1SMENRrs {}
///`write(|w| ..)` method takes [`c2ahb1smenr::W`](W) writer structure
impl crate::Writable for C2AHB1SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2AHB1SMENR to value 0x1007
impl crate::Resettable for C2AHB1SMENRrs {
    const RESET_VALUE: u32 = 0x1007;
}
