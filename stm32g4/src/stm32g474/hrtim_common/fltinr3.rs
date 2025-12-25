///Register `FLTINR3` reader
pub type R = crate::R<FLTINR3rs>;
///Register `FLTINR3` writer
pub type W = crate::W<FLTINR3rs>;
///Field `FLTBLKE(1-4)` reader - FLT%sBLKE
pub type FLTBLKE_R = crate::BitReader;
///Field `FLTBLKE(1-4)` writer - FLT%sBLKE
pub type FLTBLKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLTBLKS(1-4)` reader - FLT%sBLKS
pub type FLTBLKS_R = crate::BitReader;
///Field `FLTBLKS(1-4)` writer - FLT%sBLKS
pub type FLTBLKS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLTCNT(1-4)` reader - FLT%sCNT
pub type FLTCNT_R = crate::FieldReader;
///Field `FLTCNT(1-4)` writer - FLT%sCNT
pub type FLTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FLTCRES(1-4)` reader - FLT%sCRES
pub type FLTCRES_R = crate::BitReader;
///Field `FLTCRES(1-4)` writer - FLT%sCRES
pub type FLTCRES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT1RSTM` reader - FLT1RSTM
pub type FLT1RSTM_R = crate::BitReader;
///Field `FLT1RSTM` writer - FLT1RSTM
pub type FLT1RSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT2RSTM` reader - FLT2RSTM
pub type FLT2RSTM_R = crate::BitReader;
///Field `FLT2RSTM` writer - FLT2RSTM
pub type FLT2RSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT3RSTM` reader - FLT3RSTM
pub type FLT3RSTM_R = crate::BitReader;
///Field `FLT3RSTM` writer - FLT3RSTM
pub type FLT3RSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT4RSTM` reader - FLT4RSTM
pub type FLT4RSTM_R = crate::BitReader;
///Field `FLT4RSTM` writer - FLT4RSTM
pub type FLT4RSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///FLT(1-4)BLKE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1BLKE` field.</div>
    #[inline(always)]
    pub fn fltblke(&self, n: u8) -> FLTBLKE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FLTBLKE_R::new(((self.bits >> (n * 8)) & 1) != 0)
    }
    ///Iterator for array of:
    ///FLT(1-4)BLKE
    #[inline(always)]
    pub fn fltblke_iter(&self) -> impl Iterator<Item = FLTBLKE_R> + '_ {
        (0..4).map(move |n| FLTBLKE_R::new(((self.bits >> (n * 8)) & 1) != 0))
    }
    ///Bit 0 - FLT1BLKE
    #[inline(always)]
    pub fn flt1blke(&self) -> FLTBLKE_R {
        FLTBLKE_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - FLT2BLKE
    #[inline(always)]
    pub fn flt2blke(&self) -> FLTBLKE_R {
        FLTBLKE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - FLT3BLKE
    #[inline(always)]
    pub fn flt3blke(&self) -> FLTBLKE_R {
        FLTBLKE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - FLT4BLKE
    #[inline(always)]
    pub fn flt4blke(&self) -> FLTBLKE_R {
        FLTBLKE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///FLT(1-4)BLKS
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1BLKS` field.</div>
    #[inline(always)]
    pub fn fltblks(&self, n: u8) -> FLTBLKS_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FLTBLKS_R::new(((self.bits >> (n * 8 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///FLT(1-4)BLKS
    #[inline(always)]
    pub fn fltblks_iter(&self) -> impl Iterator<Item = FLTBLKS_R> + '_ {
        (0..4).map(move |n| FLTBLKS_R::new(((self.bits >> (n * 8 + 1)) & 1) != 0))
    }
    ///Bit 1 - FLT1BLKS
    #[inline(always)]
    pub fn flt1blks(&self) -> FLTBLKS_R {
        FLTBLKS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 9 - FLT2BLKS
    #[inline(always)]
    pub fn flt2blks(&self) -> FLTBLKS_R {
        FLTBLKS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 17 - FLT3BLKS
    #[inline(always)]
    pub fn flt3blks(&self) -> FLTBLKS_R {
        FLTBLKS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 25 - FLT4BLKS
    #[inline(always)]
    pub fn flt4blks(&self) -> FLTBLKS_R {
        FLTBLKS_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///FLT(1-4)CNT
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1CNT` field.</div>
    #[inline(always)]
    pub fn fltcnt(&self, n: u8) -> FLTCNT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FLTCNT_R::new(((self.bits >> (n * 8 + 2)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///FLT(1-4)CNT
    #[inline(always)]
    pub fn fltcnt_iter(&self) -> impl Iterator<Item = FLTCNT_R> + '_ {
        (0..4).map(move |n| FLTCNT_R::new(((self.bits >> (n * 8 + 2)) & 0x0f) as u8))
    }
    ///Bits 2:5 - FLT1CNT
    #[inline(always)]
    pub fn flt1cnt(&self) -> FLTCNT_R {
        FLTCNT_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bits 10:13 - FLT2CNT
    #[inline(always)]
    pub fn flt2cnt(&self) -> FLTCNT_R {
        FLTCNT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bits 18:21 - FLT3CNT
    #[inline(always)]
    pub fn flt3cnt(&self) -> FLTCNT_R {
        FLTCNT_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 26:29 - FLT4CNT
    #[inline(always)]
    pub fn flt4cnt(&self) -> FLTCNT_R {
        FLTCNT_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    ///FLT(1-4)CRES
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1CRES` field.</div>
    #[inline(always)]
    pub fn fltcres(&self, n: u8) -> FLTCRES_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FLTCRES_R::new(((self.bits >> (n * 8 + 6)) & 1) != 0)
    }
    ///Iterator for array of:
    ///FLT(1-4)CRES
    #[inline(always)]
    pub fn fltcres_iter(&self) -> impl Iterator<Item = FLTCRES_R> + '_ {
        (0..4).map(move |n| FLTCRES_R::new(((self.bits >> (n * 8 + 6)) & 1) != 0))
    }
    ///Bit 6 - FLT1CRES
    #[inline(always)]
    pub fn flt1cres(&self) -> FLTCRES_R {
        FLTCRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 14 - FLT2CRES
    #[inline(always)]
    pub fn flt2cres(&self) -> FLTCRES_R {
        FLTCRES_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 22 - FLT3CRES
    #[inline(always)]
    pub fn flt3cres(&self) -> FLTCRES_R {
        FLTCRES_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 30 - FLT4CRES
    #[inline(always)]
    pub fn flt4cres(&self) -> FLTCRES_R {
        FLTCRES_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 7 - FLT1RSTM
    #[inline(always)]
    pub fn flt1rstm(&self) -> FLT1RSTM_R {
        FLT1RSTM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 15 - FLT2RSTM
    #[inline(always)]
    pub fn flt2rstm(&self) -> FLT2RSTM_R {
        FLT2RSTM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 23 - FLT3RSTM
    #[inline(always)]
    pub fn flt3rstm(&self) -> FLT3RSTM_R {
        FLT3RSTM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - FLT4RSTM
    #[inline(always)]
    pub fn flt4rstm(&self) -> FLT4RSTM_R {
        FLT4RSTM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTINR3")
            .field("flt4rstm", &self.flt4rstm())
            .field("flt1cres", &self.flt1cres())
            .field("flt2cres", &self.flt2cres())
            .field("flt3cres", &self.flt3cres())
            .field("flt4cres", &self.flt4cres())
            .field("flt1cnt", &self.flt1cnt())
            .field("flt2cnt", &self.flt2cnt())
            .field("flt3cnt", &self.flt3cnt())
            .field("flt4cnt", &self.flt4cnt())
            .field("flt1blks", &self.flt1blks())
            .field("flt2blks", &self.flt2blks())
            .field("flt3blks", &self.flt3blks())
            .field("flt4blks", &self.flt4blks())
            .field("flt1blke", &self.flt1blke())
            .field("flt2blke", &self.flt2blke())
            .field("flt3blke", &self.flt3blke())
            .field("flt4blke", &self.flt4blke())
            .field("flt3rstm", &self.flt3rstm())
            .field("flt2rstm", &self.flt2rstm())
            .field("flt1rstm", &self.flt1rstm())
            .finish()
    }
}
impl W {
    ///FLT(1-4)BLKE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1BLKE` field.</div>
    #[inline(always)]
    pub fn fltblke(&mut self, n: u8) -> FLTBLKE_W<'_, FLTINR3rs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FLTBLKE_W::new(self, n * 8)
    }
    ///Bit 0 - FLT1BLKE
    #[inline(always)]
    pub fn flt1blke(&mut self) -> FLTBLKE_W<'_, FLTINR3rs> {
        FLTBLKE_W::new(self, 0)
    }
    ///Bit 8 - FLT2BLKE
    #[inline(always)]
    pub fn flt2blke(&mut self) -> FLTBLKE_W<'_, FLTINR3rs> {
        FLTBLKE_W::new(self, 8)
    }
    ///Bit 16 - FLT3BLKE
    #[inline(always)]
    pub fn flt3blke(&mut self) -> FLTBLKE_W<'_, FLTINR3rs> {
        FLTBLKE_W::new(self, 16)
    }
    ///Bit 24 - FLT4BLKE
    #[inline(always)]
    pub fn flt4blke(&mut self) -> FLTBLKE_W<'_, FLTINR3rs> {
        FLTBLKE_W::new(self, 24)
    }
    ///FLT(1-4)BLKS
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1BLKS` field.</div>
    #[inline(always)]
    pub fn fltblks(&mut self, n: u8) -> FLTBLKS_W<'_, FLTINR3rs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FLTBLKS_W::new(self, n * 8 + 1)
    }
    ///Bit 1 - FLT1BLKS
    #[inline(always)]
    pub fn flt1blks(&mut self) -> FLTBLKS_W<'_, FLTINR3rs> {
        FLTBLKS_W::new(self, 1)
    }
    ///Bit 9 - FLT2BLKS
    #[inline(always)]
    pub fn flt2blks(&mut self) -> FLTBLKS_W<'_, FLTINR3rs> {
        FLTBLKS_W::new(self, 9)
    }
    ///Bit 17 - FLT3BLKS
    #[inline(always)]
    pub fn flt3blks(&mut self) -> FLTBLKS_W<'_, FLTINR3rs> {
        FLTBLKS_W::new(self, 17)
    }
    ///Bit 25 - FLT4BLKS
    #[inline(always)]
    pub fn flt4blks(&mut self) -> FLTBLKS_W<'_, FLTINR3rs> {
        FLTBLKS_W::new(self, 25)
    }
    ///FLT(1-4)CNT
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1CNT` field.</div>
    #[inline(always)]
    pub fn fltcnt(&mut self, n: u8) -> FLTCNT_W<'_, FLTINR3rs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FLTCNT_W::new(self, n * 8 + 2)
    }
    ///Bits 2:5 - FLT1CNT
    #[inline(always)]
    pub fn flt1cnt(&mut self) -> FLTCNT_W<'_, FLTINR3rs> {
        FLTCNT_W::new(self, 2)
    }
    ///Bits 10:13 - FLT2CNT
    #[inline(always)]
    pub fn flt2cnt(&mut self) -> FLTCNT_W<'_, FLTINR3rs> {
        FLTCNT_W::new(self, 10)
    }
    ///Bits 18:21 - FLT3CNT
    #[inline(always)]
    pub fn flt3cnt(&mut self) -> FLTCNT_W<'_, FLTINR3rs> {
        FLTCNT_W::new(self, 18)
    }
    ///Bits 26:29 - FLT4CNT
    #[inline(always)]
    pub fn flt4cnt(&mut self) -> FLTCNT_W<'_, FLTINR3rs> {
        FLTCNT_W::new(self, 26)
    }
    ///FLT(1-4)CRES
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1CRES` field.</div>
    #[inline(always)]
    pub fn fltcres(&mut self, n: u8) -> FLTCRES_W<'_, FLTINR3rs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FLTCRES_W::new(self, n * 8 + 6)
    }
    ///Bit 6 - FLT1CRES
    #[inline(always)]
    pub fn flt1cres(&mut self) -> FLTCRES_W<'_, FLTINR3rs> {
        FLTCRES_W::new(self, 6)
    }
    ///Bit 14 - FLT2CRES
    #[inline(always)]
    pub fn flt2cres(&mut self) -> FLTCRES_W<'_, FLTINR3rs> {
        FLTCRES_W::new(self, 14)
    }
    ///Bit 22 - FLT3CRES
    #[inline(always)]
    pub fn flt3cres(&mut self) -> FLTCRES_W<'_, FLTINR3rs> {
        FLTCRES_W::new(self, 22)
    }
    ///Bit 30 - FLT4CRES
    #[inline(always)]
    pub fn flt4cres(&mut self) -> FLTCRES_W<'_, FLTINR3rs> {
        FLTCRES_W::new(self, 30)
    }
    ///Bit 7 - FLT1RSTM
    #[inline(always)]
    pub fn flt1rstm(&mut self) -> FLT1RSTM_W<'_, FLTINR3rs> {
        FLT1RSTM_W::new(self, 7)
    }
    ///Bit 15 - FLT2RSTM
    #[inline(always)]
    pub fn flt2rstm(&mut self) -> FLT2RSTM_W<'_, FLTINR3rs> {
        FLT2RSTM_W::new(self, 15)
    }
    ///Bit 23 - FLT3RSTM
    #[inline(always)]
    pub fn flt3rstm(&mut self) -> FLT3RSTM_W<'_, FLTINR3rs> {
        FLT3RSTM_W::new(self, 23)
    }
    ///Bit 31 - FLT4RSTM
    #[inline(always)]
    pub fn flt4rstm(&mut self) -> FLT4RSTM_W<'_, FLTINR3rs> {
        FLT4RSTM_W::new(self, 31)
    }
}
/**HRTIM Fault Input Register 3

You can [`read`](crate::Reg::read) this register and get [`fltinr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltinr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:FLTINR3)*/
pub struct FLTINR3rs;
impl crate::RegisterSpec for FLTINR3rs {
    type Ux = u32;
}
///`read()` method returns [`fltinr3::R`](R) reader structure
impl crate::Readable for FLTINR3rs {}
///`write(|w| ..)` method takes [`fltinr3::W`](W) writer structure
impl crate::Writable for FLTINR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLTINR3 to value 0
impl crate::Resettable for FLTINR3rs {}
