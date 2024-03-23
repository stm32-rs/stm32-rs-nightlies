#[doc = "Register `HWCFGR1` reader"]
pub type R = crate::R<HWCFGR1rs>;
#[doc = "Field `CFG1` reader - peripheral hardware configuration 1"]
pub type CFG1_R = crate::FieldReader;
#[doc = "Field `CFG2` reader - peripheral hardware configuration 2"]
pub type CFG2_R = crate::FieldReader;
#[doc = "Field `CFG3` reader - peripheral hardware configuration 3"]
pub type CFG3_R = crate::FieldReader;
#[doc = "Field `CFG4` reader - peripheral hardware configuration 4"]
pub type CFG4_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - peripheral hardware configuration 1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - peripheral hardware configuration 2"]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - peripheral hardware configuration 3"]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - peripheral hardware configuration 4"]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "LPTIM peripheral hardware configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR1rs;
impl crate::RegisterSpec for HWCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr1::R`](R) reader structure"]
impl crate::Readable for HWCFGR1rs {}
#[doc = "`reset()` method sets HWCFGR1 to value 0"]
impl crate::Resettable for HWCFGR1rs {
    const RESET_VALUE: u32 = 0;
}
