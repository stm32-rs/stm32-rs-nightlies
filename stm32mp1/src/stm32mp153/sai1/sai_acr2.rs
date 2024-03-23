#[doc = "Register `SAI_ACR2` reader"]
pub type R = crate::R<SAI_ACR2rs>;
#[doc = "Register `SAI_ACR2` writer"]
pub type W = crate::W<SAI_ACR2rs>;
#[doc = "Field `FTH` reader - FTH"]
pub type FTH_R = crate::FieldReader;
#[doc = "Field `FTH` writer - FTH"]
pub type FTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FFLUSH` writer - FFLUSH"]
pub type FFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIS` reader - TRIS"]
pub type TRIS_R = crate::BitReader;
#[doc = "Field `TRIS` writer - TRIS"]
pub type TRIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUTE` reader - MUTE"]
pub type MUTE_R = crate::BitReader;
#[doc = "Field `MUTE` writer - MUTE"]
pub type MUTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUTEVAL` reader - MUTEVAL"]
pub type MUTEVAL_R = crate::BitReader;
#[doc = "Field `MUTEVAL` writer - MUTEVAL"]
pub type MUTEVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUTECNT` reader - MUTECNT"]
pub type MUTECNT_R = crate::FieldReader;
#[doc = "Field `MUTECNT` writer - MUTECNT"]
pub type MUTECNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CPL` reader - CPL"]
pub type CPL_R = crate::BitReader;
#[doc = "Field `CPL` writer - CPL"]
pub type CPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP` reader - COMP"]
pub type COMP_R = crate::FieldReader;
#[doc = "Field `COMP` writer - COMP"]
pub type COMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - FTH"]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - TRIS"]
    #[inline(always)]
    pub fn tris(&self) -> TRIS_R {
        TRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MUTE"]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MUTEVAL"]
    #[inline(always)]
    pub fn muteval(&self) -> MUTEVAL_R {
        MUTEVAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:12 - MUTECNT"]
    #[inline(always)]
    pub fn mutecnt(&self) -> MUTECNT_R {
        MUTECNT_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - CPL"]
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - COMP"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - FTH"]
    #[inline(always)]
    #[must_use]
    pub fn fth(&mut self) -> FTH_W<SAI_ACR2rs> {
        FTH_W::new(self, 0)
    }
    #[doc = "Bit 3 - FFLUSH"]
    #[inline(always)]
    #[must_use]
    pub fn fflush(&mut self) -> FFLUSH_W<SAI_ACR2rs> {
        FFLUSH_W::new(self, 3)
    }
    #[doc = "Bit 4 - TRIS"]
    #[inline(always)]
    #[must_use]
    pub fn tris(&mut self) -> TRIS_W<SAI_ACR2rs> {
        TRIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - MUTE"]
    #[inline(always)]
    #[must_use]
    pub fn mute(&mut self) -> MUTE_W<SAI_ACR2rs> {
        MUTE_W::new(self, 5)
    }
    #[doc = "Bit 6 - MUTEVAL"]
    #[inline(always)]
    #[must_use]
    pub fn muteval(&mut self) -> MUTEVAL_W<SAI_ACR2rs> {
        MUTEVAL_W::new(self, 6)
    }
    #[doc = "Bits 7:12 - MUTECNT"]
    #[inline(always)]
    #[must_use]
    pub fn mutecnt(&mut self) -> MUTECNT_W<SAI_ACR2rs> {
        MUTECNT_W::new(self, 7)
    }
    #[doc = "Bit 13 - CPL"]
    #[inline(always)]
    #[must_use]
    pub fn cpl(&mut self) -> CPL_W<SAI_ACR2rs> {
        CPL_W::new(self, 13)
    }
    #[doc = "Bits 14:15 - COMP"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<SAI_ACR2rs> {
        COMP_W::new(self, 14)
    }
}
#[doc = "Configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_acr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_acr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAI_ACR2rs;
impl crate::RegisterSpec for SAI_ACR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sai_acr2::R`](R) reader structure"]
impl crate::Readable for SAI_ACR2rs {}
#[doc = "`write(|w| ..)` method takes [`sai_acr2::W`](W) writer structure"]
impl crate::Writable for SAI_ACR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAI_ACR2 to value 0"]
impl crate::Resettable for SAI_ACR2rs {
    const RESET_VALUE: u32 = 0;
}
