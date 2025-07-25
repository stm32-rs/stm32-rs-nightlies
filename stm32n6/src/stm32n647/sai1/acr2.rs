///Register `ACR2` reader
pub type R = crate::R<ACR2rs>;
///Register `ACR2` writer
pub type W = crate::W<ACR2rs>;
///Field `FTH` reader - FIFO threshold.
pub type FTH_R = crate::FieldReader;
///Field `FTH` writer - FIFO threshold.
pub type FTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FFLUSH` writer - FIFO flush.
pub type FFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIS` reader - Tristate management on data line.
pub type TRIS_R = crate::BitReader;
///Field `TRIS` writer - Tristate management on data line.
pub type TRIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUTE` reader - Mute.
pub type MUTE_R = crate::BitReader;
///Field `MUTE` writer - Mute.
pub type MUTE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUTEVAL` reader - Mute value.
pub type MUTEVAL_R = crate::BitReader;
///Field `MUTEVAL` writer - Mute value.
pub type MUTEVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUTECNT` reader - Mute counter.
pub type MUTECNT_R = crate::FieldReader;
///Field `MUTECNT` writer - Mute counter.
pub type MUTECNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CPL` reader - Complement bit.
pub type CPL_R = crate::BitReader;
///Field `CPL` writer - Complement bit.
pub type CPL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP` reader - Companding mode.
pub type COMP_R = crate::FieldReader;
///Field `COMP` writer - Companding mode.
pub type COMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - FIFO threshold.
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - Tristate management on data line.
    #[inline(always)]
    pub fn tris(&self) -> TRIS_R {
        TRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Mute.
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Mute value.
    #[inline(always)]
    pub fn muteval(&self) -> MUTEVAL_R {
        MUTEVAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:12 - Mute counter.
    #[inline(always)]
    pub fn mutecnt(&self) -> MUTECNT_R {
        MUTECNT_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    ///Bit 13 - Complement bit.
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Companding mode.
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACR2")
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
    ///Bits 0:2 - FIFO threshold.
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W<ACR2rs> {
        FTH_W::new(self, 0)
    }
    ///Bit 3 - FIFO flush.
    #[inline(always)]
    pub fn fflush(&mut self) -> FFLUSH_W<ACR2rs> {
        FFLUSH_W::new(self, 3)
    }
    ///Bit 4 - Tristate management on data line.
    #[inline(always)]
    pub fn tris(&mut self) -> TRIS_W<ACR2rs> {
        TRIS_W::new(self, 4)
    }
    ///Bit 5 - Mute.
    #[inline(always)]
    pub fn mute(&mut self) -> MUTE_W<ACR2rs> {
        MUTE_W::new(self, 5)
    }
    ///Bit 6 - Mute value.
    #[inline(always)]
    pub fn muteval(&mut self) -> MUTEVAL_W<ACR2rs> {
        MUTEVAL_W::new(self, 6)
    }
    ///Bits 7:12 - Mute counter.
    #[inline(always)]
    pub fn mutecnt(&mut self) -> MUTECNT_W<ACR2rs> {
        MUTECNT_W::new(self, 7)
    }
    ///Bit 13 - Complement bit.
    #[inline(always)]
    pub fn cpl(&mut self) -> CPL_W<ACR2rs> {
        CPL_W::new(self, 13)
    }
    ///Bits 14:15 - Companding mode.
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W<ACR2rs> {
        COMP_W::new(self, 14)
    }
}
/**SAI configuration register 2

You can [`read`](crate::Reg::read) this register and get [`acr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SAI1:ACR2)*/
pub struct ACR2rs;
impl crate::RegisterSpec for ACR2rs {
    type Ux = u32;
}
///`read()` method returns [`acr2::R`](R) reader structure
impl crate::Readable for ACR2rs {}
///`write(|w| ..)` method takes [`acr2::W`](W) writer structure
impl crate::Writable for ACR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACR2 to value 0
impl crate::Resettable for ACR2rs {}
