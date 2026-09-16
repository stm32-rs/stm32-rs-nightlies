///Register `AHB1RSTR` reader
pub type R = crate::R<AHB1RSTRrs>;
///Register `AHB1RSTR` writer
pub type W = crate::W<AHB1RSTRrs>;
///Field `GPDMA1RST` reader - GPDMA1 reset
pub type GPDMA1RST_R = crate::BitReader;
///Field `GPDMA1RST` writer - GPDMA1 reset
pub type GPDMA1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC12RST` reader - ADC12 reset
pub type ADC12RST_R = crate::BitReader;
///Field `ADC12RST` writer - ADC12 reset
pub type ADC12RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 4 - GPDMA1 reset
    #[inline(always)]
    pub fn gpdma1rst(&self) -> GPDMA1RST_R {
        GPDMA1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC12 reset
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1RSTR")
            .field("gpdma1rst", &self.gpdma1rst())
            .field("adc12rst", &self.adc12rst())
            .finish()
    }
}
impl W {
    ///Bit 4 - GPDMA1 reset
    #[inline(always)]
    pub fn gpdma1rst(&mut self) -> GPDMA1RST_W<'_, AHB1RSTRrs> {
        GPDMA1RST_W::new(self, 4)
    }
    ///Bit 5 - ADC12 reset
    #[inline(always)]
    pub fn adc12rst(&mut self) -> ADC12RST_W<'_, AHB1RSTRrs> {
        ADC12RST_W::new(self, 5)
    }
}
/**RCC AHB1 Reset register

You can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:AHB1RSTR)*/
pub struct AHB1RSTRrs;
impl crate::RegisterSpec for AHB1RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1rstr::R`](R) reader structure
impl crate::Readable for AHB1RSTRrs {}
///`write(|w| ..)` method takes [`ahb1rstr::W`](W) writer structure
impl crate::Writable for AHB1RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1RSTR to value 0
impl crate::Resettable for AHB1RSTRrs {}
