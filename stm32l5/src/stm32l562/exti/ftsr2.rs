///Register `FTSR2` reader
pub type R = crate::R<FTSR2rs>;
///Register `FTSR2` writer
pub type W = crate::W<FTSR2rs>;
///Field `FT35` reader - FT35
pub type FT35_R = crate::BitReader;
///Field `FT35` writer - FT35
pub type FT35_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT36` reader - FT36
pub type FT36_R = crate::BitReader;
///Field `FT36` writer - FT36
pub type FT36_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT37` reader - FT37
pub type FT37_R = crate::BitReader;
///Field `FT37` writer - FT37
pub type FT37_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT38` reader - FT38
pub type FT38_R = crate::BitReader;
///Field `FT38` writer - FT38
pub type FT38_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - FT35
    #[inline(always)]
    pub fn ft35(&self) -> FT35_R {
        FT35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FT36
    #[inline(always)]
    pub fn ft36(&self) -> FT36_R {
        FT36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - FT37
    #[inline(always)]
    pub fn ft37(&self) -> FT37_R {
        FT37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - FT38
    #[inline(always)]
    pub fn ft38(&self) -> FT38_R {
        FT38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTSR2")
            .field("ft35", &self.ft35())
            .field("ft36", &self.ft36())
            .field("ft37", &self.ft37())
            .field("ft38", &self.ft38())
            .finish()
    }
}
impl W {
    ///Bit 3 - FT35
    #[inline(always)]
    pub fn ft35(&mut self) -> FT35_W<'_, FTSR2rs> {
        FT35_W::new(self, 3)
    }
    ///Bit 4 - FT36
    #[inline(always)]
    pub fn ft36(&mut self) -> FT36_W<'_, FTSR2rs> {
        FT36_W::new(self, 4)
    }
    ///Bit 5 - FT37
    #[inline(always)]
    pub fn ft37(&mut self) -> FT37_W<'_, FTSR2rs> {
        FT37_W::new(self, 5)
    }
    ///Bit 6 - FT38
    #[inline(always)]
    pub fn ft38(&mut self) -> FT38_W<'_, FTSR2rs> {
        FT38_W::new(self, 6)
    }
}
/**EXTI falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#EXTI:FTSR2)*/
pub struct FTSR2rs;
impl crate::RegisterSpec for FTSR2rs {
    type Ux = u32;
}
///`read()` method returns [`ftsr2::R`](R) reader structure
impl crate::Readable for FTSR2rs {}
///`write(|w| ..)` method takes [`ftsr2::W`](W) writer structure
impl crate::Writable for FTSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FTSR2 to value 0
impl crate::Resettable for FTSR2rs {}
