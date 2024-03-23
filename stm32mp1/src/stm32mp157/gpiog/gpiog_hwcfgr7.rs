#[doc = "Register `GPIOG_HWCFGR7` reader"]
pub type R = crate::R<GPIOG_HWCFGR7rs>;
#[doc = "Field `AF_PRIO0` reader - AF_PRIO0"]
pub type AF_PRIO0_R = crate::FieldReader;
#[doc = "Field `AF_PRIO1` reader - AF_PRIO1"]
pub type AF_PRIO1_R = crate::FieldReader;
#[doc = "Field `AF_PRIO2` reader - AF_PRIO2"]
pub type AF_PRIO2_R = crate::FieldReader;
#[doc = "Field `AF_PRIO3` reader - AF_PRIO3"]
pub type AF_PRIO3_R = crate::FieldReader;
#[doc = "Field `AF_PRIO4` reader - AF_PRIO4"]
pub type AF_PRIO4_R = crate::FieldReader;
#[doc = "Field `AF_PRIO5` reader - AF_PRIO5"]
pub type AF_PRIO5_R = crate::FieldReader;
#[doc = "Field `AF_PRIO6` reader - AF_PRIO6"]
pub type AF_PRIO6_R = crate::FieldReader;
#[doc = "Field `AF_PRIO7` reader - AF_PRIO7"]
pub type AF_PRIO7_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - AF_PRIO0"]
    #[inline(always)]
    pub fn af_prio0(&self) -> AF_PRIO0_R {
        AF_PRIO0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AF_PRIO1"]
    #[inline(always)]
    pub fn af_prio1(&self) -> AF_PRIO1_R {
        AF_PRIO1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - AF_PRIO2"]
    #[inline(always)]
    pub fn af_prio2(&self) -> AF_PRIO2_R {
        AF_PRIO2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - AF_PRIO3"]
    #[inline(always)]
    pub fn af_prio3(&self) -> AF_PRIO3_R {
        AF_PRIO3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - AF_PRIO4"]
    #[inline(always)]
    pub fn af_prio4(&self) -> AF_PRIO4_R {
        AF_PRIO4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - AF_PRIO5"]
    #[inline(always)]
    pub fn af_prio5(&self) -> AF_PRIO5_R {
        AF_PRIO5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - AF_PRIO6"]
    #[inline(always)]
    pub fn af_prio6(&self) -> AF_PRIO6_R {
        AF_PRIO6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - AF_PRIO7"]
    #[inline(always)]
    pub fn af_prio7(&self) -> AF_PRIO7_R {
        AF_PRIO7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "GPIO hardware configuration register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiog_hwcfgr7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOG_HWCFGR7rs;
impl crate::RegisterSpec for GPIOG_HWCFGR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiog_hwcfgr7::R`](R) reader structure"]
impl crate::Readable for GPIOG_HWCFGR7rs {}
#[doc = "`reset()` method sets GPIOG_HWCFGR7 to value 0xffff_ffff"]
impl crate::Resettable for GPIOG_HWCFGR7rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
