#[doc = "Register `ITLINE30` reader"]
pub type R = crate::R<ITLINE30rs>;
#[doc = "Field `USART2` reader - CEC"]
pub type USART2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CEC"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 30 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline30::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE30rs;
impl crate::RegisterSpec for ITLINE30rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline30::R`](R) reader structure"]
impl crate::Readable for ITLINE30rs {}
#[doc = "`reset()` method sets ITLINE30 to value 0"]
impl crate::Resettable for ITLINE30rs {
    const RESET_VALUE: u32 = 0;
}
