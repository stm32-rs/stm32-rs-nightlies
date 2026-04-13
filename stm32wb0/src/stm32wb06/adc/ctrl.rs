///Register `CTRL` reader
pub type R = crate::R<CTRLrs>;
///Register `CTRL` writer
pub type W = crate::W<CTRLrs>;
///Field `ADC_ON_OFF` reader - ADC_ON_OFF: – 0: power off the ADC – 1: power on the ADC
pub type ADC_ON_OFF_R = crate::BitReader;
///Field `ADC_ON_OFF` writer - ADC_ON_OFF: – 0: power off the ADC – 1: power on the ADC
pub type ADC_ON_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START_CON` reader - generate a start pulse to initiate an ADC conversion
pub type START_CON_R = crate::BitReader;
///Field `START_CON` writer - generate a start pulse to initiate an ADC conversion
pub type START_CON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP_OP_MOD` reader - stop the on-going OP_MODE (ADC mode, Analog audio mode, Full
pub type STOP_OP_MOD_R = crate::BitReader;
///Field `STOP_OP_MOD` writer - stop the on-going OP_MODE (ADC mode, Analog audio mode, Full
pub type STOP_OP_MOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIG_AUD_MODE` reader - enable the digital audio mode (the data path uses the decimation filter)
pub type DIG_AUD_MODE_R = crate::BitReader;
///Field `DIG_AUD_MODE` writer - enable the digital audio mode (the data path uses the decimation filter)
pub type DIG_AUD_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEST_MODE` reader - select the functional or the test mode of the ADC
pub type TEST_MODE_R = crate::BitReader;
///Field `TEST_MODE` writer - select the functional or the test mode of the ADC
pub type TEST_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_LDO_ENA` reader - enable the LDO associated to the ADC block
pub type ADC_LDO_ENA_R = crate::BitReader;
///Field `ADC_LDO_ENA` writer - enable the LDO associated to the ADC block
pub type ADC_LDO_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ADC_ON_OFF: – 0: power off the ADC – 1: power on the ADC
    #[inline(always)]
    pub fn adc_on_off(&self) -> ADC_ON_OFF_R {
        ADC_ON_OFF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - generate a start pulse to initiate an ADC conversion
    #[inline(always)]
    pub fn start_con(&self) -> START_CON_R {
        START_CON_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - stop the on-going OP_MODE (ADC mode, Analog audio mode, Full
    #[inline(always)]
    pub fn stop_op_mod(&self) -> STOP_OP_MOD_R {
        STOP_OP_MOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - enable the digital audio mode (the data path uses the decimation filter)
    #[inline(always)]
    pub fn dig_aud_mode(&self) -> DIG_AUD_MODE_R {
        DIG_AUD_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - select the functional or the test mode of the ADC
    #[inline(always)]
    pub fn test_mode(&self) -> TEST_MODE_R {
        TEST_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - enable the LDO associated to the ADC block
    #[inline(always)]
    pub fn adc_ldo_ena(&self) -> ADC_LDO_ENA_R {
        ADC_LDO_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("adc_ldo_ena", &self.adc_ldo_ena())
            .field("test_mode", &self.test_mode())
            .field("dig_aud_mode", &self.dig_aud_mode())
            .field("stop_op_mod", &self.stop_op_mod())
            .field("start_con", &self.start_con())
            .field("adc_on_off", &self.adc_on_off())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC_ON_OFF: – 0: power off the ADC – 1: power on the ADC
    #[inline(always)]
    pub fn adc_on_off(&mut self) -> ADC_ON_OFF_W<'_, CTRLrs> {
        ADC_ON_OFF_W::new(self, 0)
    }
    ///Bit 1 - generate a start pulse to initiate an ADC conversion
    #[inline(always)]
    pub fn start_con(&mut self) -> START_CON_W<'_, CTRLrs> {
        START_CON_W::new(self, 1)
    }
    ///Bit 2 - stop the on-going OP_MODE (ADC mode, Analog audio mode, Full
    #[inline(always)]
    pub fn stop_op_mod(&mut self) -> STOP_OP_MOD_W<'_, CTRLrs> {
        STOP_OP_MOD_W::new(self, 2)
    }
    ///Bit 3 - enable the digital audio mode (the data path uses the decimation filter)
    #[inline(always)]
    pub fn dig_aud_mode(&mut self) -> DIG_AUD_MODE_W<'_, CTRLrs> {
        DIG_AUD_MODE_W::new(self, 3)
    }
    ///Bit 4 - select the functional or the test mode of the ADC
    #[inline(always)]
    pub fn test_mode(&mut self) -> TEST_MODE_W<'_, CTRLrs> {
        TEST_MODE_W::new(self, 4)
    }
    ///Bit 5 - enable the LDO associated to the ADC block
    #[inline(always)]
    pub fn adc_ldo_ena(&mut self) -> ADC_LDO_ENA_W<'_, CTRLrs> {
        ADC_LDO_ENA_W::new(self, 5)
    }
}
/**ADC control register

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#ADC:CTRL)*/
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
