///Register `FLTINR1` reader
pub type R = crate::R<FLTINR1rs>;
///Register `FLTINR1` writer
pub type W = crate::W<FLTINR1rs>;
///Field `FLT1E` reader - FLT1E
pub type FLT1E_R = crate::BitReader;
///Field `FLT1E` writer - FLT1E
pub type FLT1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT1P` reader - FLT1P
pub type FLT1P_R = crate::BitReader;
///Field `FLT1P` writer - FLT1P
pub type FLT1P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT1SRC` reader - FLT1SRC
pub type FLT1SRC_R = crate::BitReader;
///Field `FLT1SRC` writer - FLT1SRC
pub type FLT1SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT1F` reader - FLT1F
pub type FLT1F_R = crate::FieldReader;
///Field `FLT1F` writer - FLT1F
pub type FLT1F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FLT1LCK` reader - FLT1LCK
pub type FLT1LCK_R = crate::BitReader;
///Field `FLT1LCK` writer - FLT1LCK
pub type FLT1LCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT2E` reader - FLT2E
pub type FLT2E_R = crate::BitReader;
///Field `FLT2E` writer - FLT2E
pub type FLT2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT2P` reader - FLT2P
pub type FLT2P_R = crate::BitReader;
///Field `FLT2P` writer - FLT2P
pub type FLT2P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT2SRC` reader - FLT2SRC
pub type FLT2SRC_R = crate::BitReader;
///Field `FLT2SRC` writer - FLT2SRC
pub type FLT2SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT2F` reader - FLT2F
pub type FLT2F_R = crate::FieldReader;
///Field `FLT2F` writer - FLT2F
pub type FLT2F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FLT2LCK` reader - FLT2LCK
pub type FLT2LCK_R = crate::BitReader;
///Field `FLT2LCK` writer - FLT2LCK
pub type FLT2LCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT3E` reader - FLT3E
pub type FLT3E_R = crate::BitReader;
///Field `FLT3E` writer - FLT3E
pub type FLT3E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT3P` reader - FLT3P
pub type FLT3P_R = crate::BitReader;
///Field `FLT3P` writer - FLT3P
pub type FLT3P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT3SRC` reader - FLT3SRC
pub type FLT3SRC_R = crate::BitReader;
///Field `FLT3SRC` writer - FLT3SRC
pub type FLT3SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT3F` reader - FLT3F
pub type FLT3F_R = crate::FieldReader;
///Field `FLT3F` writer - FLT3F
pub type FLT3F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FLT3LCK` reader - FLT3LCK
pub type FLT3LCK_R = crate::BitReader;
///Field `FLT3LCK` writer - FLT3LCK
pub type FLT3LCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT4E` reader - FLT4E
pub type FLT4E_R = crate::BitReader;
///Field `FLT4E` writer - FLT4E
pub type FLT4E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT4P` reader - FLT4P
pub type FLT4P_R = crate::BitReader;
///Field `FLT4P` writer - FLT4P
pub type FLT4P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT4SRC` reader - FLT4SRC
pub type FLT4SRC_R = crate::BitReader;
///Field `FLT4SRC` writer - FLT4SRC
pub type FLT4SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT4F` reader - FLT4F
pub type FLT4F_R = crate::FieldReader;
///Field `FLT4F` writer - FLT4F
pub type FLT4F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FLT4LCK` reader - FLT4LCK
pub type FLT4LCK_R = crate::BitReader;
///Field `FLT4LCK` writer - FLT4LCK
pub type FLT4LCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - FLT1E
    #[inline(always)]
    pub fn flt1e(&self) -> FLT1E_R {
        FLT1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FLT1P
    #[inline(always)]
    pub fn flt1p(&self) -> FLT1P_R {
        FLT1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FLT1SRC
    #[inline(always)]
    pub fn flt1src(&self) -> FLT1SRC_R {
        FLT1SRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:6 - FLT1F
    #[inline(always)]
    pub fn flt1f(&self) -> FLT1F_R {
        FLT1F_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    ///Bit 7 - FLT1LCK
    #[inline(always)]
    pub fn flt1lck(&self) -> FLT1LCK_R {
        FLT1LCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - FLT2E
    #[inline(always)]
    pub fn flt2e(&self) -> FLT2E_R {
        FLT2E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - FLT2P
    #[inline(always)]
    pub fn flt2p(&self) -> FLT2P_R {
        FLT2P_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - FLT2SRC
    #[inline(always)]
    pub fn flt2src(&self) -> FLT2SRC_R {
        FLT2SRC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:14 - FLT2F
    #[inline(always)]
    pub fn flt2f(&self) -> FLT2F_R {
        FLT2F_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    ///Bit 15 - FLT2LCK
    #[inline(always)]
    pub fn flt2lck(&self) -> FLT2LCK_R {
        FLT2LCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - FLT3E
    #[inline(always)]
    pub fn flt3e(&self) -> FLT3E_R {
        FLT3E_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - FLT3P
    #[inline(always)]
    pub fn flt3p(&self) -> FLT3P_R {
        FLT3P_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - FLT3SRC
    #[inline(always)]
    pub fn flt3src(&self) -> FLT3SRC_R {
        FLT3SRC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:22 - FLT3F
    #[inline(always)]
    pub fn flt3f(&self) -> FLT3F_R {
        FLT3F_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    ///Bit 23 - FLT3LCK
    #[inline(always)]
    pub fn flt3lck(&self) -> FLT3LCK_R {
        FLT3LCK_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - FLT4E
    #[inline(always)]
    pub fn flt4e(&self) -> FLT4E_R {
        FLT4E_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - FLT4P
    #[inline(always)]
    pub fn flt4p(&self) -> FLT4P_R {
        FLT4P_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - FLT4SRC
    #[inline(always)]
    pub fn flt4src(&self) -> FLT4SRC_R {
        FLT4SRC_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:30 - FLT4F
    #[inline(always)]
    pub fn flt4f(&self) -> FLT4F_R {
        FLT4F_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
    ///Bit 31 - FLT4LCK
    #[inline(always)]
    pub fn flt4lck(&self) -> FLT4LCK_R {
        FLT4LCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTINR1")
            .field("flt4lck", &self.flt4lck())
            .field("flt4f", &self.flt4f())
            .field("flt4src", &self.flt4src())
            .field("flt4p", &self.flt4p())
            .field("flt4e", &self.flt4e())
            .field("flt3lck", &self.flt3lck())
            .field("flt3f", &self.flt3f())
            .field("flt3src", &self.flt3src())
            .field("flt3p", &self.flt3p())
            .field("flt3e", &self.flt3e())
            .field("flt2lck", &self.flt2lck())
            .field("flt2f", &self.flt2f())
            .field("flt2src", &self.flt2src())
            .field("flt2p", &self.flt2p())
            .field("flt2e", &self.flt2e())
            .field("flt1lck", &self.flt1lck())
            .field("flt1f", &self.flt1f())
            .field("flt1src", &self.flt1src())
            .field("flt1p", &self.flt1p())
            .field("flt1e", &self.flt1e())
            .finish()
    }
}
impl W {
    ///Bit 0 - FLT1E
    #[inline(always)]
    #[must_use]
    pub fn flt1e(&mut self) -> FLT1E_W<FLTINR1rs> {
        FLT1E_W::new(self, 0)
    }
    ///Bit 1 - FLT1P
    #[inline(always)]
    #[must_use]
    pub fn flt1p(&mut self) -> FLT1P_W<FLTINR1rs> {
        FLT1P_W::new(self, 1)
    }
    ///Bit 2 - FLT1SRC
    #[inline(always)]
    #[must_use]
    pub fn flt1src(&mut self) -> FLT1SRC_W<FLTINR1rs> {
        FLT1SRC_W::new(self, 2)
    }
    ///Bits 3:6 - FLT1F
    #[inline(always)]
    #[must_use]
    pub fn flt1f(&mut self) -> FLT1F_W<FLTINR1rs> {
        FLT1F_W::new(self, 3)
    }
    ///Bit 7 - FLT1LCK
    #[inline(always)]
    #[must_use]
    pub fn flt1lck(&mut self) -> FLT1LCK_W<FLTINR1rs> {
        FLT1LCK_W::new(self, 7)
    }
    ///Bit 8 - FLT2E
    #[inline(always)]
    #[must_use]
    pub fn flt2e(&mut self) -> FLT2E_W<FLTINR1rs> {
        FLT2E_W::new(self, 8)
    }
    ///Bit 9 - FLT2P
    #[inline(always)]
    #[must_use]
    pub fn flt2p(&mut self) -> FLT2P_W<FLTINR1rs> {
        FLT2P_W::new(self, 9)
    }
    ///Bit 10 - FLT2SRC
    #[inline(always)]
    #[must_use]
    pub fn flt2src(&mut self) -> FLT2SRC_W<FLTINR1rs> {
        FLT2SRC_W::new(self, 10)
    }
    ///Bits 11:14 - FLT2F
    #[inline(always)]
    #[must_use]
    pub fn flt2f(&mut self) -> FLT2F_W<FLTINR1rs> {
        FLT2F_W::new(self, 11)
    }
    ///Bit 15 - FLT2LCK
    #[inline(always)]
    #[must_use]
    pub fn flt2lck(&mut self) -> FLT2LCK_W<FLTINR1rs> {
        FLT2LCK_W::new(self, 15)
    }
    ///Bit 16 - FLT3E
    #[inline(always)]
    #[must_use]
    pub fn flt3e(&mut self) -> FLT3E_W<FLTINR1rs> {
        FLT3E_W::new(self, 16)
    }
    ///Bit 17 - FLT3P
    #[inline(always)]
    #[must_use]
    pub fn flt3p(&mut self) -> FLT3P_W<FLTINR1rs> {
        FLT3P_W::new(self, 17)
    }
    ///Bit 18 - FLT3SRC
    #[inline(always)]
    #[must_use]
    pub fn flt3src(&mut self) -> FLT3SRC_W<FLTINR1rs> {
        FLT3SRC_W::new(self, 18)
    }
    ///Bits 19:22 - FLT3F
    #[inline(always)]
    #[must_use]
    pub fn flt3f(&mut self) -> FLT3F_W<FLTINR1rs> {
        FLT3F_W::new(self, 19)
    }
    ///Bit 23 - FLT3LCK
    #[inline(always)]
    #[must_use]
    pub fn flt3lck(&mut self) -> FLT3LCK_W<FLTINR1rs> {
        FLT3LCK_W::new(self, 23)
    }
    ///Bit 24 - FLT4E
    #[inline(always)]
    #[must_use]
    pub fn flt4e(&mut self) -> FLT4E_W<FLTINR1rs> {
        FLT4E_W::new(self, 24)
    }
    ///Bit 25 - FLT4P
    #[inline(always)]
    #[must_use]
    pub fn flt4p(&mut self) -> FLT4P_W<FLTINR1rs> {
        FLT4P_W::new(self, 25)
    }
    ///Bit 26 - FLT4SRC
    #[inline(always)]
    #[must_use]
    pub fn flt4src(&mut self) -> FLT4SRC_W<FLTINR1rs> {
        FLT4SRC_W::new(self, 26)
    }
    ///Bits 27:30 - FLT4F
    #[inline(always)]
    #[must_use]
    pub fn flt4f(&mut self) -> FLT4F_W<FLTINR1rs> {
        FLT4F_W::new(self, 27)
    }
    ///Bit 31 - FLT4LCK
    #[inline(always)]
    #[must_use]
    pub fn flt4lck(&mut self) -> FLT4LCK_W<FLTINR1rs> {
        FLT4LCK_W::new(self, 31)
    }
}
/**HRTIM Fault Input Register 1

You can [`read`](crate::Reg::read) this register and get [`fltinr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltinr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#HRTIM_Common:FLTINR1)*/
pub struct FLTINR1rs;
impl crate::RegisterSpec for FLTINR1rs {
    type Ux = u32;
}
///`read()` method returns [`fltinr1::R`](R) reader structure
impl crate::Readable for FLTINR1rs {}
///`write(|w| ..)` method takes [`fltinr1::W`](W) writer structure
impl crate::Writable for FLTINR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLTINR1 to value 0
impl crate::Resettable for FLTINR1rs {
    const RESET_VALUE: u32 = 0;
}
