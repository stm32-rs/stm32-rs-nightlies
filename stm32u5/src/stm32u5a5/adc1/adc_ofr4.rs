///Register `ADC_OFR4` reader
pub type R = crate::R<ADC_OFR4rs>;
///Register `ADC_OFR4` writer
pub type W = crate::W<ADC_OFR4rs>;
///Field `OFFSET` reader - OFFSET
pub type OFFSET_R = crate::FieldReader<u32>;
///Field `OFFSET` writer - OFFSET
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
///Field `POSOFF` reader - POSOFF
pub type POSOFF_R = crate::BitReader;
///Field `POSOFF` writer - POSOFF
pub type POSOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USAT` reader - USAT
pub type USAT_R = crate::BitReader;
///Field `USAT` writer - USAT
pub type USAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSAT` reader - SSAT
pub type SSAT_R = crate::BitReader;
///Field `SSAT` writer - SSAT
pub type SSAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OFFSET_CH` reader - OFFSET_CH
pub type OFFSET_CH_R = crate::FieldReader;
///Field `OFFSET_CH` writer - OFFSET_CH
pub type OFFSET_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:23 - OFFSET
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bit 24 - POSOFF
    #[inline(always)]
    pub fn posoff(&self) -> POSOFF_R {
        POSOFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - USAT
    #[inline(always)]
    pub fn usat(&self) -> USAT_R {
        USAT_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SSAT
    #[inline(always)]
    pub fn ssat(&self) -> SSAT_R {
        SSAT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:31 - OFFSET_CH
    #[inline(always)]
    pub fn offset_ch(&self) -> OFFSET_CH_R {
        OFFSET_CH_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_OFR4")
            .field("offset_ch", &self.offset_ch())
            .field("ssat", &self.ssat())
            .field("usat", &self.usat())
            .field("posoff", &self.posoff())
            .field("offset", &self.offset())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - OFFSET
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<ADC_OFR4rs> {
        OFFSET_W::new(self, 0)
    }
    ///Bit 24 - POSOFF
    #[inline(always)]
    #[must_use]
    pub fn posoff(&mut self) -> POSOFF_W<ADC_OFR4rs> {
        POSOFF_W::new(self, 24)
    }
    ///Bit 25 - USAT
    #[inline(always)]
    #[must_use]
    pub fn usat(&mut self) -> USAT_W<ADC_OFR4rs> {
        USAT_W::new(self, 25)
    }
    ///Bit 26 - SSAT
    #[inline(always)]
    #[must_use]
    pub fn ssat(&mut self) -> SSAT_W<ADC_OFR4rs> {
        SSAT_W::new(self, 26)
    }
    ///Bits 27:31 - OFFSET_CH
    #[inline(always)]
    #[must_use]
    pub fn offset_ch(&mut self) -> OFFSET_CH_W<ADC_OFR4rs> {
        OFFSET_CH_W::new(self, 27)
    }
}
/**ADC offset register

You can [`read`](crate::Reg::read) this register and get [`adc_ofr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ofr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADC1:ADC_OFR4)*/
pub struct ADC_OFR4rs;
impl crate::RegisterSpec for ADC_OFR4rs {
    type Ux = u32;
}
///`read()` method returns [`adc_ofr4::R`](R) reader structure
impl crate::Readable for ADC_OFR4rs {}
///`write(|w| ..)` method takes [`adc_ofr4::W`](W) writer structure
impl crate::Writable for ADC_OFR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_OFR4 to value 0
impl crate::Resettable for ADC_OFR4rs {
    const RESET_VALUE: u32 = 0;
}
