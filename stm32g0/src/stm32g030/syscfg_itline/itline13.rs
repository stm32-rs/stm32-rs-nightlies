#[doc = "Reader of register ITLINE13"]
pub type R = crate::R<u32, super::ITLINE13>;
#[doc = "Reader of field `TIM1_CCU`"]
pub type TIM1_CCU_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM1_TRG`"]
pub type TIM1_TRG_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM1_UPD`"]
pub type TIM1_UPD_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM1_BRK`"]
pub type TIM1_BRK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TIM1_CCU"]
    #[inline(always)]
    pub fn tim1_ccu(&self) -> TIM1_CCU_R {
        TIM1_CCU_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM1_TRG"]
    #[inline(always)]
    pub fn tim1_trg(&self) -> TIM1_TRG_R {
        TIM1_TRG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM1_UPD"]
    #[inline(always)]
    pub fn tim1_upd(&self) -> TIM1_UPD_R {
        TIM1_UPD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM1_BRK"]
    #[inline(always)]
    pub fn tim1_brk(&self) -> TIM1_BRK_R {
        TIM1_BRK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
