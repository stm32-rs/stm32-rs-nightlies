#[doc = "Register `CKGAENR` reader"]
pub type R = crate::R<CKGAENRrs>;
#[doc = "Register `CKGAENR` writer"]
pub type W = crate::W<CKGAENRrs>;
#[doc = "Field `AXICKG` reader - AXI interconnect matrix clock gating This bit is set and reset by software."]
pub type AXICKG_R = crate::BitReader;
#[doc = "Field `AXICKG` writer - AXI interconnect matrix clock gating This bit is set and reset by software."]
pub type AXICKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBCKG` reader - AXI master AHB clock gating This bit is set and reset by software."]
pub type AHBCKG_R = crate::BitReader;
#[doc = "Field `AHBCKG` writer - AXI master AHB clock gating This bit is set and reset by software."]
pub type AHBCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPUCKG` reader - AXI master CPU clock gating This bit is set and reset by software."]
pub type CPUCKG_R = crate::BitReader;
#[doc = "Field `CPUCKG` writer - AXI master CPU clock gating This bit is set and reset by software."]
pub type CPUCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMCCKG` reader - AXI master SDMMC clock gating This bit is set and reset by software."]
pub type SDMMCCKG_R = crate::BitReader;
#[doc = "Field `SDMMCCKG` writer - AXI master SDMMC clock gating This bit is set and reset by software."]
pub type SDMMCCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMACKG` reader - AXI master MDMA clock gating This bit is set and reset by software."]
pub type MDMACKG_R = crate::BitReader;
#[doc = "Field `MDMACKG` writer - AXI master MDMA clock gating This bit is set and reset by software."]
pub type MDMACKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2DCKG` reader - AXI master DMA2D clock gating This bit is set and reset by software."]
pub type DMA2DCKG_R = crate::BitReader;
#[doc = "Field `DMA2DCKG` writer - AXI master DMA2D clock gating This bit is set and reset by software."]
pub type DMA2DCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTDCCKG` reader - AXI master LTDC clock gating This bit is set and reset by software."]
pub type LTDCCKG_R = crate::BitReader;
#[doc = "Field `LTDCCKG` writer - AXI master LTDC clock gating This bit is set and reset by software."]
pub type LTDCCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFXMMUMCKG` reader - AXI master GFXMMU clock gating This bit is set and reset by software."]
pub type GFXMMUMCKG_R = crate::BitReader;
#[doc = "Field `GFXMMUMCKG` writer - AXI master GFXMMU clock gating This bit is set and reset by software."]
pub type GFXMMUMCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB12CKG` reader - AXI slave AHB12 clock gating This bit is set and reset by software."]
pub type AHB12CKG_R = crate::BitReader;
#[doc = "Field `AHB12CKG` writer - AXI slave AHB12 clock gating This bit is set and reset by software."]
pub type AHB12CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB34CKG` reader - AXI slave AHB34 clock gating This bit is set and reset by software."]
pub type AHB34CKG_R = crate::BitReader;
#[doc = "Field `AHB34CKG` writer - AXI slave AHB34 clock gating This bit is set and reset by software."]
pub type AHB34CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLIFTCKG` reader - AXI slave Flash interface (FLIFT) clock gating This bit is set and reset by software."]
pub type FLIFTCKG_R = crate::BitReader;
#[doc = "Field `FLIFTCKG` writer - AXI slave Flash interface (FLIFT) clock gating This bit is set and reset by software."]
pub type FLIFTCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPI2CKG` reader - AXI slave OCTOSPI2 clock gating This bit is set and reset by software."]
pub type OCTOSPI2CKG_R = crate::BitReader;
#[doc = "Field `OCTOSPI2CKG` writer - AXI slave OCTOSPI2 clock gating This bit is set and reset by software."]
pub type OCTOSPI2CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMCCKG` reader - AXI slave FMC clock gating This bit is set and reset by software."]
pub type FMCCKG_R = crate::BitReader;
#[doc = "Field `FMCCKG` writer - AXI slave FMC clock gating This bit is set and reset by software."]
pub type FMCCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPI1CKG` reader - AXI slave OCTOSPI1 clock gating This bit is set and reset by software."]
pub type OCTOSPI1CKG_R = crate::BitReader;
#[doc = "Field `OCTOSPI1CKG` writer - AXI slave OCTOSPI1 clock gating This bit is set and reset by software."]
pub type OCTOSPI1CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXIRAM1CKG` reader - AXI slave SRAM1 clock gating This bit is set and reset by software."]
pub type AXIRAM1CKG_R = crate::BitReader;
#[doc = "Field `AXIRAM1CKG` writer - AXI slave SRAM1 clock gating This bit is set and reset by software."]
pub type AXIRAM1CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXIRAM2CKG` reader - AXI matrix slave SRAM2 clock gating This bit is set and reset by software."]
pub type AXIRAM2CKG_R = crate::BitReader;
#[doc = "Field `AXIRAM2CKG` writer - AXI matrix slave SRAM2 clock gating This bit is set and reset by software."]
pub type AXIRAM2CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXIRAM3CKG` reader - AXI matrix slave SRAM3 clock gating This bit is set and reset by software."]
pub type AXIRAM3CKG_R = crate::BitReader;
#[doc = "Field `AXIRAM3CKG` writer - AXI matrix slave SRAM3 clock gating This bit is set and reset by software."]
pub type AXIRAM3CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFXMMUSCKG` reader - AXI matrix slave GFXMMU clock gating This bit is set and reset by software."]
pub type GFXMMUSCKG_R = crate::BitReader;
#[doc = "Field `GFXMMUSCKG` writer - AXI matrix slave GFXMMU clock gating This bit is set and reset by software."]
pub type GFXMMUSCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCRAMCKG` reader - RAM error code correction (ECC) clock gating This bit is set and reset by software."]
pub type ECCRAMCKG_R = crate::BitReader;
#[doc = "Field `ECCRAMCKG` writer - RAM error code correction (ECC) clock gating This bit is set and reset by software."]
pub type ECCRAMCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTICKG` reader - EXTI clock gating This bit is set and reset by software."]
pub type EXTICKG_R = crate::BitReader;
#[doc = "Field `EXTICKG` writer - EXTI clock gating This bit is set and reset by software."]
pub type EXTICKG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAGCKG` reader - JTAG automatic clock gating This bit is set and reset by software."]
pub type JTAGCKG_R = crate::BitReader;
#[doc = "Field `JTAGCKG` writer - JTAG automatic clock gating This bit is set and reset by software."]
pub type JTAGCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AXI interconnect matrix clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn axickg(&self) -> AXICKG_R {
        AXICKG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AXI master AHB clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn ahbckg(&self) -> AHBCKG_R {
        AHBCKG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AXI master CPU clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn cpuckg(&self) -> CPUCKG_R {
        CPUCKG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AXI master SDMMC clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn sdmmcckg(&self) -> SDMMCCKG_R {
        SDMMCCKG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AXI master MDMA clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn mdmackg(&self) -> MDMACKG_R {
        MDMACKG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AXI master DMA2D clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn dma2dckg(&self) -> DMA2DCKG_R {
        DMA2DCKG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AXI master LTDC clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn ltdcckg(&self) -> LTDCCKG_R {
        LTDCCKG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AXI master GFXMMU clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn gfxmmumckg(&self) -> GFXMMUMCKG_R {
        GFXMMUMCKG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AXI slave AHB12 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn ahb12ckg(&self) -> AHB12CKG_R {
        AHB12CKG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AXI slave AHB34 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn ahb34ckg(&self) -> AHB34CKG_R {
        AHB34CKG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AXI slave Flash interface (FLIFT) clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn fliftckg(&self) -> FLIFTCKG_R {
        FLIFTCKG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AXI slave OCTOSPI2 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn octospi2ckg(&self) -> OCTOSPI2CKG_R {
        OCTOSPI2CKG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AXI slave FMC clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn fmcckg(&self) -> FMCCKG_R {
        FMCCKG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AXI slave OCTOSPI1 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn octospi1ckg(&self) -> OCTOSPI1CKG_R {
        OCTOSPI1CKG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AXI slave SRAM1 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn axiram1ckg(&self) -> AXIRAM1CKG_R {
        AXIRAM1CKG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AXI matrix slave SRAM2 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn axiram2ckg(&self) -> AXIRAM2CKG_R {
        AXIRAM2CKG_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AXI matrix slave SRAM3 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn axiram3ckg(&self) -> AXIRAM3CKG_R {
        AXIRAM3CKG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AXI matrix slave GFXMMU clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn gfxmmusckg(&self) -> GFXMMUSCKG_R {
        GFXMMUSCKG_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 29 - RAM error code correction (ECC) clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn eccramckg(&self) -> ECCRAMCKG_R {
        ECCRAMCKG_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - EXTI clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn extickg(&self) -> EXTICKG_R {
        EXTICKG_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - JTAG automatic clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn jtagckg(&self) -> JTAGCKG_R {
        JTAGCKG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AXI interconnect matrix clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn axickg(&mut self) -> AXICKG_W<CKGAENRrs> {
        AXICKG_W::new(self, 0)
    }
    #[doc = "Bit 1 - AXI master AHB clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ahbckg(&mut self) -> AHBCKG_W<CKGAENRrs> {
        AHBCKG_W::new(self, 1)
    }
    #[doc = "Bit 2 - AXI master CPU clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn cpuckg(&mut self) -> CPUCKG_W<CKGAENRrs> {
        CPUCKG_W::new(self, 2)
    }
    #[doc = "Bit 3 - AXI master SDMMC clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sdmmcckg(&mut self) -> SDMMCCKG_W<CKGAENRrs> {
        SDMMCCKG_W::new(self, 3)
    }
    #[doc = "Bit 4 - AXI master MDMA clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn mdmackg(&mut self) -> MDMACKG_W<CKGAENRrs> {
        MDMACKG_W::new(self, 4)
    }
    #[doc = "Bit 5 - AXI master DMA2D clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dma2dckg(&mut self) -> DMA2DCKG_W<CKGAENRrs> {
        DMA2DCKG_W::new(self, 5)
    }
    #[doc = "Bit 6 - AXI master LTDC clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ltdcckg(&mut self) -> LTDCCKG_W<CKGAENRrs> {
        LTDCCKG_W::new(self, 6)
    }
    #[doc = "Bit 7 - AXI master GFXMMU clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gfxmmumckg(&mut self) -> GFXMMUMCKG_W<CKGAENRrs> {
        GFXMMUMCKG_W::new(self, 7)
    }
    #[doc = "Bit 8 - AXI slave AHB12 clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ahb12ckg(&mut self) -> AHB12CKG_W<CKGAENRrs> {
        AHB12CKG_W::new(self, 8)
    }
    #[doc = "Bit 9 - AXI slave AHB34 clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ahb34ckg(&mut self) -> AHB34CKG_W<CKGAENRrs> {
        AHB34CKG_W::new(self, 9)
    }
    #[doc = "Bit 10 - AXI slave Flash interface (FLIFT) clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fliftckg(&mut self) -> FLIFTCKG_W<CKGAENRrs> {
        FLIFTCKG_W::new(self, 10)
    }
    #[doc = "Bit 11 - AXI slave OCTOSPI2 clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn octospi2ckg(&mut self) -> OCTOSPI2CKG_W<CKGAENRrs> {
        OCTOSPI2CKG_W::new(self, 11)
    }
    #[doc = "Bit 12 - AXI slave FMC clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmcckg(&mut self) -> FMCCKG_W<CKGAENRrs> {
        FMCCKG_W::new(self, 12)
    }
    #[doc = "Bit 13 - AXI slave OCTOSPI1 clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn octospi1ckg(&mut self) -> OCTOSPI1CKG_W<CKGAENRrs> {
        OCTOSPI1CKG_W::new(self, 13)
    }
    #[doc = "Bit 14 - AXI slave SRAM1 clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn axiram1ckg(&mut self) -> AXIRAM1CKG_W<CKGAENRrs> {
        AXIRAM1CKG_W::new(self, 14)
    }
    #[doc = "Bit 15 - AXI matrix slave SRAM2 clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn axiram2ckg(&mut self) -> AXIRAM2CKG_W<CKGAENRrs> {
        AXIRAM2CKG_W::new(self, 15)
    }
    #[doc = "Bit 16 - AXI matrix slave SRAM3 clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn axiram3ckg(&mut self) -> AXIRAM3CKG_W<CKGAENRrs> {
        AXIRAM3CKG_W::new(self, 16)
    }
    #[doc = "Bit 17 - AXI matrix slave GFXMMU clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gfxmmusckg(&mut self) -> GFXMMUSCKG_W<CKGAENRrs> {
        GFXMMUSCKG_W::new(self, 17)
    }
    #[doc = "Bit 29 - RAM error code correction (ECC) clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn eccramckg(&mut self) -> ECCRAMCKG_W<CKGAENRrs> {
        ECCRAMCKG_W::new(self, 29)
    }
    #[doc = "Bit 30 - EXTI clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn extickg(&mut self) -> EXTICKG_W<CKGAENRrs> {
        EXTICKG_W::new(self, 30)
    }
    #[doc = "Bit 31 - JTAG automatic clock gating This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn jtagckg(&mut self) -> JTAGCKG_W<CKGAENRrs> {
        JTAGCKG_W::new(self, 31)
    }
}
#[doc = "RCC AXI clocks gating enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgaenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckgaenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CKGAENRrs;
impl crate::RegisterSpec for CKGAENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgaenr::R`](R) reader structure"]
impl crate::Readable for CKGAENRrs {}
#[doc = "`write(|w| ..)` method takes [`ckgaenr::W`](W) writer structure"]
impl crate::Writable for CKGAENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CKGAENR to value 0"]
impl crate::Resettable for CKGAENRrs {
    const RESET_VALUE: u32 = 0;
}
