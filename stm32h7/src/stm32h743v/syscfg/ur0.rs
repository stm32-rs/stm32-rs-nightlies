#[doc = "Register `UR0` reader"]
pub type R = crate::R<UR0rs>;
#[doc = "Field `BKS` reader - Bank Swap"]
pub type BKS_R = crate::BitReader;
#[doc = "Field `RDP` reader - Readout protection"]
pub type RDP_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Bank Swap"]
    #[inline(always)]
    pub fn bks(&self) -> BKS_R {
        BKS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - Readout protection"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "SYSCFG user register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR0rs;
impl crate::RegisterSpec for UR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur0::R`](R) reader structure"]
impl crate::Readable for UR0rs {}
#[doc = "`reset()` method sets UR0 to value 0"]
impl crate::Resettable for UR0rs {
    const RESET_VALUE: u32 = 0;
}
