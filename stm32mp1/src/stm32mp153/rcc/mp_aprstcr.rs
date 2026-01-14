///Register `MP_APRSTCR` reader
pub type R = crate::R<MP_APRSTCRrs>;
///Register `MP_APRSTCR` writer
pub type W = crate::W<MP_APRSTCRrs>;
///Field `RDCTLEN` reader - RDCTLEN
pub type RDCTLEN_R = crate::BitReader;
///Field `RDCTLEN` writer - RDCTLEN
pub type RDCTLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTTO` reader - RSTTO
pub type RSTTO_R = crate::FieldReader;
///Field `RSTTO` writer - RSTTO
pub type RSTTO_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bit 0 - RDCTLEN
    #[inline(always)]
    pub fn rdctlen(&self) -> RDCTLEN_R {
        RDCTLEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:14 - RSTTO
    #[inline(always)]
    pub fn rstto(&self) -> RSTTO_R {
        RSTTO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_APRSTCR")
            .field("rdctlen", &self.rdctlen())
            .field("rstto", &self.rstto())
            .finish()
    }
}
impl W {
    ///Bit 0 - RDCTLEN
    #[inline(always)]
    pub fn rdctlen(&mut self) -> RDCTLEN_W<'_, MP_APRSTCRrs> {
        RDCTLEN_W::new(self, 0)
    }
    ///Bits 8:14 - RSTTO
    #[inline(always)]
    pub fn rstto(&mut self) -> RSTTO_W<'_, MP_APRSTCRrs> {
        RSTTO_W::new(self, 8)
    }
}
/**This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_aprstcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_aprstcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APRSTCR)*/
pub struct MP_APRSTCRrs;
impl crate::RegisterSpec for MP_APRSTCRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_aprstcr::R`](R) reader structure
impl crate::Readable for MP_APRSTCRrs {}
///`write(|w| ..)` method takes [`mp_aprstcr::W`](W) writer structure
impl crate::Writable for MP_APRSTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_APRSTCR to value 0x7f00
impl crate::Resettable for MP_APRSTCRrs {
    const RESET_VALUE: u32 = 0x7f00;
}
