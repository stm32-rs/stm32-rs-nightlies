///Register `TIMER_CONF` reader
pub type R = crate::R<TIMER_CONFrs>;
///Register `TIMER_CONF` writer
pub type W = crate::W<TIMER_CONFrs>;
///Field `ADC_LDO_DELAY` reader - define the duration of a waiting time to be inserted between the ADC_LDO enable and the ADC ON to let time to the LDO to stabilize before starting a conversion.
pub type ADC_LDO_DELAY_R = crate::FieldReader;
///Field `ADC_LDO_DELAY` writer - define the duration of a waiting time to be inserted between the ADC_LDO enable and the ADC ON to let time to the LDO to stabilize before starting a conversion.
pub type ADC_LDO_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `VBIAS_PRECH_DELAY` reader - define the duration of a waiting time starting at rising edge of PGA_EN signal and corresponding to the VBIAS precharge duration
pub type VBIAS_PRECH_DELAY_R = crate::FieldReader;
///Field `VBIAS_PRECH_DELAY` writer - define the duration of a waiting time starting at rising edge of PGA_EN signal and corresponding to the VBIAS precharge duration
pub type VBIAS_PRECH_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PRECH_DELAY_SEL` reader - Select the time step PD_STEP for the VBIAS_PRECH_DELAY timer
pub type PRECH_DELAY_SEL_R = crate::BitReader;
///Field `PRECH_DELAY_SEL` writer - Select the time step PD_STEP for the VBIAS_PRECH_DELAY timer
pub type PRECH_DELAY_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - define the duration of a waiting time to be inserted between the ADC_LDO enable and the ADC ON to let time to the LDO to stabilize before starting a conversion.
    #[inline(always)]
    pub fn adc_ldo_delay(&self) -> ADC_LDO_DELAY_R {
        ADC_LDO_DELAY_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - define the duration of a waiting time starting at rising edge of PGA_EN signal and corresponding to the VBIAS precharge duration
    #[inline(always)]
    pub fn vbias_prech_delay(&self) -> VBIAS_PRECH_DELAY_R {
        VBIAS_PRECH_DELAY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - Select the time step PD_STEP for the VBIAS_PRECH_DELAY timer
    #[inline(always)]
    pub fn prech_delay_sel(&self) -> PRECH_DELAY_SEL_R {
        PRECH_DELAY_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_CONF")
            .field("prech_delay_sel", &self.prech_delay_sel())
            .field("vbias_prech_delay", &self.vbias_prech_delay())
            .field("adc_ldo_delay", &self.adc_ldo_delay())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - define the duration of a waiting time to be inserted between the ADC_LDO enable and the ADC ON to let time to the LDO to stabilize before starting a conversion.
    #[inline(always)]
    pub fn adc_ldo_delay(&mut self) -> ADC_LDO_DELAY_W<'_, TIMER_CONFrs> {
        ADC_LDO_DELAY_W::new(self, 0)
    }
    ///Bits 8:15 - define the duration of a waiting time starting at rising edge of PGA_EN signal and corresponding to the VBIAS precharge duration
    #[inline(always)]
    pub fn vbias_prech_delay(&mut self) -> VBIAS_PRECH_DELAY_W<'_, TIMER_CONFrs> {
        VBIAS_PRECH_DELAY_W::new(self, 8)
    }
    ///Bit 16 - Select the time step PD_STEP for the VBIAS_PRECH_DELAY timer
    #[inline(always)]
    pub fn prech_delay_sel(&mut self) -> PRECH_DELAY_SEL_W<'_, TIMER_CONFrs> {
        PRECH_DELAY_SEL_W::new(self, 16)
    }
}
/**Time to add after an LDO Enable or ADC Enable to let the HW to be stable before using it

You can [`read`](crate::Reg::read) this register and get [`timer_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#ADC:TIMER_CONF)*/
pub struct TIMER_CONFrs;
impl crate::RegisterSpec for TIMER_CONFrs {
    type Ux = u32;
}
///`read()` method returns [`timer_conf::R`](R) reader structure
impl crate::Readable for TIMER_CONFrs {}
///`write(|w| ..)` method takes [`timer_conf::W`](W) writer structure
impl crate::Writable for TIMER_CONFrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIMER_CONF to value 0x9628
impl crate::Resettable for TIMER_CONFrs {
    const RESET_VALUE: u32 = 0x9628;
}
