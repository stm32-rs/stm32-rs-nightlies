///Register `TS0SDIFDONER` reader
pub type R = crate::R<TS0SDIFDONERrs>;
///Field `SDIF_SMPL_DONE` reader - Sample done flag
pub type SDIF_SMPL_DONE_R = crate::BitReader;
impl R {
    ///Bit 0 - Sample done flag
    #[inline(always)]
    pub fn sdif_smpl_done(&self) -> SDIF_SMPL_DONE_R {
        SDIF_SMPL_DONE_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TS0SDIFDONER")
            .field("sdif_smpl_done", &self.sdif_smpl_done())
            .finish()
    }
}
/**DTS TS0 SDIF done register

You can [`read`](crate::Reg::read) this register and get [`ts0sdifdoner::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DTS:TS0SDIFDONER)*/
pub struct TS0SDIFDONERrs;
impl crate::RegisterSpec for TS0SDIFDONERrs {
    type Ux = u32;
}
///`read()` method returns [`ts0sdifdoner::R`](R) reader structure
impl crate::Readable for TS0SDIFDONERrs {}
///`reset()` method sets TS0SDIFDONER to value 0
impl crate::Resettable for TS0SDIFDONERrs {}
