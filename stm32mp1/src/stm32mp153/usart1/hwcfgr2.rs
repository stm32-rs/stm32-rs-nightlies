#[doc = "Register `HWCFGR2` reader"]
pub type R = crate::R<HWCFGR2rs>;
#[doc = "Field `CFG1` reader - CFG1"]
pub type CFG1_R = crate::FieldReader;
#[doc = "Field `CFG2` reader - CFG2"]
pub type CFG2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - CFG1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CFG2"]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "USART Hardware Configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR2rs;
impl crate::RegisterSpec for HWCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr2::R`](R) reader structure"]
impl crate::Readable for HWCFGR2rs {}
#[doc = "`reset()` method sets HWCFGR2 to value 0x14"]
impl crate::Resettable for HWCFGR2rs {
    const RESET_VALUE: u32 = 0x14;
}
