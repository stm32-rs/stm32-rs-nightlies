#[doc = "Register `TZC_PID4` reader"]
pub type R = crate::R<TZC_PID4rs>;
#[doc = "Field `PER_ID_4` reader - PER_ID_4"]
pub type PER_ID_4_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PER_ID_4"]
    #[inline(always)]
    pub fn per_id_4(&self) -> PER_ID_4_R {
        PER_ID_4_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_pid4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_PID4rs;
impl crate::RegisterSpec for TZC_PID4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_pid4::R`](R) reader structure"]
impl crate::Readable for TZC_PID4rs {}
#[doc = "`reset()` method sets TZC_PID4 to value 0x04"]
impl crate::Resettable for TZC_PID4rs {
    const RESET_VALUE: u32 = 0x04;
}
