///Register `AFRL` reader
pub type R = crate::R<AFRLrs>;
///Register `AFRL` writer
pub type W = crate::W<AFRLrs>;
///Field `AFRL0` reader - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL0_R = crate::FieldReader;
///Field `AFRL0` writer - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFRL1` reader - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL1_R = crate::FieldReader;
///Field `AFRL1` writer - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFRL2` reader - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL2_R = crate::FieldReader;
///Field `AFRL2` writer - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFRL3` reader - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL3_R = crate::FieldReader;
///Field `AFRL3` writer - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFRL4` reader - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL4_R = crate::FieldReader;
///Field `AFRL4` writer - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFRL5` reader - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL5_R = crate::FieldReader;
///Field `AFRL5` writer - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFRL6` reader - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL6_R = crate::FieldReader;
///Field `AFRL6` writer - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFRL7` reader - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL7_R = crate::FieldReader;
///Field `AFRL7` writer - Alternate function selection for port x bit y (y = 0..7)
pub type AFRL7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl0(&self) -> AFRL0_R {
        AFRL0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl1(&self) -> AFRL1_R {
        AFRL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl2(&self) -> AFRL2_R {
        AFRL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl3(&self) -> AFRL3_R {
        AFRL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl4(&self) -> AFRL4_R {
        AFRL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl5(&self) -> AFRL5_R {
        AFRL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl6(&self) -> AFRL6_R {
        AFRL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl7(&self) -> AFRL7_R {
        AFRL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRL")
            .field("afrl7", &self.afrl7())
            .field("afrl6", &self.afrl6())
            .field("afrl5", &self.afrl5())
            .field("afrl4", &self.afrl4())
            .field("afrl3", &self.afrl3())
            .field("afrl2", &self.afrl2())
            .field("afrl1", &self.afrl1())
            .field("afrl0", &self.afrl0())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl0(&mut self) -> AFRL0_W<'_, AFRLrs> {
        AFRL0_W::new(self, 0)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl1(&mut self) -> AFRL1_W<'_, AFRLrs> {
        AFRL1_W::new(self, 4)
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl2(&mut self) -> AFRL2_W<'_, AFRLrs> {
        AFRL2_W::new(self, 8)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl3(&mut self) -> AFRL3_W<'_, AFRLrs> {
        AFRL3_W::new(self, 12)
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl4(&mut self) -> AFRL4_W<'_, AFRLrs> {
        AFRL4_W::new(self, 16)
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl5(&mut self) -> AFRL5_W<'_, AFRLrs> {
        AFRL5_W::new(self, 20)
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl6(&mut self) -> AFRL6_W<'_, AFRLrs> {
        AFRL6_W::new(self, 24)
    }
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl7(&mut self) -> AFRL7_W<'_, AFRLrs> {
        AFRL7_W::new(self, 28)
    }
}
/**GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`afrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#GPIOB:AFRL)*/
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
