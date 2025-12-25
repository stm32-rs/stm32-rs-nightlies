///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `MULT` reader - Multi ADC mode selection
pub type MULT_R = crate::FieldReader;
///Field `MULT` writer - Multi ADC mode selection
pub type MULT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DELAY` reader - Delay between 2 sampling phases
pub type DELAY_R = crate::FieldReader;
///Field `DELAY` writer - Delay between 2 sampling phases
pub type DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DDS` reader - DMA disable selection for multi-ADC mode
pub type DDS_R = crate::BitReader;
///Field `DDS` writer - DMA disable selection for multi-ADC mode
pub type DDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA` reader - Direct memory access mode for multi ADC mode
pub type DMA_R = crate::FieldReader;
///Field `DMA` writer - Direct memory access mode for multi ADC mode
pub type DMA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ADCPRE` reader - ADC prescaler
pub type ADCPRE_R = crate::FieldReader;
///Field `ADCPRE` writer - ADC prescaler
pub type ADCPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VBATE` reader - VBAT enable
pub type VBATE_R = crate::BitReader;
///Field `VBATE` writer - VBAT enable
pub type VBATE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSVREFE` reader - Temperature sensor and VREFINT enable
pub type TSVREFE_R = crate::BitReader;
///Field `TSVREFE` writer - Temperature sensor and VREFINT enable
pub type TSVREFE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Multi ADC mode selection
    #[inline(always)]
    pub fn mult(&self) -> MULT_R {
        MULT_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 13 - DMA disable selection for multi-ADC mode
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Direct memory access mode for multi ADC mode
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - ADC prescaler
    #[inline(always)]
    pub fn adcpre(&self) -> ADCPRE_R {
        ADCPRE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 22 - VBAT enable
    #[inline(always)]
    pub fn vbate(&self) -> VBATE_R {
        VBATE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Temperature sensor and VREFINT enable
    #[inline(always)]
    pub fn tsvrefe(&self) -> TSVREFE_R {
        TSVREFE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("tsvrefe", &self.tsvrefe())
            .field("vbate", &self.vbate())
            .field("adcpre", &self.adcpre())
            .field("dma", &self.dma())
            .field("dds", &self.dds())
            .field("delay", &self.delay())
            .field("mult", &self.mult())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Multi ADC mode selection
    #[inline(always)]
    pub fn mult(&mut self) -> MULT_W<'_, CCRrs> {
        MULT_W::new(self, 0)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W<'_, CCRrs> {
        DELAY_W::new(self, 8)
    }
    ///Bit 13 - DMA disable selection for multi-ADC mode
    #[inline(always)]
    pub fn dds(&mut self) -> DDS_W<'_, CCRrs> {
        DDS_W::new(self, 13)
    }
    ///Bits 14:15 - Direct memory access mode for multi ADC mode
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W<'_, CCRrs> {
        DMA_W::new(self, 14)
    }
    ///Bits 16:17 - ADC prescaler
    #[inline(always)]
    pub fn adcpre(&mut self) -> ADCPRE_W<'_, CCRrs> {
        ADCPRE_W::new(self, 16)
    }
    ///Bit 22 - VBAT enable
    #[inline(always)]
    pub fn vbate(&mut self) -> VBATE_W<'_, CCRrs> {
        VBATE_W::new(self, 22)
    }
    ///Bit 23 - Temperature sensor and VREFINT enable
    #[inline(always)]
    pub fn tsvrefe(&mut self) -> TSVREFE_W<'_, CCRrs> {
        TSVREFE_W::new(self, 23)
    }
}
/**ADC common control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#C_ADC:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
