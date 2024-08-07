///Register `BSEC_SCRATCH` reader
pub type R = crate::R<BSEC_SCRATCHrs>;
///Register `BSEC_SCRATCH` writer
pub type W = crate::W<BSEC_SCRATCHrs>;
///Field `DATA` reader - DATA
pub type DATA_R = crate::FieldReader<u32>;
///Field `DATA` writer - DATA
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DATA
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSEC_SCRATCH")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DATA
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<BSEC_SCRATCHrs> {
        DATA_W::new(self, 0)
    }
}
/**BSEC scratch register

You can [`read`](crate::Reg::read) this register and get [`bsec_scratch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_scratch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_SCRATCH)*/
pub struct BSEC_SCRATCHrs;
impl crate::RegisterSpec for BSEC_SCRATCHrs {
    type Ux = u32;
}
///`read()` method returns [`bsec_scratch::R`](R) reader structure
impl crate::Readable for BSEC_SCRATCHrs {}
///`write(|w| ..)` method takes [`bsec_scratch::W`](W) writer structure
impl crate::Writable for BSEC_SCRATCHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BSEC_SCRATCH to value 0
impl crate::Resettable for BSEC_SCRATCHrs {
    const RESET_VALUE: u32 = 0;
}
