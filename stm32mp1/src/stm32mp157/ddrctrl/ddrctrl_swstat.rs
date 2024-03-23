#[doc = "Register `DDRCTRL_SWSTAT` reader"]
pub type R = crate::R<DDRCTRL_SWSTATrs>;
#[doc = "Field `SW_DONE_ACK` reader - SW_DONE_ACK"]
pub type SW_DONE_ACK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SW_DONE_ACK"]
    #[inline(always)]
    pub fn sw_done_ack(&self) -> SW_DONE_ACK_R {
        SW_DONE_ACK_R::new((self.bits & 1) != 0)
    }
}
#[doc = "DDRCTRL software register programming control status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_swstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_SWSTATrs;
impl crate::RegisterSpec for DDRCTRL_SWSTATrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_swstat::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_SWSTATrs {}
#[doc = "`reset()` method sets DDRCTRL_SWSTAT to value 0x01"]
impl crate::Resettable for DDRCTRL_SWSTATrs {
    const RESET_VALUE: u32 = 0x01;
}
