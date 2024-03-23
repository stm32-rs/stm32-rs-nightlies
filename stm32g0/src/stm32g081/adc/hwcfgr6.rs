#[doc = "Register `HWCFGR6` reader"]
pub type R = crate::R<HWCFGR6rs>;
#[doc = "Register `HWCFGR6` writer"]
pub type W = crate::W<HWCFGR6rs>;
#[doc = "Field `CHMAP20` reader - Input channel mapping"]
pub type CHMAP20_R = crate::FieldReader;
#[doc = "Field `CHMAP20` writer - Input channel mapping"]
pub type CHMAP20_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CHMAP21` reader - Input channel mapping"]
pub type CHMAP21_R = crate::FieldReader;
#[doc = "Field `CHMAP21` writer - Input channel mapping"]
pub type CHMAP21_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CHMAP22` reader - Input channel mapping"]
pub type CHMAP22_R = crate::FieldReader;
#[doc = "Field `CHMAP22` writer - Input channel mapping"]
pub type CHMAP22_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CHMAP23` reader - Input channel mapping"]
pub type CHMAP23_R = crate::FieldReader;
#[doc = "Field `CHMAP23` writer - Input channel mapping"]
pub type CHMAP23_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap20(&self) -> CHMAP20_R {
        CHMAP20_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap21(&self) -> CHMAP21_R {
        CHMAP21_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap22(&self) -> CHMAP22_R {
        CHMAP22_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap23(&self) -> CHMAP23_R {
        CHMAP23_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap20(&mut self) -> CHMAP20_W<HWCFGR6rs> {
        CHMAP20_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap21(&mut self) -> CHMAP21_W<HWCFGR6rs> {
        CHMAP21_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap22(&mut self) -> CHMAP22_W<HWCFGR6rs> {
        CHMAP22_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap23(&mut self) -> CHMAP23_W<HWCFGR6rs> {
        CHMAP23_W::new(self, 24)
    }
}
#[doc = "Hardware Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR6rs;
impl crate::RegisterSpec for HWCFGR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr6::R`](R) reader structure"]
impl crate::Readable for HWCFGR6rs {}
#[doc = "`write(|w| ..)` method takes [`hwcfgr6::W`](W) writer structure"]
impl crate::Writable for HWCFGR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWCFGR6 to value 0x1f1f_1f1f"]
impl crate::Resettable for HWCFGR6rs {
    const RESET_VALUE: u32 = 0x1f1f_1f1f;
}
