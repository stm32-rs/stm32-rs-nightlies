#[doc = "Register `LPTIM_PIDR` reader"]
pub type R = crate::R<LPTIM_PIDRrs>;
#[doc = "Field `P_ID` reader - P_ID"]
pub type P_ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - P_ID"]
    #[inline(always)]
    pub fn p_id(&self) -> P_ID_R {
        P_ID_R::new(self.bits)
    }
}
#[doc = "LPTIM peripheral type identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_pidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPTIM_PIDRrs;
impl crate::RegisterSpec for LPTIM_PIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim_pidr::R`](R) reader structure"]
impl crate::Readable for LPTIM_PIDRrs {}
#[doc = "`reset()` method sets LPTIM_PIDR to value 0x0012_0011"]
impl crate::Resettable for LPTIM_PIDRrs {
    const RESET_VALUE: u32 = 0x0012_0011;
}
