#[doc = "Register `HWCFGR2` reader"]
pub type R = crate::R<HWCFGR2rs>;
#[doc = "Register `HWCFGR2` writer"]
pub type W = crate::W<HWCFGR2rs>;
#[doc = "Field `CHMAP7` reader - Input channel mapping"]
pub type CHMAP7_R = crate::FieldReader;
#[doc = "Field `CHMAP7` writer - Input channel mapping"]
pub type CHMAP7_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CHMAP6` reader - Input channel mapping"]
pub type CHMAP6_R = crate::FieldReader;
#[doc = "Field `CHMAP6` writer - Input channel mapping"]
pub type CHMAP6_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CHMAP5` reader - Input channel mapping"]
pub type CHMAP5_R = crate::FieldReader;
#[doc = "Field `CHMAP5` writer - Input channel mapping"]
pub type CHMAP5_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CHMAP4` reader - Input channel mapping"]
pub type CHMAP4_R = crate::FieldReader;
#[doc = "Field `CHMAP4` writer - Input channel mapping"]
pub type CHMAP4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap7(&self) -> CHMAP7_R {
        CHMAP7_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap6(&self) -> CHMAP6_R {
        CHMAP6_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap5(&self) -> CHMAP5_R {
        CHMAP5_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap4(&self) -> CHMAP4_R {
        CHMAP4_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap7(&mut self) -> CHMAP7_W<HWCFGR2rs> {
        CHMAP7_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap6(&mut self) -> CHMAP6_W<HWCFGR2rs> {
        CHMAP6_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap5(&mut self) -> CHMAP5_W<HWCFGR2rs> {
        CHMAP5_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap4(&mut self) -> CHMAP4_W<HWCFGR2rs> {
        CHMAP4_W::new(self, 24)
    }
}
#[doc = "Hardware Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR2rs;
impl crate::RegisterSpec for HWCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr2::R`](R) reader structure"]
impl crate::Readable for HWCFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`hwcfgr2::W`](W) writer structure"]
impl crate::Writable for HWCFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWCFGR2 to value 0x0505_0404"]
impl crate::Resettable for HWCFGR2rs {
    const RESET_VALUE: u32 = 0x0505_0404;
}
