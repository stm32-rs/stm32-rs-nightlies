#[doc = "Register `FMC_CSQCFGR1` reader"]
pub type R = crate::R<FMC_CSQCFGR1rs>;
#[doc = "Register `FMC_CSQCFGR1` writer"]
pub type W = crate::W<FMC_CSQCFGR1rs>;
#[doc = "Field `CMD2EN` reader - CMD2EN"]
pub type CMD2EN_R = crate::BitReader;
#[doc = "Field `CMD2EN` writer - CMD2EN"]
pub type CMD2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMADEN` reader - DMADEN"]
pub type DMADEN_R = crate::BitReader;
#[doc = "Field `DMADEN` writer - DMADEN"]
pub type DMADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACYNBR` reader - ACYNBR"]
pub type ACYNBR_R = crate::FieldReader;
#[doc = "Field `ACYNBR` writer - ACYNBR"]
pub type ACYNBR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMD1` reader - CMD1"]
pub type CMD1_R = crate::FieldReader;
#[doc = "Field `CMD1` writer - CMD1"]
pub type CMD1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMD2` reader - CMD2"]
pub type CMD2_R = crate::FieldReader;
#[doc = "Field `CMD2` writer - CMD2"]
pub type CMD2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMD1T` reader - CMD1T"]
pub type CMD1T_R = crate::BitReader;
#[doc = "Field `CMD1T` writer - CMD1T"]
pub type CMD1T_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD2T` reader - CMD2T"]
pub type CMD2T_R = crate::BitReader;
#[doc = "Field `CMD2T` writer - CMD2T"]
pub type CMD2T_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - CMD2EN"]
    #[inline(always)]
    pub fn cmd2en(&self) -> CMD2EN_R {
        CMD2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMADEN"]
    #[inline(always)]
    pub fn dmaden(&self) -> DMADEN_R {
        DMADEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - ACYNBR"]
    #[inline(always)]
    pub fn acynbr(&self) -> ACYNBR_R {
        ACYNBR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:15 - CMD1"]
    #[inline(always)]
    pub fn cmd1(&self) -> CMD1_R {
        CMD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CMD2"]
    #[inline(always)]
    pub fn cmd2(&self) -> CMD2_R {
        CMD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - CMD1T"]
    #[inline(always)]
    pub fn cmd1t(&self) -> CMD1T_R {
        CMD1T_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CMD2T"]
    #[inline(always)]
    pub fn cmd2t(&self) -> CMD2T_R {
        CMD2T_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CMD2EN"]
    #[inline(always)]
    #[must_use]
    pub fn cmd2en(&mut self) -> CMD2EN_W<FMC_CSQCFGR1rs> {
        CMD2EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMADEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaden(&mut self) -> DMADEN_W<FMC_CSQCFGR1rs> {
        DMADEN_W::new(self, 2)
    }
    #[doc = "Bits 4:6 - ACYNBR"]
    #[inline(always)]
    #[must_use]
    pub fn acynbr(&mut self) -> ACYNBR_W<FMC_CSQCFGR1rs> {
        ACYNBR_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - CMD1"]
    #[inline(always)]
    #[must_use]
    pub fn cmd1(&mut self) -> CMD1_W<FMC_CSQCFGR1rs> {
        CMD1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - CMD2"]
    #[inline(always)]
    #[must_use]
    pub fn cmd2(&mut self) -> CMD2_W<FMC_CSQCFGR1rs> {
        CMD2_W::new(self, 16)
    }
    #[doc = "Bit 24 - CMD1T"]
    #[inline(always)]
    #[must_use]
    pub fn cmd1t(&mut self) -> CMD1T_W<FMC_CSQCFGR1rs> {
        CMD1T_W::new(self, 24)
    }
    #[doc = "Bit 25 - CMD2T"]
    #[inline(always)]
    #[must_use]
    pub fn cmd2t(&mut self) -> CMD2T_W<FMC_CSQCFGR1rs> {
        CMD2T_W::new(self, 25)
    }
}
#[doc = "FMC NAND Command Sequencer Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_csqcfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_csqcfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_CSQCFGR1rs;
impl crate::RegisterSpec for FMC_CSQCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_csqcfgr1::R`](R) reader structure"]
impl crate::Readable for FMC_CSQCFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`fmc_csqcfgr1::W`](W) writer structure"]
impl crate::Writable for FMC_CSQCFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMC_CSQCFGR1 to value 0"]
impl crate::Resettable for FMC_CSQCFGR1rs {
    const RESET_VALUE: u32 = 0;
}
