///Register `IDCODER` reader
pub type R = crate::R<IDCODERrs>;
///Field `DEV_ID` reader - Device ID
pub type DEV_ID_R = crate::FieldReader<u16>;
///Field `REV_ID` reader - Revision
pub type REV_ID_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - Device ID
    #[inline(always)]
    pub fn dev_id(&self) -> DEV_ID_R {
        DEV_ID_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:31 - Revision
    #[inline(always)]
    pub fn rev_id(&self) -> REV_ID_R {
        REV_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDCODER")
            .field("dev_id", &self.dev_id())
            .field("rev_id", &self.rev_id())
            .finish()
    }
}
/**DBGMCU Identity Code Register

You can [`read`](crate::Reg::read) this register and get [`idcoder::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#DBGMCU:IDCODER)*/
pub struct IDCODERrs;
impl crate::RegisterSpec for IDCODERrs {
    type Ux = u32;
}
///`read()` method returns [`idcoder::R`](R) reader structure
impl crate::Readable for IDCODERrs {}
///`reset()` method sets IDCODER to value 0x1000_6497
impl crate::Resettable for IDCODERrs {
    const RESET_VALUE: u32 = 0x1000_6497;
}
