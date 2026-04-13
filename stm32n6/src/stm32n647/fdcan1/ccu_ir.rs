///Register `CCU_IR` reader
pub type R = crate::R<CCU_IRrs>;
///Register `CCU_IR` writer
pub type W = crate::W<CCU_IRrs>;
///Field `CWE` reader - Calibration watchdog event
pub type CWE_R = crate::BitReader;
///Field `CWE` writer - Calibration watchdog event
pub type CWE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSC` reader - Calibration state changed
pub type CSC_R = crate::BitReader;
///Field `CSC` writer - Calibration state changed
pub type CSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Calibration watchdog event
    #[inline(always)]
    pub fn cwe(&self) -> CWE_R {
        CWE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Calibration state changed
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCU_IR")
            .field("cwe", &self.cwe())
            .field("csc", &self.csc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Calibration watchdog event
    #[inline(always)]
    pub fn cwe(&mut self) -> CWE_W<'_, CCU_IRrs> {
        CWE_W::new(self, 0)
    }
    ///Bit 1 - Calibration state changed
    #[inline(always)]
    pub fn csc(&mut self) -> CSC_W<'_, CCU_IRrs> {
        CSC_W::new(self, 1)
    }
}
/**FDCAN test register

You can [`read`](crate::Reg::read) this register and get [`ccu_ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccu_ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:CCU_IR)*/
pub struct CCU_IRrs;
impl crate::RegisterSpec for CCU_IRrs {
    type Ux = u32;
}
///`read()` method returns [`ccu_ir::R`](R) reader structure
impl crate::Readable for CCU_IRrs {}
///`write(|w| ..)` method takes [`ccu_ir::W`](W) writer structure
impl crate::Writable for CCU_IRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCU_IR to value 0
impl crate::Resettable for CCU_IRrs {}
