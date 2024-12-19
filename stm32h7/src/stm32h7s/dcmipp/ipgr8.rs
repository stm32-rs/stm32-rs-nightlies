///Register `IPGR8` reader
pub type R = crate::R<IPGR8rs>;
///Field `DID` reader - Division identifier (0x14)
pub type DID_R = crate::FieldReader;
///Field `REVID` reader - Revision identifier (0x03)
pub type REVID_R = crate::FieldReader;
///Field `ARCHIID` reader - Architecture identifier (0x04)
pub type ARCHIID_R = crate::FieldReader;
///Field `IPPID` reader - IP identifier (0xAA)
pub type IPPID_R = crate::FieldReader;
impl R {
    ///Bits 0:5 - Division identifier (0x14)
    #[inline(always)]
    pub fn did(&self) -> DID_R {
        DID_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:12 - Revision identifier (0x03)
    #[inline(always)]
    pub fn revid(&self) -> REVID_R {
        REVID_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - Architecture identifier (0x04)
    #[inline(always)]
    pub fn archiid(&self) -> ARCHIID_R {
        ARCHIID_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:31 - IP identifier (0xAA)
    #[inline(always)]
    pub fn ippid(&self) -> IPPID_R {
        IPPID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPGR8")
            .field("did", &self.did())
            .field("revid", &self.revid())
            .field("archiid", &self.archiid())
            .field("ippid", &self.ippid())
            .finish()
    }
}
/**DCMIPP IP-Plug identification register

You can [`read`](crate::Reg::read) this register and get [`ipgr8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DCMIPP:IPGR8)*/
pub struct IPGR8rs;
impl crate::RegisterSpec for IPGR8rs {
    type Ux = u32;
}
///`read()` method returns [`ipgr8::R`](R) reader structure
impl crate::Readable for IPGR8rs {}
///`reset()` method sets IPGR8 to value 0xaa04_0314
impl crate::Resettable for IPGR8rs {
    const RESET_VALUE: u32 = 0xaa04_0314;
}
