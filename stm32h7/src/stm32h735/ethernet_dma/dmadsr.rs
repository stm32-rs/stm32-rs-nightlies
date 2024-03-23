#[doc = "Register `DMADSR` reader"]
pub type R = crate::R<DMADSRrs>;
#[doc = "Register `DMADSR` writer"]
pub type W = crate::W<DMADSRrs>;
#[doc = "Field `AXWHSTS` reader - AHB Master Write Channel"]
pub type AXWHSTS_R = crate::BitReader;
#[doc = "Field `AXWHSTS` writer - AHB Master Write Channel"]
pub type AXWHSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPS0` reader - DMA Channel Receive Process State"]
pub type RPS0_R = crate::FieldReader;
#[doc = "Field `RPS0` writer - DMA Channel Receive Process State"]
pub type RPS0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TPS0` reader - DMA Channel Transmit Process State"]
pub type TPS0_R = crate::FieldReader;
#[doc = "Field `TPS0` writer - DMA Channel Transmit Process State"]
pub type TPS0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - AHB Master Write Channel"]
    #[inline(always)]
    pub fn axwhsts(&self) -> AXWHSTS_R {
        AXWHSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - DMA Channel Receive Process State"]
    #[inline(always)]
    pub fn rps0(&self) -> RPS0_R {
        RPS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DMA Channel Transmit Process State"]
    #[inline(always)]
    pub fn tps0(&self) -> TPS0_R {
        TPS0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AHB Master Write Channel"]
    #[inline(always)]
    #[must_use]
    pub fn axwhsts(&mut self) -> AXWHSTS_W<DMADSRrs> {
        AXWHSTS_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - DMA Channel Receive Process State"]
    #[inline(always)]
    #[must_use]
    pub fn rps0(&mut self) -> RPS0_W<DMADSRrs> {
        RPS0_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - DMA Channel Transmit Process State"]
    #[inline(always)]
    #[must_use]
    pub fn tps0(&mut self) -> TPS0_W<DMADSRrs> {
        TPS0_W::new(self, 12)
    }
}
#[doc = "Debug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmadsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmadsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMADSRrs;
impl crate::RegisterSpec for DMADSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmadsr::R`](R) reader structure"]
impl crate::Readable for DMADSRrs {}
#[doc = "`write(|w| ..)` method takes [`dmadsr::W`](W) writer structure"]
impl crate::Writable for DMADSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMADSR to value 0"]
impl crate::Resettable for DMADSRrs {
    const RESET_VALUE: u32 = 0;
}
