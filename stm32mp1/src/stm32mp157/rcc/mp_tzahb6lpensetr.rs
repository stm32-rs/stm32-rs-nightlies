///Register `MP_TZAHB6LPENSETR` reader
pub type R = crate::R<MP_TZAHB6LPENSETRrs>;
///Register `MP_TZAHB6LPENSETR` writer
pub type W = crate::W<MP_TZAHB6LPENSETRrs>;
///Field `MDMALPEN` reader - MDMALPEN
pub type MDMALPEN_R = crate::BitReader;
///Field `MDMALPEN` writer - MDMALPEN
pub type MDMALPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MDMALPEN
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_TZAHB6LPENSETR")
            .field("mdmalpen", &self.mdmalpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - MDMALPEN
    #[inline(always)]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W<'_, MP_TZAHB6LPENSETRrs> {
        MDMALPEN_W::new(self, 0)
    }
}
/**This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_tzahb6lpensetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_tzahb6lpensetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MP_TZAHB6LPENSETR)*/
pub struct MP_TZAHB6LPENSETRrs;
impl crate::RegisterSpec for MP_TZAHB6LPENSETRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_tzahb6lpensetr::R`](R) reader structure
impl crate::Readable for MP_TZAHB6LPENSETRrs {}
///`write(|w| ..)` method takes [`mp_tzahb6lpensetr::W`](W) writer structure
impl crate::Writable for MP_TZAHB6LPENSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_TZAHB6LPENSETR to value 0x01
impl crate::Resettable for MP_TZAHB6LPENSETRrs {
    const RESET_VALUE: u32 = 0x01;
}
