///Register `PAR` reader
pub type R = crate::R<PARrs>;
///Register `PAR` writer
pub type W = crate::W<PARrs>;
///Field `PA` reader - Peripheral address
pub type PA_R = crate::FieldReader<u32>;
///Field `PA` writer - Peripheral address
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Peripheral address
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAR").field("pa", &self.pa()).finish()
    }
}
impl W {
    ///Bits 0:31 - Peripheral address
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<PARrs> {
        PA_W::new(self, 0)
    }
}
/**stream x peripheral address register

You can [`read`](crate::Reg::read) this register and get [`par::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`par::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PARrs;
impl crate::RegisterSpec for PARrs {
    type Ux = u32;
}
///`read()` method returns [`par::R`](R) reader structure
impl crate::Readable for PARrs {}
///`write(|w| ..)` method takes [`par::W`](W) writer structure
impl crate::Writable for PARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PAR to value 0
impl crate::Resettable for PARrs {
    const RESET_VALUE: u32 = 0;
}
