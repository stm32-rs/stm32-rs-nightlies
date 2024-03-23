#[doc = "Register `SYSCFG_ITLINE21` reader"]
pub type R = crate::R<SYSCFG_ITLINE21rs>;
#[doc = "Field `TIM16` reader - Timer 16 interrupt request pending"]
pub type TIM16_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer 16 interrupt request pending"]
    #[inline(always)]
    pub fn tim16(&self) -> TIM16_R {
        TIM16_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 21 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline21::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_ITLINE21rs;
impl crate::RegisterSpec for SYSCFG_ITLINE21rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline21::R`](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE21rs {}
#[doc = "`reset()` method sets SYSCFG_ITLINE21 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE21rs {
    const RESET_VALUE: u32 = 0;
}
