///Register `PCONFR` reader
pub type R = crate::R<PCONFRrs>;
///Register `PCONFR` writer
pub type W = crate::W<PCONFRrs>;
///Field `NL` reader - Number of Lanes
pub type NL_R = crate::FieldReader;
///Field `NL` writer - Number of Lanes
pub type NL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SW_TIME` reader - Stop Wait Time
pub type SW_TIME_R = crate::FieldReader;
///Field `SW_TIME` writer - Stop Wait Time
pub type SW_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:1 - Number of Lanes
    #[inline(always)]
    pub fn nl(&self) -> NL_R {
        NL_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:15 - Stop Wait Time
    #[inline(always)]
    pub fn sw_time(&self) -> SW_TIME_R {
        SW_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCONFR")
            .field("nl", &self.nl())
            .field("sw_time", &self.sw_time())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Number of Lanes
    #[inline(always)]
    pub fn nl(&mut self) -> NL_W<'_, PCONFRrs> {
        NL_W::new(self, 0)
    }
    ///Bits 8:15 - Stop Wait Time
    #[inline(always)]
    pub fn sw_time(&mut self) -> SW_TIME_W<'_, PCONFRrs> {
        SW_TIME_W::new(self, 8)
    }
}
/**DSI Host PHY Configuration Register

You can [`read`](crate::Reg::read) this register and get [`pconfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#DSI:PCONFR)*/
pub struct PCONFRrs;
impl crate::RegisterSpec for PCONFRrs {
    type Ux = u32;
}
///`read()` method returns [`pconfr::R`](R) reader structure
impl crate::Readable for PCONFRrs {}
///`write(|w| ..)` method takes [`pconfr::W`](W) writer structure
impl crate::Writable for PCONFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCONFR to value 0
impl crate::Resettable for PCONFRrs {}
