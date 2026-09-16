///Register `PGA_CONF` reader
pub type R = crate::R<PGA_CONFrs>;
///Register `PGA_CONF` writer
pub type W = crate::W<PGA_CONFrs>;
///Field `PGA_GAIN` reader - from 6 to 30 dB
pub type PGA_GAIN_R = crate::FieldReader;
///Field `PGA_GAIN` writer - from 6 to 30 dB
pub type PGA_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PGA_BIAS` reader - set the microphone bias voltage
pub type PGA_BIAS_R = crate::FieldReader;
///Field `PGA_BIAS` writer - set the microphone bias voltage
pub type PGA_BIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:3 - from 6 to 30 dB
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - set the microphone bias voltage
    #[inline(always)]
    pub fn pga_bias(&self) -> PGA_BIAS_R {
        PGA_BIAS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PGA_CONF")
            .field("pga_bias", &self.pga_bias())
            .field("pga_gain", &self.pga_gain())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - from 6 to 30 dB
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<'_, PGA_CONFrs> {
        PGA_GAIN_W::new(self, 0)
    }
    ///Bits 4:6 - set the microphone bias voltage
    #[inline(always)]
    pub fn pga_bias(&mut self) -> PGA_BIAS_W<'_, PGA_CONFrs> {
        PGA_BIAS_W::new(self, 4)
    }
}
/**PGA configuration register

You can [`read`](crate::Reg::read) this register and get [`pga_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pga_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:PGA_CONF)*/
pub struct PGA_CONFrs;
impl crate::RegisterSpec for PGA_CONFrs {
    type Ux = u32;
}
///`read()` method returns [`pga_conf::R`](R) reader structure
impl crate::Readable for PGA_CONFrs {}
///`write(|w| ..)` method takes [`pga_conf::W`](W) writer structure
impl crate::Writable for PGA_CONFrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PGA_CONF to value 0
impl crate::Resettable for PGA_CONFrs {}
