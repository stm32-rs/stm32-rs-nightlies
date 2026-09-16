///Register `IDCODE` reader
pub type R = crate::R<IDCODErs>;
///Field `DEV_ID` reader - device identification
pub type DEV_ID_R = crate::FieldReader<u16>;
///Field `REV_ID` reader - revision This field indicates the revision of the device.
pub type REV_ID_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - device identification
    #[inline(always)]
    pub fn dev_id(&self) -> DEV_ID_R {
        DEV_ID_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:31 - revision This field indicates the revision of the device.
    #[inline(always)]
    pub fn rev_id(&self) -> REV_ID_R {
        REV_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDCODE")
            .field("dev_id", &self.dev_id())
            .field("rev_id", &self.rev_id())
            .finish()
    }
}
/**DBGMCU identity code register

You can [`read`](crate::Reg::read) this register and get [`idcode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#DBGMCU:IDCODE)*/
pub struct IDCODErs;
impl crate::RegisterSpec for IDCODErs {
    type Ux = u32;
}
///`read()` method returns [`idcode::R`](R) reader structure
impl crate::Readable for IDCODErs {}
///`reset()` method sets IDCODE to value 0x6000
impl crate::Resettable for IDCODErs {
    const RESET_VALUE: u32 = 0x6000;
}
