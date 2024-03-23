#[doc = "Register `UR6` reader"]
pub type R = crate::R<UR6rs>;
#[doc = "Field `PA_BEG_1` reader - Protected area start address for bank 1"]
pub type PA_BEG_1_R = crate::FieldReader<u16>;
#[doc = "Field `PA_END_1` reader - Protected area end address for bank 1"]
pub type PA_END_1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Protected area start address for bank 1"]
    #[inline(always)]
    pub fn pa_beg_1(&self) -> PA_BEG_1_R {
        PA_BEG_1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Protected area end address for bank 1"]
    #[inline(always)]
    pub fn pa_end_1(&self) -> PA_END_1_R {
        PA_END_1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "SYSCFG user register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR6rs;
impl crate::RegisterSpec for UR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur6::R`](R) reader structure"]
impl crate::Readable for UR6rs {}
#[doc = "`reset()` method sets UR6 to value 0"]
impl crate::Resettable for UR6rs {
    const RESET_VALUE: u32 = 0;
}
