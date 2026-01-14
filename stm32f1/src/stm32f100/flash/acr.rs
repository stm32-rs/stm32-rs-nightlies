///Register `ACR` reader
pub type R = crate::R<ACRrs>;
///Register `ACR` writer
pub type W = crate::W<ACRrs>;
///Field `HLFCYA` reader - Flash half cycle access enable
pub type HLFCYA_R = crate::BitReader;
///Field `HLFCYA` writer - Flash half cycle access enable
pub type HLFCYA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - Flash half cycle access enable
    #[inline(always)]
    pub fn hlfcya(&self) -> HLFCYA_R {
        HLFCYA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACR")
            .field("hlfcya", &self.hlfcya())
            .finish()
    }
}
impl W {
    ///Bit 3 - Flash half cycle access enable
    #[inline(always)]
    pub fn hlfcya(&mut self) -> HLFCYA_W<'_, ACRrs> {
        HLFCYA_W::new(self, 3)
    }
}
/**Flash access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#FLASH:ACR)*/
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
///`read()` method returns [`acr::R`](R) reader structure
impl crate::Readable for ACRrs {}
///`write(|w| ..)` method takes [`acr::W`](W) writer structure
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACR to value 0
impl crate::Resettable for ACRrs {}
