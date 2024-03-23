#[doc = "Register `DDRCTRL_DFISTAT` reader"]
pub type R = crate::R<DDRCTRL_DFISTATrs>;
#[doc = "Field `DFI_INIT_COMPLETE` reader - DFI_INIT_COMPLETE"]
pub type DFI_INIT_COMPLETE_R = crate::BitReader;
#[doc = "Field `DFI_LP_ACK` reader - DFI_LP_ACK"]
pub type DFI_LP_ACK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DFI_INIT_COMPLETE"]
    #[inline(always)]
    pub fn dfi_init_complete(&self) -> DFI_INIT_COMPLETE_R {
        DFI_INIT_COMPLETE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DFI_LP_ACK"]
    #[inline(always)]
    pub fn dfi_lp_ack(&self) -> DFI_LP_ACK_R {
        DFI_LP_ACK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "DDRCTRL DFI status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dfistat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DFISTATrs;
impl crate::RegisterSpec for DDRCTRL_DFISTATrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dfistat::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DFISTATrs {}
#[doc = "`reset()` method sets DDRCTRL_DFISTAT to value 0"]
impl crate::Resettable for DDRCTRL_DFISTATrs {
    const RESET_VALUE: u32 = 0;
}
