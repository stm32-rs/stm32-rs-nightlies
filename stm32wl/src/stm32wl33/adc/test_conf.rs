///Register `TEST_CONF` reader
pub type R = crate::R<TEST_CONFrs>;
///Register `TEST_CONF` writer
pub type W = crate::W<TEST_CONFrs>;
///Field `ADC_SWITCH_EN` reader - ADC_SWITCH_EN\[15:0\]: enable individually each connection of the switching matrix at the ADC input. For each bit: 0: switch X is ON 1: switch X is OFF Bit mapping (corresponding to AUXADC_INSEL_1V2\[15:0\]): Bit 0: VINM\[0\] to ADC negative input Bit 1: VINM\[1\] to ADC negative input Bit 2: VINM\[2\] to ADC negative input Bit 3: VINM\[3\] to ADC negative input Bit4: GND to ADC negative input Bit5: VBAT to ADC negative input Bit6: GND to ADC negative input Bit7: VDDA to ADC negative input Bit8: VINP\[0\] to ADC positive input Bit9: VINP\[1\] to ADC positive input Bit10: VINP\[2\] to ADC positive input Bit11: VINP\[3\] to ADC positive input Bit12: VBAT to ADC positive input Bit13: TEMP to ADC positive input Bit14: GND to ADC positive input Bit15: VDDA to ADC positive input.
pub type ADC_SWITCH_EN_R = crate::FieldReader<u16>;
///Field `ADC_SWITCH_EN` writer - ADC_SWITCH_EN\[15:0\]: enable individually each connection of the switching matrix at the ADC input. For each bit: 0: switch X is ON 1: switch X is OFF Bit mapping (corresponding to AUXADC_INSEL_1V2\[15:0\]): Bit 0: VINM\[0\] to ADC negative input Bit 1: VINM\[1\] to ADC negative input Bit 2: VINM\[2\] to ADC negative input Bit 3: VINM\[3\] to ADC negative input Bit4: GND to ADC negative input Bit5: VBAT to ADC negative input Bit6: GND to ADC negative input Bit7: VDDA to ADC negative input Bit8: VINP\[0\] to ADC positive input Bit9: VINP\[1\] to ADC positive input Bit10: VINP\[2\] to ADC positive input Bit11: VINP\[3\] to ADC positive input Bit12: VBAT to ADC positive input Bit13: TEMP to ADC positive input Bit14: GND to ADC positive input Bit15: VDDA to ADC positive input.
pub type ADC_SWITCH_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `SEL_VIN_TYPE` reader - SEL_VIN_TYPE\[1:0\]: operation mode of the selected VIN 00: ADC single negative input 01: ADC single positive input 10: ADC differential input mode 11: reserved
pub type SEL_VIN_TYPE_R = crate::FieldReader;
///Field `SEL_VIN_TYPE` writer - SEL_VIN_TYPE\[1:0\]: operation mode of the selected VIN 00: ADC single negative input 01: ADC single positive input 10: ADC differential input mode 11: reserved
pub type SEL_VIN_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ADC_RUN` reader - ADC_RUN: Start/stop ADC conversion. 0: stop the ADC conversion, 1: starts the ADC conversion.
pub type ADC_RUN_R = crate::BitReader;
///Field `ADC_RUN` writer - ADC_RUN: Start/stop ADC conversion. 0: stop the ADC conversion, 1: starts the ADC conversion.
pub type ADC_RUN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_ENABLE` reader - ADC_ENABLE: 0: disable the ADC (power OFF) 1: enable the ADC (power ON)
pub type ADC_ENABLE_R = crate::BitReader;
///Field `ADC_ENABLE` writer - ADC_ENABLE: 0: disable the ADC (power OFF) 1: enable the ADC (power ON)
pub type ADC_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - ADC_SWITCH_EN\[15:0\]: enable individually each connection of the switching matrix at the ADC input. For each bit: 0: switch X is ON 1: switch X is OFF Bit mapping (corresponding to AUXADC_INSEL_1V2\[15:0\]): Bit 0: VINM\[0\] to ADC negative input Bit 1: VINM\[1\] to ADC negative input Bit 2: VINM\[2\] to ADC negative input Bit 3: VINM\[3\] to ADC negative input Bit4: GND to ADC negative input Bit5: VBAT to ADC negative input Bit6: GND to ADC negative input Bit7: VDDA to ADC negative input Bit8: VINP\[0\] to ADC positive input Bit9: VINP\[1\] to ADC positive input Bit10: VINP\[2\] to ADC positive input Bit11: VINP\[3\] to ADC positive input Bit12: VBAT to ADC positive input Bit13: TEMP to ADC positive input Bit14: GND to ADC positive input Bit15: VDDA to ADC positive input.
    #[inline(always)]
    pub fn adc_switch_en(&self) -> ADC_SWITCH_EN_R {
        ADC_SWITCH_EN_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 18:19 - SEL_VIN_TYPE\[1:0\]: operation mode of the selected VIN 00: ADC single negative input 01: ADC single positive input 10: ADC differential input mode 11: reserved
    #[inline(always)]
    pub fn sel_vin_type(&self) -> SEL_VIN_TYPE_R {
        SEL_VIN_TYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 21 - ADC_RUN: Start/stop ADC conversion. 0: stop the ADC conversion, 1: starts the ADC conversion.
    #[inline(always)]
    pub fn adc_run(&self) -> ADC_RUN_R {
        ADC_RUN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ADC_ENABLE: 0: disable the ADC (power OFF) 1: enable the ADC (power ON)
    #[inline(always)]
    pub fn adc_enable(&self) -> ADC_ENABLE_R {
        ADC_ENABLE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST_CONF")
            .field("adc_switch_en", &self.adc_switch_en())
            .field("sel_vin_type", &self.sel_vin_type())
            .field("adc_run", &self.adc_run())
            .field("adc_enable", &self.adc_enable())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - ADC_SWITCH_EN\[15:0\]: enable individually each connection of the switching matrix at the ADC input. For each bit: 0: switch X is ON 1: switch X is OFF Bit mapping (corresponding to AUXADC_INSEL_1V2\[15:0\]): Bit 0: VINM\[0\] to ADC negative input Bit 1: VINM\[1\] to ADC negative input Bit 2: VINM\[2\] to ADC negative input Bit 3: VINM\[3\] to ADC negative input Bit4: GND to ADC negative input Bit5: VBAT to ADC negative input Bit6: GND to ADC negative input Bit7: VDDA to ADC negative input Bit8: VINP\[0\] to ADC positive input Bit9: VINP\[1\] to ADC positive input Bit10: VINP\[2\] to ADC positive input Bit11: VINP\[3\] to ADC positive input Bit12: VBAT to ADC positive input Bit13: TEMP to ADC positive input Bit14: GND to ADC positive input Bit15: VDDA to ADC positive input.
    #[inline(always)]
    pub fn adc_switch_en(&mut self) -> ADC_SWITCH_EN_W<'_, TEST_CONFrs> {
        ADC_SWITCH_EN_W::new(self, 0)
    }
    ///Bits 18:19 - SEL_VIN_TYPE\[1:0\]: operation mode of the selected VIN 00: ADC single negative input 01: ADC single positive input 10: ADC differential input mode 11: reserved
    #[inline(always)]
    pub fn sel_vin_type(&mut self) -> SEL_VIN_TYPE_W<'_, TEST_CONFrs> {
        SEL_VIN_TYPE_W::new(self, 18)
    }
    ///Bit 21 - ADC_RUN: Start/stop ADC conversion. 0: stop the ADC conversion, 1: starts the ADC conversion.
    #[inline(always)]
    pub fn adc_run(&mut self) -> ADC_RUN_W<'_, TEST_CONFrs> {
        ADC_RUN_W::new(self, 21)
    }
    ///Bit 22 - ADC_ENABLE: 0: disable the ADC (power OFF) 1: enable the ADC (power ON)
    #[inline(always)]
    pub fn adc_enable(&mut self) -> ADC_ENABLE_W<'_, TEST_CONFrs> {
        ADC_ENABLE_W::new(self, 22)
    }
}
/**TEST_CONF register

You can [`read`](crate::Reg::read) this register and get [`test_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:TEST_CONF)*/
pub struct TEST_CONFrs;
impl crate::RegisterSpec for TEST_CONFrs {
    type Ux = u32;
}
///`read()` method returns [`test_conf::R`](R) reader structure
impl crate::Readable for TEST_CONFrs {}
///`write(|w| ..)` method takes [`test_conf::W`](W) writer structure
impl crate::Writable for TEST_CONFrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TEST_CONF to value 0
impl crate::Resettable for TEST_CONFrs {}
