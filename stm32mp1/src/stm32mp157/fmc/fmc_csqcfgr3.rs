#[doc = "Register `FMC_CSQCFGR3` reader"]
pub type R = crate::R<FMC_CSQCFGR3rs>;
#[doc = "Register `FMC_CSQCFGR3` writer"]
pub type W = crate::W<FMC_CSQCFGR3rs>;
#[doc = "Field `SNBR` reader - SNBR"]
pub type SNBR_R = crate::FieldReader;
#[doc = "Field `SNBR` writer - SNBR"]
pub type SNBR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `AC1T` reader - AC1T"]
pub type AC1T_R = crate::BitReader;
#[doc = "Field `AC1T` writer - AC1T"]
pub type AC1T_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AC2T` reader - AC2T"]
pub type AC2T_R = crate::BitReader;
#[doc = "Field `AC2T` writer - AC2T"]
pub type AC2T_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AC3T` reader - AC3T"]
pub type AC3T_R = crate::BitReader;
#[doc = "Field `AC3T` writer - AC3T"]
pub type AC3T_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AC4T` reader - AC4T"]
pub type AC4T_R = crate::BitReader;
#[doc = "Field `AC4T` writer - AC4T"]
pub type AC4T_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AC5T` reader - AC5T"]
pub type AC5T_R = crate::BitReader;
#[doc = "Field `AC5T` writer - AC5T"]
pub type AC5T_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDT` reader - SDT"]
pub type SDT_R = crate::BitReader;
#[doc = "Field `SDT` writer - SDT"]
pub type SDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAC1T` reader - RAC1T"]
pub type RAC1T_R = crate::BitReader;
#[doc = "Field `RAC1T` writer - RAC1T"]
pub type RAC1T_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAC2T` reader - RAC2T"]
pub type RAC2T_R = crate::BitReader;
#[doc = "Field `RAC2T` writer - RAC2T"]
pub type RAC2T_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:13 - SNBR"]
    #[inline(always)]
    pub fn snbr(&self) -> SNBR_R {
        SNBR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - AC1T"]
    #[inline(always)]
    pub fn ac1t(&self) -> AC1T_R {
        AC1T_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AC2T"]
    #[inline(always)]
    pub fn ac2t(&self) -> AC2T_R {
        AC2T_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - AC3T"]
    #[inline(always)]
    pub fn ac3t(&self) -> AC3T_R {
        AC3T_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - AC4T"]
    #[inline(always)]
    pub fn ac4t(&self) -> AC4T_R {
        AC4T_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - AC5T"]
    #[inline(always)]
    pub fn ac5t(&self) -> AC5T_R {
        AC5T_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SDT"]
    #[inline(always)]
    pub fn sdt(&self) -> SDT_R {
        SDT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - RAC1T"]
    #[inline(always)]
    pub fn rac1t(&self) -> RAC1T_R {
        RAC1T_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - RAC2T"]
    #[inline(always)]
    pub fn rac2t(&self) -> RAC2T_R {
        RAC2T_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:13 - SNBR"]
    #[inline(always)]
    #[must_use]
    pub fn snbr(&mut self) -> SNBR_W<FMC_CSQCFGR3rs> {
        SNBR_W::new(self, 8)
    }
    #[doc = "Bit 16 - AC1T"]
    #[inline(always)]
    #[must_use]
    pub fn ac1t(&mut self) -> AC1T_W<FMC_CSQCFGR3rs> {
        AC1T_W::new(self, 16)
    }
    #[doc = "Bit 17 - AC2T"]
    #[inline(always)]
    #[must_use]
    pub fn ac2t(&mut self) -> AC2T_W<FMC_CSQCFGR3rs> {
        AC2T_W::new(self, 17)
    }
    #[doc = "Bit 18 - AC3T"]
    #[inline(always)]
    #[must_use]
    pub fn ac3t(&mut self) -> AC3T_W<FMC_CSQCFGR3rs> {
        AC3T_W::new(self, 18)
    }
    #[doc = "Bit 19 - AC4T"]
    #[inline(always)]
    #[must_use]
    pub fn ac4t(&mut self) -> AC4T_W<FMC_CSQCFGR3rs> {
        AC4T_W::new(self, 19)
    }
    #[doc = "Bit 20 - AC5T"]
    #[inline(always)]
    #[must_use]
    pub fn ac5t(&mut self) -> AC5T_W<FMC_CSQCFGR3rs> {
        AC5T_W::new(self, 20)
    }
    #[doc = "Bit 21 - SDT"]
    #[inline(always)]
    #[must_use]
    pub fn sdt(&mut self) -> SDT_W<FMC_CSQCFGR3rs> {
        SDT_W::new(self, 21)
    }
    #[doc = "Bit 22 - RAC1T"]
    #[inline(always)]
    #[must_use]
    pub fn rac1t(&mut self) -> RAC1T_W<FMC_CSQCFGR3rs> {
        RAC1T_W::new(self, 22)
    }
    #[doc = "Bit 23 - RAC2T"]
    #[inline(always)]
    #[must_use]
    pub fn rac2t(&mut self) -> RAC2T_W<FMC_CSQCFGR3rs> {
        RAC2T_W::new(self, 23)
    }
}
#[doc = "FMC NAND sequencer configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_csqcfgr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_csqcfgr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_CSQCFGR3rs;
impl crate::RegisterSpec for FMC_CSQCFGR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_csqcfgr3::R`](R) reader structure"]
impl crate::Readable for FMC_CSQCFGR3rs {}
#[doc = "`write(|w| ..)` method takes [`fmc_csqcfgr3::W`](W) writer structure"]
impl crate::Writable for FMC_CSQCFGR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMC_CSQCFGR3 to value 0"]
impl crate::Resettable for FMC_CSQCFGR3rs {
    const RESET_VALUE: u32 = 0;
}
