///Register `COMP_1` reader
pub type R = crate::R<COMP_1rs>;
///Register `COMP_1` writer
pub type W = crate::W<COMP_1rs>;
///Field `GAIN1` reader - GAIN1\[11:0\]: first calibration point: gain AUXADC_GAIN_1V2\[11:0\]
pub type GAIN1_R = crate::FieldReader<u16>;
///Field `GAIN1` writer - GAIN1\[11:0\]: first calibration point: gain AUXADC_GAIN_1V2\[11:0\]
pub type GAIN1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `OFFSET1` reader - OFFSET1\[7:0\]: first calibration point: offset compensation\[7:0\] with sign
pub type OFFSET1_R = crate::FieldReader;
///Field `OFFSET1` writer - OFFSET1\[7:0\]: first calibration point: offset compensation\[7:0\] with sign
pub type OFFSET1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:11 - GAIN1\[11:0\]: first calibration point: gain AUXADC_GAIN_1V2\[11:0\]
    #[inline(always)]
    pub fn gain1(&self) -> GAIN1_R {
        GAIN1_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:19 - OFFSET1\[7:0\]: first calibration point: offset compensation\[7:0\] with sign
    #[inline(always)]
    pub fn offset1(&self) -> OFFSET1_R {
        OFFSET1_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP_1")
            .field("gain1", &self.gain1())
            .field("offset1", &self.offset1())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - GAIN1\[11:0\]: first calibration point: gain AUXADC_GAIN_1V2\[11:0\]
    #[inline(always)]
    pub fn gain1(&mut self) -> GAIN1_W<'_, COMP_1rs> {
        GAIN1_W::new(self, 0)
    }
    ///Bits 12:19 - OFFSET1\[7:0\]: first calibration point: offset compensation\[7:0\] with sign
    #[inline(always)]
    pub fn offset1(&mut self) -> OFFSET1_W<'_, COMP_1rs> {
        OFFSET1_W::new(self, 12)
    }
}
/**COMP_1 register

You can [`read`](crate::Reg::read) this register and get [`comp_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#ADC:COMP_1)*/
pub struct COMP_1rs;
impl crate::RegisterSpec for COMP_1rs {
    type Ux = u32;
}
///`read()` method returns [`comp_1::R`](R) reader structure
impl crate::Readable for COMP_1rs {}
///`write(|w| ..)` method takes [`comp_1::W`](W) writer structure
impl crate::Writable for COMP_1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP_1 to value 0x0555
impl crate::Resettable for COMP_1rs {
    const RESET_VALUE: u32 = 0x0555;
}
