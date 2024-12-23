///Register `DAC_CCR` reader
pub type R = crate::R<DAC_CCRrs>;
///Register `DAC_CCR` writer
pub type W = crate::W<DAC_CCRrs>;
///Field `OTRIM1` reader - DAC channel1 offset trimming value
pub type OTRIM1_R = crate::FieldReader;
///Field `OTRIM1` writer - DAC channel1 offset trimming value
pub type OTRIM1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - DAC channel1 offset trimming value
    #[inline(always)]
    pub fn otrim1(&self) -> OTRIM1_R {
        OTRIM1_R::new((self.bits & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_CCR")
            .field("otrim1", &self.otrim1())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - DAC channel1 offset trimming value
    #[inline(always)]
    pub fn otrim1(&mut self) -> OTRIM1_W<DAC_CCRrs> {
        OTRIM1_W::new(self, 0)
    }
}
/**DAC calibration control register

You can [`read`](crate::Reg::read) this register and get [`dac_ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#DAC:DAC_CCR)*/
pub struct DAC_CCRrs;
impl crate::RegisterSpec for DAC_CCRrs {
    type Ux = u32;
}
///`read()` method returns [`dac_ccr::R`](R) reader structure
impl crate::Readable for DAC_CCRrs {}
///`write(|w| ..)` method takes [`dac_ccr::W`](W) writer structure
impl crate::Writable for DAC_CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_CCR to value 0
impl crate::Resettable for DAC_CCRrs {
    const RESET_VALUE: u32 = 0;
}
