///Register `MCR` reader
pub type R = crate::R<MCRrs>;
///Register `MCR` writer
pub type W = crate::W<MCRrs>;
///Field `MODE1` reader - MODE1
pub type MODE1_R = crate::FieldReader;
///Field `MODE1` writer - MODE1
pub type MODE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MODE2` reader - MODE2
pub type MODE2_R = crate::FieldReader;
///Field `MODE2` writer - MODE2
pub type MODE2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - MODE1
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 7) as u8)
    }
    ///Bits 16:18 - MODE2
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCR")
            .field("mode1", &self.mode1())
            .field("mode2", &self.mode2())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - MODE1
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W<'_, MCRrs> {
        MODE1_W::new(self, 0)
    }
    ///Bits 16:18 - MODE2
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W<'_, MCRrs> {
        MODE2_W::new(self, 16)
    }
}
/**DAC mode control register

You can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DAC1:MCR)*/
pub struct MCRrs;
impl crate::RegisterSpec for MCRrs {
    type Ux = u32;
}
///`read()` method returns [`mcr::R`](R) reader structure
impl crate::Readable for MCRrs {}
///`write(|w| ..)` method takes [`mcr::W`](W) writer structure
impl crate::Writable for MCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MCR to value 0
impl crate::Resettable for MCRrs {}
