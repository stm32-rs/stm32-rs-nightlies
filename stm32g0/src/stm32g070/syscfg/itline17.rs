///Register `ITLINE17` reader
pub type R = crate::R<ITLINE17rs>;
///Field `TIM6` reader - TIM6
pub type TIM6_R = crate::BitReader;
///Field `DAC` reader - DAC
pub type DAC_R = crate::BitReader;
///Field `LPTIM1` reader - LPTIM1
pub type LPTIM1_R = crate::BitReader;
impl R {
    ///Bit 0 - TIM6
    #[inline(always)]
    pub fn tim6(&self) -> TIM6_R {
        TIM6_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DAC
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LPTIM1
    #[inline(always)]
    pub fn lptim1(&self) -> LPTIM1_R {
        LPTIM1_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE17")
            .field("tim6", &self.tim6())
            .field("dac", &self.dac())
            .field("lptim1", &self.lptim1())
            .finish()
    }
}
/**interrupt line 17 status register

You can [`read`](crate::Reg::read) this register and get [`itline17::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#SYSCFG:ITLINE17)*/
pub struct ITLINE17rs;
impl crate::RegisterSpec for ITLINE17rs {
    type Ux = u32;
}
///`read()` method returns [`itline17::R`](R) reader structure
impl crate::Readable for ITLINE17rs {}
///`reset()` method sets ITLINE17 to value 0
impl crate::Resettable for ITLINE17rs {
    const RESET_VALUE: u32 = 0;
}
