///Register `SECCR2` reader
pub type R = crate::R<SECCR2rs>;
///Register `SECCR2` writer
pub type W = crate::W<SECCR2rs>;
///Field `PS` reader - Program suspend request
pub type PS_R = crate::BitReader;
///Field `PS` writer - Program suspend request
pub type PS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ES` reader - Erase suspend request
pub type ES_R = crate::BitReader;
///Field `ES` writer - Erase suspend request
pub type ES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Program suspend request
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Erase suspend request
    #[inline(always)]
    pub fn es(&self) -> ES_R {
        ES_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCR2")
            .field("ps", &self.ps())
            .field("es", &self.es())
            .finish()
    }
}
impl W {
    ///Bit 0 - Program suspend request
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<'_, SECCR2rs> {
        PS_W::new(self, 0)
    }
    ///Bit 1 - Erase suspend request
    #[inline(always)]
    pub fn es(&mut self) -> ES_W<'_, SECCR2rs> {
        ES_W::new(self, 1)
    }
}
/**FLASH secure control 2 register

You can [`read`](crate::Reg::read) this register and get [`seccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#FLASH:SECCR2)*/
pub struct SECCR2rs;
impl crate::RegisterSpec for SECCR2rs {
    type Ux = u32;
}
///`read()` method returns [`seccr2::R`](R) reader structure
impl crate::Readable for SECCR2rs {}
///`write(|w| ..)` method takes [`seccr2::W`](W) writer structure
impl crate::Writable for SECCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCR2 to value 0
impl crate::Resettable for SECCR2rs {}
