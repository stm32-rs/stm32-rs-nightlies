#[doc = "Register `TZC_PID6` reader"]
pub type R = crate::R<TZC_PID6rs>;
#[doc = "Field `PER_ID_6` reader - PER_ID_6"]
pub type PER_ID_6_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PER_ID_6"]
    #[inline(always)]
    pub fn per_id_6(&self) -> PER_ID_6_R {
        PER_ID_6_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID 6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_pid6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_PID6rs;
impl crate::RegisterSpec for TZC_PID6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_pid6::R`](R) reader structure"]
impl crate::Readable for TZC_PID6rs {}
#[doc = "`reset()` method sets TZC_PID6 to value 0"]
impl crate::Resettable for TZC_PID6rs {
    const RESET_VALUE: u32 = 0;
}
