///Register `IR` reader
pub type R = crate::R<IRrs>;
///Register `IR` writer
pub type W = crate::W<IRrs>;
///Field `CWE` reader - Calibration Watchdog Event
pub type CWE_R = crate::BitReader;
///Field `CWE` writer - Calibration Watchdog Event
pub type CWE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSC` reader - Calibration State Changed
pub type CSC_R = crate::BitReader;
///Field `CSC` writer - Calibration State Changed
pub type CSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Calibration Watchdog Event
    #[inline(always)]
    pub fn cwe(&self) -> CWE_R {
        CWE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Calibration State Changed
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IR")
            .field("cwe", &self.cwe())
            .field("csc", &self.csc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Calibration Watchdog Event
    #[inline(always)]
    pub fn cwe(&mut self) -> CWE_W<'_, IRrs> {
        CWE_W::new(self, 0)
    }
    ///Bit 1 - Calibration State Changed
    #[inline(always)]
    pub fn csc(&mut self) -> CSC_W<'_, IRrs> {
        CSC_W::new(self, 1)
    }
}
/**Clock Calibration Unit Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#CAN_CCU:IR)*/
pub struct IRrs;
impl crate::RegisterSpec for IRrs {
    type Ux = u32;
}
///`read()` method returns [`ir::R`](R) reader structure
impl crate::Readable for IRrs {}
///`write(|w| ..)` method takes [`ir::W`](W) writer structure
impl crate::Writable for IRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IR to value 0
impl crate::Resettable for IRrs {}
