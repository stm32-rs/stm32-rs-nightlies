///Register `CALSOR` reader
pub type R = crate::R<CALSORrs>;
///Register `CALSOR` writer
pub type W = crate::W<CALSORrs>;
///Field `FINE` reader - None
pub type FINE_R = crate::FieldReader;
///Field `FINE` writer - None
pub type FINE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `COARSE` reader - None
pub type COARSE_R = crate::FieldReader;
///Field `COARSE` writer - None
pub type COARSE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:6 - None
    #[inline(always)]
    pub fn fine(&self) -> FINE_R {
        FINE_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:20 - None
    #[inline(always)]
    pub fn coarse(&self) -> COARSE_R {
        COARSE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALSOR")
            .field("fine", &self.fine())
            .field("coarse", &self.coarse())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - None
    #[inline(always)]
    pub fn fine(&mut self) -> FINE_W<'_, CALSORrs> {
        FINE_W::new(self, 0)
    }
    ///Bits 16:20 - None
    #[inline(always)]
    pub fn coarse(&mut self) -> COARSE_W<'_, CALSORrs> {
        COARSE_W::new(self, 16)
    }
}
/**XSPI DLL slave output calibration configuration

You can [`read`](crate::Reg::read) this register and get [`calsor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calsor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#XSPI1:CALSOR)*/
pub struct CALSORrs;
impl crate::RegisterSpec for CALSORrs {
    type Ux = u32;
}
///`read()` method returns [`calsor::R`](R) reader structure
impl crate::Readable for CALSORrs {}
///`write(|w| ..)` method takes [`calsor::W`](W) writer structure
impl crate::Writable for CALSORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CALSOR to value 0
impl crate::Resettable for CALSORrs {}
