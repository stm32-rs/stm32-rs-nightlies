#[doc = "Reader of register FMC_HPR"]
pub type R = crate::R<u32, super::FMC_HPR>;
#[doc = "Reader of field `HPR`"]
pub type HPR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - HPR"]
    #[inline(always)]
    pub fn hpr(&self) -> HPR_R {
        HPR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
