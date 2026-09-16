///Register `FDRL` reader
pub type R = crate::R<FDRLrs>;
///Register `FDRL` writer
pub type W = crate::W<FDRLrs>;
///Field `FDATAL` reader - Failing data low
pub type FDATAL_R = crate::FieldReader<u32>;
///Field `FDATAL` writer - Failing data low
pub type FDATAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Failing data low
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDRL")
            .field("fdatal", &self.fdatal())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Failing data low
    #[inline(always)]
    pub fn fdatal(&mut self) -> FDATAL_W<'_, FDRLrs> {
        FDATAL_W::new(self, 0)
    }
}
/**RAMECC monitor x failing data low register

You can [`read`](crate::Reg::read) this register and get [`fdrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FDRLrs;
impl crate::RegisterSpec for FDRLrs {
    type Ux = u32;
}
///`read()` method returns [`fdrl::R`](R) reader structure
impl crate::Readable for FDRLrs {}
///`write(|w| ..)` method takes [`fdrl::W`](W) writer structure
impl crate::Writable for FDRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDRL to value 0
impl crate::Resettable for FDRLrs {}
