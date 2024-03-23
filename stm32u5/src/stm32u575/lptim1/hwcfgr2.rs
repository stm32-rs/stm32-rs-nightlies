#[doc = "Register `HWCFGR2` reader"]
pub type R = crate::R<HWCFGR2rs>;
#[doc = "Field `CFG1` reader - peripheral hardware configuration 1"]
pub type CFG1_R = crate::FieldReader;
#[doc = "Field `CFG2` reader - peripheral hardware configuration 2"]
pub type CFG2_R = crate::FieldReader;
#[doc = "Field `CFG3` reader - peripheral hardware configuration 3"]
pub type CFG3_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - peripheral hardware configuration 1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - peripheral hardware configuration 2"]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - peripheral hardware configuration 3"]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "LPTIM peripheral hardware configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR2rs;
impl crate::RegisterSpec for HWCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr2::R`](R) reader structure"]
impl crate::Readable for HWCFGR2rs {}
#[doc = "`reset()` method sets HWCFGR2 to value 0"]
impl crate::Resettable for HWCFGR2rs {
    const RESET_VALUE: u32 = 0;
}
