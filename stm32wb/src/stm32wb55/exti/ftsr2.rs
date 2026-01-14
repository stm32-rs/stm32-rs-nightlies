///Register `FTSR2` reader
pub type R = crate::R<FTSR2rs>;
///Register `FTSR2` writer
pub type W = crate::W<FTSR2rs>;
///Field `FT33` reader - Falling trigger event configuration bit of Configurable Event input
pub type FT33_R = crate::BitReader;
///Field `FT33` writer - Falling trigger event configuration bit of Configurable Event input
pub type FT33_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT40_41` reader - Falling trigger event configuration bit of Configurable Event input
pub type FT40_41_R = crate::FieldReader;
///Field `FT40_41` writer - Falling trigger event configuration bit of Configurable Event input
pub type FT40_41_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 1 - Falling trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn ft33(&self) -> FT33_R {
        FT33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 8:9 - Falling trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn ft40_41(&self) -> FT40_41_R {
        FT40_41_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTSR2")
            .field("ft33", &self.ft33())
            .field("ft40_41", &self.ft40_41())
            .finish()
    }
}
impl W {
    ///Bit 1 - Falling trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn ft33(&mut self) -> FT33_W<'_, FTSR2rs> {
        FT33_W::new(self, 1)
    }
    ///Bits 8:9 - Falling trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn ft40_41(&mut self) -> FT40_41_W<'_, FTSR2rs> {
        FT40_41_W::new(self, 8)
    }
}
/**falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:FTSR2)*/
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
