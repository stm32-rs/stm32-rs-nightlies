#[doc = "Register `PWR_CSR1` reader"]
pub type R = crate::R<PWR_CSR1rs>;
#[doc = "Field `PVDO` reader - PVDO"]
pub type PVDO_R = crate::BitReader;
#[doc = "Field `AVDO` reader - AVDO"]
pub type AVDO_R = crate::BitReader;
impl R {
    #[doc = "Bit 4 - PVDO"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - AVDO"]
    #[inline(always)]
    pub fn avdo(&self) -> AVDO_R {
        AVDO_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Reset on any system reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_csr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_CSR1rs;
impl crate::RegisterSpec for PWR_CSR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_csr1::R`](R) reader structure"]
impl crate::Readable for PWR_CSR1rs {}
#[doc = "`reset()` method sets PWR_CSR1 to value 0"]
impl crate::Resettable for PWR_CSR1rs {
    const RESET_VALUE: u32 = 0;
}
