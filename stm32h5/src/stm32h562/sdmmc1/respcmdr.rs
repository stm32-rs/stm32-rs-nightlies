#[doc = "Register `RESPCMDR` reader"]
pub type R = crate::R<RESPCMDRrs>;
#[doc = "Field `RESPCMD` reader - Response command index Read-only bit field. Contains the command index of the last command response received."]
pub type RESPCMD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Response command index Read-only bit field. Contains the command index of the last command response received."]
    #[inline(always)]
    pub fn respcmd(&self) -> RESPCMD_R {
        RESPCMD_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "SDMMC command response register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`respcmdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESPCMDRrs;
impl crate::RegisterSpec for RESPCMDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`respcmdr::R`](R) reader structure"]
impl crate::Readable for RESPCMDRrs {}
#[doc = "`reset()` method sets RESPCMDR to value 0"]
impl crate::Resettable for RESPCMDRrs {
    const RESET_VALUE: u32 = 0;
}
