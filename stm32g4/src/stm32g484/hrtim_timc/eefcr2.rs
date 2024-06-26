///Register `EEFCR2` reader
pub type R = crate::R<EEFCR2rs>;
///Register `EEFCR2` writer
pub type W = crate::W<EEFCR2rs>;
///Field `EE6LTCH` reader - External Event 6 latch
pub type EE6LTCH_R = crate::BitReader;
///Field `EE6LTCH` writer - External Event 6 latch
pub type EE6LTCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EE6FLTR` reader - External Event 6 filter
pub type EE6FLTR_R = crate::FieldReader;
///Field `EE6FLTR` writer - External Event 6 filter
pub type EE6FLTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EE7LTCH` reader - External Event 7 latch
pub type EE7LTCH_R = crate::BitReader;
///Field `EE7LTCH` writer - External Event 7 latch
pub type EE7LTCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EE7FLTR` reader - External Event 7 filter
pub type EE7FLTR_R = crate::FieldReader;
///Field `EE7FLTR` writer - External Event 7 filter
pub type EE7FLTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EE8LTCH` reader - External Event 8 latch
pub type EE8LTCH_R = crate::BitReader;
///Field `EE8LTCH` writer - External Event 8 latch
pub type EE8LTCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EE8FLTR` reader - External Event 8 filter
pub type EE8FLTR_R = crate::FieldReader;
///Field `EE8FLTR` writer - External Event 8 filter
pub type EE8FLTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EE9LTCH` reader - External Event 9 latch
pub type EE9LTCH_R = crate::BitReader;
///Field `EE9LTCH` writer - External Event 9 latch
pub type EE9LTCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EE9FLTR` reader - External Event 9 filter
pub type EE9FLTR_R = crate::FieldReader;
///Field `EE9FLTR` writer - External Event 9 filter
pub type EE9FLTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EE10LTCH` reader - External Event 10 latch
pub type EE10LTCH_R = crate::BitReader;
///Field `EE10LTCH` writer - External Event 10 latch
pub type EE10LTCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EE10FLTR` reader - External Event 10 filter
pub type EE10FLTR_R = crate::FieldReader;
///Field `EE10FLTR` writer - External Event 10 filter
pub type EE10FLTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - External Event 6 latch
    #[inline(always)]
    pub fn ee6ltch(&self) -> EE6LTCH_R {
        EE6LTCH_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:4 - External Event 6 filter
    #[inline(always)]
    pub fn ee6fltr(&self) -> EE6FLTR_R {
        EE6FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bit 6 - External Event 7 latch
    #[inline(always)]
    pub fn ee7ltch(&self) -> EE7LTCH_R {
        EE7LTCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:10 - External Event 7 filter
    #[inline(always)]
    pub fn ee7fltr(&self) -> EE7FLTR_R {
        EE7FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    ///Bit 12 - External Event 8 latch
    #[inline(always)]
    pub fn ee8ltch(&self) -> EE8LTCH_R {
        EE8LTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:16 - External Event 8 filter
    #[inline(always)]
    pub fn ee8fltr(&self) -> EE8FLTR_R {
        EE8FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bit 18 - External Event 9 latch
    #[inline(always)]
    pub fn ee9ltch(&self) -> EE9LTCH_R {
        EE9LTCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:22 - External Event 9 filter
    #[inline(always)]
    pub fn ee9fltr(&self) -> EE9FLTR_R {
        EE9FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    ///Bit 24 - External Event 10 latch
    #[inline(always)]
    pub fn ee10ltch(&self) -> EE10LTCH_R {
        EE10LTCH_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:28 - External Event 10 filter
    #[inline(always)]
    pub fn ee10fltr(&self) -> EE10FLTR_R {
        EE10FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EEFCR2")
            .field("ee10fltr", &self.ee10fltr())
            .field("ee10ltch", &self.ee10ltch())
            .field("ee9fltr", &self.ee9fltr())
            .field("ee9ltch", &self.ee9ltch())
            .field("ee8fltr", &self.ee8fltr())
            .field("ee8ltch", &self.ee8ltch())
            .field("ee7fltr", &self.ee7fltr())
            .field("ee7ltch", &self.ee7ltch())
            .field("ee6fltr", &self.ee6fltr())
            .field("ee6ltch", &self.ee6ltch())
            .finish()
    }
}
impl W {
    ///Bit 0 - External Event 6 latch
    #[inline(always)]
    #[must_use]
    pub fn ee6ltch(&mut self) -> EE6LTCH_W<EEFCR2rs> {
        EE6LTCH_W::new(self, 0)
    }
    ///Bits 1:4 - External Event 6 filter
    #[inline(always)]
    #[must_use]
    pub fn ee6fltr(&mut self) -> EE6FLTR_W<EEFCR2rs> {
        EE6FLTR_W::new(self, 1)
    }
    ///Bit 6 - External Event 7 latch
    #[inline(always)]
    #[must_use]
    pub fn ee7ltch(&mut self) -> EE7LTCH_W<EEFCR2rs> {
        EE7LTCH_W::new(self, 6)
    }
    ///Bits 7:10 - External Event 7 filter
    #[inline(always)]
    #[must_use]
    pub fn ee7fltr(&mut self) -> EE7FLTR_W<EEFCR2rs> {
        EE7FLTR_W::new(self, 7)
    }
    ///Bit 12 - External Event 8 latch
    #[inline(always)]
    #[must_use]
    pub fn ee8ltch(&mut self) -> EE8LTCH_W<EEFCR2rs> {
        EE8LTCH_W::new(self, 12)
    }
    ///Bits 13:16 - External Event 8 filter
    #[inline(always)]
    #[must_use]
    pub fn ee8fltr(&mut self) -> EE8FLTR_W<EEFCR2rs> {
        EE8FLTR_W::new(self, 13)
    }
    ///Bit 18 - External Event 9 latch
    #[inline(always)]
    #[must_use]
    pub fn ee9ltch(&mut self) -> EE9LTCH_W<EEFCR2rs> {
        EE9LTCH_W::new(self, 18)
    }
    ///Bits 19:22 - External Event 9 filter
    #[inline(always)]
    #[must_use]
    pub fn ee9fltr(&mut self) -> EE9FLTR_W<EEFCR2rs> {
        EE9FLTR_W::new(self, 19)
    }
    ///Bit 24 - External Event 10 latch
    #[inline(always)]
    #[must_use]
    pub fn ee10ltch(&mut self) -> EE10LTCH_W<EEFCR2rs> {
        EE10LTCH_W::new(self, 24)
    }
    ///Bits 25:28 - External Event 10 filter
    #[inline(always)]
    #[must_use]
    pub fn ee10fltr(&mut self) -> EE10FLTR_W<EEFCR2rs> {
        EE10FLTR_W::new(self, 25)
    }
}
/**Timerx External Event Filtering Register 2

You can [`read`](crate::Reg::read) this register and get [`eefcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484xx.html#HRTIM_TIMC:EEFCR2)*/
pub struct EEFCR2rs;
impl crate::RegisterSpec for EEFCR2rs {
    type Ux = u32;
}
///`read()` method returns [`eefcr2::R`](R) reader structure
impl crate::Readable for EEFCR2rs {}
///`write(|w| ..)` method takes [`eefcr2::W`](W) writer structure
impl crate::Writable for EEFCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EEFCR2 to value 0
impl crate::Resettable for EEFCR2rs {
    const RESET_VALUE: u32 = 0;
}
