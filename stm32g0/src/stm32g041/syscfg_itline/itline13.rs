///Register `ITLINE13` reader
pub type R = crate::R<ITLINE13rs>;
///Field `TIM1_CCU` reader - TIM1_CCU
pub type TIM1_CCU_R = crate::BitReader;
///Field `TIM1_TRG` reader - TIM1_TRG
pub type TIM1_TRG_R = crate::BitReader;
///Field `TIM1_UPD` reader - TIM1_UPD
pub type TIM1_UPD_R = crate::BitReader;
///Field `TIM1_BRK` reader - TIM1_BRK
pub type TIM1_BRK_R = crate::BitReader;
impl R {
    ///Bit 0 - TIM1_CCU
    #[inline(always)]
    pub fn tim1_ccu(&self) -> TIM1_CCU_R {
        TIM1_CCU_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM1_TRG
    #[inline(always)]
    pub fn tim1_trg(&self) -> TIM1_TRG_R {
        TIM1_TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM1_UPD
    #[inline(always)]
    pub fn tim1_upd(&self) -> TIM1_UPD_R {
        TIM1_UPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM1_BRK
    #[inline(always)]
    pub fn tim1_brk(&self) -> TIM1_BRK_R {
        TIM1_BRK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE13")
            .field("tim1_ccu", &self.tim1_ccu())
            .field("tim1_trg", &self.tim1_trg())
            .field("tim1_upd", &self.tim1_upd())
            .field("tim1_brk", &self.tim1_brk())
            .finish()
    }
}
/**interrupt line 13 status register

You can [`read`](crate::Reg::read) this register and get [`itline13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G041.html#SYSCFG_ITLINE:ITLINE13)*/
pub struct ITLINE13rs;
impl crate::RegisterSpec for ITLINE13rs {
    type Ux = u32;
}
///`read()` method returns [`itline13::R`](R) reader structure
impl crate::Readable for ITLINE13rs {}
///`reset()` method sets ITLINE13 to value 0
impl crate::Resettable for ITLINE13rs {
    const RESET_VALUE: u32 = 0;
}
