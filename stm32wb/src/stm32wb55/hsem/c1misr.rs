#[doc = "Reader of register C1MISR"]
pub type R = crate::R<u32, super::C1MISR>;
#[doc = "Reader of field `MISFm`"]
pub type MISFM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - masked CPU(n) semaphore m status bit after enable (mask)."]
    #[inline(always)]
    pub fn misfm(&self) -> MISFM_R {
        MISFM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
