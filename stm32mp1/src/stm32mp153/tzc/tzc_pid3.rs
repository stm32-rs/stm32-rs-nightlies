#[doc = "Register `TZC_PID3` reader"]
pub type R = crate::R<TZC_PID3rs>;
#[doc = "Field `PER_ID_3` reader - PER_ID_3"]
pub type PER_ID_3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PER_ID_3"]
    #[inline(always)]
    pub fn per_id_3(&self) -> PER_ID_3_R {
        PER_ID_3_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_pid3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_PID3rs;
impl crate::RegisterSpec for TZC_PID3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_pid3::R`](R) reader structure"]
impl crate::Readable for TZC_PID3rs {}
#[doc = "`reset()` method sets TZC_PID3 to value 0"]
impl crate::Resettable for TZC_PID3rs {
    const RESET_VALUE: u32 = 0;
}
