///Register `BCR2` reader
pub type R = crate::R<BCR2rs>;
///Register `BCR2` writer
pub type W = crate::W<BCR2rs>;
///Field `FTH` reader - FTH
pub type FTH_R = crate::FieldReader;
///Field `FTH` writer - FTH
pub type FTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FFLUSH` writer - FFLUSH
pub type FFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIS` reader - TRIS
pub type TRIS_R = crate::BitReader;
///Field `TRIS` writer - TRIS
pub type TRIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUTE` reader - MUTE
pub type MUTE_R = crate::BitReader;
///Field `MUTE` writer - MUTE
pub type MUTE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUTEVAL` reader - MUTEVAL
pub type MUTEVAL_R = crate::BitReader;
///Field `MUTEVAL` writer - MUTEVAL
pub type MUTEVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUTECNT` reader - MUTECNT
pub type MUTECNT_R = crate::FieldReader;
///Field `MUTECNT` writer - MUTECNT
pub type MUTECNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CPL` reader - CPL
pub type CPL_R = crate::BitReader;
///Field `CPL` writer - CPL
pub type CPL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP` reader - COMP
pub type COMP_R = crate::FieldReader;
///Field `COMP` writer - COMP
pub type COMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - FTH
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - TRIS
    #[inline(always)]
    pub fn tris(&self) -> TRIS_R {
        TRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MUTE
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MUTEVAL
    #[inline(always)]
    pub fn muteval(&self) -> MUTEVAL_R {
        MUTEVAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:12 - MUTECNT
    #[inline(always)]
    pub fn mutecnt(&self) -> MUTECNT_R {
        MUTECNT_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    ///Bit 13 - CPL
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - COMP
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCR2")
            .field("fth", &self.fth())
            .field("tris", &self.tris())
            .field("mute", &self.mute())
            .field("muteval", &self.muteval())
            .field("mutecnt", &self.mutecnt())
            .field("cpl", &self.cpl())
            .field("comp", &self.comp())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - FTH
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W<BCR2rs> {
        FTH_W::new(self, 0)
    }
    ///Bit 3 - FFLUSH
    #[inline(always)]
    pub fn fflush(&mut self) -> FFLUSH_W<BCR2rs> {
        FFLUSH_W::new(self, 3)
    }
    ///Bit 4 - TRIS
    #[inline(always)]
    pub fn tris(&mut self) -> TRIS_W<BCR2rs> {
        TRIS_W::new(self, 4)
    }
    ///Bit 5 - MUTE
    #[inline(always)]
    pub fn mute(&mut self) -> MUTE_W<BCR2rs> {
        MUTE_W::new(self, 5)
    }
    ///Bit 6 - MUTEVAL
    #[inline(always)]
    pub fn muteval(&mut self) -> MUTEVAL_W<BCR2rs> {
        MUTEVAL_W::new(self, 6)
    }
    ///Bits 7:12 - MUTECNT
    #[inline(always)]
    pub fn mutecnt(&mut self) -> MUTECNT_W<BCR2rs> {
        MUTECNT_W::new(self, 7)
    }
    ///Bit 13 - CPL
    #[inline(always)]
    pub fn cpl(&mut self) -> CPL_W<BCR2rs> {
        CPL_W::new(self, 13)
    }
    ///Bits 14:15 - COMP
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W<BCR2rs> {
        COMP_W::new(self, 14)
    }
}
/**Configuration register 2

You can [`read`](crate::Reg::read) this register and get [`bcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SAI1:BCR2)*/
pub struct BCR2rs;
impl crate::RegisterSpec for BCR2rs {
    type Ux = u32;
}
///`read()` method returns [`bcr2::R`](R) reader structure
impl crate::Readable for BCR2rs {}
///`write(|w| ..)` method takes [`bcr2::W`](W) writer structure
impl crate::Writable for BCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCR2 to value 0
impl crate::Resettable for BCR2rs {}
