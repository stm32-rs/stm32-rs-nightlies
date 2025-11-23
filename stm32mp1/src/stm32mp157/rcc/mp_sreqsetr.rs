///Register `MP_SREQSETR` reader
pub type R = crate::R<MP_SREQSETRrs>;
///Register `MP_SREQSETR` writer
pub type W = crate::W<MP_SREQSETRrs>;
///Field `STPREQ_P0` reader - STPREQ_P0
pub type STPREQ_P0_R = crate::BitReader;
///Field `STPREQ_P0` writer - STPREQ_P0
pub type STPREQ_P0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STPREQ_P1` reader - STPREQ_P1
pub type STPREQ_P1_R = crate::BitReader;
///Field `STPREQ_P1` writer - STPREQ_P1
pub type STPREQ_P1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - STPREQ_P0
    #[inline(always)]
    pub fn stpreq_p0(&self) -> STPREQ_P0_R {
        STPREQ_P0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - STPREQ_P1
    #[inline(always)]
    pub fn stpreq_p1(&self) -> STPREQ_P1_R {
        STPREQ_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_SREQSETR")
            .field("stpreq_p0", &self.stpreq_p0())
            .field("stpreq_p1", &self.stpreq_p1())
            .finish()
    }
}
impl W {
    ///Bit 0 - STPREQ_P0
    #[inline(always)]
    pub fn stpreq_p0(&mut self) -> STPREQ_P0_W<'_, MP_SREQSETRrs> {
        STPREQ_P0_W::new(self, 0)
    }
    ///Bit 1 - STPREQ_P1
    #[inline(always)]
    pub fn stpreq_p1(&mut self) -> STPREQ_P1_W<'_, MP_SREQSETRrs> {
        STPREQ_P1_W::new(self, 1)
    }
}
/**Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_sreqsetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_sreqsetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MP_SREQSETR)*/
pub struct MP_SREQSETRrs;
impl crate::RegisterSpec for MP_SREQSETRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_sreqsetr::R`](R) reader structure
impl crate::Readable for MP_SREQSETRrs {}
///`write(|w| ..)` method takes [`mp_sreqsetr::W`](W) writer structure
impl crate::Writable for MP_SREQSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_SREQSETR to value 0
impl crate::Resettable for MP_SREQSETRrs {}
