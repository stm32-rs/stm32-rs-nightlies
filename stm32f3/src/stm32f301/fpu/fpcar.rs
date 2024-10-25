///Register `FPCAR` reader
pub type R = crate::R<FPCARrs>;
///Register `FPCAR` writer
pub type W = crate::W<FPCARrs>;
///Field `ADDRESS` reader - Location of unpopulated floating-point
pub type ADDRESS_R = crate::FieldReader<u32>;
///Field `ADDRESS` writer - Location of unpopulated floating-point
pub type ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bits 3:31 - Location of unpopulated floating-point
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPCAR")
            .field("address", &self.address())
            .finish()
    }
}
impl W {
    ///Bits 3:31 - Location of unpopulated floating-point
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<FPCARrs> {
        ADDRESS_W::new(self, 3)
    }
}
/**Floating-point context address register

You can [`read`](crate::Reg::read) this register and get [`fpcar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpcar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F301.html#FPU:FPCAR)*/
pub struct FPCARrs;
impl crate::RegisterSpec for FPCARrs {
    type Ux = u32;
}
///`read()` method returns [`fpcar::R`](R) reader structure
impl crate::Readable for FPCARrs {}
///`write(|w| ..)` method takes [`fpcar::W`](W) writer structure
impl crate::Writable for FPCARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FPCAR to value 0
impl crate::Resettable for FPCARrs {
    const RESET_VALUE: u32 = 0;
}
