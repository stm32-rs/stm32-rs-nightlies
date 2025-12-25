///Register `CLRISR` reader
pub type R = crate::R<CLRISRrs>;
///Register `CLRISR` writer
pub type W = crate::W<CLRISRrs>;
///Field `CLREOCALF` reader - Clear the end of calibration flag
pub type CLREOCALF_R = crate::BitReader;
///Field `CLREOCALF` writer - Clear the end of calibration flag
pub type CLREOCALF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLRJOVRF` reader - Clear the injected conversion overrun flag
pub type CLRJOVRF_R = crate::BitReader;
///Field `CLRJOVRF` writer - Clear the injected conversion overrun flag
pub type CLRJOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLRROVRF` reader - Clear the regular conversion overrun flag
pub type CLRROVRF_R = crate::BitReader;
///Field `CLRROVRF` writer - Clear the regular conversion overrun flag
pub type CLRROVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clear the end of calibration flag
    #[inline(always)]
    pub fn clreocalf(&self) -> CLREOCALF_R {
        CLREOCALF_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Clear the injected conversion overrun flag
    #[inline(always)]
    pub fn clrjovrf(&self) -> CLRJOVRF_R {
        CLRJOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Clear the regular conversion overrun flag
    #[inline(always)]
    pub fn clrrovrf(&self) -> CLRROVRF_R {
        CLRROVRF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLRISR")
            .field("clrrovrf", &self.clrrovrf())
            .field("clrjovrf", &self.clrjovrf())
            .field("clreocalf", &self.clreocalf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clear the end of calibration flag
    #[inline(always)]
    pub fn clreocalf(&mut self) -> CLREOCALF_W<'_, CLRISRrs> {
        CLREOCALF_W::new(self, 0)
    }
    ///Bit 2 - Clear the injected conversion overrun flag
    #[inline(always)]
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W<'_, CLRISRrs> {
        CLRJOVRF_W::new(self, 2)
    }
    ///Bit 4 - Clear the regular conversion overrun flag
    #[inline(always)]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W<'_, CLRISRrs> {
        CLRROVRF_W::new(self, 4)
    }
}
/**interrupt and status clear register

You can [`read`](crate::Reg::read) this register and get [`clrisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clrisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:CLRISR)*/
pub struct CLRISRrs;
impl crate::RegisterSpec for CLRISRrs {
    type Ux = u32;
}
///`read()` method returns [`clrisr::R`](R) reader structure
impl crate::Readable for CLRISRrs {}
///`write(|w| ..)` method takes [`clrisr::W`](W) writer structure
impl crate::Writable for CLRISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLRISR to value 0
impl crate::Resettable for CLRISRrs {}
