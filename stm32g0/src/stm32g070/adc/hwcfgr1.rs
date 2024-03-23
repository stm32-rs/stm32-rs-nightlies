#[doc = "Register `HWCFGR1` reader"]
pub type R = crate::R<HWCFGR1rs>;
#[doc = "Register `HWCFGR1` writer"]
pub type W = crate::W<HWCFGR1rs>;
#[doc = "Field `CHMAP3` reader - Input channel mapping"]
pub type CHMAP3_R = crate::FieldReader;
#[doc = "Field `CHMAP3` writer - Input channel mapping"]
pub type CHMAP3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CHMAP2` reader - Input channel mapping"]
pub type CHMAP2_R = crate::FieldReader;
#[doc = "Field `CHMAP2` writer - Input channel mapping"]
pub type CHMAP2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CHMAP1` reader - Input channel mapping"]
pub type CHMAP1_R = crate::FieldReader;
#[doc = "Field `CHMAP1` writer - Input channel mapping"]
pub type CHMAP1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CHMAP0` reader - Input channel mapping"]
pub type CHMAP0_R = crate::FieldReader;
#[doc = "Field `CHMAP0` writer - Input channel mapping"]
pub type CHMAP0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap3(&self) -> CHMAP3_R {
        CHMAP3_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap2(&self) -> CHMAP2_R {
        CHMAP2_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap1(&self) -> CHMAP1_R {
        CHMAP1_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap0(&self) -> CHMAP0_R {
        CHMAP0_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap3(&mut self) -> CHMAP3_W<HWCFGR1rs> {
        CHMAP3_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap2(&mut self) -> CHMAP2_W<HWCFGR1rs> {
        CHMAP2_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap1(&mut self) -> CHMAP1_W<HWCFGR1rs> {
        CHMAP1_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    #[must_use]
    pub fn chmap0(&mut self) -> CHMAP0_W<HWCFGR1rs> {
        CHMAP0_W::new(self, 24)
    }
}
#[doc = "Hardware Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR1rs;
impl crate::RegisterSpec for HWCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr1::R`](R) reader structure"]
impl crate::Readable for HWCFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`hwcfgr1::W`](W) writer structure"]
impl crate::Writable for HWCFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWCFGR1 to value 0x0302_0100"]
impl crate::Resettable for HWCFGR1rs {
    const RESET_VALUE: u32 = 0x0302_0100;
}
