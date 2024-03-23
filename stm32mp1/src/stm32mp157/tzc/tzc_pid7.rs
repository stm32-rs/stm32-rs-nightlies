#[doc = "Register `TZC_PID7` reader"]
pub type R = crate::R<TZC_PID7rs>;
#[doc = "Field `PER_ID_7` reader - PER_ID_7"]
pub type PER_ID_7_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PER_ID_7"]
    #[inline(always)]
    pub fn per_id_7(&self) -> PER_ID_7_R {
        PER_ID_7_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID 7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_pid7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_PID7rs;
impl crate::RegisterSpec for TZC_PID7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_pid7::R`](R) reader structure"]
impl crate::Readable for TZC_PID7rs {}
#[doc = "`reset()` method sets TZC_PID7 to value 0"]
impl crate::Resettable for TZC_PID7rs {
    const RESET_VALUE: u32 = 0;
}
