///Register `GPIOZ_AFRL` reader
pub type R = crate::R<GPIOZ_AFRLrs>;
///Register `GPIOZ_AFRL` writer
pub type W = crate::W<GPIOZ_AFRLrs>;
///Field `AFR0` reader - AFR0
pub type AFR0_R = crate::FieldReader;
///Field `AFR0` writer - AFR0
pub type AFR0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFR1` reader - AFR1
pub type AFR1_R = crate::FieldReader;
///Field `AFR1` writer - AFR1
pub type AFR1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFR2` reader - AFR2
pub type AFR2_R = crate::FieldReader;
///Field `AFR2` writer - AFR2
pub type AFR2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFR3` reader - AFR3
pub type AFR3_R = crate::FieldReader;
///Field `AFR3` writer - AFR3
pub type AFR3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFR4` reader - AFR4
pub type AFR4_R = crate::FieldReader;
///Field `AFR4` writer - AFR4
pub type AFR4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFR5` reader - AFR5
pub type AFR5_R = crate::FieldReader;
///Field `AFR5` writer - AFR5
pub type AFR5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFR6` reader - AFR6
pub type AFR6_R = crate::FieldReader;
///Field `AFR6` writer - AFR6
pub type AFR6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFR7` reader - AFR7
pub type AFR7_R = crate::FieldReader;
///Field `AFR7` writer - AFR7
pub type AFR7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - AFR0
    #[inline(always)]
    pub fn afr0(&self) -> AFR0_R {
        AFR0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - AFR1
    #[inline(always)]
    pub fn afr1(&self) -> AFR1_R {
        AFR1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - AFR2
    #[inline(always)]
    pub fn afr2(&self) -> AFR2_R {
        AFR2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - AFR3
    #[inline(always)]
    pub fn afr3(&self) -> AFR3_R {
        AFR3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - AFR4
    #[inline(always)]
    pub fn afr4(&self) -> AFR4_R {
        AFR4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - AFR5
    #[inline(always)]
    pub fn afr5(&self) -> AFR5_R {
        AFR5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - AFR6
    #[inline(always)]
    pub fn afr6(&self) -> AFR6_R {
        AFR6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - AFR7
    #[inline(always)]
    pub fn afr7(&self) -> AFR7_R {
        AFR7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOZ_AFRL")
            .field("afr0", &self.afr0())
            .field("afr1", &self.afr1())
            .field("afr2", &self.afr2())
            .field("afr3", &self.afr3())
            .field("afr4", &self.afr4())
            .field("afr5", &self.afr5())
            .field("afr6", &self.afr6())
            .field("afr7", &self.afr7())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - AFR0
    #[inline(always)]
    pub fn afr0(&mut self) -> AFR0_W<'_, GPIOZ_AFRLrs> {
        AFR0_W::new(self, 0)
    }
    ///Bits 4:7 - AFR1
    #[inline(always)]
    pub fn afr1(&mut self) -> AFR1_W<'_, GPIOZ_AFRLrs> {
        AFR1_W::new(self, 4)
    }
    ///Bits 8:11 - AFR2
    #[inline(always)]
    pub fn afr2(&mut self) -> AFR2_W<'_, GPIOZ_AFRLrs> {
        AFR2_W::new(self, 8)
    }
    ///Bits 12:15 - AFR3
    #[inline(always)]
    pub fn afr3(&mut self) -> AFR3_W<'_, GPIOZ_AFRLrs> {
        AFR3_W::new(self, 12)
    }
    ///Bits 16:19 - AFR4
    #[inline(always)]
    pub fn afr4(&mut self) -> AFR4_W<'_, GPIOZ_AFRLrs> {
        AFR4_W::new(self, 16)
    }
    ///Bits 20:23 - AFR5
    #[inline(always)]
    pub fn afr5(&mut self) -> AFR5_W<'_, GPIOZ_AFRLrs> {
        AFR5_W::new(self, 20)
    }
    ///Bits 24:27 - AFR6
    #[inline(always)]
    pub fn afr6(&mut self) -> AFR6_W<'_, GPIOZ_AFRLrs> {
        AFR6_W::new(self, 24)
    }
    ///Bits 28:31 - AFR7
    #[inline(always)]
    pub fn afr7(&mut self) -> AFR7_W<'_, GPIOZ_AFRLrs> {
        AFR7_W::new(self, 28)
    }
}
/**GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`gpioz_afrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioz_afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOZ:GPIOZ_AFRL)*/
pub struct GPIOZ_AFRLrs;
impl crate::RegisterSpec for GPIOZ_AFRLrs {
    type Ux = u32;
}
///`read()` method returns [`gpioz_afrl::R`](R) reader structure
impl crate::Readable for GPIOZ_AFRLrs {}
///`write(|w| ..)` method takes [`gpioz_afrl::W`](W) writer structure
impl crate::Writable for GPIOZ_AFRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPIOZ_AFRL to value 0
impl crate::Resettable for GPIOZ_AFRLrs {}
