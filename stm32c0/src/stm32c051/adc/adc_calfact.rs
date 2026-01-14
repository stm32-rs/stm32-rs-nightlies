///Register `ADC_CALFACT` reader
pub type R = crate::R<ADC_CALFACTrs>;
///Register `ADC_CALFACT` writer
pub type W = crate::W<ADC_CALFACTrs>;
///Field `CALFACT` reader - Calibration factor These bits are written by hardware or by software. Once a calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it is then applied once a new conversion is launched. Just after a calibration is complete, DATA\[6:0\] contains the calibration factor. Note: Software can write these bits only when ADEN=1 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
pub type CALFACT_R = crate::FieldReader;
///Field `CALFACT` writer - Calibration factor These bits are written by hardware or by software. Once a calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it is then applied once a new conversion is launched. Just after a calibration is complete, DATA\[6:0\] contains the calibration factor. Note: Software can write these bits only when ADEN=1 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
pub type CALFACT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - Calibration factor These bits are written by hardware or by software. Once a calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it is then applied once a new conversion is launched. Just after a calibration is complete, DATA\[6:0\] contains the calibration factor. Note: Software can write these bits only when ADEN=1 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
    #[inline(always)]
    pub fn calfact(&self) -> CALFACT_R {
        CALFACT_R::new((self.bits & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CALFACT")
            .field("calfact", &self.calfact())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Calibration factor These bits are written by hardware or by software. Once a calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it is then applied once a new conversion is launched. Just after a calibration is complete, DATA\[6:0\] contains the calibration factor. Note: Software can write these bits only when ADEN=1 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
    #[inline(always)]
    pub fn calfact(&mut self) -> CALFACT_W<'_, ADC_CALFACTrs> {
        CALFACT_W::new(self, 0)
    }
}
/**ADC calibration factor

You can [`read`](crate::Reg::read) this register and get [`adc_calfact::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_calfact::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#ADC:ADC_CALFACT)*/
pub struct ADC_CALFACTrs;
impl crate::RegisterSpec for ADC_CALFACTrs {
    type Ux = u32;
}
///`read()` method returns [`adc_calfact::R`](R) reader structure
impl crate::Readable for ADC_CALFACTrs {}
///`write(|w| ..)` method takes [`adc_calfact::W`](W) writer structure
impl crate::Writable for ADC_CALFACTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADC_CALFACT to value 0
impl crate::Resettable for ADC_CALFACTrs {}
