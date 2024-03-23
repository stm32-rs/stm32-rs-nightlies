#[doc = "Register `SYSCFG_ITLINE23` reader"]
pub type R = crate::R<SYSCFG_ITLINE23rs>;
#[doc = "Field `I2C1` reader - I2C1 interrupt request pending, combined with EXTI line 23"]
pub type I2C1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - I2C1 interrupt request pending, combined with EXTI line 23"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 23 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline23::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_ITLINE23rs;
impl crate::RegisterSpec for SYSCFG_ITLINE23rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline23::R`](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE23rs {}
#[doc = "`reset()` method sets SYSCFG_ITLINE23 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE23rs {
    const RESET_VALUE: u32 = 0;
}
