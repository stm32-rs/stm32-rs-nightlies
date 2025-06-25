///Register `AFRH` reader
pub type R = crate::R<AFRHrs>;
///Register `AFRH` writer
pub type W = crate::W<AFRHrs>;
///Field `AFSEL13` reader - Alternate function selection for port C I/O pin 13
pub type AFSEL13_R = crate::FieldReader;
///Field `AFSEL13` writer - Alternate function selection for port C I/O pin 13
pub type AFSEL13_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFSEL14` reader - Alternate function selection for port C I/O pin 14
pub type AFSEL14_R = crate::FieldReader;
///Field `AFSEL14` writer - Alternate function selection for port C I/O pin 14
pub type AFSEL14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFSEL15` reader - Alternate function selection for port C I/O pin 15 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOC SEC15.
pub type AFSEL15_R = crate::FieldReader;
///Field `AFSEL15` writer - Alternate function selection for port C I/O pin 15 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOC SEC15.
pub type AFSEL15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 20:23 - Alternate function selection for port C I/O pin 13
    #[inline(always)]
    pub fn afsel13(&self) -> AFSEL13_R {
        AFSEL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port C I/O pin 14
    #[inline(always)]
    pub fn afsel14(&self) -> AFSEL14_R {
        AFSEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Alternate function selection for port C I/O pin 15 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOC SEC15.
    #[inline(always)]
    pub fn afsel15(&self) -> AFSEL15_R {
        AFSEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRH")
            .field("afsel13", &self.afsel13())
            .field("afsel14", &self.afsel14())
            .field("afsel15", &self.afsel15())
            .finish()
    }
}
impl W {
    ///Bits 20:23 - Alternate function selection for port C I/O pin 13
    #[inline(always)]
    pub fn afsel13(&mut self) -> AFSEL13_W<AFRHrs> {
        AFSEL13_W::new(self, 20)
    }
    ///Bits 24:27 - Alternate function selection for port C I/O pin 14
    #[inline(always)]
    pub fn afsel14(&mut self) -> AFSEL14_W<AFRHrs> {
        AFSEL14_W::new(self, 24)
    }
    ///Bits 28:31 - Alternate function selection for port C I/O pin 15 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOC SEC15.
    #[inline(always)]
    pub fn afsel15(&mut self) -> AFSEL15_W<AFRHrs> {
        AFSEL15_W::new(self, 28)
    }
}
/**GPIO port C alternate function high register

You can [`read`](crate::Reg::read) this register and get [`afrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#GPIOC:AFRH)*/
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
