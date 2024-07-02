///Register `DAC_SHSR2` reader
pub type R = crate::R<DAC_SHSR2rs>;
///Register `DAC_SHSR2` writer
pub type W = crate::W<DAC_SHSR2rs>;
///Field `TSAMPLE2` reader - DAC Channel 2 sample Time (only valid in sample and hold mode)
pub type TSAMPLE2_R = crate::FieldReader<u16>;
///Field `TSAMPLE2` writer - DAC Channel 2 sample Time (only valid in sample and hold mode)
pub type TSAMPLE2_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - DAC Channel 2 sample Time (only valid in sample and hold mode)
    #[inline(always)]
    pub fn tsample2(&self) -> TSAMPLE2_R {
        TSAMPLE2_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_SHSR2")
            .field("tsample2", &self.tsample2())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - DAC Channel 2 sample Time (only valid in sample and hold mode)
    #[inline(always)]
    #[must_use]
    pub fn tsample2(&mut self) -> TSAMPLE2_W<DAC_SHSR2rs> {
        TSAMPLE2_W::new(self, 0)
    }
}
/**DAC channel2 sample and hold sample time register

You can [`read`](crate::Reg::read) this register and get [`dac_shsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_shsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#DAC1:DAC_SHSR2)*/
pub struct DAC_SHSR2rs;
impl crate::RegisterSpec for DAC_SHSR2rs {
    type Ux = u32;
}
///`read()` method returns [`dac_shsr2::R`](R) reader structure
impl crate::Readable for DAC_SHSR2rs {}
///`write(|w| ..)` method takes [`dac_shsr2::W`](W) writer structure
impl crate::Writable for DAC_SHSR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_SHSR2 to value 0
impl crate::Resettable for DAC_SHSR2rs {
    const RESET_VALUE: u32 = 0;
}
