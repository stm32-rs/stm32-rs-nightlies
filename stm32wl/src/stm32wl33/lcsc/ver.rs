///Register `VER` reader
pub type R = crate::R<VERrs>;
///Field `REV` reader - Revision of the RFIP to be used for metal fixes)
pub type REV_R = crate::FieldReader;
///Field `VER` reader - Version of the RFIP (to be used for cut upgrades)
pub type VER_R = crate::FieldReader;
///Field `PROD` reader - Used for major upgrades (new protocols support / new features)
pub type PROD_R = crate::FieldReader;
impl R {
    ///Bits 4:7 - Revision of the RFIP to be used for metal fixes)
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Version of the RFIP (to be used for cut upgrades)
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Used for major upgrades (new protocols support / new features)
    #[inline(always)]
    pub fn prod(&self) -> PROD_R {
        PROD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VER")
            .field("rev", &self.rev())
            .field("ver", &self.ver())
            .field("prod", &self.prod())
            .finish()
    }
}
/**LCSC_VER register

You can [`read`](crate::Reg::read) this register and get [`ver::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:VER)*/
pub struct VERrs;
impl crate::RegisterSpec for VERrs {
    type Ux = u32;
}
///`read()` method returns [`ver::R`](R) reader structure
impl crate::Readable for VERrs {}
///`reset()` method sets VER to value 0x1000
impl crate::Resettable for VERrs {
    const RESET_VALUE: u32 = 0x1000;
}
