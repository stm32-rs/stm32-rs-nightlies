#[doc = "Register `MMCTIR` reader"]
pub type R = crate::R<MMCTIRrs>;
#[doc = "Field `TGFSCS` reader - Transmitted good frames single collision status"]
pub type TGFSCS_R = crate::BitReader;
#[doc = "Field `TGFMSCS` reader - Transmitted good frames more than single collision status"]
pub type TGFMSCS_R = crate::BitReader;
#[doc = "Field `TGFS` reader - Transmitted good frames status"]
pub type TGFS_R = crate::BitReader;
impl R {
    #[doc = "Bit 14 - Transmitted good frames single collision status"]
    #[inline(always)]
    pub fn tgfscs(&self) -> TGFSCS_R {
        TGFSCS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmitted good frames more than single collision status"]
    #[inline(always)]
    pub fn tgfmscs(&self) -> TGFMSCS_R {
        TGFMSCS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames status"]
    #[inline(always)]
    pub fn tgfs(&self) -> TGFS_R {
        TGFS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Ethernet MMC transmit interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctir::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCTIRrs;
impl crate::RegisterSpec for MMCTIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctir::R`](R) reader structure"]
impl crate::Readable for MMCTIRrs {}
#[doc = "`reset()` method sets MMCTIR to value 0"]
impl crate::Resettable for MMCTIRrs {
    const RESET_VALUE: u32 = 0;
}
