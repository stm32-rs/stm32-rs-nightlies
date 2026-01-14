///Register `GPIOZ_AFRH` reader
pub type R = crate::R<GPIOZ_AFRHrs>;
///Register `GPIOZ_AFRH` writer
pub type W = crate::W<GPIOZ_AFRHrs>;
///Field `AFR8` reader - AFR8
pub type AFR8_R = crate::FieldReader;
///Field `AFR8` writer - AFR8
pub type AFR8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFR9` reader - AFR9
pub type AFR9_R = crate::FieldReader;
///Field `AFR9` writer - AFR9
pub type AFR9_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFR10` reader - AFR10
pub type AFR10_R = crate::FieldReader;
///Field `AFR10` writer - AFR10
pub type AFR10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFR11` reader - AFR11
pub type AFR11_R = crate::FieldReader;
///Field `AFR11` writer - AFR11
pub type AFR11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFR12` reader - AFR12
pub type AFR12_R = crate::FieldReader;
///Field `AFR12` writer - AFR12
pub type AFR12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFR13` reader - AFR13
pub type AFR13_R = crate::FieldReader;
///Field `AFR13` writer - AFR13
pub type AFR13_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFR14` reader - AFR14
pub type AFR14_R = crate::FieldReader;
///Field `AFR14` writer - AFR14
pub type AFR14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFR15` reader - AFR15
pub type AFR15_R = crate::FieldReader;
///Field `AFR15` writer - AFR15
pub type AFR15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - AFR8
    #[inline(always)]
    pub fn afr8(&self) -> AFR8_R {
        AFR8_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - AFR9
    #[inline(always)]
    pub fn afr9(&self) -> AFR9_R {
        AFR9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - AFR10
    #[inline(always)]
    pub fn afr10(&self) -> AFR10_R {
        AFR10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - AFR11
    #[inline(always)]
    pub fn afr11(&self) -> AFR11_R {
        AFR11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - AFR12
    #[inline(always)]
    pub fn afr12(&self) -> AFR12_R {
        AFR12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - AFR13
    #[inline(always)]
    pub fn afr13(&self) -> AFR13_R {
        AFR13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - AFR14
    #[inline(always)]
    pub fn afr14(&self) -> AFR14_R {
        AFR14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - AFR15
    #[inline(always)]
    pub fn afr15(&self) -> AFR15_R {
        AFR15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOZ_AFRH")
            .field("afr8", &self.afr8())
            .field("afr9", &self.afr9())
            .field("afr10", &self.afr10())
            .field("afr11", &self.afr11())
            .field("afr12", &self.afr12())
            .field("afr13", &self.afr13())
            .field("afr14", &self.afr14())
            .field("afr15", &self.afr15())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - AFR8
    #[inline(always)]
    pub fn afr8(&mut self) -> AFR8_W<'_, GPIOZ_AFRHrs> {
        AFR8_W::new(self, 0)
    }
    ///Bits 4:7 - AFR9
    #[inline(always)]
    pub fn afr9(&mut self) -> AFR9_W<'_, GPIOZ_AFRHrs> {
        AFR9_W::new(self, 4)
    }
    ///Bits 8:11 - AFR10
    #[inline(always)]
    pub fn afr10(&mut self) -> AFR10_W<'_, GPIOZ_AFRHrs> {
        AFR10_W::new(self, 8)
    }
    ///Bits 12:15 - AFR11
    #[inline(always)]
    pub fn afr11(&mut self) -> AFR11_W<'_, GPIOZ_AFRHrs> {
        AFR11_W::new(self, 12)
    }
    ///Bits 16:19 - AFR12
    #[inline(always)]
    pub fn afr12(&mut self) -> AFR12_W<'_, GPIOZ_AFRHrs> {
        AFR12_W::new(self, 16)
    }
    ///Bits 20:23 - AFR13
    #[inline(always)]
    pub fn afr13(&mut self) -> AFR13_W<'_, GPIOZ_AFRHrs> {
        AFR13_W::new(self, 20)
    }
    ///Bits 24:27 - AFR14
    #[inline(always)]
    pub fn afr14(&mut self) -> AFR14_W<'_, GPIOZ_AFRHrs> {
        AFR14_W::new(self, 24)
    }
    ///Bits 28:31 - AFR15
    #[inline(always)]
    pub fn afr15(&mut self) -> AFR15_W<'_, GPIOZ_AFRHrs> {
        AFR15_W::new(self, 28)
    }
}
/**GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`gpioz_afrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioz_afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOZ:GPIOZ_AFRH)*/
pub struct GPIOZ_AFRHrs;
impl crate::RegisterSpec for GPIOZ_AFRHrs {
    type Ux = u32;
}
///`read()` method returns [`gpioz_afrh::R`](R) reader structure
impl crate::Readable for GPIOZ_AFRHrs {}
///`write(|w| ..)` method takes [`gpioz_afrh::W`](W) writer structure
impl crate::Writable for GPIOZ_AFRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPIOZ_AFRH to value 0
impl crate::Resettable for GPIOZ_AFRHrs {}
