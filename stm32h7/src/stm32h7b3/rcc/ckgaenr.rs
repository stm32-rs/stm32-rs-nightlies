///Register `CKGAENR` reader
pub type R = crate::R<CKGAENRrs>;
///Register `CKGAENR` writer
pub type W = crate::W<CKGAENRrs>;
///Field `AXICKG` reader - AXI interconnect matrix clock gating This bit is set and reset by software.
pub type AXICKG_R = crate::BitReader;
///Field `AXICKG` writer - AXI interconnect matrix clock gating This bit is set and reset by software.
pub type AXICKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBCKG` reader - AXI master AHB clock gating This bit is set and reset by software.
pub type AHBCKG_R = crate::BitReader;
///Field `AHBCKG` writer - AXI master AHB clock gating This bit is set and reset by software.
pub type AHBCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPUCKG` reader - AXI master CPU clock gating This bit is set and reset by software.
pub type CPUCKG_R = crate::BitReader;
///Field `CPUCKG` writer - AXI master CPU clock gating This bit is set and reset by software.
pub type CPUCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMCCKG` reader - AXI master SDMMC clock gating This bit is set and reset by software.
pub type SDMMCCKG_R = crate::BitReader;
///Field `SDMMCCKG` writer - AXI master SDMMC clock gating This bit is set and reset by software.
pub type SDMMCCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDMACKG` reader - AXI master MDMA clock gating This bit is set and reset by software.
pub type MDMACKG_R = crate::BitReader;
///Field `MDMACKG` writer - AXI master MDMA clock gating This bit is set and reset by software.
pub type MDMACKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2DCKG` reader - AXI master DMA2D clock gating This bit is set and reset by software.
pub type DMA2DCKG_R = crate::BitReader;
///Field `DMA2DCKG` writer - AXI master DMA2D clock gating This bit is set and reset by software.
pub type DMA2DCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LTDCCKG` reader - AXI master LTDC clock gating This bit is set and reset by software.
pub type LTDCCKG_R = crate::BitReader;
///Field `LTDCCKG` writer - AXI master LTDC clock gating This bit is set and reset by software.
pub type LTDCCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXMMUMCKG` reader - AXI master GFXMMU clock gating This bit is set and reset by software.
pub type GFXMMUMCKG_R = crate::BitReader;
///Field `GFXMMUMCKG` writer - AXI master GFXMMU clock gating This bit is set and reset by software.
pub type GFXMMUMCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB12CKG` reader - AXI slave AHB12 clock gating This bit is set and reset by software.
pub type AHB12CKG_R = crate::BitReader;
///Field `AHB12CKG` writer - AXI slave AHB12 clock gating This bit is set and reset by software.
pub type AHB12CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB34CKG` reader - AXI slave AHB34 clock gating This bit is set and reset by software.
pub type AHB34CKG_R = crate::BitReader;
///Field `AHB34CKG` writer - AXI slave AHB34 clock gating This bit is set and reset by software.
pub type AHB34CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLIFTCKG` reader - AXI slave Flash interface (FLIFT) clock gating This bit is set and reset by software.
pub type FLIFTCKG_R = crate::BitReader;
///Field `FLIFTCKG` writer - AXI slave Flash interface (FLIFT) clock gating This bit is set and reset by software.
pub type FLIFTCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPI2CKG` reader - AXI slave OCTOSPI2 clock gating This bit is set and reset by software.
pub type OCTOSPI2CKG_R = crate::BitReader;
///Field `OCTOSPI2CKG` writer - AXI slave OCTOSPI2 clock gating This bit is set and reset by software.
pub type OCTOSPI2CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCCKG` reader - AXI slave FMC clock gating This bit is set and reset by software.
pub type FMCCKG_R = crate::BitReader;
///Field `FMCCKG` writer - AXI slave FMC clock gating This bit is set and reset by software.
pub type FMCCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPI1CKG` reader - AXI slave OCTOSPI1 clock gating This bit is set and reset by software.
pub type OCTOSPI1CKG_R = crate::BitReader;
///Field `OCTOSPI1CKG` writer - AXI slave OCTOSPI1 clock gating This bit is set and reset by software.
pub type OCTOSPI1CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXIRAM1CKG` reader - AXI slave SRAM1 clock gating This bit is set and reset by software.
pub type AXIRAM1CKG_R = crate::BitReader;
///Field `AXIRAM1CKG` writer - AXI slave SRAM1 clock gating This bit is set and reset by software.
pub type AXIRAM1CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXIRAM2CKG` reader - AXI matrix slave SRAM2 clock gating This bit is set and reset by software.
pub type AXIRAM2CKG_R = crate::BitReader;
///Field `AXIRAM2CKG` writer - AXI matrix slave SRAM2 clock gating This bit is set and reset by software.
pub type AXIRAM2CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXIRAM3CKG` reader - AXI matrix slave SRAM3 clock gating This bit is set and reset by software.
pub type AXIRAM3CKG_R = crate::BitReader;
///Field `AXIRAM3CKG` writer - AXI matrix slave SRAM3 clock gating This bit is set and reset by software.
pub type AXIRAM3CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXMMUSCKG` reader - AXI matrix slave GFXMMU clock gating This bit is set and reset by software.
pub type GFXMMUSCKG_R = crate::BitReader;
///Field `GFXMMUSCKG` writer - AXI matrix slave GFXMMU clock gating This bit is set and reset by software.
pub type GFXMMUSCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCRAMCKG` reader - RAM error code correction (ECC) clock gating This bit is set and reset by software.
pub type ECCRAMCKG_R = crate::BitReader;
///Field `ECCRAMCKG` writer - RAM error code correction (ECC) clock gating This bit is set and reset by software.
pub type ECCRAMCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTICKG` reader - EXTI clock gating This bit is set and reset by software.
pub type EXTICKG_R = crate::BitReader;
///Field `EXTICKG` writer - EXTI clock gating This bit is set and reset by software.
pub type EXTICKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JTAGCKG` reader - JTAG automatic clock gating This bit is set and reset by software.
pub type JTAGCKG_R = crate::BitReader;
///Field `JTAGCKG` writer - JTAG automatic clock gating This bit is set and reset by software.
pub type JTAGCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AXI interconnect matrix clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn axickg(&self) -> AXICKG_R {
        AXICKG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AXI master AHB clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn ahbckg(&self) -> AHBCKG_R {
        AHBCKG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AXI master CPU clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn cpuckg(&self) -> CPUCKG_R {
        CPUCKG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AXI master SDMMC clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn sdmmcckg(&self) -> SDMMCCKG_R {
        SDMMCCKG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AXI master MDMA clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn mdmackg(&self) -> MDMACKG_R {
        MDMACKG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AXI master DMA2D clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn dma2dckg(&self) -> DMA2DCKG_R {
        DMA2DCKG_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AXI master LTDC clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn ltdcckg(&self) -> LTDCCKG_R {
        LTDCCKG_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AXI master GFXMMU clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn gfxmmumckg(&self) -> GFXMMUMCKG_R {
        GFXMMUMCKG_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AXI slave AHB12 clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn ahb12ckg(&self) -> AHB12CKG_R {
        AHB12CKG_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AXI slave AHB34 clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn ahb34ckg(&self) -> AHB34CKG_R {
        AHB34CKG_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - AXI slave Flash interface (FLIFT) clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn fliftckg(&self) -> FLIFTCKG_R {
        FLIFTCKG_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - AXI slave OCTOSPI2 clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn octospi2ckg(&self) -> OCTOSPI2CKG_R {
        OCTOSPI2CKG_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - AXI slave FMC clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn fmcckg(&self) -> FMCCKG_R {
        FMCCKG_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - AXI slave OCTOSPI1 clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn octospi1ckg(&self) -> OCTOSPI1CKG_R {
        OCTOSPI1CKG_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - AXI slave SRAM1 clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn axiram1ckg(&self) -> AXIRAM1CKG_R {
        AXIRAM1CKG_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - AXI matrix slave SRAM2 clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn axiram2ckg(&self) -> AXIRAM2CKG_R {
        AXIRAM2CKG_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - AXI matrix slave SRAM3 clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn axiram3ckg(&self) -> AXIRAM3CKG_R {
        AXIRAM3CKG_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AXI matrix slave GFXMMU clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn gfxmmusckg(&self) -> GFXMMUSCKG_R {
        GFXMMUSCKG_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 29 - RAM error code correction (ECC) clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn eccramckg(&self) -> ECCRAMCKG_R {
        ECCRAMCKG_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - EXTI clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn extickg(&self) -> EXTICKG_R {
        EXTICKG_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - JTAG automatic clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn jtagckg(&self) -> JTAGCKG_R {
        JTAGCKG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKGAENR")
            .field("axickg", &self.axickg())
            .field("ahbckg", &self.ahbckg())
            .field("cpuckg", &self.cpuckg())
            .field("sdmmcckg", &self.sdmmcckg())
            .field("mdmackg", &self.mdmackg())
            .field("dma2dckg", &self.dma2dckg())
            .field("ltdcckg", &self.ltdcckg())
            .field("gfxmmumckg", &self.gfxmmumckg())
            .field("ahb12ckg", &self.ahb12ckg())
            .field("ahb34ckg", &self.ahb34ckg())
            .field("fliftckg", &self.fliftckg())
            .field("octospi2ckg", &self.octospi2ckg())
            .field("fmcckg", &self.fmcckg())
            .field("octospi1ckg", &self.octospi1ckg())
            .field("axiram1ckg", &self.axiram1ckg())
            .field("axiram2ckg", &self.axiram2ckg())
            .field("axiram3ckg", &self.axiram3ckg())
            .field("gfxmmusckg", &self.gfxmmusckg())
            .field("eccramckg", &self.eccramckg())
            .field("extickg", &self.extickg())
            .field("jtagckg", &self.jtagckg())
            .finish()
    }
}
impl W {
    ///Bit 0 - AXI interconnect matrix clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn axickg(&mut self) -> AXICKG_W<CKGAENRrs> {
        AXICKG_W::new(self, 0)
    }
    ///Bit 1 - AXI master AHB clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn ahbckg(&mut self) -> AHBCKG_W<CKGAENRrs> {
        AHBCKG_W::new(self, 1)
    }
    ///Bit 2 - AXI master CPU clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn cpuckg(&mut self) -> CPUCKG_W<CKGAENRrs> {
        CPUCKG_W::new(self, 2)
    }
    ///Bit 3 - AXI master SDMMC clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn sdmmcckg(&mut self) -> SDMMCCKG_W<CKGAENRrs> {
        SDMMCCKG_W::new(self, 3)
    }
    ///Bit 4 - AXI master MDMA clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn mdmackg(&mut self) -> MDMACKG_W<CKGAENRrs> {
        MDMACKG_W::new(self, 4)
    }
    ///Bit 5 - AXI master DMA2D clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn dma2dckg(&mut self) -> DMA2DCKG_W<CKGAENRrs> {
        DMA2DCKG_W::new(self, 5)
    }
    ///Bit 6 - AXI master LTDC clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn ltdcckg(&mut self) -> LTDCCKG_W<CKGAENRrs> {
        LTDCCKG_W::new(self, 6)
    }
    ///Bit 7 - AXI master GFXMMU clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn gfxmmumckg(&mut self) -> GFXMMUMCKG_W<CKGAENRrs> {
        GFXMMUMCKG_W::new(self, 7)
    }
    ///Bit 8 - AXI slave AHB12 clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn ahb12ckg(&mut self) -> AHB12CKG_W<CKGAENRrs> {
        AHB12CKG_W::new(self, 8)
    }
    ///Bit 9 - AXI slave AHB34 clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn ahb34ckg(&mut self) -> AHB34CKG_W<CKGAENRrs> {
        AHB34CKG_W::new(self, 9)
    }
    ///Bit 10 - AXI slave Flash interface (FLIFT) clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn fliftckg(&mut self) -> FLIFTCKG_W<CKGAENRrs> {
        FLIFTCKG_W::new(self, 10)
    }
    ///Bit 11 - AXI slave OCTOSPI2 clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn octospi2ckg(&mut self) -> OCTOSPI2CKG_W<CKGAENRrs> {
        OCTOSPI2CKG_W::new(self, 11)
    }
    ///Bit 12 - AXI slave FMC clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn fmcckg(&mut self) -> FMCCKG_W<CKGAENRrs> {
        FMCCKG_W::new(self, 12)
    }
    ///Bit 13 - AXI slave OCTOSPI1 clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn octospi1ckg(&mut self) -> OCTOSPI1CKG_W<CKGAENRrs> {
        OCTOSPI1CKG_W::new(self, 13)
    }
    ///Bit 14 - AXI slave SRAM1 clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn axiram1ckg(&mut self) -> AXIRAM1CKG_W<CKGAENRrs> {
        AXIRAM1CKG_W::new(self, 14)
    }
    ///Bit 15 - AXI matrix slave SRAM2 clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn axiram2ckg(&mut self) -> AXIRAM2CKG_W<CKGAENRrs> {
        AXIRAM2CKG_W::new(self, 15)
    }
    ///Bit 16 - AXI matrix slave SRAM3 clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn axiram3ckg(&mut self) -> AXIRAM3CKG_W<CKGAENRrs> {
        AXIRAM3CKG_W::new(self, 16)
    }
    ///Bit 17 - AXI matrix slave GFXMMU clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn gfxmmusckg(&mut self) -> GFXMMUSCKG_W<CKGAENRrs> {
        GFXMMUSCKG_W::new(self, 17)
    }
    ///Bit 29 - RAM error code correction (ECC) clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn eccramckg(&mut self) -> ECCRAMCKG_W<CKGAENRrs> {
        ECCRAMCKG_W::new(self, 29)
    }
    ///Bit 30 - EXTI clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn extickg(&mut self) -> EXTICKG_W<CKGAENRrs> {
        EXTICKG_W::new(self, 30)
    }
    ///Bit 31 - JTAG automatic clock gating This bit is set and reset by software.
    #[inline(always)]
    pub fn jtagckg(&mut self) -> JTAGCKG_W<CKGAENRrs> {
        JTAGCKG_W::new(self, 31)
    }
}
/**RCC AXI clocks gating enable register

You can [`read`](crate::Reg::read) this register and get [`ckgaenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgaenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#RCC:CKGAENR)*/
pub struct CKGAENRrs;
impl crate::RegisterSpec for CKGAENRrs {
    type Ux = u32;
}
///`read()` method returns [`ckgaenr::R`](R) reader structure
impl crate::Readable for CKGAENRrs {}
///`write(|w| ..)` method takes [`ckgaenr::W`](W) writer structure
impl crate::Writable for CKGAENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CKGAENR to value 0
impl crate::Resettable for CKGAENRrs {
    const RESET_VALUE: u32 = 0;
}
