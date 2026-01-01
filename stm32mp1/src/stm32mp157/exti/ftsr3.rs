///Register `FTSR3` reader
pub type R = crate::R<FTSR3rs>;
///Register `FTSR3` writer
pub type W = crate::W<FTSR3rs>;
///Field `FT65` reader - FT65
pub type FT65_R = crate::BitReader;
///Field `FT65` writer - FT65
pub type FT65_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT66` reader - FT66
pub type FT66_R = crate::BitReader;
///Field `FT66` writer - FT66
pub type FT66_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT68` reader - FT68
pub type FT68_R = crate::BitReader;
///Field `FT68` writer - FT68
pub type FT68_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT73` reader - FT73
pub type FT73_R = crate::BitReader;
///Field `FT73` writer - FT73
pub type FT73_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT74` reader - FT74
pub type FT74_R = crate::BitReader;
///Field `FT74` writer - FT74
pub type FT74_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - FT65
    #[inline(always)]
    pub fn ft65(&self) -> FT65_R {
        FT65_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FT66
    #[inline(always)]
    pub fn ft66(&self) -> FT66_R {
        FT66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - FT68
    #[inline(always)]
    pub fn ft68(&self) -> FT68_R {
        FT68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - FT73
    #[inline(always)]
    pub fn ft73(&self) -> FT73_R {
        FT73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - FT74
    #[inline(always)]
    pub fn ft74(&self) -> FT74_R {
        FT74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTSR3")
            .field("ft65", &self.ft65())
            .field("ft66", &self.ft66())
            .field("ft68", &self.ft68())
            .field("ft73", &self.ft73())
            .field("ft74", &self.ft74())
            .finish()
    }
}
impl W {
    ///Bit 1 - FT65
    #[inline(always)]
    pub fn ft65(&mut self) -> FT65_W<'_, FTSR3rs> {
        FT65_W::new(self, 1)
    }
    ///Bit 2 - FT66
    #[inline(always)]
    pub fn ft66(&mut self) -> FT66_W<'_, FTSR3rs> {
        FT66_W::new(self, 2)
    }
    ///Bit 4 - FT68
    #[inline(always)]
    pub fn ft68(&mut self) -> FT68_W<'_, FTSR3rs> {
        FT68_W::new(self, 4)
    }
    ///Bit 9 - FT73
    #[inline(always)]
    pub fn ft73(&mut self) -> FT73_W<'_, FTSR3rs> {
        FT73_W::new(self, 9)
    }
    ///Bit 10 - FT74
    #[inline(always)]
    pub fn ft74(&mut self) -> FT74_W<'_, FTSR3rs> {
        FT74_W::new(self, 10)
    }
}
/**Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`ftsr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:FTSR3)*/
pub struct FTSR3rs;
impl crate::RegisterSpec for FTSR3rs {
    type Ux = u32;
}
///`read()` method returns [`ftsr3::R`](R) reader structure
impl crate::Readable for FTSR3rs {}
///`write(|w| ..)` method takes [`ftsr3::W`](W) writer structure
impl crate::Writable for FTSR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FTSR3 to value 0
impl crate::Resettable for FTSR3rs {}
