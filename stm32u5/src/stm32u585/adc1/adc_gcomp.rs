///Register `ADC_GCOMP` reader
pub type R = crate::R<ADC_GCOMPrs>;
///Register `ADC_GCOMP` writer
pub type W = crate::W<ADC_GCOMPrs>;
///Field `GCOMPCOEFF` reader - Gain compensation coefficient These bits are set and cleared by software to program the gain compensation coefficient. ... ... The coefficient is divided by 4096 to get the gain factor ranging from 0 to 3.999756. Note: This gain compensation is only applied when GCOMP bit of ADCx_CFGR2 register is 1.
pub type GCOMPCOEFF_R = crate::FieldReader<u16>;
///Field `GCOMPCOEFF` writer - Gain compensation coefficient These bits are set and cleared by software to program the gain compensation coefficient. ... ... The coefficient is divided by 4096 to get the gain factor ranging from 0 to 3.999756. Note: This gain compensation is only applied when GCOMP bit of ADCx_CFGR2 register is 1.
pub type GCOMPCOEFF_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `GCOMP` reader - Gain compensation mode This bit is set and cleared by software to enable the gain compensation mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type GCOMP_R = crate::BitReader;
///Field `GCOMP` writer - Gain compensation mode This bit is set and cleared by software to enable the gain compensation mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type GCOMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:13 - Gain compensation coefficient These bits are set and cleared by software to program the gain compensation coefficient. ... ... The coefficient is divided by 4096 to get the gain factor ranging from 0 to 3.999756. Note: This gain compensation is only applied when GCOMP bit of ADCx_CFGR2 register is 1.
    #[inline(always)]
    pub fn gcompcoeff(&self) -> GCOMPCOEFF_R {
        GCOMPCOEFF_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bit 31 - Gain compensation mode This bit is set and cleared by software to enable the gain compensation mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn gcomp(&self) -> GCOMP_R {
        GCOMP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_GCOMP")
            .field("gcompcoeff", &self.gcompcoeff())
            .field("gcomp", &self.gcomp())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - Gain compensation coefficient These bits are set and cleared by software to program the gain compensation coefficient. ... ... The coefficient is divided by 4096 to get the gain factor ranging from 0 to 3.999756. Note: This gain compensation is only applied when GCOMP bit of ADCx_CFGR2 register is 1.
    #[inline(always)]
    #[must_use]
    pub fn gcompcoeff(&mut self) -> GCOMPCOEFF_W<ADC_GCOMPrs> {
        GCOMPCOEFF_W::new(self, 0)
    }
    ///Bit 31 - Gain compensation mode This bit is set and cleared by software to enable the gain compensation mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn gcomp(&mut self) -> GCOMP_W<ADC_GCOMPrs> {
        GCOMP_W::new(self, 31)
    }
}
/**ADC gain compensation register

You can [`read`](crate::Reg::read) this register and get [`adc_gcomp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_gcomp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#ADC1:ADC_GCOMP)*/
pub struct ADC_GCOMPrs;
impl crate::RegisterSpec for ADC_GCOMPrs {
    type Ux = u32;
}
///`read()` method returns [`adc_gcomp::R`](R) reader structure
impl crate::Readable for ADC_GCOMPrs {}
///`write(|w| ..)` method takes [`adc_gcomp::W`](W) writer structure
impl crate::Writable for ADC_GCOMPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_GCOMP to value 0
impl crate::Resettable for ADC_GCOMPrs {
    const RESET_VALUE: u32 = 0;
}
