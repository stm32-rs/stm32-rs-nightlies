///Register `AFRL` reader
pub type R = crate::R<AFRLrs>;
///Register `AFRL` writer
pub type W = crate::W<AFRLrs>;
///Field `AFSEL0` reader - Alternate function selection for port x bit y (y = 0..7)
pub type AFSEL0_R = crate::FieldReader;
///Field `AFSEL0` writer - Alternate function selection for port x bit y (y = 0..7)
pub type AFSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFSEL1` reader - Alternate function selection for port x bit y (y = 0..7)
pub type AFSEL1_R = crate::FieldReader;
///Field `AFSEL1` writer - Alternate function selection for port x bit y (y = 0..7)
pub type AFSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFSEL3` reader - Alternate function selection for port x bit y (y = 0..7)
pub type AFSEL3_R = crate::FieldReader;
///Field `AFSEL3` writer - Alternate function selection for port x bit y (y = 0..7)
pub type AFSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel0(&self) -> AFSEL0_R {
        AFSEL0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel1(&self) -> AFSEL1_R {
        AFSEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel3(&self) -> AFSEL3_R {
        AFSEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRL")
            .field("afsel3", &self.afsel3())
            .field("afsel1", &self.afsel1())
            .field("afsel0", &self.afsel0())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel0(&mut self) -> AFSEL0_W<'_, AFRLrs> {
        AFSEL0_W::new(self, 0)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel1(&mut self) -> AFSEL1_W<'_, AFRLrs> {
        AFSEL1_W::new(self, 4)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel3(&mut self) -> AFSEL3_W<'_, AFRLrs> {
        AFSEL3_W::new(self, 12)
    }
}
/**GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`afrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOH:AFRL)*/
pub struct AFRLrs;
impl crate::RegisterSpec for AFRLrs {
    type Ux = u32;
}
///`read()` method returns [`afrl::R`](R) reader structure
impl crate::Readable for AFRLrs {}
///`write(|w| ..)` method takes [`afrl::W`](W) writer structure
impl crate::Writable for AFRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AFRL to value 0
impl crate::Resettable for AFRLrs {}
