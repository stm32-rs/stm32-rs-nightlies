///Register `CALFACT2` reader
pub type R = crate::R<CALFACT2rs>;
///Register `CALFACT2` writer
pub type W = crate::W<CALFACT2rs>;
///Field `LINCALFACT` reader - Linearity Calibration Factor These bits are written by hardware or by software. They hold 30-bit out of the 160-bit linearity calibration factor. Once a single-ended inputs calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it will then be applied once a new single-ended calibration is launched. Note: The software is allowed to write these bits only when ADEN=1, ADSTART=0 and JADSTART=0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
pub type LINCALFACT_R = crate::FieldReader<u32>;
///Field `LINCALFACT` writer - Linearity Calibration Factor These bits are written by hardware or by software. They hold 30-bit out of the 160-bit linearity calibration factor. Once a single-ended inputs calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it will then be applied once a new single-ended calibration is launched. Note: The software is allowed to write these bits only when ADEN=1, ADSTART=0 and JADSTART=0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
pub type LINCALFACT_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32, crate::Safe>;
impl R {
    ///Bits 0:29 - Linearity Calibration Factor These bits are written by hardware or by software. They hold 30-bit out of the 160-bit linearity calibration factor. Once a single-ended inputs calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it will then be applied once a new single-ended calibration is launched. Note: The software is allowed to write these bits only when ADEN=1, ADSTART=0 and JADSTART=0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
    #[inline(always)]
    pub fn lincalfact(&self) -> LINCALFACT_R {
        LINCALFACT_R::new(self.bits & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALFACT2")
            .field("lincalfact", &self.lincalfact())
            .finish()
    }
}
impl W {
    ///Bits 0:29 - Linearity Calibration Factor These bits are written by hardware or by software. They hold 30-bit out of the 160-bit linearity calibration factor. Once a single-ended inputs calibration is complete, they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it will then be applied once a new single-ended calibration is launched. Note: The software is allowed to write these bits only when ADEN=1, ADSTART=0 and JADSTART=0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).
    #[inline(always)]
    pub fn lincalfact(&mut self) -> LINCALFACT_W<'_, CALFACT2rs> {
        LINCALFACT_W::new(self, 0)
    }
}
/**ADC calibration factor register 2

You can [`read`](crate::Reg::read) this register and get [`calfact2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#ADC1:CALFACT2)*/
pub struct CALFACT2rs;
impl crate::RegisterSpec for CALFACT2rs {
    type Ux = u32;
}
///`read()` method returns [`calfact2::R`](R) reader structure
impl crate::Readable for CALFACT2rs {}
///`write(|w| ..)` method takes [`calfact2::W`](W) writer structure
impl crate::Writable for CALFACT2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CALFACT2 to value 0
impl crate::Resettable for CALFACT2rs {}
