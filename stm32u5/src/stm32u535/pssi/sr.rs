#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `RTT4B` reader - RTT4B"]
pub type RTT4B_R = crate::BitReader;
#[doc = "Field `RTT1B` reader - RTT1B"]
pub type RTT1B_R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - RTT4B"]
    #[inline(always)]
    pub fn rtt4b(&self) -> RTT4B_R {
        RTT4B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTT1B"]
    #[inline(always)]
    pub fn rtt1b(&self) -> RTT1B_R {
        RTT1B_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "PSSI status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
