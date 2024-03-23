#[doc = "Register `TZC_PID0` reader"]
pub type R = crate::R<TZC_PID0rs>;
#[doc = "Field `PER_ID_0` reader - PER_ID_0"]
pub type PER_ID_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PER_ID_0"]
    #[inline(always)]
    pub fn per_id_0(&self) -> PER_ID_0_R {
        PER_ID_0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_pid0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_PID0rs;
impl crate::RegisterSpec for TZC_PID0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_pid0::R`](R) reader structure"]
impl crate::Readable for TZC_PID0rs {}
#[doc = "`reset()` method sets TZC_PID0 to value 0x60"]
impl crate::Resettable for TZC_PID0rs {
    const RESET_VALUE: u32 = 0x60;
}
