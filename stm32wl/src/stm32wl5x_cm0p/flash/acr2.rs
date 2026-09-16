///Register `ACR2` reader
pub type R = crate::R<ACR2rs>;
///Register `ACR2` writer
pub type W = crate::W<ACR2rs>;
///Field `PRIVMODE` reader - CFI privileged mode enable
pub type PRIVMODE_R = crate::BitReader;
///Field `PRIVMODE` writer - CFI privileged mode enable
pub type PRIVMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDPADIS` reader - Flash user hide protection area access disable
pub type HDPADIS_R = crate::BitReader;
///Field `HDPADIS` writer - Flash user hide protection area access disable
pub type HDPADIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `C2SWDBGEN` reader - CPU2 Software debug enable
pub type C2SWDBGEN_R = crate::BitReader;
///Field `C2SWDBGEN` writer - CPU2 Software debug enable
pub type C2SWDBGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CFI privileged mode enable
    #[inline(always)]
    pub fn privmode(&self) -> PRIVMODE_R {
        PRIVMODE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Flash user hide protection area access disable
    #[inline(always)]
    pub fn hdpadis(&self) -> HDPADIS_R {
        HDPADIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU2 Software debug enable
    #[inline(always)]
    pub fn c2swdbgen(&self) -> C2SWDBGEN_R {
        C2SWDBGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACR2")
            .field("privmode", &self.privmode())
            .field("hdpadis", &self.hdpadis())
            .field("c2swdbgen", &self.c2swdbgen())
            .finish()
    }
}
impl W {
    ///Bit 0 - CFI privileged mode enable
    #[inline(always)]
    pub fn privmode(&mut self) -> PRIVMODE_W<'_, ACR2rs> {
        PRIVMODE_W::new(self, 0)
    }
    ///Bit 1 - Flash user hide protection area access disable
    #[inline(always)]
    pub fn hdpadis(&mut self) -> HDPADIS_W<'_, ACR2rs> {
        HDPADIS_W::new(self, 1)
    }
    ///Bit 2 - CPU2 Software debug enable
    #[inline(always)]
    pub fn c2swdbgen(&mut self) -> C2SWDBGEN_W<'_, ACR2rs> {
        C2SWDBGEN_W::new(self, 2)
    }
}
/**Flash access control register 2

You can [`read`](crate::Reg::read) this register and get [`acr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#FLASH:ACR2)*/
pub struct ACR2rs;
impl crate::RegisterSpec for ACR2rs {
    type Ux = u32;
}
///`read()` method returns [`acr2::R`](R) reader structure
impl crate::Readable for ACR2rs {}
///`write(|w| ..)` method takes [`acr2::W`](W) writer structure
impl crate::Writable for ACR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACR2 to value 0
impl crate::Resettable for ACR2rs {}
