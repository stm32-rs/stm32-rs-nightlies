///Register `COMP_2` reader
pub type R = crate::R<COMP_2rs>;
///Register `COMP_2` writer
pub type W = crate::W<COMP_2rs>;
///Field `GAIN2` reader - GAIN2\[11:0\]: second calibration point: gain AUXADC_GAIN_1V2\[11:0\]
pub type GAIN2_R = crate::FieldReader<u16>;
///Field `GAIN2` writer - GAIN2\[11:0\]: second calibration point: gain AUXADC_GAIN_1V2\[11:0\]
pub type GAIN2_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `OFFSET2` reader - OFFSET2\[7:0\]: second calibration point: offset compensation\[7:0\] with sign
pub type OFFSET2_R = crate::FieldReader;
///Field `OFFSET2` writer - OFFSET2\[7:0\]: second calibration point: offset compensation\[7:0\] with sign
pub type OFFSET2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:11 - GAIN2\[11:0\]: second calibration point: gain AUXADC_GAIN_1V2\[11:0\]
    #[inline(always)]
    pub fn gain2(&self) -> GAIN2_R {
        GAIN2_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:19 - OFFSET2\[7:0\]: second calibration point: offset compensation\[7:0\] with sign
    #[inline(always)]
    pub fn offset2(&self) -> OFFSET2_R {
        OFFSET2_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP_2")
            .field("gain2", &self.gain2())
            .field("offset2", &self.offset2())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - GAIN2\[11:0\]: second calibration point: gain AUXADC_GAIN_1V2\[11:0\]
    #[inline(always)]
    pub fn gain2(&mut self) -> GAIN2_W<'_, COMP_2rs> {
        GAIN2_W::new(self, 0)
    }
    ///Bits 12:19 - OFFSET2\[7:0\]: second calibration point: offset compensation\[7:0\] with sign
    #[inline(always)]
    pub fn offset2(&mut self) -> OFFSET2_W<'_, COMP_2rs> {
        OFFSET2_W::new(self, 12)
    }
}
/**COMP_2 register

You can [`read`](crate::Reg::read) this register and get [`comp_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#ADC:COMP_2)*/
pub struct COMP_2rs;
impl crate::RegisterSpec for COMP_2rs {
    type Ux = u32;
}
///`read()` method returns [`comp_2::R`](R) reader structure
impl crate::Readable for COMP_2rs {}
///`write(|w| ..)` method takes [`comp_2::W`](W) writer structure
impl crate::Writable for COMP_2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP_2 to value 0x0555
impl crate::Resettable for COMP_2rs {
    const RESET_VALUE: u32 = 0x0555;
}
