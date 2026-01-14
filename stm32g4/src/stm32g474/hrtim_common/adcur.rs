///Register `ADCUR` reader
pub type R = crate::R<ADCURrs>;
///Register `ADCUR` writer
pub type W = crate::W<ADCURrs>;
///Field `AD5USRC` reader - AD5USRC
pub type AD5USRC_R = crate::FieldReader;
///Field `AD5USRC` writer - AD5USRC
pub type AD5USRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AD6USRC` reader - AD6USRC
pub type AD6USRC_R = crate::FieldReader;
///Field `AD6USRC` writer - AD6USRC
pub type AD6USRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AD7USRC` reader - AD7USRC
pub type AD7USRC_R = crate::FieldReader;
///Field `AD7USRC` writer - AD7USRC
pub type AD7USRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AD8USRC` reader - AD8USRC
pub type AD8USRC_R = crate::FieldReader;
///Field `AD8USRC` writer - AD8USRC
pub type AD8USRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AD9USRC` reader - AD9USRC
pub type AD9USRC_R = crate::FieldReader;
///Field `AD9USRC` writer - AD9USRC
pub type AD9USRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AD10USRC` reader - AD10USRC
pub type AD10USRC_R = crate::FieldReader;
///Field `AD10USRC` writer - AD10USRC
pub type AD10USRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - AD5USRC
    #[inline(always)]
    pub fn ad5usrc(&self) -> AD5USRC_R {
        AD5USRC_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - AD6USRC
    #[inline(always)]
    pub fn ad6usrc(&self) -> AD6USRC_R {
        AD6USRC_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - AD7USRC
    #[inline(always)]
    pub fn ad7usrc(&self) -> AD7USRC_R {
        AD7USRC_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - AD8USRC
    #[inline(always)]
    pub fn ad8usrc(&self) -> AD8USRC_R {
        AD8USRC_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:18 - AD9USRC
    #[inline(always)]
    pub fn ad9usrc(&self) -> AD9USRC_R {
        AD9USRC_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - AD10USRC
    #[inline(always)]
    pub fn ad10usrc(&self) -> AD10USRC_R {
        AD10USRC_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCUR")
            .field("ad10usrc", &self.ad10usrc())
            .field("ad9usrc", &self.ad9usrc())
            .field("ad8usrc", &self.ad8usrc())
            .field("ad7usrc", &self.ad7usrc())
            .field("ad6usrc", &self.ad6usrc())
            .field("ad5usrc", &self.ad5usrc())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - AD5USRC
    #[inline(always)]
    pub fn ad5usrc(&mut self) -> AD5USRC_W<'_, ADCURrs> {
        AD5USRC_W::new(self, 0)
    }
    ///Bits 4:6 - AD6USRC
    #[inline(always)]
    pub fn ad6usrc(&mut self) -> AD6USRC_W<'_, ADCURrs> {
        AD6USRC_W::new(self, 4)
    }
    ///Bits 8:10 - AD7USRC
    #[inline(always)]
    pub fn ad7usrc(&mut self) -> AD7USRC_W<'_, ADCURrs> {
        AD7USRC_W::new(self, 8)
    }
    ///Bits 12:14 - AD8USRC
    #[inline(always)]
    pub fn ad8usrc(&mut self) -> AD8USRC_W<'_, ADCURrs> {
        AD8USRC_W::new(self, 12)
    }
    ///Bits 16:18 - AD9USRC
    #[inline(always)]
    pub fn ad9usrc(&mut self) -> AD9USRC_W<'_, ADCURrs> {
        AD9USRC_W::new(self, 16)
    }
    ///Bits 20:22 - AD10USRC
    #[inline(always)]
    pub fn ad10usrc(&mut self) -> AD10USRC_W<'_, ADCURrs> {
        AD10USRC_W::new(self, 20)
    }
}
/**HRTIM ADC Trigger Update Register

You can [`read`](crate::Reg::read) this register and get [`adcur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:ADCUR)*/
pub struct ADCURrs;
impl crate::RegisterSpec for ADCURrs {
    type Ux = u32;
}
///`read()` method returns [`adcur::R`](R) reader structure
impl crate::Readable for ADCURrs {}
///`write(|w| ..)` method takes [`adcur::W`](W) writer structure
impl crate::Writable for ADCURrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCUR to value 0
impl crate::Resettable for ADCURrs {}
