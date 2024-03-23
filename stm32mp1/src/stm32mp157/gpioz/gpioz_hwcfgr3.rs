#[doc = "Register `GPIOZ_HWCFGR3` reader"]
pub type R = crate::R<GPIOZ_HWCFGR3rs>;
#[doc = "Field `ODR_RES` reader - ODR_RES"]
pub type ODR_RES_R = crate::FieldReader<u16>;
#[doc = "Field `OTYPER_RES` reader - OTYPER_RES"]
pub type OTYPER_RES_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - ODR_RES"]
    #[inline(always)]
    pub fn odr_res(&self) -> ODR_RES_R {
        ODR_RES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OTYPER_RES"]
    #[inline(always)]
    pub fn otyper_res(&self) -> OTYPER_RES_R {
        OTYPER_RES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "GPIO hardware configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_hwcfgr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOZ_HWCFGR3rs;
impl crate::RegisterSpec for GPIOZ_HWCFGR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioz_hwcfgr3::R`](R) reader structure"]
impl crate::Readable for GPIOZ_HWCFGR3rs {}
#[doc = "`reset()` method sets GPIOZ_HWCFGR3 to value 0"]
impl crate::Resettable for GPIOZ_HWCFGR3rs {
    const RESET_VALUE: u32 = 0;
}
