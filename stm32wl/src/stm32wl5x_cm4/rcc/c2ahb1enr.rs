///Register `C2AHB1ENR` reader
pub type R = crate::R<C2AHB1ENRrs>;
///Register `C2AHB1ENR` writer
pub type W = crate::W<C2AHB1ENRrs>;
/**CPU2 DMA1 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1EN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<DMA1EN> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1EN` reader - CPU2 DMA1 clock enable
pub type DMA1EN_R = crate::BitReader<DMA1EN>;
impl DMA1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMA1EN {
        match self.bits {
            false => DMA1EN::Disabled,
            true => DMA1EN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1EN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1EN::Enabled
    }
}
///Field `DMA1EN` writer - CPU2 DMA1 clock enable
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1EN>;
impl<'a, REG> DMA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Enabled)
    }
}
///Field `DMA2EN` reader - CPU2 DMA2 clock enable
pub use DMA1EN_R as DMA2EN_R;
///Field `DMAMUX1EN` reader - CPU2 DMAMUX1 clock enable
pub use DMA1EN_R as DMAMUX1EN_R;
///Field `CRCEN` reader - CPU2 CRC clock enable
pub use DMA1EN_R as CRCEN_R;
///Field `DMA2EN` writer - CPU2 DMA2 clock enable
pub use DMA1EN_W as DMA2EN_W;
///Field `DMAMUX1EN` writer - CPU2 DMAMUX1 clock enable
pub use DMA1EN_W as DMAMUX1EN_W;
///Field `CRCEN` writer - CPU2 CRC clock enable
pub use DMA1EN_W as CRCEN_W;
impl R {
    ///Bit 0 - CPU2 DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU2 DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU2 DMAMUX1 clock enable
    #[inline(always)]
    pub fn dmamux1en(&self) -> DMAMUX1EN_R {
        DMAMUX1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 12 - CPU2 CRC clock enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2AHB1ENR")
            .field("dma1en", &self.dma1en())
            .field("crcen", &self.crcen())
            .field("dmamux1en", &self.dmamux1en())
            .field("dma2en", &self.dma2en())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU2 DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<C2AHB1ENRrs> {
        DMA1EN_W::new(self, 0)
    }
    ///Bit 1 - CPU2 DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W<C2AHB1ENRrs> {
        DMA2EN_W::new(self, 1)
    }
    ///Bit 2 - CPU2 DMAMUX1 clock enable
    #[inline(always)]
    pub fn dmamux1en(&mut self) -> DMAMUX1EN_W<C2AHB1ENRrs> {
        DMAMUX1EN_W::new(self, 2)
    }
    ///Bit 12 - CPU2 CRC clock enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<C2AHB1ENRrs> {
        CRCEN_W::new(self, 12)
    }
}
/**CPU2 AHB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`c2ahb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ahb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#RCC:C2AHB1ENR)*/
pub struct C2AHB1ENRrs;
impl crate::RegisterSpec for C2AHB1ENRrs {
    type Ux = u32;
}
///`read()` method returns [`c2ahb1enr::R`](R) reader structure
impl crate::Readable for C2AHB1ENRrs {}
///`write(|w| ..)` method takes [`c2ahb1enr::W`](W) writer structure
impl crate::Writable for C2AHB1ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2AHB1ENR to value 0
impl crate::Resettable for C2AHB1ENRrs {}
