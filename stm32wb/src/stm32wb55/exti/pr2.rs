///Register `PR2` reader
pub type R = crate::R<PR2rs>;
///Register `PR2` writer
pub type W = crate::W<PR2rs>;
///Field `PIF33` reader - Configurable event inputs x+32 Pending bit.
pub type PIF33_R = crate::BitReader;
///Field `PIF33` writer - Configurable event inputs x+32 Pending bit.
pub type PIF33_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PIF40_41` reader - Configurable event inputs x+32 Pending bit.
pub type PIF40_41_R = crate::FieldReader;
///Field `PIF40_41` writer - Configurable event inputs x+32 Pending bit.
pub type PIF40_41_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 1 - Configurable event inputs x+32 Pending bit.
    #[inline(always)]
    pub fn pif33(&self) -> PIF33_R {
        PIF33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 8:9 - Configurable event inputs x+32 Pending bit.
    #[inline(always)]
    pub fn pif40_41(&self) -> PIF40_41_R {
        PIF40_41_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR2")
            .field("pif33", &self.pif33())
            .field("pif40_41", &self.pif40_41())
            .finish()
    }
}
impl W {
    ///Bit 1 - Configurable event inputs x+32 Pending bit.
    #[inline(always)]
    pub fn pif33(&mut self) -> PIF33_W<'_, PR2rs> {
        PIF33_W::new(self, 1)
    }
    ///Bits 8:9 - Configurable event inputs x+32 Pending bit.
    #[inline(always)]
    pub fn pif40_41(&mut self) -> PIF40_41_W<'_, PR2rs> {
        PIF40_41_W::new(self, 8)
    }
}
/**pending register

You can [`read`](crate::Reg::read) this register and get [`pr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:PR2)*/
pub struct PR2rs;
impl crate::RegisterSpec for PR2rs {
    type Ux = u32;
}
///`read()` method returns [`pr2::R`](R) reader structure
impl crate::Readable for PR2rs {}
///`write(|w| ..)` method takes [`pr2::W`](W) writer structure
impl crate::Writable for PR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PR2 to value 0
impl crate::Resettable for PR2rs {}
