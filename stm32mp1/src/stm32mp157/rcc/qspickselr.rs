///Register `QSPICKSELR` reader
pub type R = crate::R<QSPICKSELRrs>;
///Register `QSPICKSELR` writer
pub type W = crate::W<QSPICKSELRrs>;
///Field `QSPISRC` reader - QSPISRC
pub type QSPISRC_R = crate::FieldReader;
///Field `QSPISRC` writer - QSPISRC
pub type QSPISRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - QSPISRC
    #[inline(always)]
    pub fn qspisrc(&self) -> QSPISRC_R {
        QSPISRC_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QSPICKSELR")
            .field("qspisrc", &self.qspisrc())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - QSPISRC
    #[inline(always)]
    pub fn qspisrc(&mut self) -> QSPISRC_W<'_, QSPICKSELRrs> {
        QSPISRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`qspickselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspickselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:QSPICKSELR)*/
pub struct QSPICKSELRrs;
impl crate::RegisterSpec for QSPICKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`qspickselr::R`](R) reader structure
impl crate::Readable for QSPICKSELRrs {}
///`write(|w| ..)` method takes [`qspickselr::W`](W) writer structure
impl crate::Writable for QSPICKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QSPICKSELR to value 0
impl crate::Resettable for QSPICKSELRrs {}
