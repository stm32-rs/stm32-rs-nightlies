///Register `COMP_3` reader
pub type R = crate::R<COMP_3rs>;
///Register `COMP_3` writer
pub type W = crate::W<COMP_3rs>;
///Field `GAIN3` reader - GAIN3\[11:0\]: third calibration point: gain AUXADC_GAIN_1V2\[11:0\]
pub type GAIN3_R = crate::FieldReader<u16>;
///Field `GAIN3` writer - GAIN3\[11:0\]: third calibration point: gain AUXADC_GAIN_1V2\[11:0\]
pub type GAIN3_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `OFFSET3` reader - OFFSET3\[7:0\]: third calibration point
pub type OFFSET3_R = crate::FieldReader;
///Field `OFFSET3` writer - OFFSET3\[7:0\]: third calibration point
pub type OFFSET3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:11 - GAIN3\[11:0\]: third calibration point: gain AUXADC_GAIN_1V2\[11:0\]
    #[inline(always)]
    pub fn gain3(&self) -> GAIN3_R {
        GAIN3_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:19 - OFFSET3\[7:0\]: third calibration point
    #[inline(always)]
    pub fn offset3(&self) -> OFFSET3_R {
        OFFSET3_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP_3")
            .field("gain3", &self.gain3())
            .field("offset3", &self.offset3())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - GAIN3\[11:0\]: third calibration point: gain AUXADC_GAIN_1V2\[11:0\]
    #[inline(always)]
    pub fn gain3(&mut self) -> GAIN3_W<'_, COMP_3rs> {
        GAIN3_W::new(self, 0)
    }
    ///Bits 12:19 - OFFSET3\[7:0\]: third calibration point
    #[inline(always)]
    pub fn offset3(&mut self) -> OFFSET3_W<'_, COMP_3rs> {
        OFFSET3_W::new(self, 12)
    }
}
/**COMP_3 register

You can [`read`](crate::Reg::read) this register and get [`comp_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:COMP_3)*/
pub struct COMP_3rs;
impl crate::RegisterSpec for COMP_3rs {
    type Ux = u32;
}
///`read()` method returns [`comp_3::R`](R) reader structure
impl crate::Readable for COMP_3rs {}
///`write(|w| ..)` method takes [`comp_3::W`](W) writer structure
impl crate::Writable for COMP_3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP_3 to value 0x0555
impl crate::Resettable for COMP_3rs {
    const RESET_VALUE: u32 = 0x0555;
}
