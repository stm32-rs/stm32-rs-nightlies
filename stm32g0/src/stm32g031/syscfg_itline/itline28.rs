#[doc = "Register `ITLINE28` reader"]
pub type R = crate::R<ITLINE28rs>;
#[doc = "Field `USART2` reader - USART2"]
pub type USART2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USART2"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 28 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline28::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE28rs;
impl crate::RegisterSpec for ITLINE28rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline28::R`](R) reader structure"]
impl crate::Readable for ITLINE28rs {}
#[doc = "`reset()` method sets ITLINE28 to value 0"]
impl crate::Resettable for ITLINE28rs {
    const RESET_VALUE: u32 = 0;
}
