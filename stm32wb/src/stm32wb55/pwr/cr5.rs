#[doc = "Register `CR5` reader"]
pub type R = crate::R<CR5rs>;
#[doc = "Register `CR5` writer"]
pub type W = crate::W<CR5rs>;
#[doc = "Field `SDVOS` reader - Step Down converter voltage output scaling"]
pub type SDVOS_R = crate::FieldReader;
#[doc = "Field `SDVOS` writer - Step Down converter voltage output scaling"]
pub type SDVOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDSC` reader - Step Down converter supplt startup current selection"]
pub type SDSC_R = crate::FieldReader;
#[doc = "Field `SDSC` writer - Step Down converter supplt startup current selection"]
pub type SDSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BORHC` reader - BORH configuration selection"]
pub type BORHC_R = crate::BitReader;
#[doc = "Field `BORHC` writer - BORH configuration selection"]
pub type BORHC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMPSCFG` reader - VOS configuration selection (non user)"]
pub type SMPSCFG_R = crate::BitReader;
#[doc = "Field `SMPSCFG` writer - VOS configuration selection (non user)"]
pub type SMPSCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDBEN` reader - Enable Step Down converter Bypass mode enabled"]
pub type SDBEN_R = crate::BitReader;
#[doc = "Field `SDBEN` writer - Enable Step Down converter Bypass mode enabled"]
pub type SDBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDEB` reader - Enable Step Down converter SMPS mode enabled"]
pub type SDEB_R = crate::BitReader;
#[doc = "Field `SDEB` writer - Enable Step Down converter SMPS mode enabled"]
pub type SDEB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Step Down converter voltage output scaling"]
    #[inline(always)]
    pub fn sdvos(&self) -> SDVOS_R {
        SDVOS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Step Down converter supplt startup current selection"]
    #[inline(always)]
    pub fn sdsc(&self) -> SDSC_R {
        SDSC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - BORH configuration selection"]
    #[inline(always)]
    pub fn borhc(&self) -> BORHC_R {
        BORHC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VOS configuration selection (non user)"]
    #[inline(always)]
    pub fn smpscfg(&self) -> SMPSCFG_R {
        SMPSCFG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Step Down converter Bypass mode enabled"]
    #[inline(always)]
    pub fn sdben(&self) -> SDBEN_R {
        SDBEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Step Down converter SMPS mode enabled"]
    #[inline(always)]
    pub fn sdeb(&self) -> SDEB_R {
        SDEB_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Step Down converter voltage output scaling"]
    #[inline(always)]
    #[must_use]
    pub fn sdvos(&mut self) -> SDVOS_W<CR5rs> {
        SDVOS_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Step Down converter supplt startup current selection"]
    #[inline(always)]
    #[must_use]
    pub fn sdsc(&mut self) -> SDSC_W<CR5rs> {
        SDSC_W::new(self, 4)
    }
    #[doc = "Bit 8 - BORH configuration selection"]
    #[inline(always)]
    #[must_use]
    pub fn borhc(&mut self) -> BORHC_W<CR5rs> {
        BORHC_W::new(self, 8)
    }
    #[doc = "Bit 9 - VOS configuration selection (non user)"]
    #[inline(always)]
    #[must_use]
    pub fn smpscfg(&mut self) -> SMPSCFG_W<CR5rs> {
        SMPSCFG_W::new(self, 9)
    }
    #[doc = "Bit 14 - Enable Step Down converter Bypass mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sdben(&mut self) -> SDBEN_W<CR5rs> {
        SDBEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Step Down converter SMPS mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sdeb(&mut self) -> SDEB_W<CR5rs> {
        SDEB_W::new(self, 15)
    }
}
#[doc = "Power control register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR5rs;
impl crate::RegisterSpec for CR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr5::R`](R) reader structure"]
impl crate::Readable for CR5rs {}
#[doc = "`write(|w| ..)` method takes [`cr5::W`](W) writer structure"]
impl crate::Writable for CR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR5 to value 0x4270"]
impl crate::Resettable for CR5rs {
    const RESET_VALUE: u32 = 0x4270;
}
