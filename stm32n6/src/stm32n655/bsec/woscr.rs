///Register `WOSCR%s` reader
pub type R = crate::R<WOSCRrs>;
///Register `WOSCR%s` writer
pub type W = crate::W<WOSCRrs>;
///Field `WOSDATA` reader - Write once scratch data
pub type WOSDATA_R = crate::FieldReader<u32>;
///Field `WOSDATA` writer - Write once scratch data
pub type WOSDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Write once scratch data
    #[inline(always)]
    pub fn wosdata(&self) -> WOSDATA_R {
        WOSDATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WOSCR")
            .field("wosdata", &self.wosdata())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Write once scratch data
    #[inline(always)]
    pub fn wosdata(&mut self) -> WOSDATA_W<'_, WOSCRrs> {
        WOSDATA_W::new(self, 0)
    }
}
/**BSEC write once scratch register %s

You can [`read`](crate::Reg::read) this register and get [`woscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`woscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#BSEC:WOSCR[0])*/
pub struct WOSCRrs;
impl crate::RegisterSpec for WOSCRrs {
    type Ux = u32;
}
///`read()` method returns [`woscr::R`](R) reader structure
impl crate::Readable for WOSCRrs {}
///`write(|w| ..)` method takes [`woscr::W`](W) writer structure
impl crate::Writable for WOSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WOSCR%s to value 0
impl crate::Resettable for WOSCRrs {}
