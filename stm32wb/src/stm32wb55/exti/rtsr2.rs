///Register `RTSR2` reader
pub type R = crate::R<RTSR2rs>;
///Register `RTSR2` writer
pub type W = crate::W<RTSR2rs>;
///Field `RT33` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT33_R = crate::BitReader;
///Field `RT33` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT33_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT40_41` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT40_41_R = crate::FieldReader;
///Field `RT40_41` writer - Rising trigger event configuration bit of Configurable Event input
pub type RT40_41_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt33(&self) -> RT33_R {
        RT33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 8:9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt40_41(&self) -> RT40_41_R {
        RT40_41_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTSR2")
            .field("rt33", &self.rt33())
            .field("rt40_41", &self.rt40_41())
            .finish()
    }
}
impl W {
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt33(&mut self) -> RT33_W<'_, RTSR2rs> {
        RT33_W::new(self, 1)
    }
    ///Bits 8:9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt40_41(&mut self) -> RT40_41_W<'_, RTSR2rs> {
        RT40_41_W::new(self, 8)
    }
}
/**rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:RTSR2)*/
pub struct RTSR2rs;
impl crate::RegisterSpec for RTSR2rs {
    type Ux = u32;
}
///`read()` method returns [`rtsr2::R`](R) reader structure
impl crate::Readable for RTSR2rs {}
///`write(|w| ..)` method takes [`rtsr2::W`](W) writer structure
impl crate::Writable for RTSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTSR2 to value 0
impl crate::Resettable for RTSR2rs {}
