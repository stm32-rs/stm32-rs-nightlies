///Register `IDR` reader
pub type R = crate::R<IDRrs>;
///Field `REV` reader - REV
pub type REV_R = crate::FieldReader;
///Field `MINVER` reader - MINVER
pub type MINVER_R = crate::FieldReader;
///Field `MAJVER` reader - MAJVER
pub type MAJVER_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - REV
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - MINVER
    #[inline(always)]
    pub fn minver(&self) -> MINVER_R {
        MINVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - MAJVER
    #[inline(always)]
    pub fn majver(&self) -> MAJVER_R {
        MAJVER_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDR")
            .field("rev", &self.rev())
            .field("minver", &self.minver())
            .field("majver", &self.majver())
            .finish()
    }
}
/**LTDC identification register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LTDC:IDR)*/
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`reset()` method sets IDR to value 0x0001_0300
impl crate::Resettable for IDRrs {
    const RESET_VALUE: u32 = 0x0001_0300;
}
