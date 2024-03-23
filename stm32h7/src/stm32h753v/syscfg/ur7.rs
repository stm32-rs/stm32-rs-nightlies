#[doc = "Register `UR7` reader"]
pub type R = crate::R<UR7rs>;
#[doc = "Field `SA_BEG_1` reader - Secured area start address for bank 1"]
pub type SA_BEG_1_R = crate::FieldReader<u16>;
#[doc = "Field `SA_END_1` reader - Secured area end address for bank 1"]
pub type SA_END_1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Secured area start address for bank 1"]
    #[inline(always)]
    pub fn sa_beg_1(&self) -> SA_BEG_1_R {
        SA_BEG_1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Secured area end address for bank 1"]
    #[inline(always)]
    pub fn sa_end_1(&self) -> SA_END_1_R {
        SA_END_1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "SYSCFG user register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR7rs;
impl crate::RegisterSpec for UR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur7::R`](R) reader structure"]
impl crate::Readable for UR7rs {}
#[doc = "`reset()` method sets UR7 to value 0"]
impl crate::Resettable for UR7rs {
    const RESET_VALUE: u32 = 0;
}
