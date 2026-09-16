///Register `IDCODE` reader
pub type R = crate::R<IDCODErs>;
///Field `DEV_ID` reader - Device identifier
pub type DEV_ID_R = crate::FieldReader<u16>;
///Field `REV_ID` reader - Revision identifie
pub type REV_ID_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - Device identifier
    #[inline(always)]
    pub fn dev_id(&self) -> DEV_ID_R {
        DEV_ID_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:31 - Revision identifie
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
/**DBGMCU_IDCODE

You can [`read`](crate::Reg::read) this register and get [`idcode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#DBGMCU:IDCODE)*/
pub struct IDCODErs;
impl crate::RegisterSpec for IDCODErs {
    type Ux = u32;
}
///`read()` method returns [`idcode::R`](R) reader structure
impl crate::Readable for IDCODErs {}
///`reset()` method sets IDCODE to value 0
impl crate::Resettable for IDCODErs {}
