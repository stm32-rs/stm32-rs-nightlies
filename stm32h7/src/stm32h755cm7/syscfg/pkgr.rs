///Register `PKGR` reader
pub type R = crate::R<PKGRrs>;
///Field `PKG` reader - Package
pub type PKG_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - Package
    #[inline(always)]
    pub fn pkg(&self) -> PKG_R {
        PKG_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PKGR").field("pkg", &self.pkg()).finish()
    }
}
/**SYSCFG package register

You can [`read`](crate::Reg::read) this register and get [`pkgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#SYSCFG:PKGR)*/
pub struct PKGRrs;
impl crate::RegisterSpec for PKGRrs {
    type Ux = u32;
}
///`read()` method returns [`pkgr::R`](R) reader structure
impl crate::Readable for PKGRrs {}
///`reset()` method sets PKGR to value 0
impl crate::Resettable for PKGRrs {}
