///Register `ACR` reader
pub type R = crate::R<ACRrs>;
///Register `ACR` writer
pub type W = crate::W<ACRrs>;
///Field `LATENCY` reader - Read latency
pub type LATENCY_R = crate::FieldReader;
///Field `LATENCY` writer - Read latency
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `WRHIGHFREQ` reader - Flash signal delay
pub type WRHIGHFREQ_R = crate::FieldReader;
///Field `WRHIGHFREQ` writer - Flash signal delay
pub type WRHIGHFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - Read latency
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - Flash signal delay
    #[inline(always)]
    pub fn wrhighfreq(&self) -> WRHIGHFREQ_R {
        WRHIGHFREQ_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACR")
            .field("wrhighfreq", &self.wrhighfreq())
            .field("latency", &self.latency())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Read latency
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W<'_, ACRrs> {
        LATENCY_W::new(self, 0)
    }
    ///Bits 4:5 - Flash signal delay
    #[inline(always)]
    pub fn wrhighfreq(&mut self) -> WRHIGHFREQ_W<'_, ACRrs> {
        WRHIGHFREQ_W::new(self, 4)
    }
}
/**FLASH access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#FLASH:ACR)*/
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
///`reset()` method sets ACR to value 0x37
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0x37;
}
