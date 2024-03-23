#[doc = "Register `UR5` reader"]
pub type R = crate::R<UR5rs>;
#[doc = "Field `MESAD_1` reader - Mass erase secured area disabled for bank 1"]
pub type MESAD_1_R = crate::BitReader;
#[doc = "Field `WRPN_1` reader - Write protection for flash bank 1"]
pub type WRPN_1_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Mass erase secured area disabled for bank 1"]
    #[inline(always)]
    pub fn mesad_1(&self) -> MESAD_1_R {
        MESAD_1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - Write protection for flash bank 1"]
    #[inline(always)]
    pub fn wrpn_1(&self) -> WRPN_1_R {
        WRPN_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "SYSCFG user register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR5rs;
impl crate::RegisterSpec for UR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur5::R`](R) reader structure"]
impl crate::Readable for UR5rs {}
#[doc = "`reset()` method sets UR5 to value 0"]
impl crate::Resettable for UR5rs {
    const RESET_VALUE: u32 = 0;
}
