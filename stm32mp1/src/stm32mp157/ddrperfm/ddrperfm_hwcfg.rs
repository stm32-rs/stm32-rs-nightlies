#[doc = "Reader of register DDRPERFM_HWCFG"]
pub type R = crate::R<u32, super::DDRPERFM_HWCFG>;
#[doc = "Reader of field `NCNT`"]
pub type NCNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - NCNT"]
    #[inline(always)]
    pub fn ncnt(&self) -> NCNT_R {
        NCNT_R::new((self.bits & 0x0f) as u8)
    }
}
