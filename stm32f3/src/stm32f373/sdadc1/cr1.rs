///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `EOCALIE` reader - End of calibration interrupt enable
pub type EOCALIE_R = crate::BitReader;
///Field `EOCALIE` writer - End of calibration interrupt enable
pub type EOCALIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JEOCIE` reader - Injected end of conversion interrupt enable
pub type JEOCIE_R = crate::BitReader;
///Field `JEOCIE` writer - Injected end of conversion interrupt enable
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JOVRIE` reader - Injected data overrun interrupt enable
pub type JOVRIE_R = crate::BitReader;
///Field `JOVRIE` writer - Injected data overrun interrupt enable
pub type JOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REOCIE` reader - Regular end of conversion interrupt enable
pub type REOCIE_R = crate::BitReader;
///Field `REOCIE` writer - Regular end of conversion interrupt enable
pub type REOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROVRIE` reader - Regular data overrun interrupt enable
pub type ROVRIE_R = crate::BitReader;
///Field `ROVRIE` writer - Regular data overrun interrupt enable
pub type ROVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REFV` reader - Reference voltage selection
pub type REFV_R = crate::FieldReader;
///Field `REFV` writer - Reference voltage selection
pub type REFV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SLOWCK` reader - Slow clock mode enable
pub type SLOWCK_R = crate::BitReader;
///Field `SLOWCK` writer - Slow clock mode enable
pub type SLOWCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBI` reader - Enter Standby mode when idle
pub type SBI_R = crate::BitReader;
///Field `SBI` writer - Enter Standby mode when idle
pub type SBI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDI` reader - Enter power down mode when idle
pub type PDI_R = crate::BitReader;
///Field `PDI` writer - Enter power down mode when idle
pub type PDI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JSYNC` reader - Launch a injected conversion synchronously with SDADC1
pub type JSYNC_R = crate::BitReader;
///Field `JSYNC` writer - Launch a injected conversion synchronously with SDADC1
pub type JSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSYNC` reader - Launch regular conversion synchronously with SDADC1
pub type RSYNC_R = crate::BitReader;
///Field `RSYNC` writer - Launch regular conversion synchronously with SDADC1
pub type RSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JDMAEN` reader - DMA channel enabled to read data for the injected channel group
pub type JDMAEN_R = crate::BitReader;
///Field `JDMAEN` writer - DMA channel enabled to read data for the injected channel group
pub type JDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDMAEN` reader - DMA channel enabled to read data for the regular channel
pub type RDMAEN_R = crate::BitReader;
///Field `RDMAEN` writer - DMA channel enabled to read data for the regular channel
pub type RDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INIT` reader - Initialization mode request
pub type INIT_R = crate::BitReader;
///Field `INIT` writer - Initialization mode request
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - End of calibration interrupt enable
    #[inline(always)]
    pub fn eocalie(&self) -> EOCALIE_R {
        EOCALIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Injected end of conversion interrupt enable
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Injected data overrun interrupt enable
    #[inline(always)]
    pub fn jovrie(&self) -> JOVRIE_R {
        JOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Regular end of conversion interrupt enable
    #[inline(always)]
    pub fn reocie(&self) -> REOCIE_R {
        REOCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Regular data overrun interrupt enable
    #[inline(always)]
    pub fn rovrie(&self) -> ROVRIE_R {
        ROVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:9 - Reference voltage selection
    #[inline(always)]
    pub fn refv(&self) -> REFV_R {
        REFV_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Slow clock mode enable
    #[inline(always)]
    pub fn slowck(&self) -> SLOWCK_R {
        SLOWCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Enter Standby mode when idle
    #[inline(always)]
    pub fn sbi(&self) -> SBI_R {
        SBI_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enter power down mode when idle
    #[inline(always)]
    pub fn pdi(&self) -> PDI_R {
        PDI_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Launch a injected conversion synchronously with SDADC1
    #[inline(always)]
    pub fn jsync(&self) -> JSYNC_R {
        JSYNC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Launch regular conversion synchronously with SDADC1
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - DMA channel enabled to read data for the injected channel group
    #[inline(always)]
    pub fn jdmaen(&self) -> JDMAEN_R {
        JDMAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DMA channel enabled to read data for the regular channel
    #[inline(always)]
    pub fn rdmaen(&self) -> RDMAEN_R {
        RDMAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 31 - Initialization mode request
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("eocalie", &self.eocalie())
            .field("jeocie", &self.jeocie())
            .field("jovrie", &self.jovrie())
            .field("reocie", &self.reocie())
            .field("rovrie", &self.rovrie())
            .field("refv", &self.refv())
            .field("slowck", &self.slowck())
            .field("sbi", &self.sbi())
            .field("pdi", &self.pdi())
            .field("jsync", &self.jsync())
            .field("rsync", &self.rsync())
            .field("jdmaen", &self.jdmaen())
            .field("rdmaen", &self.rdmaen())
            .field("init", &self.init())
            .finish()
    }
}
impl W {
    ///Bit 0 - End of calibration interrupt enable
    #[inline(always)]
    pub fn eocalie(&mut self) -> EOCALIE_W<'_, CR1rs> {
        EOCALIE_W::new(self, 0)
    }
    ///Bit 1 - Injected end of conversion interrupt enable
    #[inline(always)]
    pub fn jeocie(&mut self) -> JEOCIE_W<'_, CR1rs> {
        JEOCIE_W::new(self, 1)
    }
    ///Bit 2 - Injected data overrun interrupt enable
    #[inline(always)]
    pub fn jovrie(&mut self) -> JOVRIE_W<'_, CR1rs> {
        JOVRIE_W::new(self, 2)
    }
    ///Bit 3 - Regular end of conversion interrupt enable
    #[inline(always)]
    pub fn reocie(&mut self) -> REOCIE_W<'_, CR1rs> {
        REOCIE_W::new(self, 3)
    }
    ///Bit 4 - Regular data overrun interrupt enable
    #[inline(always)]
    pub fn rovrie(&mut self) -> ROVRIE_W<'_, CR1rs> {
        ROVRIE_W::new(self, 4)
    }
    ///Bits 8:9 - Reference voltage selection
    #[inline(always)]
    pub fn refv(&mut self) -> REFV_W<'_, CR1rs> {
        REFV_W::new(self, 8)
    }
    ///Bit 10 - Slow clock mode enable
    #[inline(always)]
    pub fn slowck(&mut self) -> SLOWCK_W<'_, CR1rs> {
        SLOWCK_W::new(self, 10)
    }
    ///Bit 11 - Enter Standby mode when idle
    #[inline(always)]
    pub fn sbi(&mut self) -> SBI_W<'_, CR1rs> {
        SBI_W::new(self, 11)
    }
    ///Bit 12 - Enter power down mode when idle
    #[inline(always)]
    pub fn pdi(&mut self) -> PDI_W<'_, CR1rs> {
        PDI_W::new(self, 12)
    }
    ///Bit 14 - Launch a injected conversion synchronously with SDADC1
    #[inline(always)]
    pub fn jsync(&mut self) -> JSYNC_W<'_, CR1rs> {
        JSYNC_W::new(self, 14)
    }
    ///Bit 15 - Launch regular conversion synchronously with SDADC1
    #[inline(always)]
    pub fn rsync(&mut self) -> RSYNC_W<'_, CR1rs> {
        RSYNC_W::new(self, 15)
    }
    ///Bit 16 - DMA channel enabled to read data for the injected channel group
    #[inline(always)]
    pub fn jdmaen(&mut self) -> JDMAEN_W<'_, CR1rs> {
        JDMAEN_W::new(self, 16)
    }
    ///Bit 17 - DMA channel enabled to read data for the regular channel
    #[inline(always)]
    pub fn rdmaen(&mut self) -> RDMAEN_W<'_, CR1rs> {
        RDMAEN_W::new(self, 17)
    }
    ///Bit 31 - Initialization mode request
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<'_, CR1rs> {
        INIT_W::new(self, 31)
    }
}
/**control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
