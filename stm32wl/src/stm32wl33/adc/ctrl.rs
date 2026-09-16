///Register `CTRL` reader
pub type R = crate::R<CTRLrs>;
///Register `CTRL` writer
pub type W = crate::W<CTRLrs>;
///Field `ADC_ON_OFF` reader - ADC_ON_OFF: 0: power off the ADC 1: power on the ADC
pub type ADC_ON_OFF_R = crate::BitReader;
///Field `ADC_ON_OFF` writer - ADC_ON_OFF: 0: power off the ADC 1: power on the ADC
pub type ADC_ON_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START_CONV` writer - START_CONV (1): generate a start pulse to initiate an ADC conversion: 0: no effect 1: start the ADC conversion Note: this bit is set by software and cleared by hardware.
pub type START_CONV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP_OP_MODE` writer - STOP_OP_MODE (1): stop the on-going OP_MODE (ADC mode, Analog audio mode, Full mode): 0: no effect 1: stop on-going ADC mode Note: this bit is set by software and cleared by hardware. When setting the STOP_MODE_OP, the user has to wait around 10 us before to start a new ADC conversion by setting the START_CONV bit.
pub type STOP_OP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEST_MODE` reader - TEST_MODE: select the functional or the test mode of the ADC: 0: functional mode (one of the four main functional modes is used) 1: test mode (for debug, test, calibration)
pub type TEST_MODE_R = crate::BitReader;
///Field `TEST_MODE` writer - TEST_MODE: select the functional or the test mode of the ADC: 0: functional mode (one of the four main functional modes is used) 1: test mode (for debug, test, calibration)
pub type TEST_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ADC_ON_OFF: 0: power off the ADC 1: power on the ADC
    #[inline(always)]
    pub fn adc_on_off(&self) -> ADC_ON_OFF_R {
        ADC_ON_OFF_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - TEST_MODE: select the functional or the test mode of the ADC: 0: functional mode (one of the four main functional modes is used) 1: test mode (for debug, test, calibration)
    #[inline(always)]
    pub fn test_mode(&self) -> TEST_MODE_R {
        TEST_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("adc_on_off", &self.adc_on_off())
            .field("test_mode", &self.test_mode())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC_ON_OFF: 0: power off the ADC 1: power on the ADC
    #[inline(always)]
    pub fn adc_on_off(&mut self) -> ADC_ON_OFF_W<'_, CTRLrs> {
        ADC_ON_OFF_W::new(self, 0)
    }
    ///Bit 1 - START_CONV (1): generate a start pulse to initiate an ADC conversion: 0: no effect 1: start the ADC conversion Note: this bit is set by software and cleared by hardware.
    #[inline(always)]
    pub fn start_conv(&mut self) -> START_CONV_W<'_, CTRLrs> {
        START_CONV_W::new(self, 1)
    }
    ///Bit 2 - STOP_OP_MODE (1): stop the on-going OP_MODE (ADC mode, Analog audio mode, Full mode): 0: no effect 1: stop on-going ADC mode Note: this bit is set by software and cleared by hardware. When setting the STOP_MODE_OP, the user has to wait around 10 us before to start a new ADC conversion by setting the START_CONV bit.
    #[inline(always)]
    pub fn stop_op_mode(&mut self) -> STOP_OP_MODE_W<'_, CTRLrs> {
        STOP_OP_MODE_W::new(self, 2)
    }
    ///Bit 4 - TEST_MODE: select the functional or the test mode of the ADC: 0: functional mode (one of the four main functional modes is used) 1: test mode (for debug, test, calibration)
    #[inline(always)]
    pub fn test_mode(&mut self) -> TEST_MODE_W<'_, CTRLrs> {
        TEST_MODE_W::new(self, 4)
    }
}
/**CTRL register

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:CTRL)*/
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ctrl::R`](R) reader structure
impl crate::Readable for CTRLrs {}
///`write(|w| ..)` method takes [`ctrl::W`](W) writer structure
impl crate::Writable for CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTRL to value 0
impl crate::Resettable for CTRLrs {}
