///Register `SQR4` reader
pub type R = crate::R<SQR4rs>;
///Register `SQR4` writer
pub type W = crate::W<SQR4rs>;
///Field `SQ15` reader - 15th conversion in regular sequence These bits are written by software with the channel number (0 to 18) assigned as the 15th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type SQ15_R = crate::FieldReader;
///Field `SQ15` writer - 15th conversion in regular sequence These bits are written by software with the channel number (0 to 18) assigned as the 15th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type SQ15_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQ16` reader - 16th conversion in regular sequence These bits are written by software with the channel number (0 to 18) assigned as the 16th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type SQ16_R = crate::FieldReader;
///Field `SQ16` writer - 16th conversion in regular sequence These bits are written by software with the channel number (0 to 18) assigned as the 16th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type SQ16_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - 15th conversion in regular sequence These bits are written by software with the channel number (0 to 18) assigned as the 15th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn sq15(&self) -> SQ15_R {
        SQ15_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:10 - 16th conversion in regular sequence These bits are written by software with the channel number (0 to 18) assigned as the 16th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn sq16(&self) -> SQ16_R {
        SQ16_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SQR4")
            .field("sq15", &self.sq15())
            .field("sq16", &self.sq16())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - 15th conversion in regular sequence These bits are written by software with the channel number (0 to 18) assigned as the 15th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn sq15(&mut self) -> SQ15_W<SQR4rs> {
        SQ15_W::new(self, 0)
    }
    ///Bits 6:10 - 16th conversion in regular sequence These bits are written by software with the channel number (0 to 18) assigned as the 16th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn sq16(&mut self) -> SQ16_W<SQR4rs> {
        SQ16_W::new(self, 6)
    }
}
/**ADC regular sequence register 4

You can [`read`](crate::Reg::read) this register and get [`sqr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ADC1:SQR4)*/
pub struct SQR4rs;
impl crate::RegisterSpec for SQR4rs {
    type Ux = u32;
}
///`read()` method returns [`sqr4::R`](R) reader structure
impl crate::Readable for SQR4rs {}
///`write(|w| ..)` method takes [`sqr4::W`](W) writer structure
impl crate::Writable for SQR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SQR4 to value 0
impl crate::Resettable for SQR4rs {}
