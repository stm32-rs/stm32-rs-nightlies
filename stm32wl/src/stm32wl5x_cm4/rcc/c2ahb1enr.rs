#[doc = "Register `C2AHB1ENR` reader"]
pub type R = crate::R<C2AHB1ENRrs>;
#[doc = "Register `C2AHB1ENR` writer"]
pub type W = crate::W<C2AHB1ENRrs>;
#[doc = "CPU2 DMA1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1EN {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<DMA1EN> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1EN` reader - CPU2 DMA1 clock enable"]
pub type DMA1EN_R = crate::BitReader<DMA1EN>;
impl DMA1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA1EN {
        match self.bits {
            false => DMA1EN::Disabled,
            true => DMA1EN::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1EN::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1EN::Enabled
    }
}
#[doc = "Field `DMA1EN` writer - CPU2 DMA1 clock enable"]
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1EN>;
impl<'a, REG> DMA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Enabled)
    }
}
#[doc = "Field `DMA2EN` reader - CPU2 DMA2 clock enable"]
pub use DMA1EN_R as DMA2EN_R;
#[doc = "Field `DMAMUX1EN` reader - CPU2 DMAMUX1 clock enable"]
pub use DMA1EN_R as DMAMUX1EN_R;
#[doc = "Field `CRCEN` reader - CPU2 CRC clock enable"]
pub use DMA1EN_R as CRCEN_R;
#[doc = "Field `DMA2EN` writer - CPU2 DMA2 clock enable"]
pub use DMA1EN_W as DMA2EN_W;
#[doc = "Field `DMAMUX1EN` writer - CPU2 DMAMUX1 clock enable"]
pub use DMA1EN_W as DMAMUX1EN_W;
#[doc = "Field `CRCEN` writer - CPU2 CRC clock enable"]
pub use DMA1EN_W as CRCEN_W;
impl R {
    #[doc = "Bit 0 - CPU2 DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU2 DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU2 DMAMUX1 clock enable"]
    #[inline(always)]
    pub fn dmamux1en(&self) -> DMAMUX1EN_R {
        DMAMUX1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU2 CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU2 DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<C2AHB1ENRrs> {
        DMA1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU2 DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<C2AHB1ENRrs> {
        DMA2EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU2 DMAMUX1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1en(&mut self) -> DMAMUX1EN_W<C2AHB1ENRrs> {
        DMAMUX1EN_W::new(self, 2)
    }
    #[doc = "Bit 12 - CPU2 CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<C2AHB1ENRrs> {
        CRCEN_W::new(self, 12)
    }
}
#[doc = "CPU2 AHB1 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ahb1enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ahb1enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2AHB1ENRrs;
impl crate::RegisterSpec for C2AHB1ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2ahb1enr::R`](R) reader structure"]
impl crate::Readable for C2AHB1ENRrs {}
#[doc = "`write(|w| ..)` method takes [`c2ahb1enr::W`](W) writer structure"]
impl crate::Writable for C2AHB1ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2AHB1ENR to value 0"]
impl crate::Resettable for C2AHB1ENRrs {
    const RESET_VALUE: u32 = 0;
}
