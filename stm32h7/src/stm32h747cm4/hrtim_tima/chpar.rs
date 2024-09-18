///Register `CHPAR` reader
pub type R = crate::R<CHPARrs>;
///Register `CHPAR` writer
pub type W = crate::W<CHPARrs>;
///Field `CHPFRQ` reader - Timerx carrier frequency value
pub type CHPFRQ_R = crate::FieldReader;
///Field `CHPFRQ` writer - Timerx carrier frequency value
pub type CHPFRQ_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CHPDTY` reader - Timerx chopper duty cycle value
pub type CHPDTY_R = crate::FieldReader;
///Field `CHPDTY` writer - Timerx chopper duty cycle value
pub type CHPDTY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `STRTPW` reader - STRTPW
pub type STRTPW_R = crate::FieldReader;
///Field `STRTPW` writer - STRTPW
pub type STRTPW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Timerx carrier frequency value
    #[inline(always)]
    pub fn chpfrq(&self) -> CHPFRQ_R {
        CHPFRQ_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - Timerx chopper duty cycle value
    #[inline(always)]
    pub fn chpdty(&self) -> CHPDTY_R {
        CHPDTY_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:10 - STRTPW
    #[inline(always)]
    pub fn strtpw(&self) -> STRTPW_R {
        STRTPW_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHPAR")
            .field("strtpw", &self.strtpw())
            .field("chpdty", &self.chpdty())
            .field("chpfrq", &self.chpfrq())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Timerx carrier frequency value
    #[inline(always)]
    #[must_use]
    pub fn chpfrq(&mut self) -> CHPFRQ_W<CHPARrs> {
        CHPFRQ_W::new(self, 0)
    }
    ///Bits 4:6 - Timerx chopper duty cycle value
    #[inline(always)]
    #[must_use]
    pub fn chpdty(&mut self) -> CHPDTY_W<CHPARrs> {
        CHPDTY_W::new(self, 4)
    }
    ///Bits 7:10 - STRTPW
    #[inline(always)]
    #[must_use]
    pub fn strtpw(&mut self) -> STRTPW_W<CHPARrs> {
        STRTPW_W::new(self, 7)
    }
}
/**Timerx Chopper Register

You can [`read`](crate::Reg::read) this register and get [`chpar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chpar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#HRTIM_TIMA:CHPAR)*/
pub struct CHPARrs;
impl crate::RegisterSpec for CHPARrs {
    type Ux = u32;
}
///`read()` method returns [`chpar::R`](R) reader structure
impl crate::Readable for CHPARrs {}
///`write(|w| ..)` method takes [`chpar::W`](W) writer structure
impl crate::Writable for CHPARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CHPAR to value 0
impl crate::Resettable for CHPARrs {
    const RESET_VALUE: u32 = 0;
}
