///Register `ITLINE14` reader
pub type R = crate::R<ITLINE14rs>;
///Field `TIM1_CC1` reader - Timer 1 capture compare 1 interrupt request pending
pub type TIM1_CC1_R = crate::BitReader;
///Field `TIM1_CC2` reader - Timer 1 capture compare 2 interrupt request pending
pub type TIM1_CC2_R = crate::BitReader;
///Field `TIM1_CC3` reader - Timer 1 capture compare 3 interrupt request pending
pub type TIM1_CC3_R = crate::BitReader;
///Field `TIM1_CC4` reader - Timer 1 capture compare 4 interrupt request pending
pub type TIM1_CC4_R = crate::BitReader;
impl R {
    ///Bit 0 - Timer 1 capture compare 1 interrupt request pending
    #[inline(always)]
    pub fn tim1_cc1(&self) -> TIM1_CC1_R {
        TIM1_CC1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer 1 capture compare 2 interrupt request pending
    #[inline(always)]
    pub fn tim1_cc2(&self) -> TIM1_CC2_R {
        TIM1_CC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer 1 capture compare 3 interrupt request pending
    #[inline(always)]
    pub fn tim1_cc3(&self) -> TIM1_CC3_R {
        TIM1_CC3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer 1 capture compare 4 interrupt request pending
    #[inline(always)]
    pub fn tim1_cc4(&self) -> TIM1_CC4_R {
        TIM1_CC4_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE14")
            .field("tim1_cc1", &self.tim1_cc1())
            .field("tim1_cc2", &self.tim1_cc2())
            .field("tim1_cc3", &self.tim1_cc3())
            .field("tim1_cc4", &self.tim1_cc4())
            .finish()
    }
}
/**SYSCFG interrupt line 14 status register

You can [`read`](crate::Reg::read) this register and get [`itline14::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#SYSCFG:ITLINE14)*/
pub struct ITLINE14rs;
impl crate::RegisterSpec for ITLINE14rs {
    type Ux = u32;
}
///`read()` method returns [`itline14::R`](R) reader structure
impl crate::Readable for ITLINE14rs {}
///`reset()` method sets ITLINE14 to value 0
impl crate::Resettable for ITLINE14rs {}
