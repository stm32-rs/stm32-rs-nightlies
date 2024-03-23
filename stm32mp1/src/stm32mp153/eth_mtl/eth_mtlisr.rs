#[doc = "Register `ETH_MTLISR` reader"]
pub type R = crate::R<ETH_MTLISRrs>;
#[doc = "Field `Q0IS` reader - Q0IS"]
pub type Q0IS_R = crate::BitReader;
#[doc = "Field `Q1IS` reader - Q1IS"]
pub type Q1IS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Q0IS"]
    #[inline(always)]
    pub fn q0is(&self) -> Q0IS_R {
        Q0IS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Q1IS"]
    #[inline(always)]
    pub fn q1is(&self) -> Q1IS_R {
        Q1IS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlisr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MTLISRrs;
impl crate::RegisterSpec for ETH_MTLISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mtlisr::R`](R) reader structure"]
impl crate::Readable for ETH_MTLISRrs {}
#[doc = "`reset()` method sets ETH_MTLISR to value 0"]
impl crate::Resettable for ETH_MTLISRrs {
    const RESET_VALUE: u32 = 0;
}
