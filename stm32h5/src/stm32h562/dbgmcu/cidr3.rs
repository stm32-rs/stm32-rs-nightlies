#[doc = "Register `CIDR3` reader"]
pub type R = crate::R<CIDR3rs>;
#[doc = "Field `PREAMBLE` reader - component identification bits \\[31:24\\]"]
pub type PREAMBLE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - component identification bits \\[31:24\\]"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DBGMCU CoreSight component identity register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIDR3rs;
impl crate::RegisterSpec for CIDR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr3::R`](R) reader structure"]
impl crate::Readable for CIDR3rs {}
#[doc = "`reset()` method sets CIDR3 to value 0xb1"]
impl crate::Resettable for CIDR3rs {
    const RESET_VALUE: u32 = 0xb1;
}
