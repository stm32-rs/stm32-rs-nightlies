///Register `DATABUFFER1_PTR` reader
pub type R = crate::R<DATABUFFER1_PTRrs>;
///Register `DATABUFFER1_PTR` writer
pub type W = crate::W<DATABUFFER1_PTRrs>;
///Field `DATABUFFER1_PTR` reader - Start address to be used by the Data Buffer1
pub type DATABUFFER1_PTR_R = crate::FieldReader<u32>;
///Field `DATABUFFER1_PTR` writer - Start address to be used by the Data Buffer1
pub type DATABUFFER1_PTR_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 2:31 - Start address to be used by the Data Buffer1
    #[inline(always)]
    pub fn databuffer1_ptr(&self) -> DATABUFFER1_PTR_R {
        DATABUFFER1_PTR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATABUFFER1_PTR")
            .field("databuffer1_ptr", &self.databuffer1_ptr())
            .finish()
    }
}
impl W {
    ///Bits 2:31 - Start address to be used by the Data Buffer1
    #[inline(always)]
    pub fn databuffer1_ptr(&mut self) -> DATABUFFER1_PTR_W<'_, DATABUFFER1_PTRrs> {
        DATABUFFER1_PTR_W::new(self, 2)
    }
}
/**DATABUFFER1_PTR register

You can [`read`](crate::Reg::read) this register and get [`databuffer1_ptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuffer1_ptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:DATABUFFER1_PTR)*/
pub struct DATABUFFER1_PTRrs;
impl crate::RegisterSpec for DATABUFFER1_PTRrs {
    type Ux = u32;
}
///`read()` method returns [`databuffer1_ptr::R`](R) reader structure
impl crate::Readable for DATABUFFER1_PTRrs {}
///`write(|w| ..)` method takes [`databuffer1_ptr::W`](W) writer structure
impl crate::Writable for DATABUFFER1_PTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DATABUFFER1_PTR to value 0
impl crate::Resettable for DATABUFFER1_PTRrs {}
