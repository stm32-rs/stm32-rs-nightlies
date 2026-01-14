///Register `CKGDISR` reader
pub type R = crate::R<CKGDISRrs>;
///Register `CKGDISR` writer
pub type W = crate::W<CKGDISRrs>;
///Field `AXICKG` reader - AXI interconnect matrix clock gating disable
pub type AXICKG_R = crate::BitReader;
///Field `AXICKG` writer - AXI interconnect matrix clock gating disable
pub type AXICKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMCKG` reader - AXI master AHB clock gating disable
pub type AHBMCKG_R = crate::BitReader;
///Field `AHBMCKG` writer - AXI master AHB clock gating disable
pub type AHBMCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1CKG` reader - AXI master SDMMC1 clock gating disable
pub type SDMMC1CKG_R = crate::BitReader;
///Field `SDMMC1CKG` writer - AXI master SDMMC1 clock gating disable
pub type SDMMC1CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPDMA1CKG` reader - AXI master HPDMA1 clock gating disable
pub type HPDMA1CKG_R = crate::BitReader;
///Field `HPDMA1CKG` writer - AXI master HPDMA1 clock gating disable
pub type HPDMA1CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPUCKG` reader - AXI master CPU clock gating disable
pub type CPUCKG_R = crate::BitReader;
///Field `CPUCKG` writer - AXI master CPU clock gating disable
pub type CPUCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPU2DS0CKG` reader - AXI master 0 GPU2D clock gating disable
pub type GPU2DS0CKG_R = crate::BitReader;
///Field `GPU2DS0CKG` writer - AXI master 0 GPU2D clock gating disable
pub type GPU2DS0CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPU2DS1CKG` reader - AXI master 1 GPU2D clock gating disable
pub type GPU2DS1CKG_R = crate::BitReader;
///Field `GPU2DS1CKG` writer - AXI master 1 GPU2D clock gating disable
pub type GPU2DS1CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPU2DCLCKG` reader - AXI master cache GPU2D clock gating disable
pub type GPU2DCLCKG_R = crate::BitReader;
///Field `GPU2DCLCKG` writer - AXI master cache GPU2D clock gating disable
pub type GPU2DCLCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCMIPPCKG` reader - AXI master DCMIPP clock gating disable
pub type DCMIPPCKG_R = crate::BitReader;
///Field `DCMIPPCKG` writer - AXI master DCMIPP clock gating disable
pub type DCMIPPCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2DCKG` reader - AXI master DMA2D clock gating disable
pub type DMA2DCKG_R = crate::BitReader;
///Field `DMA2DCKG` writer - AXI master DMA2D clock gating disable
pub type DMA2DCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXMMUSCKG` reader - AXI matrix slave GFXMMU clock gating disable
pub type GFXMMUSCKG_R = crate::BitReader;
///Field `GFXMMUSCKG` writer - AXI matrix slave GFXMMU clock gating disable
pub type GFXMMUSCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LTDCCKG` reader - AXI master LTDC clock gating disable
pub type LTDCCKG_R = crate::BitReader;
///Field `LTDCCKG` writer - AXI master LTDC clock gating disable
pub type LTDCCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXMMUMCKG` reader - AXI master GFXMMU clock gating disable
pub type GFXMMUMCKG_R = crate::BitReader;
///Field `GFXMMUMCKG` writer - AXI master GFXMMU clock gating disable
pub type GFXMMUMCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSCKG` reader - AXI slave AHB clock gating disable
pub type AHBSCKG_R = crate::BitReader;
///Field `AHBSCKG` writer - AXI slave AHB clock gating disable
pub type AHBSCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCCKG` reader - AXI slave FMC and MCE3 clock gating disable
pub type FMCCKG_R = crate::BitReader;
///Field `FMCCKG` writer - AXI slave FMC and MCE3 clock gating disable
pub type FMCCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI1CKG` reader - AXI slave XSPI1 and MCE1 clock gating disable
pub type XSPI1CKG_R = crate::BitReader;
///Field `XSPI1CKG` writer - AXI slave XSPI1 and MCE1 clock gating disable
pub type XSPI1CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI2CKG` reader - AXI slave XSPI2 and MCE2 clock gating disable
pub type XSPI2CKG_R = crate::BitReader;
///Field `XSPI2CKG` writer - AXI slave XSPI2 and MCE2 clock gating disable
pub type XSPI2CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM4CKG` reader - AXI matrix slave SRAM4 clock gating disable
pub type AXISRAM4CKG_R = crate::BitReader;
///Field `AXISRAM4CKG` writer - AXI matrix slave SRAM4 clock gating disable
pub type AXISRAM4CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM3CKG` reader - AXI matrix slave SRAM3 clock gating disable
pub type AXISRAM3CKG_R = crate::BitReader;
///Field `AXISRAM3CKG` writer - AXI matrix slave SRAM3 clock gating disable
pub type AXISRAM3CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM2CKG` reader - AXI slave SRAM2 clock gating disable
pub type AXISRAM2CKG_R = crate::BitReader;
///Field `AXISRAM2CKG` writer - AXI slave SRAM2 clock gating disable
pub type AXISRAM2CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM1CKG` reader - AXI slave SRAM1 / error code correction (ECC) clock gating disable
pub type AXISRAM1CKG_R = crate::BitReader;
///Field `AXISRAM1CKG` writer - AXI slave SRAM1 / error code correction (ECC) clock gating disable
pub type AXISRAM1CKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHCKG` reader - AXI slave Flash interface (FLIFT) clock gating disable
pub type FLASHCKG_R = crate::BitReader;
///Field `FLASHCKG` writer - AXI slave Flash interface (FLIFT) clock gating disable
pub type FLASHCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTICKG` reader - EXTI clock gating disable
pub type EXTICKG_R = crate::BitReader;
///Field `EXTICKG` writer - EXTI clock gating disable
pub type EXTICKG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JTAGCKG` reader - JTAG automatic clock gating disabling
pub type JTAGCKG_R = crate::BitReader;
///Field `JTAGCKG` writer - JTAG automatic clock gating disabling
pub type JTAGCKG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AXI interconnect matrix clock gating disable
    #[inline(always)]
    pub fn axickg(&self) -> AXICKG_R {
        AXICKG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AXI master AHB clock gating disable
    #[inline(always)]
    pub fn ahbmckg(&self) -> AHBMCKG_R {
        AHBMCKG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AXI master SDMMC1 clock gating disable
    #[inline(always)]
    pub fn sdmmc1ckg(&self) -> SDMMC1CKG_R {
        SDMMC1CKG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AXI master HPDMA1 clock gating disable
    #[inline(always)]
    pub fn hpdma1ckg(&self) -> HPDMA1CKG_R {
        HPDMA1CKG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AXI master CPU clock gating disable
    #[inline(always)]
    pub fn cpuckg(&self) -> CPUCKG_R {
        CPUCKG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AXI master 0 GPU2D clock gating disable
    #[inline(always)]
    pub fn gpu2ds0ckg(&self) -> GPU2DS0CKG_R {
        GPU2DS0CKG_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AXI master 1 GPU2D clock gating disable
    #[inline(always)]
    pub fn gpu2ds1ckg(&self) -> GPU2DS1CKG_R {
        GPU2DS1CKG_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AXI master cache GPU2D clock gating disable
    #[inline(always)]
    pub fn gpu2dclckg(&self) -> GPU2DCLCKG_R {
        GPU2DCLCKG_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AXI master DCMIPP clock gating disable
    #[inline(always)]
    pub fn dcmippckg(&self) -> DCMIPPCKG_R {
        DCMIPPCKG_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AXI master DMA2D clock gating disable
    #[inline(always)]
    pub fn dma2dckg(&self) -> DMA2DCKG_R {
        DMA2DCKG_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - AXI matrix slave GFXMMU clock gating disable
    #[inline(always)]
    pub fn gfxmmusckg(&self) -> GFXMMUSCKG_R {
        GFXMMUSCKG_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - AXI master LTDC clock gating disable
    #[inline(always)]
    pub fn ltdcckg(&self) -> LTDCCKG_R {
        LTDCCKG_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - AXI master GFXMMU clock gating disable
    #[inline(always)]
    pub fn gfxmmumckg(&self) -> GFXMMUMCKG_R {
        GFXMMUMCKG_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - AXI slave AHB clock gating disable
    #[inline(always)]
    pub fn ahbsckg(&self) -> AHBSCKG_R {
        AHBSCKG_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - AXI slave FMC and MCE3 clock gating disable
    #[inline(always)]
    pub fn fmcckg(&self) -> FMCCKG_R {
        FMCCKG_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - AXI slave XSPI1 and MCE1 clock gating disable
    #[inline(always)]
    pub fn xspi1ckg(&self) -> XSPI1CKG_R {
        XSPI1CKG_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - AXI slave XSPI2 and MCE2 clock gating disable
    #[inline(always)]
    pub fn xspi2ckg(&self) -> XSPI2CKG_R {
        XSPI2CKG_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AXI matrix slave SRAM4 clock gating disable
    #[inline(always)]
    pub fn axisram4ckg(&self) -> AXISRAM4CKG_R {
        AXISRAM4CKG_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - AXI matrix slave SRAM3 clock gating disable
    #[inline(always)]
    pub fn axisram3ckg(&self) -> AXISRAM3CKG_R {
        AXISRAM3CKG_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - AXI slave SRAM2 clock gating disable
    #[inline(always)]
    pub fn axisram2ckg(&self) -> AXISRAM2CKG_R {
        AXISRAM2CKG_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - AXI slave SRAM1 / error code correction (ECC) clock gating disable
    #[inline(always)]
    pub fn axisram1ckg(&self) -> AXISRAM1CKG_R {
        AXISRAM1CKG_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - AXI slave Flash interface (FLIFT) clock gating disable
    #[inline(always)]
    pub fn flashckg(&self) -> FLASHCKG_R {
        FLASHCKG_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 30 - EXTI clock gating disable
    #[inline(always)]
    pub fn extickg(&self) -> EXTICKG_R {
        EXTICKG_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - JTAG automatic clock gating disabling
    #[inline(always)]
    pub fn jtagckg(&self) -> JTAGCKG_R {
        JTAGCKG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKGDISR")
            .field("axickg", &self.axickg())
            .field("ahbmckg", &self.ahbmckg())
            .field("sdmmc1ckg", &self.sdmmc1ckg())
            .field("hpdma1ckg", &self.hpdma1ckg())
            .field("cpuckg", &self.cpuckg())
            .field("gpu2ds0ckg", &self.gpu2ds0ckg())
            .field("gpu2ds1ckg", &self.gpu2ds1ckg())
            .field("gpu2dclckg", &self.gpu2dclckg())
            .field("dcmippckg", &self.dcmippckg())
            .field("dma2dckg", &self.dma2dckg())
            .field("gfxmmusckg", &self.gfxmmusckg())
            .field("ltdcckg", &self.ltdcckg())
            .field("gfxmmumckg", &self.gfxmmumckg())
            .field("ahbsckg", &self.ahbsckg())
            .field("fmcckg", &self.fmcckg())
            .field("xspi1ckg", &self.xspi1ckg())
            .field("xspi2ckg", &self.xspi2ckg())
            .field("axisram4ckg", &self.axisram4ckg())
            .field("axisram3ckg", &self.axisram3ckg())
            .field("axisram2ckg", &self.axisram2ckg())
            .field("axisram1ckg", &self.axisram1ckg())
            .field("flashckg", &self.flashckg())
            .field("extickg", &self.extickg())
            .field("jtagckg", &self.jtagckg())
            .finish()
    }
}
impl W {
    ///Bit 0 - AXI interconnect matrix clock gating disable
    #[inline(always)]
    pub fn axickg(&mut self) -> AXICKG_W<'_, CKGDISRrs> {
        AXICKG_W::new(self, 0)
    }
    ///Bit 1 - AXI master AHB clock gating disable
    #[inline(always)]
    pub fn ahbmckg(&mut self) -> AHBMCKG_W<'_, CKGDISRrs> {
        AHBMCKG_W::new(self, 1)
    }
    ///Bit 2 - AXI master SDMMC1 clock gating disable
    #[inline(always)]
    pub fn sdmmc1ckg(&mut self) -> SDMMC1CKG_W<'_, CKGDISRrs> {
        SDMMC1CKG_W::new(self, 2)
    }
    ///Bit 3 - AXI master HPDMA1 clock gating disable
    #[inline(always)]
    pub fn hpdma1ckg(&mut self) -> HPDMA1CKG_W<'_, CKGDISRrs> {
        HPDMA1CKG_W::new(self, 3)
    }
    ///Bit 4 - AXI master CPU clock gating disable
    #[inline(always)]
    pub fn cpuckg(&mut self) -> CPUCKG_W<'_, CKGDISRrs> {
        CPUCKG_W::new(self, 4)
    }
    ///Bit 5 - AXI master 0 GPU2D clock gating disable
    #[inline(always)]
    pub fn gpu2ds0ckg(&mut self) -> GPU2DS0CKG_W<'_, CKGDISRrs> {
        GPU2DS0CKG_W::new(self, 5)
    }
    ///Bit 6 - AXI master 1 GPU2D clock gating disable
    #[inline(always)]
    pub fn gpu2ds1ckg(&mut self) -> GPU2DS1CKG_W<'_, CKGDISRrs> {
        GPU2DS1CKG_W::new(self, 6)
    }
    ///Bit 7 - AXI master cache GPU2D clock gating disable
    #[inline(always)]
    pub fn gpu2dclckg(&mut self) -> GPU2DCLCKG_W<'_, CKGDISRrs> {
        GPU2DCLCKG_W::new(self, 7)
    }
    ///Bit 8 - AXI master DCMIPP clock gating disable
    #[inline(always)]
    pub fn dcmippckg(&mut self) -> DCMIPPCKG_W<'_, CKGDISRrs> {
        DCMIPPCKG_W::new(self, 8)
    }
    ///Bit 9 - AXI master DMA2D clock gating disable
    #[inline(always)]
    pub fn dma2dckg(&mut self) -> DMA2DCKG_W<'_, CKGDISRrs> {
        DMA2DCKG_W::new(self, 9)
    }
    ///Bit 10 - AXI matrix slave GFXMMU clock gating disable
    #[inline(always)]
    pub fn gfxmmusckg(&mut self) -> GFXMMUSCKG_W<'_, CKGDISRrs> {
        GFXMMUSCKG_W::new(self, 10)
    }
    ///Bit 11 - AXI master LTDC clock gating disable
    #[inline(always)]
    pub fn ltdcckg(&mut self) -> LTDCCKG_W<'_, CKGDISRrs> {
        LTDCCKG_W::new(self, 11)
    }
    ///Bit 12 - AXI master GFXMMU clock gating disable
    #[inline(always)]
    pub fn gfxmmumckg(&mut self) -> GFXMMUMCKG_W<'_, CKGDISRrs> {
        GFXMMUMCKG_W::new(self, 12)
    }
    ///Bit 13 - AXI slave AHB clock gating disable
    #[inline(always)]
    pub fn ahbsckg(&mut self) -> AHBSCKG_W<'_, CKGDISRrs> {
        AHBSCKG_W::new(self, 13)
    }
    ///Bit 14 - AXI slave FMC and MCE3 clock gating disable
    #[inline(always)]
    pub fn fmcckg(&mut self) -> FMCCKG_W<'_, CKGDISRrs> {
        FMCCKG_W::new(self, 14)
    }
    ///Bit 15 - AXI slave XSPI1 and MCE1 clock gating disable
    #[inline(always)]
    pub fn xspi1ckg(&mut self) -> XSPI1CKG_W<'_, CKGDISRrs> {
        XSPI1CKG_W::new(self, 15)
    }
    ///Bit 16 - AXI slave XSPI2 and MCE2 clock gating disable
    #[inline(always)]
    pub fn xspi2ckg(&mut self) -> XSPI2CKG_W<'_, CKGDISRrs> {
        XSPI2CKG_W::new(self, 16)
    }
    ///Bit 17 - AXI matrix slave SRAM4 clock gating disable
    #[inline(always)]
    pub fn axisram4ckg(&mut self) -> AXISRAM4CKG_W<'_, CKGDISRrs> {
        AXISRAM4CKG_W::new(self, 17)
    }
    ///Bit 18 - AXI matrix slave SRAM3 clock gating disable
    #[inline(always)]
    pub fn axisram3ckg(&mut self) -> AXISRAM3CKG_W<'_, CKGDISRrs> {
        AXISRAM3CKG_W::new(self, 18)
    }
    ///Bit 19 - AXI slave SRAM2 clock gating disable
    #[inline(always)]
    pub fn axisram2ckg(&mut self) -> AXISRAM2CKG_W<'_, CKGDISRrs> {
        AXISRAM2CKG_W::new(self, 19)
    }
    ///Bit 20 - AXI slave SRAM1 / error code correction (ECC) clock gating disable
    #[inline(always)]
    pub fn axisram1ckg(&mut self) -> AXISRAM1CKG_W<'_, CKGDISRrs> {
        AXISRAM1CKG_W::new(self, 20)
    }
    ///Bit 21 - AXI slave Flash interface (FLIFT) clock gating disable
    #[inline(always)]
    pub fn flashckg(&mut self) -> FLASHCKG_W<'_, CKGDISRrs> {
        FLASHCKG_W::new(self, 21)
    }
    ///Bit 30 - EXTI clock gating disable
    #[inline(always)]
    pub fn extickg(&mut self) -> EXTICKG_W<'_, CKGDISRrs> {
        EXTICKG_W::new(self, 30)
    }
    ///Bit 31 - JTAG automatic clock gating disabling
    #[inline(always)]
    pub fn jtagckg(&mut self) -> JTAGCKG_W<'_, CKGDISRrs> {
        JTAGCKG_W::new(self, 31)
    }
}
/**RCC AXI clocks gating disable register

You can [`read`](crate::Reg::read) this register and get [`ckgdisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgdisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:CKGDISR)*/
pub struct CKGDISRrs;
impl crate::RegisterSpec for CKGDISRrs {
    type Ux = u32;
}
///`read()` method returns [`ckgdisr::R`](R) reader structure
impl crate::Readable for CKGDISRrs {}
///`write(|w| ..)` method takes [`ckgdisr::W`](W) writer structure
impl crate::Writable for CKGDISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKGDISR to value 0x8000_0000
impl crate::Resettable for CKGDISRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
