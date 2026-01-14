///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `ADON` reader - A/D Converter ON / OFF
pub type ADON_R = crate::BitReader;
///Field `ADON` writer - A/D Converter ON / OFF
pub type ADON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CONT` reader - Continuous conversion
pub type CONT_R = crate::BitReader;
///Field `CONT` writer - Continuous conversion
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_CFG` reader - ADC configuration
pub type ADC_CFG_R = crate::BitReader;
///Field `ADC_CFG` writer - ADC configuration
pub type ADC_CFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DELS` reader - Delay selection
pub type DELS_R = crate::FieldReader;
///Field `DELS` writer - Delay selection
pub type DELS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DMA` reader - Direct memory access mode
pub type DMA_R = crate::BitReader;
///Field `DMA` writer - Direct memory access mode
pub type DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDS` reader - DMA disable selection
pub type DDS_R = crate::BitReader;
///Field `DDS` writer - DMA disable selection
pub type DDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOCS` reader - End of conversion selection
pub type EOCS_R = crate::BitReader;
///Field `EOCS` writer - End of conversion selection
pub type EOCS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALIGN` reader - Data alignment
pub type ALIGN_R = crate::BitReader;
///Field `ALIGN` writer - Data alignment
pub type ALIGN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JEXTSEL` reader - External event select for injected group
pub type JEXTSEL_R = crate::FieldReader;
///Field `JEXTSEL` writer - External event select for injected group
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `JEXTEN` reader - External trigger enable for injected channels
pub type JEXTEN_R = crate::FieldReader;
///Field `JEXTEN` writer - External trigger enable for injected channels
pub type JEXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `JSWSTART` reader - Start conversion of injected channels
pub type JSWSTART_R = crate::BitReader;
///Field `JSWSTART` writer - Start conversion of injected channels
pub type JSWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTSEL` reader - External event select for regular group
pub type EXTSEL_R = crate::FieldReader;
///Field `EXTSEL` writer - External event select for regular group
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EXTEN` reader - External trigger enable for regular channels
pub type EXTEN_R = crate::FieldReader;
///Field `EXTEN` writer - External trigger enable for regular channels
pub type EXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SWSTART` reader - Start conversion of regular channels
pub type SWSTART_R = crate::BitReader;
///Field `SWSTART` writer - Start conversion of regular channels
pub type SWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - A/D Converter ON / OFF
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Continuous conversion
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADC configuration
    #[inline(always)]
    pub fn adc_cfg(&self) -> ADC_CFG_R {
        ADC_CFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:6 - Delay selection
    #[inline(always)]
    pub fn dels(&self) -> DELS_R {
        DELS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - Direct memory access mode
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DMA disable selection
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - End of conversion selection
    #[inline(always)]
    pub fn eocs(&self) -> EOCS_R {
        EOCS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Data alignment
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 16:19 - External event select for injected group
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:21 - External trigger enable for injected channels
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Start conversion of injected channels
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 24:27 - External event select for regular group
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:29 - External trigger enable for regular channels
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - Start conversion of regular channels
    #[inline(always)]
    pub fn swstart(&self) -> SWSTART_R {
        SWSTART_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("swstart", &self.swstart())
            .field("exten", &self.exten())
            .field("extsel", &self.extsel())
            .field("jswstart", &self.jswstart())
            .field("jexten", &self.jexten())
            .field("jextsel", &self.jextsel())
            .field("align", &self.align())
            .field("eocs", &self.eocs())
            .field("dds", &self.dds())
            .field("dma", &self.dma())
            .field("dels", &self.dels())
            .field("adc_cfg", &self.adc_cfg())
            .field("cont", &self.cont())
            .field("adon", &self.adon())
            .finish()
    }
}
impl W {
    ///Bit 0 - A/D Converter ON / OFF
    #[inline(always)]
    pub fn adon(&mut self) -> ADON_W<'_, CR2rs> {
        ADON_W::new(self, 0)
    }
    ///Bit 1 - Continuous conversion
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<'_, CR2rs> {
        CONT_W::new(self, 1)
    }
    ///Bit 2 - ADC configuration
    #[inline(always)]
    pub fn adc_cfg(&mut self) -> ADC_CFG_W<'_, CR2rs> {
        ADC_CFG_W::new(self, 2)
    }
    ///Bits 4:6 - Delay selection
    #[inline(always)]
    pub fn dels(&mut self) -> DELS_W<'_, CR2rs> {
        DELS_W::new(self, 4)
    }
    ///Bit 8 - Direct memory access mode
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W<'_, CR2rs> {
        DMA_W::new(self, 8)
    }
    ///Bit 9 - DMA disable selection
    #[inline(always)]
    pub fn dds(&mut self) -> DDS_W<'_, CR2rs> {
        DDS_W::new(self, 9)
    }
    ///Bit 10 - End of conversion selection
    #[inline(always)]
    pub fn eocs(&mut self) -> EOCS_W<'_, CR2rs> {
        EOCS_W::new(self, 10)
    }
    ///Bit 11 - Data alignment
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W<'_, CR2rs> {
        ALIGN_W::new(self, 11)
    }
    ///Bits 16:19 - External event select for injected group
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W<'_, CR2rs> {
        JEXTSEL_W::new(self, 16)
    }
    ///Bits 20:21 - External trigger enable for injected channels
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W<'_, CR2rs> {
        JEXTEN_W::new(self, 20)
    }
    ///Bit 22 - Start conversion of injected channels
    #[inline(always)]
    pub fn jswstart(&mut self) -> JSWSTART_W<'_, CR2rs> {
        JSWSTART_W::new(self, 22)
    }
    ///Bits 24:27 - External event select for regular group
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W<'_, CR2rs> {
        EXTSEL_W::new(self, 24)
    }
    ///Bits 28:29 - External trigger enable for regular channels
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W<'_, CR2rs> {
        EXTEN_W::new(self, 28)
    }
    ///Bit 30 - Start conversion of regular channels
    #[inline(always)]
    pub fn swstart(&mut self) -> SWSTART_W<'_, CR2rs> {
        SWSTART_W::new(self, 30)
    }
}
/**control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#ADC:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
