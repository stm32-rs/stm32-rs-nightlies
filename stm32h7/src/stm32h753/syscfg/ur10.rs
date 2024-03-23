#[doc = "Register `UR10` reader"]
pub type R = crate::R<UR10rs>;
#[doc = "Field `PA_END_2` reader - Protected area end address for bank 2"]
pub type PA_END_2_R = crate::FieldReader<u16>;
#[doc = "Field `SA_BEG_2` reader - Secured area start address for bank 2"]
pub type SA_BEG_2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Protected area end address for bank 2"]
    #[inline(always)]
    pub fn pa_end_2(&self) -> PA_END_2_R {
        PA_END_2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Secured area start address for bank 2"]
    #[inline(always)]
    pub fn sa_beg_2(&self) -> SA_BEG_2_R {
        SA_BEG_2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "SYSCFG user register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur10::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR10rs;
impl crate::RegisterSpec for UR10rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur10::R`](R) reader structure"]
impl crate::Readable for UR10rs {}
#[doc = "`reset()` method sets UR10 to value 0"]
impl crate::Resettable for UR10rs {
    const RESET_VALUE: u32 = 0;
}
