#[doc = "Register `DDRCTRL_ZQSTAT` reader"]
pub type R = crate::R<DDRCTRL_ZQSTATrs>;
#[doc = "Field `ZQ_RESET_BUSY` reader - ZQ_RESET_BUSY"]
pub type ZQ_RESET_BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ZQ_RESET_BUSY"]
    #[inline(always)]
    pub fn zq_reset_busy(&self) -> ZQ_RESET_BUSY_R {
        ZQ_RESET_BUSY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "DDRCTRL ZQ status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_zqstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_ZQSTATrs;
impl crate::RegisterSpec for DDRCTRL_ZQSTATrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_zqstat::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_ZQSTATrs {}
#[doc = "`reset()` method sets DDRCTRL_ZQSTAT to value 0"]
impl crate::Resettable for DDRCTRL_ZQSTATrs {
    const RESET_VALUE: u32 = 0;
}
