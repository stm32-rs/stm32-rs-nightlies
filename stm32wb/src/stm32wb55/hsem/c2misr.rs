#[doc = "Reader of register C2MISR"]
pub type R = crate::R<u32, super::C2MISR>;
#[doc = "Reader of field `MISFm`"]
pub type MISFM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - masked CPU(2) semaphore m status bit after enable (mask)."]
    #[inline(always)]
    pub fn misfm(&self) -> MISFM_R {
        MISFM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
