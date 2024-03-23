#[doc = "Register `CIDR2` reader"]
pub type R = crate::R<CIDR2rs>;
#[doc = "Field `PREAMBLE` reader - component identification bits \\[23:16\\]"]
pub type PREAMBLE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - component identification bits \\[23:16\\]"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DBGMCU CoreSight component identity register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIDR2rs;
impl crate::RegisterSpec for CIDR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr2::R`](R) reader structure"]
impl crate::Readable for CIDR2rs {}
#[doc = "`reset()` method sets CIDR2 to value 0x05"]
impl crate::Resettable for CIDR2rs {
    const RESET_VALUE: u32 = 0x05;
}
