#[doc = "Register `HWCFGR5` reader"]
pub type R = crate::R<HWCFGR5rs>;
#[doc = "Register `HWCFGR5` writer"]
pub type W = crate::W<HWCFGR5rs>;
#[doc = "Field `CHMAP19` reader - Input channel mapping"]
pub type CHMAP19_R = crate::FieldReader;
#[doc = "Field `CHMAP19` writer - Input channel mapping"]
pub type CHMAP19_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CHMAP18` reader - Input channel mapping"]
pub type CHMAP18_R = crate::FieldReader;
#[doc = "Field `CHMAP18` writer - Input channel mapping"]
pub type CHMAP18_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CHMAP17` reader - Input channel mapping"]
pub type CHMAP17_R = crate::FieldReader;
#[doc = "Field `CHMAP17` writer - Input channel mapping"]
pub type CHMAP17_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CHMAP16` reader - Input channel mapping"]
pub type CHMAP16_R = crate::FieldReader;
#[doc = "Field `CHMAP16` writer - Input channel mapping"]
pub type CHMAP16_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap19(&self) -> CHMAP19_R {
        CHMAP19_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap18(&self) -> CHMAP18_R {
        CHMAP18_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap17(&self) -> CHMAP17_R {
        CHMAP17_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap16(&self) -> CHMAP16_R {
        CHMAP16_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap19(&mut self) -> CHMAP19_W<HWCFGR5rs> {
        CHMAP19_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap18(&mut self) -> CHMAP18_W<HWCFGR5rs> {
        CHMAP18_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap17(&mut self) -> CHMAP17_W<HWCFGR5rs> {
        CHMAP17_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap16(&mut self) -> CHMAP16_W<HWCFGR5rs> {
        CHMAP16_W::new(self, 24)
    }
}
#[doc = "Hardware Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR5rs;
impl crate::RegisterSpec for HWCFGR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr5::R`](R) reader structure"]
impl crate::Readable for HWCFGR5rs {}
#[doc = "`write(|w| ..)` method takes [`hwcfgr5::W`](W) writer structure"]
impl crate::Writable for HWCFGR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWCFGR5 to value 0x1f08_0807"]
impl crate::Resettable for HWCFGR5rs {
    const RESET_VALUE: u32 = 0x1f08_0807;
}
