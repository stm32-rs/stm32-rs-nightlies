#[doc = "Register `HWCFGR2` reader"]
pub type R = crate::R<HWCFGR2rs>;
#[doc = "Register `HWCFGR2` writer"]
pub type W = crate::W<HWCFGR2rs>;
#[doc = "Field `CFG1` reader - LUART hardware configuration 1"]
pub type CFG1_R = crate::FieldReader;
#[doc = "Field `CFG1` writer - LUART hardware configuration 1"]
pub type CFG1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG2` reader - LUART hardware configuration 2"]
pub type CFG2_R = crate::FieldReader;
#[doc = "Field `CFG2` writer - LUART hardware configuration 2"]
pub type CFG2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - LUART hardware configuration 1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - LUART hardware configuration 1"]
    #[inline(always)]
    #[must_use]
    pub fn cfg1(&mut self) -> CFG1_W<HWCFGR2rs> {
        CFG1_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - LUART hardware configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn cfg2(&mut self) -> CFG2_W<HWCFGR2rs> {
        CFG2_W::new(self, 4)
    }
}
#[doc = "LPUART Hardware Configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets HWCFGR2 to value 0x13"]
impl crate::Resettable for HWCFGR2rs {
    const RESET_VALUE: u32 = 0x13;
}
