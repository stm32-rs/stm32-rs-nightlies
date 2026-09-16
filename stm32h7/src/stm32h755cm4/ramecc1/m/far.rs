///Register `FAR` reader
pub type R = crate::R<FARrs>;
///Register `FAR` writer
pub type W = crate::W<FARrs>;
///Field `FADD` reader - ECC error failing address
pub type FADD_R = crate::FieldReader<u32>;
///Field `FADD` writer - ECC error failing address
pub type FADD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ECC error failing address
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FAR").field("fadd", &self.fadd()).finish()
    }
}
impl W {
    ///Bits 0:31 - ECC error failing address
    #[inline(always)]
    pub fn fadd(&mut self) -> FADD_W<'_, FARrs> {
        FADD_W::new(self, 0)
    }
}
/**RAMECC monitor x failing address register

You can [`read`](crate::Reg::read) this register and get [`far::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`far::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FARrs;
impl crate::RegisterSpec for FARrs {
    type Ux = u32;
}
///`read()` method returns [`far::R`](R) reader structure
impl crate::Readable for FARrs {}
///`write(|w| ..)` method takes [`far::W`](W) writer structure
impl crate::Writable for FARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FAR to value 0
impl crate::Resettable for FARrs {}
