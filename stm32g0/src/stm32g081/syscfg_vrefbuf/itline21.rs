#[doc = "Register `ITLINE21` reader"]
pub type R = crate::R<ITLINE21rs>;
#[doc = "Field `TIM16` reader - TIM16"]
pub type TIM16_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM16"]
    #[inline(always)]
    pub fn tim16(&self) -> TIM16_R {
        TIM16_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 21 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline21::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE21rs;
impl crate::RegisterSpec for ITLINE21rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline21::R`](R) reader structure"]
impl crate::Readable for ITLINE21rs {}
#[doc = "`reset()` method sets ITLINE21 to value 0"]
impl crate::Resettable for ITLINE21rs {
    const RESET_VALUE: u32 = 0;
}
