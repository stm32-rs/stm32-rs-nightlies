///Register `CALFACT` reader
pub type R = crate::R<CALFACTrs>;
///Register `CALFACT` writer
pub type W = crate::W<CALFACTrs>;
///Field `CALFACT_S` reader - Calibration Factors In Single-Ended mode These bits are written by hardware or by software. Once a single-ended inputs calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it will then be applied once a new single-ended conversion is launched. Note: The software is allowed to write these bits only when ADEN=1, ADSTART=0 and JADSTART=0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
pub type CALFACT_S_R = crate::FieldReader<u16>;
///Field `CALFACT_S` writer - Calibration Factors In Single-Ended mode These bits are written by hardware or by software. Once a single-ended inputs calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it will then be applied once a new single-ended conversion is launched. Note: The software is allowed to write these bits only when ADEN=1, ADSTART=0 and JADSTART=0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
pub type CALFACT_S_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16, crate::Safe>;
///Field `CALFACT_D` reader - Calibration Factors in differential mode These bits are written by hardware or by software. Once a differential inputs calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it will then be applied once a new differential conversion is launched. Note: The software is allowed to write these bits only when ADEN=1, ADSTART=0 and JADSTART=0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
pub type CALFACT_D_R = crate::FieldReader<u16>;
///Field `CALFACT_D` writer - Calibration Factors in differential mode These bits are written by hardware or by software. Once a differential inputs calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it will then be applied once a new differential conversion is launched. Note: The software is allowed to write these bits only when ADEN=1, ADSTART=0 and JADSTART=0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
pub type CALFACT_D_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16, crate::Safe>;
impl R {
    ///Bits 0:10 - Calibration Factors In Single-Ended mode These bits are written by hardware or by software. Once a single-ended inputs calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it will then be applied once a new single-ended conversion is launched. Note: The software is allowed to write these bits only when ADEN=1, ADSTART=0 and JADSTART=0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
    #[inline(always)]
    pub fn calfact_s(&self) -> CALFACT_S_R {
        CALFACT_S_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Calibration Factors in differential mode These bits are written by hardware or by software. Once a differential inputs calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it will then be applied once a new differential conversion is launched. Note: The software is allowed to write these bits only when ADEN=1, ADSTART=0 and JADSTART=0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
    #[inline(always)]
    pub fn calfact_d(&self) -> CALFACT_D_R {
        CALFACT_D_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALFACT")
            .field("calfact_s", &self.calfact_s())
            .field("calfact_d", &self.calfact_d())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Calibration Factors In Single-Ended mode These bits are written by hardware or by software. Once a single-ended inputs calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it will then be applied once a new single-ended conversion is launched. Note: The software is allowed to write these bits only when ADEN=1, ADSTART=0 and JADSTART=0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
    #[inline(always)]
    pub fn calfact_s(&mut self) -> CALFACT_S_W<'_, CALFACTrs> {
        CALFACT_S_W::new(self, 0)
    }
    ///Bits 16:26 - Calibration Factors in differential mode These bits are written by hardware or by software. Once a differential inputs calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it will then be applied once a new differential conversion is launched. Note: The software is allowed to write these bits only when ADEN=1, ADSTART=0 and JADSTART=0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
    #[inline(always)]
    pub fn calfact_d(&mut self) -> CALFACT_D_W<'_, CALFACTrs> {
        CALFACT_D_W::new(self, 16)
    }
}
/**ADC calibration factors register

You can [`read`](crate::Reg::read) this register and get [`calfact::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#ADC1:CALFACT)*/
pub struct CALFACTrs;
impl crate::RegisterSpec for CALFACTrs {
    type Ux = u32;
}
///`read()` method returns [`calfact::R`](R) reader structure
impl crate::Readable for CALFACTrs {}
///`write(|w| ..)` method takes [`calfact::W`](W) writer structure
impl crate::Writable for CALFACTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CALFACT to value 0
impl crate::Resettable for CALFACTrs {}
