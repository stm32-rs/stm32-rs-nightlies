///Register `FPUIMR` reader
pub type R = crate::R<FPUIMRrs>;
///Register `FPUIMR` writer
pub type W = crate::W<FPUIMRrs>;
///Field `FPU_IE` reader - FPU interrupt enable Set and cleared by software to enable the Cortex-M7 FPU interrupts xxxxx1: Invalid operation interrupt enabled (xxxxx0 to disable) xxxx1x: Divide-by-zero interrupt enabled (xxxx0x to disable) xxx1xx: Underflow interrupt enabled (xxx0xx to disable) xx1xxx: Overflow interrupt enabled (xx0xxx to disable) x1xxxx: Input denormal interrupt enabled (x0xxxx to disable) 1xxxxx: Inexact interrupt enabled (0xxxxx to disable), disabled by default
pub type FPU_IE_R = crate::FieldReader;
///Field `FPU_IE` writer - FPU interrupt enable Set and cleared by software to enable the Cortex-M7 FPU interrupts xxxxx1: Invalid operation interrupt enabled (xxxxx0 to disable) xxxx1x: Divide-by-zero interrupt enabled (xxxx0x to disable) xxx1xx: Underflow interrupt enabled (xxx0xx to disable) xx1xxx: Overflow interrupt enabled (xx0xxx to disable) x1xxxx: Input denormal interrupt enabled (x0xxxx to disable) 1xxxxx: Inexact interrupt enabled (0xxxxx to disable), disabled by default
pub type FPU_IE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - FPU interrupt enable Set and cleared by software to enable the Cortex-M7 FPU interrupts xxxxx1: Invalid operation interrupt enabled (xxxxx0 to disable) xxxx1x: Divide-by-zero interrupt enabled (xxxx0x to disable) xxx1xx: Underflow interrupt enabled (xxx0xx to disable) xx1xxx: Overflow interrupt enabled (xx0xxx to disable) x1xxxx: Input denormal interrupt enabled (x0xxxx to disable) 1xxxxx: Inexact interrupt enabled (0xxxxx to disable), disabled by default
    #[inline(always)]
    pub fn fpu_ie(&self) -> FPU_IE_R {
        FPU_IE_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPUIMR")
            .field("fpu_ie", &self.fpu_ie())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - FPU interrupt enable Set and cleared by software to enable the Cortex-M7 FPU interrupts xxxxx1: Invalid operation interrupt enabled (xxxxx0 to disable) xxxx1x: Divide-by-zero interrupt enabled (xxxx0x to disable) xxx1xx: Underflow interrupt enabled (xxx0xx to disable) xx1xxx: Overflow interrupt enabled (xx0xxx to disable) x1xxxx: Input denormal interrupt enabled (x0xxxx to disable) 1xxxxx: Inexact interrupt enabled (0xxxxx to disable), disabled by default
    #[inline(always)]
    pub fn fpu_ie(&mut self) -> FPU_IE_W<'_, FPUIMRrs> {
        FPU_IE_W::new(self, 0)
    }
}
/**SBS FPU interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`fpuimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpuimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#SBS:FPUIMR)*/
pub struct FPUIMRrs;
impl crate::RegisterSpec for FPUIMRrs {
    type Ux = u32;
}
///`read()` method returns [`fpuimr::R`](R) reader structure
impl crate::Readable for FPUIMRrs {}
///`write(|w| ..)` method takes [`fpuimr::W`](W) writer structure
impl crate::Writable for FPUIMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FPUIMR to value 0x1f
impl crate::Resettable for FPUIMRrs {
    const RESET_VALUE: u32 = 0x1f;
}
