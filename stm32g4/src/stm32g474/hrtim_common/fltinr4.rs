///Register `FLTINR4` reader
pub type R = crate::R<FLTINR4rs>;
///Register `FLTINR4` writer
pub type W = crate::W<FLTINR4rs>;
///Field `FLTBLKE(5-6)` reader - FLT%sBLKE
pub type FLTBLKE_R = crate::BitReader;
///Field `FLTBLKE(5-6)` writer - FLT%sBLKE
pub type FLTBLKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLTBLKS(5-6)` reader - FLT%sBLKS
pub type FLTBLKS_R = crate::BitReader;
///Field `FLTBLKS(5-6)` writer - FLT%sBLKS
pub type FLTBLKS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLTCNT(5-6)` reader - FLT%sCNT
pub type FLTCNT_R = crate::FieldReader;
///Field `FLTCNT(5-6)` writer - FLT%sCNT
pub type FLTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FLTCRES(5-6)` reader - FLT%sCRES
pub type FLTCRES_R = crate::BitReader;
///Field `FLTCRES(5-6)` writer - FLT%sCRES
pub type FLTCRES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT5RSTM` reader - FLT5RSTM
pub type FLT5RSTM_R = crate::BitReader;
///Field `FLT5RSTM` writer - FLT5RSTM
pub type FLT5RSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT6RSTM` reader - FLT6RSTM
pub type FLT6RSTM_R = crate::BitReader;
///Field `FLT6RSTM` writer - FLT6RSTM
pub type FLT6RSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///FLT(5-6)BLKE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT5BLKE` field.</div>
    #[inline(always)]
    pub fn fltblke(&self, n: u8) -> FLTBLKE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        FLTBLKE_R::new(((self.bits >> (n * 8)) & 1) != 0)
    }
    ///Iterator for array of:
    ///FLT(5-6)BLKE
    #[inline(always)]
    pub fn fltblke_iter(&self) -> impl Iterator<Item = FLTBLKE_R> + '_ {
        (0..2).map(move |n| FLTBLKE_R::new(((self.bits >> (n * 8)) & 1) != 0))
    }
    ///Bit 0 - FLT5BLKE
    #[inline(always)]
    pub fn flt5blke(&self) -> FLTBLKE_R {
        FLTBLKE_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - FLT6BLKE
    #[inline(always)]
    pub fn flt6blke(&self) -> FLTBLKE_R {
        FLTBLKE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///FLT(5-6)BLKS
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT5BLKS` field.</div>
    #[inline(always)]
    pub fn fltblks(&self, n: u8) -> FLTBLKS_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        FLTBLKS_R::new(((self.bits >> (n * 8 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///FLT(5-6)BLKS
    #[inline(always)]
    pub fn fltblks_iter(&self) -> impl Iterator<Item = FLTBLKS_R> + '_ {
        (0..2).map(move |n| FLTBLKS_R::new(((self.bits >> (n * 8 + 1)) & 1) != 0))
    }
    ///Bit 1 - FLT5BLKS
    #[inline(always)]
    pub fn flt5blks(&self) -> FLTBLKS_R {
        FLTBLKS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 9 - FLT6BLKS
    #[inline(always)]
    pub fn flt6blks(&self) -> FLTBLKS_R {
        FLTBLKS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///FLT(5-6)CNT
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT5CNT` field.</div>
    #[inline(always)]
    pub fn fltcnt(&self, n: u8) -> FLTCNT_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        FLTCNT_R::new(((self.bits >> (n * 8 + 2)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///FLT(5-6)CNT
    #[inline(always)]
    pub fn fltcnt_iter(&self) -> impl Iterator<Item = FLTCNT_R> + '_ {
        (0..2).map(move |n| FLTCNT_R::new(((self.bits >> (n * 8 + 2)) & 0x0f) as u8))
    }
    ///Bits 2:5 - FLT5CNT
    #[inline(always)]
    pub fn flt5cnt(&self) -> FLTCNT_R {
        FLTCNT_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bits 10:13 - FLT6CNT
    #[inline(always)]
    pub fn flt6cnt(&self) -> FLTCNT_R {
        FLTCNT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///FLT(5-6)CRES
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT5CRES` field.</div>
    #[inline(always)]
    pub fn fltcres(&self, n: u8) -> FLTCRES_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        FLTCRES_R::new(((self.bits >> (n * 8 + 6)) & 1) != 0)
    }
    ///Iterator for array of:
    ///FLT(5-6)CRES
    #[inline(always)]
    pub fn fltcres_iter(&self) -> impl Iterator<Item = FLTCRES_R> + '_ {
        (0..2).map(move |n| FLTCRES_R::new(((self.bits >> (n * 8 + 6)) & 1) != 0))
    }
    ///Bit 6 - FLT5CRES
    #[inline(always)]
    pub fn flt5cres(&self) -> FLTCRES_R {
        FLTCRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 14 - FLT6CRES
    #[inline(always)]
    pub fn flt6cres(&self) -> FLTCRES_R {
        FLTCRES_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 7 - FLT5RSTM
    #[inline(always)]
    pub fn flt5rstm(&self) -> FLT5RSTM_R {
        FLT5RSTM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 15 - FLT6RSTM
    #[inline(always)]
    pub fn flt6rstm(&self) -> FLT6RSTM_R {
        FLT6RSTM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTINR4")
            .field("flt6rstm", &self.flt6rstm())
            .field("flt5cres", &self.flt5cres())
            .field("flt6cres", &self.flt6cres())
            .field("flt5cnt", &self.flt5cnt())
            .field("flt6cnt", &self.flt6cnt())
            .field("flt5blks", &self.flt5blks())
            .field("flt6blks", &self.flt6blks())
            .field("flt5blke", &self.flt5blke())
            .field("flt6blke", &self.flt6blke())
            .field("flt5rstm", &self.flt5rstm())
            .finish()
    }
}
impl W {
    ///FLT(5-6)BLKE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT5BLKE` field.</div>
    #[inline(always)]
    pub fn fltblke(&mut self, n: u8) -> FLTBLKE_W<'_, FLTINR4rs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        FLTBLKE_W::new(self, n * 8)
    }
    ///Bit 0 - FLT5BLKE
    #[inline(always)]
    pub fn flt5blke(&mut self) -> FLTBLKE_W<'_, FLTINR4rs> {
        FLTBLKE_W::new(self, 0)
    }
    ///Bit 8 - FLT6BLKE
    #[inline(always)]
    pub fn flt6blke(&mut self) -> FLTBLKE_W<'_, FLTINR4rs> {
        FLTBLKE_W::new(self, 8)
    }
    ///FLT(5-6)BLKS
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT5BLKS` field.</div>
    #[inline(always)]
    pub fn fltblks(&mut self, n: u8) -> FLTBLKS_W<'_, FLTINR4rs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        FLTBLKS_W::new(self, n * 8 + 1)
    }
    ///Bit 1 - FLT5BLKS
    #[inline(always)]
    pub fn flt5blks(&mut self) -> FLTBLKS_W<'_, FLTINR4rs> {
        FLTBLKS_W::new(self, 1)
    }
    ///Bit 9 - FLT6BLKS
    #[inline(always)]
    pub fn flt6blks(&mut self) -> FLTBLKS_W<'_, FLTINR4rs> {
        FLTBLKS_W::new(self, 9)
    }
    ///FLT(5-6)CNT
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT5CNT` field.</div>
    #[inline(always)]
    pub fn fltcnt(&mut self, n: u8) -> FLTCNT_W<'_, FLTINR4rs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        FLTCNT_W::new(self, n * 8 + 2)
    }
    ///Bits 2:5 - FLT5CNT
    #[inline(always)]
    pub fn flt5cnt(&mut self) -> FLTCNT_W<'_, FLTINR4rs> {
        FLTCNT_W::new(self, 2)
    }
    ///Bits 10:13 - FLT6CNT
    #[inline(always)]
    pub fn flt6cnt(&mut self) -> FLTCNT_W<'_, FLTINR4rs> {
        FLTCNT_W::new(self, 10)
    }
    ///FLT(5-6)CRES
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT5CRES` field.</div>
    #[inline(always)]
    pub fn fltcres(&mut self, n: u8) -> FLTCRES_W<'_, FLTINR4rs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        FLTCRES_W::new(self, n * 8 + 6)
    }
    ///Bit 6 - FLT5CRES
    #[inline(always)]
    pub fn flt5cres(&mut self) -> FLTCRES_W<'_, FLTINR4rs> {
        FLTCRES_W::new(self, 6)
    }
    ///Bit 14 - FLT6CRES
    #[inline(always)]
    pub fn flt6cres(&mut self) -> FLTCRES_W<'_, FLTINR4rs> {
        FLTCRES_W::new(self, 14)
    }
    ///Bit 7 - FLT5RSTM
    #[inline(always)]
    pub fn flt5rstm(&mut self) -> FLT5RSTM_W<'_, FLTINR4rs> {
        FLT5RSTM_W::new(self, 7)
    }
    ///Bit 15 - FLT6RSTM
    #[inline(always)]
    pub fn flt6rstm(&mut self) -> FLT6RSTM_W<'_, FLTINR4rs> {
        FLT6RSTM_W::new(self, 15)
    }
}
/**HRTIM Fault Input Register 4

You can [`read`](crate::Reg::read) this register and get [`fltinr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltinr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:FLTINR4)*/
pub struct FLTINR4rs;
impl crate::RegisterSpec for FLTINR4rs {
    type Ux = u32;
}
///`read()` method returns [`fltinr4::R`](R) reader structure
impl crate::Readable for FLTINR4rs {}
///`write(|w| ..)` method takes [`fltinr4::W`](W) writer structure
impl crate::Writable for FLTINR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLTINR4 to value 0
impl crate::Resettable for FLTINR4rs {}
