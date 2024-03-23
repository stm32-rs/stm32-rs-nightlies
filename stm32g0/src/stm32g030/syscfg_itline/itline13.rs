#[doc = "Register `ITLINE13` reader"]
pub type R = crate::R<ITLINE13rs>;
#[doc = "Field `TIM1_CCU` reader - TIM1_CCU"]
pub type TIM1_CCU_R = crate::BitReader;
#[doc = "Field `TIM1_TRG` reader - TIM1_TRG"]
pub type TIM1_TRG_R = crate::BitReader;
#[doc = "Field `TIM1_UPD` reader - TIM1_UPD"]
pub type TIM1_UPD_R = crate::BitReader;
#[doc = "Field `TIM1_BRK` reader - TIM1_BRK"]
pub type TIM1_BRK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM1_CCU"]
    #[inline(always)]
    pub fn tim1_ccu(&self) -> TIM1_CCU_R {
        TIM1_CCU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM1_TRG"]
    #[inline(always)]
    pub fn tim1_trg(&self) -> TIM1_TRG_R {
        TIM1_TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM1_UPD"]
    #[inline(always)]
    pub fn tim1_upd(&self) -> TIM1_UPD_R {
        TIM1_UPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM1_BRK"]
    #[inline(always)]
    pub fn tim1_brk(&self) -> TIM1_BRK_R {
        TIM1_BRK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "interrupt line 13 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline13::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE13rs;
impl crate::RegisterSpec for ITLINE13rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline13::R`](R) reader structure"]
impl crate::Readable for ITLINE13rs {}
#[doc = "`reset()` method sets ITLINE13 to value 0"]
impl crate::Resettable for ITLINE13rs {
    const RESET_VALUE: u32 = 0;
}
