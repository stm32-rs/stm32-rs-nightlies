///Register `FLTINR3` reader
pub type R = crate::R<FLTINR3rs>;
///Register `FLTINR3` writer
pub type W = crate::W<FLTINR3rs>;
///Field `FLT1BLKE` reader - FLT1BLKE
pub type FLT1BLKE_R = crate::BitReader;
///Field `FLT1BLKE` writer - FLT1BLKE
pub type FLT1BLKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT1BLKS` reader - FLT1BLKS
pub type FLT1BLKS_R = crate::BitReader;
///Field `FLT1BLKS` writer - FLT1BLKS
pub type FLT1BLKS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT1CNT` reader - FLT1CNT
pub type FLT1CNT_R = crate::FieldReader;
///Field `FLT1CNT` writer - FLT1CNT
pub type FLT1CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FLT1CRES` reader - FLT1CRES
pub type FLT1CRES_R = crate::BitReader;
///Field `FLT1CRES` writer - FLT1CRES
pub type FLT1CRES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT1RSTM` reader - FLT1RSTM
pub type FLT1RSTM_R = crate::BitReader;
///Field `FLT1RSTM` writer - FLT1RSTM
pub type FLT1RSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT2BLKE` reader - FLT2BLKE
pub type FLT2BLKE_R = crate::BitReader;
///Field `FLT2BLKE` writer - FLT2BLKE
pub type FLT2BLKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT2BLKS` reader - FLT2BLKS
pub type FLT2BLKS_R = crate::BitReader;
///Field `FLT2BLKS` writer - FLT2BLKS
pub type FLT2BLKS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT2CNT` reader - FLT2CNT
pub type FLT2CNT_R = crate::FieldReader;
///Field `FLT2CNT` writer - FLT2CNT
pub type FLT2CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FLT2CRES` reader - FLT2CRES
pub type FLT2CRES_R = crate::BitReader;
///Field `FLT2CRES` writer - FLT2CRES
pub type FLT2CRES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT2RSTM` reader - FLT2RSTM
pub type FLT2RSTM_R = crate::BitReader;
///Field `FLT2RSTM` writer - FLT2RSTM
pub type FLT2RSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT3BLKE` reader - FLT3BLKE
pub type FLT3BLKE_R = crate::BitReader;
///Field `FLT3BLKE` writer - FLT3BLKE
pub type FLT3BLKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT3BLKS` reader - FLT3BLKS
pub type FLT3BLKS_R = crate::BitReader;
///Field `FLT3BLKS` writer - FLT3BLKS
pub type FLT3BLKS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT3CNT` reader - FLT3CNT
pub type FLT3CNT_R = crate::FieldReader;
///Field `FLT3CNT` writer - FLT3CNT
pub type FLT3CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FLT3CRES` reader - FLT3CRES
pub type FLT3CRES_R = crate::BitReader;
///Field `FLT3CRES` writer - FLT3CRES
pub type FLT3CRES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT3RSTM` reader - FLT3RSTM
pub type FLT3RSTM_R = crate::BitReader;
///Field `FLT3RSTM` writer - FLT3RSTM
pub type FLT3RSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT4BLKE` reader - FLT4BLKE
pub type FLT4BLKE_R = crate::BitReader;
///Field `FLT4BLKE` writer - FLT4BLKE
pub type FLT4BLKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT4BLKS` reader - FLT4BLKS
pub type FLT4BLKS_R = crate::BitReader;
///Field `FLT4BLKS` writer - FLT4BLKS
pub type FLT4BLKS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT4CNT` reader - FLT4CNT
pub type FLT4CNT_R = crate::FieldReader;
///Field `FLT4CNT` writer - FLT4CNT
pub type FLT4CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FLT4CRES` reader - FLT4CRES
pub type FLT4CRES_R = crate::BitReader;
///Field `FLT4CRES` writer - FLT4CRES
pub type FLT4CRES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT4RSTM` reader - FLT4RSTM
pub type FLT4RSTM_R = crate::BitReader;
///Field `FLT4RSTM` writer - FLT4RSTM
pub type FLT4RSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - FLT1BLKE
    #[inline(always)]
    pub fn flt1blke(&self) -> FLT1BLKE_R {
        FLT1BLKE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FLT1BLKS
    #[inline(always)]
    pub fn flt1blks(&self) -> FLT1BLKS_R {
        FLT1BLKS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5 - FLT1CNT
    #[inline(always)]
    pub fn flt1cnt(&self) -> FLT1CNT_R {
        FLT1CNT_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bit 6 - FLT1CRES
    #[inline(always)]
    pub fn flt1cres(&self) -> FLT1CRES_R {
        FLT1CRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - FLT1RSTM
    #[inline(always)]
    pub fn flt1rstm(&self) -> FLT1RSTM_R {
        FLT1RSTM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - FLT2BLKE
    #[inline(always)]
    pub fn flt2blke(&self) -> FLT2BLKE_R {
        FLT2BLKE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - FLT2BLKS
    #[inline(always)]
    pub fn flt2blks(&self) -> FLT2BLKS_R {
        FLT2BLKS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:13 - FLT2CNT
    #[inline(always)]
    pub fn flt2cnt(&self) -> FLT2CNT_R {
        FLT2CNT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 14 - FLT2CRES
    #[inline(always)]
    pub fn flt2cres(&self) -> FLT2CRES_R {
        FLT2CRES_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - FLT2RSTM
    #[inline(always)]
    pub fn flt2rstm(&self) -> FLT2RSTM_R {
        FLT2RSTM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - FLT3BLKE
    #[inline(always)]
    pub fn flt3blke(&self) -> FLT3BLKE_R {
        FLT3BLKE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - FLT3BLKS
    #[inline(always)]
    pub fn flt3blks(&self) -> FLT3BLKS_R {
        FLT3BLKS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:21 - FLT3CNT
    #[inline(always)]
    pub fn flt3cnt(&self) -> FLT3CNT_R {
        FLT3CNT_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bit 22 - FLT3CRES
    #[inline(always)]
    pub fn flt3cres(&self) -> FLT3CRES_R {
        FLT3CRES_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - FLT3RSTM
    #[inline(always)]
    pub fn flt3rstm(&self) -> FLT3RSTM_R {
        FLT3RSTM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - FLT4BLKE
    #[inline(always)]
    pub fn flt4blke(&self) -> FLT4BLKE_R {
        FLT4BLKE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - FLT4BLKS
    #[inline(always)]
    pub fn flt4blks(&self) -> FLT4BLKS_R {
        FLT4BLKS_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:29 - FLT4CNT
    #[inline(always)]
    pub fn flt4cnt(&self) -> FLT4CNT_R {
        FLT4CNT_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    ///Bit 30 - FLT4CRES
    #[inline(always)]
    pub fn flt4cres(&self) -> FLT4CRES_R {
        FLT4CRES_R::new(((self.bits >> 30) & 1) != 0)
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
            .field("flt4cres", &self.flt4cres())
            .field("flt4cnt", &self.flt4cnt())
            .field("flt4blks", &self.flt4blks())
            .field("flt4blke", &self.flt4blke())
            .field("flt3rstm", &self.flt3rstm())
            .field("flt3cres", &self.flt3cres())
            .field("flt3cnt", &self.flt3cnt())
            .field("flt3blks", &self.flt3blks())
            .field("flt3blke", &self.flt3blke())
            .field("flt2rstm", &self.flt2rstm())
            .field("flt2cres", &self.flt2cres())
            .field("flt2cnt", &self.flt2cnt())
            .field("flt2blks", &self.flt2blks())
            .field("flt2blke", &self.flt2blke())
            .field("flt1rstm", &self.flt1rstm())
            .field("flt1cres", &self.flt1cres())
            .field("flt1cnt", &self.flt1cnt())
            .field("flt1blks", &self.flt1blks())
            .field("flt1blke", &self.flt1blke())
            .finish()
    }
}
impl W {
    ///Bit 0 - FLT1BLKE
    #[inline(always)]
    #[must_use]
    pub fn flt1blke(&mut self) -> FLT1BLKE_W<FLTINR3rs> {
        FLT1BLKE_W::new(self, 0)
    }
    ///Bit 1 - FLT1BLKS
    #[inline(always)]
    #[must_use]
    pub fn flt1blks(&mut self) -> FLT1BLKS_W<FLTINR3rs> {
        FLT1BLKS_W::new(self, 1)
    }
    ///Bits 2:5 - FLT1CNT
    #[inline(always)]
    #[must_use]
    pub fn flt1cnt(&mut self) -> FLT1CNT_W<FLTINR3rs> {
        FLT1CNT_W::new(self, 2)
    }
    ///Bit 6 - FLT1CRES
    #[inline(always)]
    #[must_use]
    pub fn flt1cres(&mut self) -> FLT1CRES_W<FLTINR3rs> {
        FLT1CRES_W::new(self, 6)
    }
    ///Bit 7 - FLT1RSTM
    #[inline(always)]
    #[must_use]
    pub fn flt1rstm(&mut self) -> FLT1RSTM_W<FLTINR3rs> {
        FLT1RSTM_W::new(self, 7)
    }
    ///Bit 8 - FLT2BLKE
    #[inline(always)]
    #[must_use]
    pub fn flt2blke(&mut self) -> FLT2BLKE_W<FLTINR3rs> {
        FLT2BLKE_W::new(self, 8)
    }
    ///Bit 9 - FLT2BLKS
    #[inline(always)]
    #[must_use]
    pub fn flt2blks(&mut self) -> FLT2BLKS_W<FLTINR3rs> {
        FLT2BLKS_W::new(self, 9)
    }
    ///Bits 10:13 - FLT2CNT
    #[inline(always)]
    #[must_use]
    pub fn flt2cnt(&mut self) -> FLT2CNT_W<FLTINR3rs> {
        FLT2CNT_W::new(self, 10)
    }
    ///Bit 14 - FLT2CRES
    #[inline(always)]
    #[must_use]
    pub fn flt2cres(&mut self) -> FLT2CRES_W<FLTINR3rs> {
        FLT2CRES_W::new(self, 14)
    }
    ///Bit 15 - FLT2RSTM
    #[inline(always)]
    #[must_use]
    pub fn flt2rstm(&mut self) -> FLT2RSTM_W<FLTINR3rs> {
        FLT2RSTM_W::new(self, 15)
    }
    ///Bit 16 - FLT3BLKE
    #[inline(always)]
    #[must_use]
    pub fn flt3blke(&mut self) -> FLT3BLKE_W<FLTINR3rs> {
        FLT3BLKE_W::new(self, 16)
    }
    ///Bit 17 - FLT3BLKS
    #[inline(always)]
    #[must_use]
    pub fn flt3blks(&mut self) -> FLT3BLKS_W<FLTINR3rs> {
        FLT3BLKS_W::new(self, 17)
    }
    ///Bits 18:21 - FLT3CNT
    #[inline(always)]
    #[must_use]
    pub fn flt3cnt(&mut self) -> FLT3CNT_W<FLTINR3rs> {
        FLT3CNT_W::new(self, 18)
    }
    ///Bit 22 - FLT3CRES
    #[inline(always)]
    #[must_use]
    pub fn flt3cres(&mut self) -> FLT3CRES_W<FLTINR3rs> {
        FLT3CRES_W::new(self, 22)
    }
    ///Bit 23 - FLT3RSTM
    #[inline(always)]
    #[must_use]
    pub fn flt3rstm(&mut self) -> FLT3RSTM_W<FLTINR3rs> {
        FLT3RSTM_W::new(self, 23)
    }
    ///Bit 24 - FLT4BLKE
    #[inline(always)]
    #[must_use]
    pub fn flt4blke(&mut self) -> FLT4BLKE_W<FLTINR3rs> {
        FLT4BLKE_W::new(self, 24)
    }
    ///Bit 25 - FLT4BLKS
    #[inline(always)]
    #[must_use]
    pub fn flt4blks(&mut self) -> FLT4BLKS_W<FLTINR3rs> {
        FLT4BLKS_W::new(self, 25)
    }
    ///Bits 26:29 - FLT4CNT
    #[inline(always)]
    #[must_use]
    pub fn flt4cnt(&mut self) -> FLT4CNT_W<FLTINR3rs> {
        FLT4CNT_W::new(self, 26)
    }
    ///Bit 30 - FLT4CRES
    #[inline(always)]
    #[must_use]
    pub fn flt4cres(&mut self) -> FLT4CRES_W<FLTINR3rs> {
        FLT4CRES_W::new(self, 30)
    }
    ///Bit 31 - FLT4RSTM
    #[inline(always)]
    #[must_use]
    pub fn flt4rstm(&mut self) -> FLT4RSTM_W<FLTINR3rs> {
        FLT4RSTM_W::new(self, 31)
    }
}
/**HRTIM Fault Input Register 3

You can [`read`](crate::Reg::read) this register and get [`fltinr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltinr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_Common:FLTINR3)*/
pub struct FLTINR3rs;
impl crate::RegisterSpec for FLTINR3rs {
    type Ux = u32;
}
///`read()` method returns [`fltinr3::R`](R) reader structure
impl crate::Readable for FLTINR3rs {}
///`write(|w| ..)` method takes [`fltinr3::W`](W) writer structure
impl crate::Writable for FLTINR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLTINR3 to value 0
impl crate::Resettable for FLTINR3rs {
    const RESET_VALUE: u32 = 0;
}
