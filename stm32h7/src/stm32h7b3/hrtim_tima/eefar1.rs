///Register `EEFAR1` reader
pub type R = crate::R<EEFAR1rs>;
///Register `EEFAR1` writer
pub type W = crate::W<EEFAR1rs>;
///Field `EE1LTCH` reader - External Event 1 latch
pub type EE1LTCH_R = crate::BitReader;
///Field `EE1LTCH` writer - External Event 1 latch
pub type EE1LTCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EE1FLTR` reader - External Event 1 filter
pub type EE1FLTR_R = crate::FieldReader;
///Field `EE1FLTR` writer - External Event 1 filter
pub type EE1FLTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EE2LTCH` reader - External Event 2 latch
pub type EE2LTCH_R = crate::BitReader;
///Field `EE2LTCH` writer - External Event 2 latch
pub type EE2LTCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EE2FLTR` reader - External Event 2 filter
pub type EE2FLTR_R = crate::FieldReader;
///Field `EE2FLTR` writer - External Event 2 filter
pub type EE2FLTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EE3LTCH` reader - External Event 3 latch
pub type EE3LTCH_R = crate::BitReader;
///Field `EE3LTCH` writer - External Event 3 latch
pub type EE3LTCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EE3FLTR` reader - External Event 3 filter
pub type EE3FLTR_R = crate::FieldReader;
///Field `EE3FLTR` writer - External Event 3 filter
pub type EE3FLTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EE4LTCH` reader - External Event 4 latch
pub type EE4LTCH_R = crate::BitReader;
///Field `EE4LTCH` writer - External Event 4 latch
pub type EE4LTCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EE4FLTR` reader - External Event 4 filter
pub type EE4FLTR_R = crate::FieldReader;
///Field `EE4FLTR` writer - External Event 4 filter
pub type EE4FLTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EE5LTCH` reader - External Event 5 latch
pub type EE5LTCH_R = crate::BitReader;
///Field `EE5LTCH` writer - External Event 5 latch
pub type EE5LTCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EE5FLTR` reader - External Event 5 filter
pub type EE5FLTR_R = crate::FieldReader;
///Field `EE5FLTR` writer - External Event 5 filter
pub type EE5FLTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - External Event 1 latch
    #[inline(always)]
    pub fn ee1ltch(&self) -> EE1LTCH_R {
        EE1LTCH_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:4 - External Event 1 filter
    #[inline(always)]
    pub fn ee1fltr(&self) -> EE1FLTR_R {
        EE1FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bit 6 - External Event 2 latch
    #[inline(always)]
    pub fn ee2ltch(&self) -> EE2LTCH_R {
        EE2LTCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:10 - External Event 2 filter
    #[inline(always)]
    pub fn ee2fltr(&self) -> EE2FLTR_R {
        EE2FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    ///Bit 12 - External Event 3 latch
    #[inline(always)]
    pub fn ee3ltch(&self) -> EE3LTCH_R {
        EE3LTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:16 - External Event 3 filter
    #[inline(always)]
    pub fn ee3fltr(&self) -> EE3FLTR_R {
        EE3FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bit 18 - External Event 4 latch
    #[inline(always)]
    pub fn ee4ltch(&self) -> EE4LTCH_R {
        EE4LTCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:22 - External Event 4 filter
    #[inline(always)]
    pub fn ee4fltr(&self) -> EE4FLTR_R {
        EE4FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    ///Bit 24 - External Event 5 latch
    #[inline(always)]
    pub fn ee5ltch(&self) -> EE5LTCH_R {
        EE5LTCH_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:28 - External Event 5 filter
    #[inline(always)]
    pub fn ee5fltr(&self) -> EE5FLTR_R {
        EE5FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EEFAR1")
            .field("ee5fltr", &self.ee5fltr())
            .field("ee5ltch", &self.ee5ltch())
            .field("ee4fltr", &self.ee4fltr())
            .field("ee4ltch", &self.ee4ltch())
            .field("ee3fltr", &self.ee3fltr())
            .field("ee3ltch", &self.ee3ltch())
            .field("ee2fltr", &self.ee2fltr())
            .field("ee2ltch", &self.ee2ltch())
            .field("ee1fltr", &self.ee1fltr())
            .field("ee1ltch", &self.ee1ltch())
            .finish()
    }
}
impl W {
    ///Bit 0 - External Event 1 latch
    #[inline(always)]
    #[must_use]
    pub fn ee1ltch(&mut self) -> EE1LTCH_W<EEFAR1rs> {
        EE1LTCH_W::new(self, 0)
    }
    ///Bits 1:4 - External Event 1 filter
    #[inline(always)]
    #[must_use]
    pub fn ee1fltr(&mut self) -> EE1FLTR_W<EEFAR1rs> {
        EE1FLTR_W::new(self, 1)
    }
    ///Bit 6 - External Event 2 latch
    #[inline(always)]
    #[must_use]
    pub fn ee2ltch(&mut self) -> EE2LTCH_W<EEFAR1rs> {
        EE2LTCH_W::new(self, 6)
    }
    ///Bits 7:10 - External Event 2 filter
    #[inline(always)]
    #[must_use]
    pub fn ee2fltr(&mut self) -> EE2FLTR_W<EEFAR1rs> {
        EE2FLTR_W::new(self, 7)
    }
    ///Bit 12 - External Event 3 latch
    #[inline(always)]
    #[must_use]
    pub fn ee3ltch(&mut self) -> EE3LTCH_W<EEFAR1rs> {
        EE3LTCH_W::new(self, 12)
    }
    ///Bits 13:16 - External Event 3 filter
    #[inline(always)]
    #[must_use]
    pub fn ee3fltr(&mut self) -> EE3FLTR_W<EEFAR1rs> {
        EE3FLTR_W::new(self, 13)
    }
    ///Bit 18 - External Event 4 latch
    #[inline(always)]
    #[must_use]
    pub fn ee4ltch(&mut self) -> EE4LTCH_W<EEFAR1rs> {
        EE4LTCH_W::new(self, 18)
    }
    ///Bits 19:22 - External Event 4 filter
    #[inline(always)]
    #[must_use]
    pub fn ee4fltr(&mut self) -> EE4FLTR_W<EEFAR1rs> {
        EE4FLTR_W::new(self, 19)
    }
    ///Bit 24 - External Event 5 latch
    #[inline(always)]
    #[must_use]
    pub fn ee5ltch(&mut self) -> EE5LTCH_W<EEFAR1rs> {
        EE5LTCH_W::new(self, 24)
    }
    ///Bits 25:28 - External Event 5 filter
    #[inline(always)]
    #[must_use]
    pub fn ee5fltr(&mut self) -> EE5FLTR_W<EEFAR1rs> {
        EE5FLTR_W::new(self, 25)
    }
}
/**Timerx External Event Filtering Register 1

You can [`read`](crate::Reg::read) this register and get [`eefar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_TIMA:EEFAR1)*/
pub struct EEFAR1rs;
impl crate::RegisterSpec for EEFAR1rs {
    type Ux = u32;
}
///`read()` method returns [`eefar1::R`](R) reader structure
impl crate::Readable for EEFAR1rs {}
///`write(|w| ..)` method takes [`eefar1::W`](W) writer structure
impl crate::Writable for EEFAR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EEFAR1 to value 0
impl crate::Resettable for EEFAR1rs {
    const RESET_VALUE: u32 = 0;
}
