///Register `SMPR2` reader
pub type R = crate::R<SMPR2rs>;
///Register `SMPR2` writer
pub type W = crate::W<SMPR2rs>;
///Field `SMP10` reader - SMP10
pub type SMP10_R = crate::FieldReader;
///Field `SMP10` writer - SMP10
pub type SMP10_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP11` reader - SMP11
pub type SMP11_R = crate::FieldReader;
///Field `SMP11` writer - SMP11
pub type SMP11_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP12` reader - SMP12
pub type SMP12_R = crate::FieldReader;
///Field `SMP12` writer - SMP12
pub type SMP12_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP13` reader - SMP13
pub type SMP13_R = crate::FieldReader;
///Field `SMP13` writer - SMP13
pub type SMP13_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP14` reader - SMP14
pub type SMP14_R = crate::FieldReader;
///Field `SMP14` writer - SMP14
pub type SMP14_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP15` reader - SMP15
pub type SMP15_R = crate::FieldReader;
///Field `SMP15` writer - SMP15
pub type SMP15_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP16` reader - SMP16
pub type SMP16_R = crate::FieldReader;
///Field `SMP16` writer - SMP16
pub type SMP16_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP17` reader - SMP17
pub type SMP17_R = crate::FieldReader;
///Field `SMP17` writer - SMP17
pub type SMP17_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP18` reader - SMP18
pub type SMP18_R = crate::FieldReader;
///Field `SMP18` writer - SMP18
pub type SMP18_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP19` reader - SMP19
pub type SMP19_R = crate::FieldReader;
///Field `SMP19` writer - SMP19
pub type SMP19_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - SMP10
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - SMP11
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - SMP12
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - SMP13
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - SMP14
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - SMP15
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - SMP16
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - SMP17
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:26 - SMP18
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - SMP19
    #[inline(always)]
    pub fn smp19(&self) -> SMP19_R {
        SMP19_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR2")
            .field("smp10", &self.smp10())
            .field("smp11", &self.smp11())
            .field("smp12", &self.smp12())
            .field("smp13", &self.smp13())
            .field("smp14", &self.smp14())
            .field("smp15", &self.smp15())
            .field("smp16", &self.smp16())
            .field("smp17", &self.smp17())
            .field("smp18", &self.smp18())
            .field("smp19", &self.smp19())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SMP10
    #[inline(always)]
    pub fn smp10(&mut self) -> SMP10_W<'_, SMPR2rs> {
        SMP10_W::new(self, 0)
    }
    ///Bits 3:5 - SMP11
    #[inline(always)]
    pub fn smp11(&mut self) -> SMP11_W<'_, SMPR2rs> {
        SMP11_W::new(self, 3)
    }
    ///Bits 6:8 - SMP12
    #[inline(always)]
    pub fn smp12(&mut self) -> SMP12_W<'_, SMPR2rs> {
        SMP12_W::new(self, 6)
    }
    ///Bits 9:11 - SMP13
    #[inline(always)]
    pub fn smp13(&mut self) -> SMP13_W<'_, SMPR2rs> {
        SMP13_W::new(self, 9)
    }
    ///Bits 12:14 - SMP14
    #[inline(always)]
    pub fn smp14(&mut self) -> SMP14_W<'_, SMPR2rs> {
        SMP14_W::new(self, 12)
    }
    ///Bits 15:17 - SMP15
    #[inline(always)]
    pub fn smp15(&mut self) -> SMP15_W<'_, SMPR2rs> {
        SMP15_W::new(self, 15)
    }
    ///Bits 18:20 - SMP16
    #[inline(always)]
    pub fn smp16(&mut self) -> SMP16_W<'_, SMPR2rs> {
        SMP16_W::new(self, 18)
    }
    ///Bits 21:23 - SMP17
    #[inline(always)]
    pub fn smp17(&mut self) -> SMP17_W<'_, SMPR2rs> {
        SMP17_W::new(self, 21)
    }
    ///Bits 24:26 - SMP18
    #[inline(always)]
    pub fn smp18(&mut self) -> SMP18_W<'_, SMPR2rs> {
        SMP18_W::new(self, 24)
    }
    ///Bits 27:29 - SMP19
    #[inline(always)]
    pub fn smp19(&mut self) -> SMP19_W<'_, SMPR2rs> {
        SMP19_W::new(self, 27)
    }
}
/**ADC sample time register 2

You can [`read`](crate::Reg::read) this register and get [`smpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ADC:SMPR2)*/
pub struct SMPR2rs;
impl crate::RegisterSpec for SMPR2rs {
    type Ux = u32;
}
///`read()` method returns [`smpr2::R`](R) reader structure
impl crate::Readable for SMPR2rs {}
///`write(|w| ..)` method takes [`smpr2::W`](W) writer structure
impl crate::Writable for SMPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMPR2 to value 0
impl crate::Resettable for SMPR2rs {}
