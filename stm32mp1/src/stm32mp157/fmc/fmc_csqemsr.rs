#[doc = "Reader of register FMC_CSQEMSR"]
pub type R = crate::R<u32, super::FMC_CSQEMSR>;
#[doc = "Reader of field `SEM`"]
pub type SEM_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - SEM"]
    #[inline(always)]
    pub fn sem(&self) -> SEM_R {
        SEM_R::new((self.bits & 0xffff) as u16)
    }
}
