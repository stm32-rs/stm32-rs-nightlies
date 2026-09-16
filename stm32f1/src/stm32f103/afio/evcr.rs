///Register `EVCR` reader
pub type R = crate::R<EVCRrs>;
///Register `EVCR` writer
pub type W = crate::W<EVCRrs>;
///Field `PIN` reader - Pin selection
pub type PIN_R = crate::FieldReader;
///Field `PIN` writer - Pin selection
pub type PIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PORT` reader - Port selection
pub type PORT_R = crate::FieldReader;
///Field `PORT` writer - Port selection
pub type PORT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EVOE` reader - Event Output Enable
pub type EVOE_R = crate::BitReader;
///Field `EVOE` writer - Event Output Enable
pub type EVOE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Pin selection
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - Port selection
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Event Output Enable
    #[inline(always)]
    pub fn evoe(&self) -> EVOE_R {
        EVOE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVCR")
            .field("pin", &self.pin())
            .field("port", &self.port())
            .field("evoe", &self.evoe())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Pin selection
    #[inline(always)]
    pub fn pin(&mut self) -> PIN_W<'_, EVCRrs> {
        PIN_W::new(self, 0)
    }
    ///Bits 4:6 - Port selection
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W<'_, EVCRrs> {
        PORT_W::new(self, 4)
    }
    ///Bit 7 - Event Output Enable
    #[inline(always)]
    pub fn evoe(&mut self) -> EVOE_W<'_, EVCRrs> {
        EVOE_W::new(self, 7)
    }
}
/**Event Control Register (AFIO_EVCR)

You can [`read`](crate::Reg::read) this register and get [`evcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#AFIO:EVCR)*/
pub struct EVCRrs;
impl crate::RegisterSpec for EVCRrs {
    type Ux = u32;
}
///`read()` method returns [`evcr::R`](R) reader structure
impl crate::Readable for EVCRrs {}
///`write(|w| ..)` method takes [`evcr::W`](W) writer structure
impl crate::Writable for EVCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EVCR to value 0
impl crate::Resettable for EVCRrs {}
