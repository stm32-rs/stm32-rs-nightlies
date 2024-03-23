#[doc = "Register `SYSCFG_ITLINE13` reader"]
pub type R = crate::R<SYSCFG_ITLINE13rs>;
#[doc = "Field `TIM1_CCU` reader - Timer 1 commutation interrupt request pending"]
pub type TIM1_CCU_R = crate::BitReader;
#[doc = "Field `TIM1_TRG` reader - Timer 1 trigger interrupt request pending"]
pub type TIM1_TRG_R = crate::BitReader;
#[doc = "Field `TIM1_UPD` reader - Timer 1 update interrupt request pending"]
pub type TIM1_UPD_R = crate::BitReader;
#[doc = "Field `TIM1_BRK` reader - Timer 1 break interrupt request pending"]
pub type TIM1_BRK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer 1 commutation interrupt request pending"]
    #[inline(always)]
    pub fn tim1_ccu(&self) -> TIM1_CCU_R {
        TIM1_CCU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1 trigger interrupt request pending"]
    #[inline(always)]
    pub fn tim1_trg(&self) -> TIM1_TRG_R {
        TIM1_TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 1 update interrupt request pending"]
    #[inline(always)]
    pub fn tim1_upd(&self) -> TIM1_UPD_R {
        TIM1_UPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 1 break interrupt request pending"]
    #[inline(always)]
    pub fn tim1_brk(&self) -> TIM1_BRK_R {
        TIM1_BRK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 13 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline13::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_ITLINE13rs;
impl crate::RegisterSpec for SYSCFG_ITLINE13rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline13::R`](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE13rs {}
#[doc = "`reset()` method sets SYSCFG_ITLINE13 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE13rs {
    const RESET_VALUE: u32 = 0;
}
