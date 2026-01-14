///Register `FDCANCKSELR` reader
pub type R = crate::R<FDCANCKSELRrs>;
///Register `FDCANCKSELR` writer
pub type W = crate::W<FDCANCKSELRrs>;
///Field `FDCANSRC` reader - FDCANSRC
pub type FDCANSRC_R = crate::FieldReader;
///Field `FDCANSRC` writer - FDCANSRC
pub type FDCANSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - FDCANSRC
    #[inline(always)]
    pub fn fdcansrc(&self) -> FDCANSRC_R {
        FDCANSRC_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCANCKSELR")
            .field("fdcansrc", &self.fdcansrc())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - FDCANSRC
    #[inline(always)]
    pub fn fdcansrc(&mut self) -> FDCANSRC_W<'_, FDCANCKSELRrs> {
        FDCANSRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the FDCAN block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`fdcanckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcanckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:FDCANCKSELR)*/
pub struct FDCANCKSELRrs;
impl crate::RegisterSpec for FDCANCKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`fdcanckselr::R`](R) reader structure
impl crate::Readable for FDCANCKSELRrs {}
///`write(|w| ..)` method takes [`fdcanckselr::W`](W) writer structure
impl crate::Writable for FDCANCKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCANCKSELR to value 0
impl crate::Resettable for FDCANCKSELRrs {}
