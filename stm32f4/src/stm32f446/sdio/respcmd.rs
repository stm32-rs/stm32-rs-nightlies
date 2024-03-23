#[doc = "Register `RESPCMD` reader"]
pub type R = crate::R<RESPCMDrs>;
#[doc = "Field `RESPCMD` reader - Response command index Read-only bit field. Contains the command index of the last command response received."]
pub type RESPCMD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Response command index Read-only bit field. Contains the command index of the last command response received."]
    #[inline(always)]
    pub fn respcmd(&self) -> RESPCMD_R {
        RESPCMD_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`respcmd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESPCMDrs;
impl crate::RegisterSpec for RESPCMDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`respcmd::R`](R) reader structure"]
impl crate::Readable for RESPCMDrs {}
#[doc = "`reset()` method sets RESPCMD to value 0"]
impl crate::Resettable for RESPCMDrs {
    const RESET_VALUE: u32 = 0;
}
