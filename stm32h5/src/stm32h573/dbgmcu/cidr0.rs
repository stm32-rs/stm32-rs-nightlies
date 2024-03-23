#[doc = "Register `CIDR0` reader"]
pub type R = crate::R<CIDR0rs>;
#[doc = "Field `PREAMBLE` reader - component identification bits \\[7:0\\]"]
pub type PREAMBLE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - component identification bits \\[7:0\\]"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DBGMCU CoreSight component identity register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIDR0rs;
impl crate::RegisterSpec for CIDR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr0::R`](R) reader structure"]
impl crate::Readable for CIDR0rs {}
#[doc = "`reset()` method sets CIDR0 to value 0x0d"]
impl crate::Resettable for CIDR0rs {
    const RESET_VALUE: u32 = 0x0d;
}
