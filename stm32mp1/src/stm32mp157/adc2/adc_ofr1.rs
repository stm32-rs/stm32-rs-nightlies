///Register `ADC_OFR1` reader
pub type R = crate::R<ADC_OFR1rs>;
///Register `ADC_OFR1` writer
pub type W = crate::W<ADC_OFR1rs>;
///Field `OFFSET1` reader - OFFSET1
pub type OFFSET1_R = crate::FieldReader<u32>;
///Field `OFFSET1` writer - OFFSET1
pub type OFFSET1_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
///Field `OFFSET1_CH` reader - OFFSET1_CH
pub type OFFSET1_CH_R = crate::FieldReader;
///Field `OFFSET1_CH` writer - OFFSET1_CH
pub type OFFSET1_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SSATE` reader - SSATE
pub type SSATE_R = crate::BitReader;
///Field `SSATE` writer - SSATE
pub type SSATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:25 - OFFSET1
    #[inline(always)]
    pub fn offset1(&self) -> OFFSET1_R {
        OFFSET1_R::new(self.bits & 0x03ff_ffff)
    }
    ///Bits 26:30 - OFFSET1_CH
    #[inline(always)]
    pub fn offset1_ch(&self) -> OFFSET1_CH_R {
        OFFSET1_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - SSATE
    #[inline(always)]
    pub fn ssate(&self) -> SSATE_R {
        SSATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_OFR1")
            .field("offset1", &self.offset1())
            .field("offset1_ch", &self.offset1_ch())
            .field("ssate", &self.ssate())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - OFFSET1
    #[inline(always)]
    #[must_use]
    pub fn offset1(&mut self) -> OFFSET1_W<ADC_OFR1rs> {
        OFFSET1_W::new(self, 0)
    }
    ///Bits 26:30 - OFFSET1_CH
    #[inline(always)]
    #[must_use]
    pub fn offset1_ch(&mut self) -> OFFSET1_CH_W<ADC_OFR1rs> {
        OFFSET1_CH_W::new(self, 26)
    }
    ///Bit 31 - SSATE
    #[inline(always)]
    #[must_use]
    pub fn ssate(&mut self) -> SSATE_W<ADC_OFR1rs> {
        SSATE_W::new(self, 31)
    }
}
/**ADC offset register

You can [`read`](crate::Reg::read) this register and get [`adc_ofr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ofr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ADC2:ADC_OFR1)*/
pub struct ADC_OFR1rs;
impl crate::RegisterSpec for ADC_OFR1rs {
    type Ux = u32;
}
///`read()` method returns [`adc_ofr1::R`](R) reader structure
impl crate::Readable for ADC_OFR1rs {}
///`write(|w| ..)` method takes [`adc_ofr1::W`](W) writer structure
impl crate::Writable for ADC_OFR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_OFR1 to value 0
impl crate::Resettable for ADC_OFR1rs {
    const RESET_VALUE: u32 = 0;
}
