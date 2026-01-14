///Register `CPERCKSELR` reader
pub type R = crate::R<CPERCKSELRrs>;
///Register `CPERCKSELR` writer
pub type W = crate::W<CPERCKSELRrs>;
///Field `CKPERSRC` reader - CKPERSRC
pub type CKPERSRC_R = crate::FieldReader;
///Field `CKPERSRC` writer - CKPERSRC
pub type CKPERSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - CKPERSRC
    #[inline(always)]
    pub fn ckpersrc(&self) -> CKPERSRC_R {
        CKPERSRC_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPERCKSELR")
            .field("ckpersrc", &self.ckpersrc())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - CKPERSRC
    #[inline(always)]
    pub fn ckpersrc(&mut self) -> CKPERSRC_W<'_, CPERCKSELRrs> {
        CKPERSRC_W::new(self, 0)
    }
}
/**This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`cperckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cperckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:CPERCKSELR)*/
pub struct CPERCKSELRrs;
impl crate::RegisterSpec for CPERCKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`cperckselr::R`](R) reader structure
impl crate::Readable for CPERCKSELRrs {}
///`write(|w| ..)` method takes [`cperckselr::W`](W) writer structure
impl crate::Writable for CPERCKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CPERCKSELR to value 0
impl crate::Resettable for CPERCKSELRrs {}
