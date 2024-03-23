#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `ADON` reader - A/D Converter ON / OFF"]
pub type ADON_R = crate::BitReader;
#[doc = "Field `ADON` writer - A/D Converter ON / OFF"]
pub type ADON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT` reader - Continuous conversion"]
pub type CONT_R = crate::BitReader;
#[doc = "Field `CONT` writer - Continuous conversion"]
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_CFG` reader - ADC configuration"]
pub type ADC_CFG_R = crate::BitReader;
#[doc = "Field `ADC_CFG` writer - ADC configuration"]
pub type ADC_CFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELS` reader - Delay selection"]
pub type DELS_R = crate::FieldReader;
#[doc = "Field `DELS` writer - Delay selection"]
pub type DELS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DMA` reader - Direct memory access mode"]
pub type DMA_R = crate::BitReader;
#[doc = "Field `DMA` writer - Direct memory access mode"]
pub type DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDS` reader - DMA disable selection"]
pub type DDS_R = crate::BitReader;
#[doc = "Field `DDS` writer - DMA disable selection"]
pub type DDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCS` reader - End of conversion selection"]
pub type EOCS_R = crate::BitReader;
#[doc = "Field `EOCS` writer - End of conversion selection"]
pub type EOCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALIGN` reader - Data alignment"]
pub type ALIGN_R = crate::BitReader;
#[doc = "Field `ALIGN` writer - Data alignment"]
pub type ALIGN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEXTSEL` reader - External event select for injected group"]
pub type JEXTSEL_R = crate::FieldReader;
#[doc = "Field `JEXTSEL` writer - External event select for injected group"]
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `JEXTEN` reader - External trigger enable for injected channels"]
pub type JEXTEN_R = crate::FieldReader;
#[doc = "Field `JEXTEN` writer - External trigger enable for injected channels"]
pub type JEXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JSWSTART` reader - Start conversion of injected channels"]
pub type JSWSTART_R = crate::BitReader;
#[doc = "Field `JSWSTART` writer - Start conversion of injected channels"]
pub type JSWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTSEL` reader - External event select for regular group"]
pub type EXTSEL_R = crate::FieldReader;
#[doc = "Field `EXTSEL` writer - External event select for regular group"]
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTEN` reader - External trigger enable for regular channels"]
pub type EXTEN_R = crate::FieldReader;
#[doc = "Field `EXTEN` writer - External trigger enable for regular channels"]
pub type EXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWSTART` reader - Start conversion of regular channels"]
pub type SWSTART_R = crate::BitReader;
#[doc = "Field `SWSTART` writer - Start conversion of regular channels"]
pub type SWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - A/D Converter ON / OFF"]
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC configuration"]
    #[inline(always)]
    pub fn adc_cfg(&self) -> ADC_CFG_R {
        ADC_CFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Delay selection"]
    #[inline(always)]
    pub fn dels(&self) -> DELS_R {
        DELS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Direct memory access mode"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA disable selection"]
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - End of conversion selection"]
    #[inline(always)]
    pub fn eocs(&self) -> EOCS_R {
        EOCS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:19 - External event select for injected group"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - External trigger enable for injected channels"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Start conversion of injected channels"]
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - External event select for regular group"]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - External trigger enable for regular channels"]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Start conversion of regular channels"]
    #[inline(always)]
    pub fn swstart(&self) -> SWSTART_R {
        SWSTART_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D Converter ON / OFF"]
    #[inline(always)]
    #[must_use]
    pub fn adon(&mut self) -> ADON_W<CR2rs> {
        ADON_W::new(self, 0)
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<CR2rs> {
        CONT_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC configuration"]
    #[inline(always)]
    #[must_use]
    pub fn adc_cfg(&mut self) -> ADC_CFG_W<CR2rs> {
        ADC_CFG_W::new(self, 2)
    }
    #[doc = "Bits 4:6 - Delay selection"]
    #[inline(always)]
    #[must_use]
    pub fn dels(&mut self) -> DELS_W<CR2rs> {
        DELS_W::new(self, 4)
    }
    #[doc = "Bit 8 - Direct memory access mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<CR2rs> {
        DMA_W::new(self, 8)
    }
    #[doc = "Bit 9 - DMA disable selection"]
    #[inline(always)]
    #[must_use]
    pub fn dds(&mut self) -> DDS_W<CR2rs> {
        DDS_W::new(self, 9)
    }
    #[doc = "Bit 10 - End of conversion selection"]
    #[inline(always)]
    #[must_use]
    pub fn eocs(&mut self) -> EOCS_W<CR2rs> {
        EOCS_W::new(self, 10)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<CR2rs> {
        ALIGN_W::new(self, 11)
    }
    #[doc = "Bits 16:19 - External event select for injected group"]
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<CR2rs> {
        JEXTSEL_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - External trigger enable for injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jexten(&mut self) -> JEXTEN_W<CR2rs> {
        JEXTEN_W::new(self, 20)
    }
    #[doc = "Bit 22 - Start conversion of injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jswstart(&mut self) -> JSWSTART_W<CR2rs> {
        JSWSTART_W::new(self, 22)
    }
    #[doc = "Bits 24:27 - External event select for regular group"]
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<CR2rs> {
        EXTSEL_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - External trigger enable for regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> EXTEN_W<CR2rs> {
        EXTEN_W::new(self, 28)
    }
    #[doc = "Bit 30 - Start conversion of regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn swstart(&mut self) -> SWSTART_W<CR2rs> {
        SWSTART_W::new(self, 30)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
