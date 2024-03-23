#[doc = "Register `AHB1ENR` reader"]
pub type R = crate::R<AHB1ENRrs>;
#[doc = "Register `AHB1ENR` writer"]
pub type W = crate::W<AHB1ENRrs>;
#[doc = "DMA1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1EN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<DMA1EN> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
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
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1EN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1EN::Enabled
    }
}
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG, DMA1EN>;
impl<'a, REG> DMA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1EN::Enabled)
    }
}
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub use DMA1EN_R as DMA2EN_R;
#[doc = "Field `DMAMUXEN` reader - DMAMUX clock enable"]
pub use DMA1EN_R as DMAMUXEN_R;
#[doc = "Field `CORDICEN` reader - CORDIC clock enable"]
pub use DMA1EN_R as CORDICEN_R;
#[doc = "Field `FMACEN` reader - FMAC clock enable"]
pub use DMA1EN_R as FMACEN_R;
#[doc = "Field `FLASHEN` reader - Flash memory interface clock enable"]
pub use DMA1EN_R as FLASHEN_R;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub use DMA1EN_R as CRCEN_R;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub use DMA1EN_W as DMA2EN_W;
#[doc = "Field `DMAMUXEN` writer - DMAMUX clock enable"]
pub use DMA1EN_W as DMAMUXEN_W;
#[doc = "Field `CORDICEN` writer - CORDIC clock enable"]
pub use DMA1EN_W as CORDICEN_W;
#[doc = "Field `FMACEN` writer - FMAC clock enable"]
pub use DMA1EN_W as FMACEN_W;
#[doc = "Field `FLASHEN` writer - Flash memory interface clock enable"]
pub use DMA1EN_W as FLASHEN_W;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub use DMA1EN_W as CRCEN_W;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUX clock enable"]
    #[inline(always)]
    pub fn dmamuxen(&self) -> DMAMUXEN_R {
        DMAMUXEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CORDIC clock enable"]
    #[inline(always)]
    pub fn cordicen(&self) -> CORDICEN_R {
        CORDICEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FMAC clock enable"]
    #[inline(always)]
    pub fn fmacen(&self) -> FMACEN_R {
        FMACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable"]
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<AHB1ENRrs> {
        DMA1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<AHB1ENRrs> {
        DMA2EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMAMUX clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmamuxen(&mut self) -> DMAMUXEN_W<AHB1ENRrs> {
        DMAMUXEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - CORDIC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cordicen(&mut self) -> CORDICEN_W<AHB1ENRrs> {
        CORDICEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - FMAC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmacen(&mut self) -> FMACEN_W<AHB1ENRrs> {
        FMACEN_W::new(self, 4)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn flashen(&mut self) -> FLASHEN_W<AHB1ENRrs> {
        FLASHEN_W::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHB1ENRrs> {
        CRCEN_W::new(self, 12)
    }
}
#[doc = "AHB1 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1ENRrs;
impl crate::RegisterSpec for AHB1ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1enr::R`](R) reader structure"]
impl crate::Readable for AHB1ENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure"]
impl crate::Writable for AHB1ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1ENR to value 0x0100"]
impl crate::Resettable for AHB1ENRrs {
    const RESET_VALUE: u32 = 0x0100;
}
