///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
///Field `PPRE1` reader - CPU domain APB1 prescaler
pub type PPRE1_R = crate::FieldReader;
///Field `PPRE1` writer - CPU domain APB1 prescaler
pub type PPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PPRE2` reader - CPU domain APB2 prescaler
pub type PPRE2_R = crate::FieldReader;
///Field `PPRE2` writer - CPU domain APB2 prescaler
pub type PPRE2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PPRE4` reader - CPU domain APB4 prescaler
pub type PPRE4_R = crate::FieldReader;
///Field `PPRE4` writer - CPU domain APB4 prescaler
pub type PPRE4_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PPRE5` reader - CPU domain APB5 prescaler
pub type PPRE5_R = crate::FieldReader;
///Field `PPRE5` writer - CPU domain APB5 prescaler
pub type PPRE5_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HPRE` reader - AHB clock prescaler
pub type HPRE_R = crate::FieldReader;
///Field `HPRE` writer - AHB clock prescaler
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TIMPRE` reader - Timers clocks prescaler selection
pub type TIMPRE_R = crate::FieldReader;
///Field `TIMPRE` writer - Timers clocks prescaler selection
pub type TIMPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - CPU domain APB1 prescaler
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - CPU domain APB2 prescaler
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 12:14 - CPU domain APB4 prescaler
    #[inline(always)]
    pub fn ppre4(&self) -> PPRE4_R {
        PPRE4_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:18 - CPU domain APB5 prescaler
    #[inline(always)]
    pub fn ppre5(&self) -> PPRE5_R {
        PPRE5_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - AHB clock prescaler
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:25 - Timers clocks prescaler selection
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("ppre1", &self.ppre1())
            .field("ppre2", &self.ppre2())
            .field("ppre4", &self.ppre4())
            .field("ppre5", &self.ppre5())
            .field("hpre", &self.hpre())
            .field("timpre", &self.timpre())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - CPU domain APB1 prescaler
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W<'_, CFGR2rs> {
        PPRE1_W::new(self, 0)
    }
    ///Bits 4:6 - CPU domain APB2 prescaler
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W<'_, CFGR2rs> {
        PPRE2_W::new(self, 4)
    }
    ///Bits 12:14 - CPU domain APB4 prescaler
    #[inline(always)]
    pub fn ppre4(&mut self) -> PPRE4_W<'_, CFGR2rs> {
        PPRE4_W::new(self, 12)
    }
    ///Bits 16:18 - CPU domain APB5 prescaler
    #[inline(always)]
    pub fn ppre5(&mut self) -> PPRE5_W<'_, CFGR2rs> {
        PPRE5_W::new(self, 16)
    }
    ///Bits 20:22 - AHB clock prescaler
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W<'_, CFGR2rs> {
        HPRE_W::new(self, 20)
    }
    ///Bits 24:25 - Timers clocks prescaler selection
    #[inline(always)]
    pub fn timpre(&mut self) -> TIMPRE_W<'_, CFGR2rs> {
        TIMPRE_W::new(self, 24)
    }
}
/**RCC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
