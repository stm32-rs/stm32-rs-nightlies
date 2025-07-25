///Register `SADANLVR` reader
pub type R = crate::R<SADANLVRrs>;
///Field `ANLVL` reader - Ambient noise level estimation
pub type ANLVL_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:14 - Ambient noise level estimation
    #[inline(always)]
    pub fn anlvl(&self) -> ANLVL_R {
        ANLVL_R::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SADANLVR")
            .field("anlvl", &self.anlvl())
            .finish()
    }
}
/**ADF SAD ambient noise level register

You can [`read`](crate::Reg::read) this register and get [`sadanlvr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ADF:SADANLVR)*/
pub struct SADANLVRrs;
impl crate::RegisterSpec for SADANLVRrs {
    type Ux = u32;
}
///`read()` method returns [`sadanlvr::R`](R) reader structure
impl crate::Readable for SADANLVRrs {}
///`reset()` method sets SADANLVR to value 0
impl crate::Resettable for SADANLVRrs {}
