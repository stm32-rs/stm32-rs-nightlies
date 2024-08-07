///Register `DDRPERFM_IER` reader
pub type R = crate::R<DDRPERFM_IERrs>;
///Register `DDRPERFM_IER` writer
pub type W = crate::W<DDRPERFM_IERrs>;
///Field `OVFIE` reader - OVFIE
pub type OVFIE_R = crate::BitReader;
///Field `OVFIE` writer - OVFIE
pub type OVFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - OVFIE
    #[inline(always)]
    pub fn ovfie(&self) -> OVFIE_R {
        OVFIE_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRPERFM_IER")
            .field("ovfie", &self.ovfie())
            .finish()
    }
}
impl W {
    ///Bit 0 - OVFIE
    #[inline(always)]
    #[must_use]
    pub fn ovfie(&mut self) -> OVFIE_W<DDRPERFM_IERrs> {
        OVFIE_W::new(self, 0)
    }
}
/**DDRPERFM interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ddrperfm_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrperfm_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPERFM:DDRPERFM_IER)*/
pub struct DDRPERFM_IERrs;
impl crate::RegisterSpec for DDRPERFM_IERrs {
    type Ux = u32;
}
///`read()` method returns [`ddrperfm_ier::R`](R) reader structure
impl crate::Readable for DDRPERFM_IERrs {}
///`write(|w| ..)` method takes [`ddrperfm_ier::W`](W) writer structure
impl crate::Writable for DDRPERFM_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRPERFM_IER to value 0
impl crate::Resettable for DDRPERFM_IERrs {
    const RESET_VALUE: u32 = 0;
}
