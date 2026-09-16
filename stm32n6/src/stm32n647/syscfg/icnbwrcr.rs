///Register `ICNBWRCR` reader
pub type R = crate::R<ICNBWRCRrs>;
///Register `ICNBWRCR` writer
pub type W = crate::W<ICNBWRCRrs>;
///Field `ICNBWRCR` reader - Bandwidth regulator control bits
pub type ICNBWRCR_R = crate::FieldReader<u32>;
///Field `ICNBWRCR` writer - Bandwidth regulator control bits
pub type ICNBWRCR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Bandwidth regulator control bits
    #[inline(always)]
    pub fn icnbwrcr(&self) -> ICNBWRCR_R {
        ICNBWRCR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICNBWRCR")
            .field("icnbwrcr", &self.icnbwrcr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Bandwidth regulator control bits
    #[inline(always)]
    pub fn icnbwrcr(&mut self) -> ICNBWRCR_W<'_, ICNBWRCRrs> {
        ICNBWRCR_W::new(self, 0)
    }
}
/**SYSCFG ICN bandwidth regulator control register

You can [`read`](crate::Reg::read) this register and get [`icnbwrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icnbwrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SYSCFG:ICNBWRCR)*/
pub struct ICNBWRCRrs;
impl crate::RegisterSpec for ICNBWRCRrs {
    type Ux = u32;
}
///`read()` method returns [`icnbwrcr::R`](R) reader structure
impl crate::Readable for ICNBWRCRrs {}
///`write(|w| ..)` method takes [`icnbwrcr::W`](W) writer structure
impl crate::Writable for ICNBWRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICNBWRCR to value 0
impl crate::Resettable for ICNBWRCRrs {}
