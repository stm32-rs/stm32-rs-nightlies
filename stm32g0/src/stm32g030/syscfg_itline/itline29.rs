#[doc = "Register `ITLINE29` reader"]
pub type R = crate::R<ITLINE29rs>;
#[doc = "Field `USART5` reader - USART5"]
pub type USART5_R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - USART5"]
    #[inline(always)]
    pub fn usart5(&self) -> USART5_R {
        USART5_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "interrupt line 29 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline29::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE29rs;
impl crate::RegisterSpec for ITLINE29rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline29::R`](R) reader structure"]
impl crate::Readable for ITLINE29rs {}
#[doc = "`reset()` method sets ITLINE29 to value 0"]
impl crate::Resettable for ITLINE29rs {
    const RESET_VALUE: u32 = 0;
}
