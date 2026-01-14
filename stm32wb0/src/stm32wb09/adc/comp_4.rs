///Register `COMP_4` reader
pub type R = crate::R<COMP_4rs>;
///Register `COMP_4` writer
pub type W = crate::W<COMP_4rs>;
///Field `GAIN4` reader - GAIN4\[11:0\]: fourth calibration point: gain AUXADC_GAIN_1V2\[11:0\]
pub type GAIN4_R = crate::FieldReader<u16>;
///Field `GAIN4` writer - GAIN4\[11:0\]: fourth calibration point: gain AUXADC_GAIN_1V2\[11:0\]
pub type GAIN4_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `OFFSET4` reader - OFFSET4\[7:0\]: fourth calibration point: offset compensation\[7:0\] with sign
pub type OFFSET4_R = crate::FieldReader;
///Field `OFFSET4` writer - OFFSET4\[7:0\]: fourth calibration point: offset compensation\[7:0\] with sign
pub type OFFSET4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:11 - GAIN4\[11:0\]: fourth calibration point: gain AUXADC_GAIN_1V2\[11:0\]
    #[inline(always)]
    pub fn gain4(&self) -> GAIN4_R {
        GAIN4_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:19 - OFFSET4\[7:0\]: fourth calibration point: offset compensation\[7:0\] with sign
    #[inline(always)]
    pub fn offset4(&self) -> OFFSET4_R {
        OFFSET4_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP_4")
            .field("gain4", &self.gain4())
            .field("offset4", &self.offset4())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - GAIN4\[11:0\]: fourth calibration point: gain AUXADC_GAIN_1V2\[11:0\]
    #[inline(always)]
    pub fn gain4(&mut self) -> GAIN4_W<'_, COMP_4rs> {
        GAIN4_W::new(self, 0)
    }
    ///Bits 12:19 - OFFSET4\[7:0\]: fourth calibration point: offset compensation\[7:0\] with sign
    #[inline(always)]
    pub fn offset4(&mut self) -> OFFSET4_W<'_, COMP_4rs> {
        OFFSET4_W::new(self, 12)
    }
}
/**COMP_4 register

You can [`read`](crate::Reg::read) this register and get [`comp_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#ADC:COMP_4)*/
pub struct COMP_4rs;
impl crate::RegisterSpec for COMP_4rs {
    type Ux = u32;
}
///`read()` method returns [`comp_4::R`](R) reader structure
impl crate::Readable for COMP_4rs {}
///`write(|w| ..)` method takes [`comp_4::W`](W) writer structure
impl crate::Writable for COMP_4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP_4 to value 0x0555
impl crate::Resettable for COMP_4rs {
    const RESET_VALUE: u32 = 0x0555;
}
