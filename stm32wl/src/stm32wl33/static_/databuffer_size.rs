///Register `DATABUFFER_SIZE` reader
pub type R = crate::R<DATABUFFER_SIZErs>;
///Register `DATABUFFER_SIZE` writer
pub type W = crate::W<DATABUFFER_SIZErs>;
///Field `DATABUFFER_SIZE` reader - Size of the Data Buffers (Data Buffer0 and Data Buffer1) expressed in byte unit.
pub type DATABUFFER_SIZE_R = crate::FieldReader<u16>;
///Field `DATABUFFER_SIZE` writer - Size of the Data Buffers (Data Buffer0 and Data Buffer1) expressed in byte unit.
pub type DATABUFFER_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Size of the Data Buffers (Data Buffer0 and Data Buffer1) expressed in byte unit.
    #[inline(always)]
    pub fn databuffer_size(&self) -> DATABUFFER_SIZE_R {
        DATABUFFER_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATABUFFER_SIZE")
            .field("databuffer_size", &self.databuffer_size())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Size of the Data Buffers (Data Buffer0 and Data Buffer1) expressed in byte unit.
    #[inline(always)]
    pub fn databuffer_size(&mut self) -> DATABUFFER_SIZE_W<'_, DATABUFFER_SIZErs> {
        DATABUFFER_SIZE_W::new(self, 0)
    }
}
/**DATABUFFER_SIZE register

You can [`read`](crate::Reg::read) this register and get [`databuffer_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuffer_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:DATABUFFER_SIZE)*/
pub struct DATABUFFER_SIZErs;
impl crate::RegisterSpec for DATABUFFER_SIZErs {
    type Ux = u32;
}
///`read()` method returns [`databuffer_size::R`](R) reader structure
impl crate::Readable for DATABUFFER_SIZErs {}
///`write(|w| ..)` method takes [`databuffer_size::W`](W) writer structure
impl crate::Writable for DATABUFFER_SIZErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DATABUFFER_SIZE to value 0
impl crate::Resettable for DATABUFFER_SIZErs {}
