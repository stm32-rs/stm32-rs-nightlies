#[doc = "Reader of register SDMMC_ID"]
pub type R = crate::R<u32, super::SDMMC_ID>;
#[doc = "Reader of field `IP_ID`"]
pub type IP_ID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SDMMC IP identification."]
    #[inline(always)]
    pub fn ip_id(&self) -> IP_ID_R {
        IP_ID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
