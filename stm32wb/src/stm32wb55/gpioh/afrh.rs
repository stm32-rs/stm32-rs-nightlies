///Register `AFRH` reader
pub type R = crate::R<AFRHrs>;
///Register `AFRH` writer
pub type W = crate::W<AFRHrs>;
///Field `AFSEL8` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL8_R = crate::FieldReader;
///Field `AFSEL8` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFSEL9` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL9_R = crate::FieldReader;
///Field `AFSEL9` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL9_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFSEL10` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL10_R = crate::FieldReader;
///Field `AFSEL10` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFSEL11` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL11_R = crate::FieldReader;
///Field `AFSEL11` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFSEL12` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL12_R = crate::FieldReader;
///Field `AFSEL12` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFSEL13` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL13_R = crate::FieldReader;
///Field `AFSEL13` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL13_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFSEL14` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL14_R = crate::FieldReader;
///Field `AFSEL14` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFSEL15` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL15_R = crate::FieldReader;
///Field `AFSEL15` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel8(&self) -> AFSEL8_R {
        AFSEL8_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel9(&self) -> AFSEL9_R {
        AFSEL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel10(&self) -> AFSEL10_R {
        AFSEL10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel11(&self) -> AFSEL11_R {
        AFSEL11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel12(&self) -> AFSEL12_R {
        AFSEL12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel13(&self) -> AFSEL13_R {
        AFSEL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel14(&self) -> AFSEL14_R {
        AFSEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel15(&self) -> AFSEL15_R {
        AFSEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRH")
            .field("afsel15", &self.afsel15())
            .field("afsel14", &self.afsel14())
            .field("afsel13", &self.afsel13())
            .field("afsel12", &self.afsel12())
            .field("afsel11", &self.afsel11())
            .field("afsel10", &self.afsel10())
            .field("afsel9", &self.afsel9())
            .field("afsel8", &self.afsel8())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel8(&mut self) -> AFSEL8_W<AFRHrs> {
        AFSEL8_W::new(self, 0)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel9(&mut self) -> AFSEL9_W<AFRHrs> {
        AFSEL9_W::new(self, 4)
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel10(&mut self) -> AFSEL10_W<AFRHrs> {
        AFSEL10_W::new(self, 8)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel11(&mut self) -> AFSEL11_W<AFRHrs> {
        AFSEL11_W::new(self, 12)
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel12(&mut self) -> AFSEL12_W<AFRHrs> {
        AFSEL12_W::new(self, 16)
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel13(&mut self) -> AFSEL13_W<AFRHrs> {
        AFSEL13_W::new(self, 20)
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel14(&mut self) -> AFSEL14_W<AFRHrs> {
        AFSEL14_W::new(self, 24)
    }
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel15(&mut self) -> AFSEL15_W<AFRHrs> {
        AFSEL15_W::new(self, 28)
    }
}
/**GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`afrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOH:AFRH)*/
pub struct AFRHrs;
impl crate::RegisterSpec for AFRHrs {
    type Ux = u32;
}
///`read()` method returns [`afrh::R`](R) reader structure
impl crate::Readable for AFRHrs {}
///`write(|w| ..)` method takes [`afrh::W`](W) writer structure
impl crate::Writable for AFRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AFRH to value 0
impl crate::Resettable for AFRHrs {}
