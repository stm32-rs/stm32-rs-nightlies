#[doc = "Register `DDRCTRL_MRSTAT` reader"]
pub type R = crate::R<DDRCTRL_MRSTATrs>;
#[doc = "Field `MR_WR_BUSY` reader - MR_WR_BUSY"]
pub type MR_WR_BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MR_WR_BUSY"]
    #[inline(always)]
    pub fn mr_wr_busy(&self) -> MR_WR_BUSY_R {
        MR_WR_BUSY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "DDRCTRL mode register read/write status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_mrstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_MRSTATrs;
impl crate::RegisterSpec for DDRCTRL_MRSTATrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_mrstat::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_MRSTATrs {}
#[doc = "`reset()` method sets DDRCTRL_MRSTAT to value 0"]
impl crate::Resettable for DDRCTRL_MRSTATrs {
    const RESET_VALUE: u32 = 0;
}
