///Register `DAC_MCR` reader
pub type R = crate::R<DAC_MCRrs>;
///Register `DAC_MCR` writer
pub type W = crate::W<DAC_MCRrs>;
///Field `MODE1` reader - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN11=10 and bit CEN11=10 in the DAC_CR register). If EN11=11 or CEN11=11 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN11=10.
pub type MODE1_R = crate::FieldReader;
///Field `MODE1` writer - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN11=10 and bit CEN11=10 in the DAC_CR register). If EN11=11 or CEN11=11 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN11=10.
pub type MODE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN11=10 and bit CEN11=10 in the DAC_CR register). If EN11=11 or CEN11=11 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN11=10.
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_MCR")
            .field("mode1", &self.mode1())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN11=10 and bit CEN11=10 in the DAC_CR register). If EN11=11 or CEN11=11 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN11=10.
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W<DAC_MCRrs> {
        MODE1_W::new(self, 0)
    }
}
/**DAC mode control register

You can [`read`](crate::Reg::read) this register and get [`dac_mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DAC:DAC_MCR)*/
pub struct DAC_MCRrs;
impl crate::RegisterSpec for DAC_MCRrs {
    type Ux = u32;
}
///`read()` method returns [`dac_mcr::R`](R) reader structure
impl crate::Readable for DAC_MCRrs {}
///`write(|w| ..)` method takes [`dac_mcr::W`](W) writer structure
impl crate::Writable for DAC_MCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_MCR to value 0
impl crate::Resettable for DAC_MCRrs {
    const RESET_VALUE: u32 = 0;
}