#[doc = "Register `HWCFGR1` reader"]
pub type R = crate::R<HWCFGR1rs>;
#[doc = "Register `HWCFGR1` writer"]
pub type W = crate::W<HWCFGR1rs>;
#[doc = "Field `CFG1` reader - LUART hardware configuration 1"]
pub type CFG1_R = crate::FieldReader;
#[doc = "Field `CFG1` writer - LUART hardware configuration 1"]
pub type CFG1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG2` reader - LUART hardware configuration 2"]
pub type CFG2_R = crate::FieldReader;
#[doc = "Field `CFG2` writer - LUART hardware configuration 2"]
pub type CFG2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG3` reader - LUART hardware configuration 1"]
pub type CFG3_R = crate::FieldReader;
#[doc = "Field `CFG3` writer - LUART hardware configuration 1"]
pub type CFG3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG4` reader - LUART hardware configuration 2"]
pub type CFG4_R = crate::FieldReader;
#[doc = "Field `CFG4` writer - LUART hardware configuration 2"]
pub type CFG4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG5` reader - LUART hardware configuration 2"]
pub type CFG5_R = crate::FieldReader;
#[doc = "Field `CFG5` writer - LUART hardware configuration 2"]
pub type CFG5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG6` reader - LUART hardware configuration 2"]
pub type CFG6_R = crate::FieldReader;
#[doc = "Field `CFG6` writer - LUART hardware configuration 2"]
pub type CFG6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG7` reader - LUART hardware configuration 2"]
pub type CFG7_R = crate::FieldReader;
#[doc = "Field `CFG7` writer - LUART hardware configuration 2"]
pub type CFG7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG8` reader - LUART hardware configuration 2"]
pub type CFG8_R = crate::FieldReader;
#[doc = "Field `CFG8` writer - LUART hardware configuration 2"]
pub type CFG8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    #[doc = "Bits 8:11 - LUART hardware configuration 1"]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg5(&self) -> CFG5_R {
        CFG5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg6(&self) -> CFG6_R {
        CFG6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg7(&self) -> CFG7_R {
        CFG7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg8(&self) -> CFG8_R {
        CFG8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - LUART hardware configuration 1"]
    #[inline(always)]
    #[must_use]
    pub fn cfg1(&mut self) -> CFG1_W<HWCFGR1rs> {
        CFG1_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - LUART hardware configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn cfg2(&mut self) -> CFG2_W<HWCFGR1rs> {
        CFG2_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - LUART hardware configuration 1"]
    #[inline(always)]
    #[must_use]
    pub fn cfg3(&mut self) -> CFG3_W<HWCFGR1rs> {
        CFG3_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - LUART hardware configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn cfg4(&mut self) -> CFG4_W<HWCFGR1rs> {
        CFG4_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - LUART hardware configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn cfg5(&mut self) -> CFG5_W<HWCFGR1rs> {
        CFG5_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - LUART hardware configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn cfg6(&mut self) -> CFG6_W<HWCFGR1rs> {
        CFG6_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - LUART hardware configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn cfg7(&mut self) -> CFG7_W<HWCFGR1rs> {
        CFG7_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - LUART hardware configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn cfg8(&mut self) -> CFG8_W<HWCFGR1rs> {
        CFG8_W::new(self, 28)
    }
}
#[doc = "LPUART Hardware Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwcfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets HWCFGR1 to value 0x3110_0000"]
impl crate::Resettable for HWCFGR1rs {
    const RESET_VALUE: u32 = 0x3110_0000;
}
