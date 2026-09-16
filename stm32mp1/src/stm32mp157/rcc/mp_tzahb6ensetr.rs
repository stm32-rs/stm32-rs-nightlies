///Register `MP_TZAHB6ENSETR` reader
pub type R = crate::R<MP_TZAHB6ENSETRrs>;
///Register `MP_TZAHB6ENSETR` writer
pub type W = crate::W<MP_TZAHB6ENSETRrs>;
///Field `MDMAEN` reader - MDMAEN
pub type MDMAEN_R = crate::BitReader;
///Field `MDMAEN` writer - MDMAEN
pub type MDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MDMAEN
    #[inline(always)]
    pub fn mdmaen(&self) -> MDMAEN_R {
        MDMAEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_TZAHB6ENSETR")
            .field("mdmaen", &self.mdmaen())
            .finish()
    }
}
impl W {
    ///Bit 0 - MDMAEN
    #[inline(always)]
    pub fn mdmaen(&mut self) -> MDMAEN_W<'_, MP_TZAHB6ENSETRrs> {
        MDMAEN_W::new(self, 0)
    }
}
/**This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_tzahb6ensetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_tzahb6ensetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MP_TZAHB6ENSETR)*/
pub struct MP_TZAHB6ENSETRrs;
impl crate::RegisterSpec for MP_TZAHB6ENSETRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_tzahb6ensetr::R`](R) reader structure
impl crate::Readable for MP_TZAHB6ENSETRrs {}
///`write(|w| ..)` method takes [`mp_tzahb6ensetr::W`](W) writer structure
impl crate::Writable for MP_TZAHB6ENSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_TZAHB6ENSETR to value 0
impl crate::Resettable for MP_TZAHB6ENSETRrs {}
