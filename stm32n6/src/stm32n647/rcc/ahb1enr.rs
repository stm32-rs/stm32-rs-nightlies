///Register `AHB1ENR` reader
pub type R = crate::R<AHB1ENRrs>;
///Register `AHB1ENR` writer
pub type W = crate::W<AHB1ENRrs>;
///Field `GPDMA1EN` reader - GPDMA1 enable
pub type GPDMA1EN_R = crate::BitReader;
///Field `GPDMA1EN` writer - GPDMA1 enable
pub type GPDMA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC12EN` reader - ADC12 enable
pub type ADC12EN_R = crate::BitReader;
///Field `ADC12EN` writer - ADC12 enable
pub type ADC12EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 4 - GPDMA1 enable
    #[inline(always)]
    pub fn gpdma1en(&self) -> GPDMA1EN_R {
        GPDMA1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC12 enable
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1ENR")
            .field("gpdma1en", &self.gpdma1en())
            .field("adc12en", &self.adc12en())
            .finish()
    }
}
impl W {
    ///Bit 4 - GPDMA1 enable
    #[inline(always)]
    pub fn gpdma1en(&mut self) -> GPDMA1EN_W<'_, AHB1ENRrs> {
        GPDMA1EN_W::new(self, 4)
    }
    ///Bit 5 - ADC12 enable
    #[inline(always)]
    pub fn adc12en(&mut self) -> ADC12EN_W<'_, AHB1ENRrs> {
        ADC12EN_W::new(self, 5)
    }
}
/**RCC AHB1 enable register

You can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB1ENR)*/
pub struct AHB1ENRrs;
impl crate::RegisterSpec for AHB1ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1enr::R`](R) reader structure
impl crate::Readable for AHB1ENRrs {}
///`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure
impl crate::Writable for AHB1ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1ENR to value 0
impl crate::Resettable for AHB1ENRrs {}
