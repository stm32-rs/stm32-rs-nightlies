#[doc = "Register `GPIOA_HWCFGR8` reader"]
pub type R = crate::R<GPIOA_HWCFGR8rs>;
#[doc = "Field `AF_PRIO8` reader - AF_PRIO8"]
pub type AF_PRIO8_R = crate::FieldReader;
#[doc = "Field `AF_PRIO9` reader - AF_PRIO9"]
pub type AF_PRIO9_R = crate::FieldReader;
#[doc = "Field `AF_PRIO10` reader - AF_PRIO10"]
pub type AF_PRIO10_R = crate::FieldReader;
#[doc = "Field `AF_PRIO11` reader - AF_PRIO11"]
pub type AF_PRIO11_R = crate::FieldReader;
#[doc = "Field `AF_PRIO12` reader - AF_PRIO12"]
pub type AF_PRIO12_R = crate::FieldReader;
#[doc = "Field `AF_PRIO13` reader - AF_PRIO13"]
pub type AF_PRIO13_R = crate::FieldReader;
#[doc = "Field `AF_PRIO14` reader - AF_PRIO14"]
pub type AF_PRIO14_R = crate::FieldReader;
#[doc = "Field `AF_PRIO15` reader - AF_PRIO15"]
pub type AF_PRIO15_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - AF_PRIO8"]
    #[inline(always)]
    pub fn af_prio8(&self) -> AF_PRIO8_R {
        AF_PRIO8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AF_PRIO9"]
    #[inline(always)]
    pub fn af_prio9(&self) -> AF_PRIO9_R {
        AF_PRIO9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - AF_PRIO10"]
    #[inline(always)]
    pub fn af_prio10(&self) -> AF_PRIO10_R {
        AF_PRIO10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - AF_PRIO11"]
    #[inline(always)]
    pub fn af_prio11(&self) -> AF_PRIO11_R {
        AF_PRIO11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - AF_PRIO12"]
    #[inline(always)]
    pub fn af_prio12(&self) -> AF_PRIO12_R {
        AF_PRIO12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - AF_PRIO13"]
    #[inline(always)]
    pub fn af_prio13(&self) -> AF_PRIO13_R {
        AF_PRIO13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - AF_PRIO14"]
    #[inline(always)]
    pub fn af_prio14(&self) -> AF_PRIO14_R {
        AF_PRIO14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - AF_PRIO15"]
    #[inline(always)]
    pub fn af_prio15(&self) -> AF_PRIO15_R {
        AF_PRIO15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioa_hwcfgr8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOA_HWCFGR8rs;
impl crate::RegisterSpec for GPIOA_HWCFGR8rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioa_hwcfgr8::R`](R) reader structure"]
impl crate::Readable for GPIOA_HWCFGR8rs {}
#[doc = "`reset()` method sets GPIOA_HWCFGR8 to value 0"]
impl crate::Resettable for GPIOA_HWCFGR8rs {
    const RESET_VALUE: u32 = 0;
}
