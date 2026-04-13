///Register `DATABUFFER0_PTR` reader
pub type R = crate::R<DATABUFFER0_PTRrs>;
///Register `DATABUFFER0_PTR` writer
pub type W = crate::W<DATABUFFER0_PTRrs>;
///Field `DATABUFFER0_PTR` reader - Start address to be used by the Data Buffer0
pub type DATABUFFER0_PTR_R = crate::FieldReader<u32>;
///Field `DATABUFFER0_PTR` writer - Start address to be used by the Data Buffer0
pub type DATABUFFER0_PTR_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 2:31 - Start address to be used by the Data Buffer0
    #[inline(always)]
    pub fn databuffer0_ptr(&self) -> DATABUFFER0_PTR_R {
        DATABUFFER0_PTR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATABUFFER0_PTR")
            .field("databuffer0_ptr", &self.databuffer0_ptr())
            .finish()
    }
}
impl W {
    ///Bits 2:31 - Start address to be used by the Data Buffer0
    #[inline(always)]
    pub fn databuffer0_ptr(&mut self) -> DATABUFFER0_PTR_W<'_, DATABUFFER0_PTRrs> {
        DATABUFFER0_PTR_W::new(self, 2)
    }
}
/**DATABUFFER0_PTR register

You can [`read`](crate::Reg::read) this register and get [`databuffer0_ptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuffer0_ptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:DATABUFFER0_PTR)*/
pub struct DATABUFFER0_PTRrs;
impl crate::RegisterSpec for DATABUFFER0_PTRrs {
    type Ux = u32;
}
///`read()` method returns [`databuffer0_ptr::R`](R) reader structure
impl crate::Readable for DATABUFFER0_PTRrs {}
///`write(|w| ..)` method takes [`databuffer0_ptr::W`](W) writer structure
impl crate::Writable for DATABUFFER0_PTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DATABUFFER0_PTR to value 0
impl crate::Resettable for DATABUFFER0_PTRrs {}
