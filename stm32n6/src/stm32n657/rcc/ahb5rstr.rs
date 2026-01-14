///Register `AHB5RSTR` reader
pub type R = crate::R<AHB5RSTRrs>;
///Register `AHB5RSTR` writer
pub type W = crate::W<AHB5RSTRrs>;
///Field `HPDMA1RST` reader - HPDMA1 reset
pub type HPDMA1RST_R = crate::BitReader;
///Field `HPDMA1RST` writer - HPDMA1 reset
pub type HPDMA1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA2DRST` reader - DMA2D reset
pub type DMA2DRST_R = crate::BitReader;
///Field `DMA2DRST` writer - DMA2D reset
pub type DMA2DRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JPEGRST` reader - JPEG reset
pub type JPEGRST_R = crate::BitReader;
///Field `JPEGRST` writer - JPEG reset
pub type JPEGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCRST` reader - FMC reset
pub type FMCRST_R = crate::BitReader;
///Field `FMCRST` writer - FMC reset
pub type FMCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI1RST` reader - XSPI1 reset
pub type XSPI1RST_R = crate::BitReader;
///Field `XSPI1RST` writer - XSPI1 reset
pub type XSPI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSSIRST` reader - PSSI reset
pub type PSSIRST_R = crate::BitReader;
///Field `PSSIRST` writer - PSSI reset
pub type PSSIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2RST` reader - SDMMC2 reset
pub type SDMMC2RST_R = crate::BitReader;
///Field `SDMMC2RST` writer - SDMMC2 reset
pub type SDMMC2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1RST` reader - SDMMC1 reset
pub type SDMMC1RST_R = crate::BitReader;
///Field `SDMMC1RST` writer - SDMMC1 reset
pub type SDMMC1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI2RST` reader - XSPI2 reset
pub type XSPI2RST_R = crate::BitReader;
///Field `XSPI2RST` writer - XSPI2 reset
pub type XSPI2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIMRST` reader - XSPIM reset
pub type XSPIMRST_R = crate::BitReader;
///Field `XSPIMRST` writer - XSPIM reset
pub type XSPIMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPI3RST` reader - XSPI3 reset
pub type XSPI3RST_R = crate::BitReader;
///Field `XSPI3RST` writer - XSPI3 reset
pub type XSPI3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCE4RST` reader - MCE4 reset
pub type MCE4RST_R = crate::BitReader;
///Field `MCE4RST` writer - MCE4 reset
pub type MCE4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GFXMMURST` reader - GFXMMU reset
pub type GFXMMURST_R = crate::BitReader;
///Field `GFXMMURST` writer - GFXMMU reset
pub type GFXMMURST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPURST` reader - GPU reset
pub type GPURST_R = crate::BitReader;
///Field `GPURST` writer - GPU reset
pub type GPURST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFGOTGHSPHY1RST` reader - SYSCFGOTGHSPHY1 reset
pub type SYSCFGOTGHSPHY1RST_R = crate::BitReader;
///Field `SYSCFGOTGHSPHY1RST` writer - SYSCFGOTGHSPHY1 reset
pub type SYSCFGOTGHSPHY1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFGOTGHSPHY2RST` reader - SYSCFGOTGHSPHY2 reset
pub type SYSCFGOTGHSPHY2RST_R = crate::BitReader;
///Field `SYSCFGOTGHSPHY2RST` writer - SYSCFGOTGHSPHY2 reset
pub type SYSCFGOTGHSPHY2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1RST` reader - ETH1 reset
pub type ETH1RST_R = crate::BitReader;
///Field `ETH1RST` writer - ETH1 reset
pub type ETH1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG1RST` reader - OTG1 reset
pub type OTG1RST_R = crate::BitReader;
///Field `OTG1RST` writer - OTG1 reset
pub type OTG1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGPHY1RST` reader - OTGPHY1 reset
pub type OTGPHY1RST_R = crate::BitReader;
///Field `OTGPHY1RST` writer - OTGPHY1 reset
pub type OTGPHY1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGPHY2RST` reader - OTGPHY2 reset
pub type OTGPHY2RST_R = crate::BitReader;
///Field `OTGPHY2RST` writer - OTGPHY2 reset
pub type OTGPHY2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTG2RST` reader - OTG2 reset
pub type OTG2RST_R = crate::BitReader;
///Field `OTG2RST` writer - OTG2 reset
pub type OTG2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHERST` reader - NPUCACHE reset
pub type NPUCACHERST_R = crate::BitReader;
///Field `NPUCACHERST` writer - NPUCACHE reset
pub type NPUCACHERST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPURST` reader - NPU reset
pub type NPURST_R = crate::BitReader;
///Field `NPURST` writer - NPU reset
pub type NPURST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - HPDMA1 reset
    #[inline(always)]
    pub fn hpdma1rst(&self) -> HPDMA1RST_R {
        HPDMA1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2D reset
    #[inline(always)]
    pub fn dma2drst(&self) -> DMA2DRST_R {
        DMA2DRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - JPEG reset
    #[inline(always)]
    pub fn jpegrst(&self) -> JPEGRST_R {
        JPEGRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FMC reset
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - XSPI1 reset
    #[inline(always)]
    pub fn xspi1rst(&self) -> XSPI1RST_R {
        XSPI1RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PSSI reset
    #[inline(always)]
    pub fn pssirst(&self) -> PSSIRST_R {
        PSSIRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SDMMC2 reset
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SDMMC1 reset
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - XSPI2 reset
    #[inline(always)]
    pub fn xspi2rst(&self) -> XSPI2RST_R {
        XSPI2RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - XSPIM reset
    #[inline(always)]
    pub fn xspimrst(&self) -> XSPIMRST_R {
        XSPIMRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 17 - XSPI3 reset
    #[inline(always)]
    pub fn xspi3rst(&self) -> XSPI3RST_R {
        XSPI3RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - MCE4 reset
    #[inline(always)]
    pub fn mce4rst(&self) -> MCE4RST_R {
        MCE4RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - GFXMMU reset
    #[inline(always)]
    pub fn gfxmmurst(&self) -> GFXMMURST_R {
        GFXMMURST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - GPU reset
    #[inline(always)]
    pub fn gpurst(&self) -> GPURST_R {
        GPURST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 23 - SYSCFGOTGHSPHY1 reset
    #[inline(always)]
    pub fn syscfgotghsphy1rst(&self) -> SYSCFGOTGHSPHY1RST_R {
        SYSCFGOTGHSPHY1RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SYSCFGOTGHSPHY2 reset
    #[inline(always)]
    pub fn syscfgotghsphy2rst(&self) -> SYSCFGOTGHSPHY2RST_R {
        SYSCFGOTGHSPHY2RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - ETH1 reset
    #[inline(always)]
    pub fn eth1rst(&self) -> ETH1RST_R {
        ETH1RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - OTG1 reset
    #[inline(always)]
    pub fn otg1rst(&self) -> OTG1RST_R {
        OTG1RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - OTGPHY1 reset
    #[inline(always)]
    pub fn otgphy1rst(&self) -> OTGPHY1RST_R {
        OTGPHY1RST_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - OTGPHY2 reset
    #[inline(always)]
    pub fn otgphy2rst(&self) -> OTGPHY2RST_R {
        OTGPHY2RST_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - OTG2 reset
    #[inline(always)]
    pub fn otg2rst(&self) -> OTG2RST_R {
        OTG2RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - NPUCACHE reset
    #[inline(always)]
    pub fn npucacherst(&self) -> NPUCACHERST_R {
        NPUCACHERST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - NPU reset
    #[inline(always)]
    pub fn npurst(&self) -> NPURST_R {
        NPURST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB5RSTR")
            .field("hpdma1rst", &self.hpdma1rst())
            .field("dma2drst", &self.dma2drst())
            .field("jpegrst", &self.jpegrst())
            .field("fmcrst", &self.fmcrst())
            .field("xspi1rst", &self.xspi1rst())
            .field("pssirst", &self.pssirst())
            .field("sdmmc2rst", &self.sdmmc2rst())
            .field("sdmmc1rst", &self.sdmmc1rst())
            .field("xspi2rst", &self.xspi2rst())
            .field("xspimrst", &self.xspimrst())
            .field("xspi3rst", &self.xspi3rst())
            .field("mce4rst", &self.mce4rst())
            .field("gfxmmurst", &self.gfxmmurst())
            .field("gpurst", &self.gpurst())
            .field("syscfgotghsphy1rst", &self.syscfgotghsphy1rst())
            .field("syscfgotghsphy2rst", &self.syscfgotghsphy2rst())
            .field("eth1rst", &self.eth1rst())
            .field("otg1rst", &self.otg1rst())
            .field("otgphy1rst", &self.otgphy1rst())
            .field("otgphy2rst", &self.otgphy2rst())
            .field("otg2rst", &self.otg2rst())
            .field("npucacherst", &self.npucacherst())
            .field("npurst", &self.npurst())
            .finish()
    }
}
impl W {
    ///Bit 0 - HPDMA1 reset
    #[inline(always)]
    pub fn hpdma1rst(&mut self) -> HPDMA1RST_W<'_, AHB5RSTRrs> {
        HPDMA1RST_W::new(self, 0)
    }
    ///Bit 1 - DMA2D reset
    #[inline(always)]
    pub fn dma2drst(&mut self) -> DMA2DRST_W<'_, AHB5RSTRrs> {
        DMA2DRST_W::new(self, 1)
    }
    ///Bit 3 - JPEG reset
    #[inline(always)]
    pub fn jpegrst(&mut self) -> JPEGRST_W<'_, AHB5RSTRrs> {
        JPEGRST_W::new(self, 3)
    }
    ///Bit 4 - FMC reset
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FMCRST_W<'_, AHB5RSTRrs> {
        FMCRST_W::new(self, 4)
    }
    ///Bit 5 - XSPI1 reset
    #[inline(always)]
    pub fn xspi1rst(&mut self) -> XSPI1RST_W<'_, AHB5RSTRrs> {
        XSPI1RST_W::new(self, 5)
    }
    ///Bit 6 - PSSI reset
    #[inline(always)]
    pub fn pssirst(&mut self) -> PSSIRST_W<'_, AHB5RSTRrs> {
        PSSIRST_W::new(self, 6)
    }
    ///Bit 7 - SDMMC2 reset
    #[inline(always)]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W<'_, AHB5RSTRrs> {
        SDMMC2RST_W::new(self, 7)
    }
    ///Bit 8 - SDMMC1 reset
    #[inline(always)]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<'_, AHB5RSTRrs> {
        SDMMC1RST_W::new(self, 8)
    }
    ///Bit 12 - XSPI2 reset
    #[inline(always)]
    pub fn xspi2rst(&mut self) -> XSPI2RST_W<'_, AHB5RSTRrs> {
        XSPI2RST_W::new(self, 12)
    }
    ///Bit 13 - XSPIM reset
    #[inline(always)]
    pub fn xspimrst(&mut self) -> XSPIMRST_W<'_, AHB5RSTRrs> {
        XSPIMRST_W::new(self, 13)
    }
    ///Bit 17 - XSPI3 reset
    #[inline(always)]
    pub fn xspi3rst(&mut self) -> XSPI3RST_W<'_, AHB5RSTRrs> {
        XSPI3RST_W::new(self, 17)
    }
    ///Bit 18 - MCE4 reset
    #[inline(always)]
    pub fn mce4rst(&mut self) -> MCE4RST_W<'_, AHB5RSTRrs> {
        MCE4RST_W::new(self, 18)
    }
    ///Bit 19 - GFXMMU reset
    #[inline(always)]
    pub fn gfxmmurst(&mut self) -> GFXMMURST_W<'_, AHB5RSTRrs> {
        GFXMMURST_W::new(self, 19)
    }
    ///Bit 20 - GPU reset
    #[inline(always)]
    pub fn gpurst(&mut self) -> GPURST_W<'_, AHB5RSTRrs> {
        GPURST_W::new(self, 20)
    }
    ///Bit 23 - SYSCFGOTGHSPHY1 reset
    #[inline(always)]
    pub fn syscfgotghsphy1rst(&mut self) -> SYSCFGOTGHSPHY1RST_W<'_, AHB5RSTRrs> {
        SYSCFGOTGHSPHY1RST_W::new(self, 23)
    }
    ///Bit 24 - SYSCFGOTGHSPHY2 reset
    #[inline(always)]
    pub fn syscfgotghsphy2rst(&mut self) -> SYSCFGOTGHSPHY2RST_W<'_, AHB5RSTRrs> {
        SYSCFGOTGHSPHY2RST_W::new(self, 24)
    }
    ///Bit 25 - ETH1 reset
    #[inline(always)]
    pub fn eth1rst(&mut self) -> ETH1RST_W<'_, AHB5RSTRrs> {
        ETH1RST_W::new(self, 25)
    }
    ///Bit 26 - OTG1 reset
    #[inline(always)]
    pub fn otg1rst(&mut self) -> OTG1RST_W<'_, AHB5RSTRrs> {
        OTG1RST_W::new(self, 26)
    }
    ///Bit 27 - OTGPHY1 reset
    #[inline(always)]
    pub fn otgphy1rst(&mut self) -> OTGPHY1RST_W<'_, AHB5RSTRrs> {
        OTGPHY1RST_W::new(self, 27)
    }
    ///Bit 28 - OTGPHY2 reset
    #[inline(always)]
    pub fn otgphy2rst(&mut self) -> OTGPHY2RST_W<'_, AHB5RSTRrs> {
        OTGPHY2RST_W::new(self, 28)
    }
    ///Bit 29 - OTG2 reset
    #[inline(always)]
    pub fn otg2rst(&mut self) -> OTG2RST_W<'_, AHB5RSTRrs> {
        OTG2RST_W::new(self, 29)
    }
    ///Bit 30 - NPUCACHE reset
    #[inline(always)]
    pub fn npucacherst(&mut self) -> NPUCACHERST_W<'_, AHB5RSTRrs> {
        NPUCACHERST_W::new(self, 30)
    }
    ///Bit 31 - NPU reset
    #[inline(always)]
    pub fn npurst(&mut self) -> NPURST_W<'_, AHB5RSTRrs> {
        NPURST_W::new(self, 31)
    }
}
/**RCC AHB5 reset register

You can [`read`](crate::Reg::read) this register and get [`ahb5rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:AHB5RSTR)*/
pub struct AHB5RSTRrs;
impl crate::RegisterSpec for AHB5RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb5rstr::R`](R) reader structure
impl crate::Readable for AHB5RSTRrs {}
///`write(|w| ..)` method takes [`ahb5rstr::W`](W) writer structure
impl crate::Writable for AHB5RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB5RSTR to value 0
impl crate::Resettable for AHB5RSTRrs {}
