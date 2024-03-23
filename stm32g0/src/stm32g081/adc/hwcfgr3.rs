#[doc = "Register `HWCFGR3` reader"]
pub type R = crate::R<HWCFGR3rs>;
#[doc = "Register `HWCFGR3` writer"]
pub type W = crate::W<HWCFGR3rs>;
#[doc = "Field `CHMAP11` reader - Input channel mapping"]
pub type CHMAP11_R = crate::FieldReader;
#[doc = "Field `CHMAP11` writer - Input channel mapping"]
pub type CHMAP11_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CHMAP10` reader - Input channel mapping"]
pub type CHMAP10_R = crate::FieldReader;
#[doc = "Field `CHMAP10` writer - Input channel mapping"]
pub type CHMAP10_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CHMAP9` reader - Input channel mapping"]
pub type CHMAP9_R = crate::FieldReader;
#[doc = "Field `CHMAP9` writer - Input channel mapping"]
pub type CHMAP9_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CHMAP8` reader - Input channel mapping"]
pub type CHMAP8_R = crate::FieldReader;
#[doc = "Field `CHMAP8` writer - Input channel mapping"]
pub type CHMAP8_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap11(&self) -> CHMAP11_R {
        CHMAP11_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap10(&self) -> CHMAP10_R {
        CHMAP10_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap9(&self) -> CHMAP9_R {
        CHMAP9_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap8(&self) -> CHMAP8_R {
        CHMAP8_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap11(&mut self) -> CHMAP11_W<HWCFGR3rs> {
        CHMAP11_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap10(&mut self) -> CHMAP10_W<HWCFGR3rs> {
        CHMAP10_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap9(&mut self) -> CHMAP9_W<HWCFGR3rs> {
        CHMAP9_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap8(&mut self) -> CHMAP8_W<HWCFGR3rs> {
        CHMAP8_W::new(self, 24)
    }
}
#[doc = "Hardware Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR3rs;
impl crate::RegisterSpec for HWCFGR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr3::R`](R) reader structure"]
impl crate::Readable for HWCFGR3rs {}
#[doc = "`write(|w| ..)` method takes [`hwcfgr3::W`](W) writer structure"]
impl crate::Writable for HWCFGR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWCFGR3 to value 0x0706_0605"]
impl crate::Resettable for HWCFGR3rs {
    const RESET_VALUE: u32 = 0x0706_0605;
}
