///Register `R3STARTADDR` reader
pub type R = crate::R<R3STARTADDRrs>;
///Register `R3STARTADDR` writer
pub type W = crate::W<R3STARTADDRrs>;
///Field `REG3_START_ADDR` reader - Region AXI start address
pub type REG3_START_ADDR_R = crate::FieldReader<u32>;
///Field `REG3_START_ADDR` writer - Region AXI start address
pub type REG3_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Region AXI start address
    #[inline(always)]
    pub fn reg3_start_addr(&self) -> REG3_START_ADDR_R {
        REG3_START_ADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R3STARTADDR")
            .field("reg3_start_addr", &self.reg3_start_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Region AXI start address
    #[inline(always)]
    pub fn reg3_start_addr(&mut self) -> REG3_START_ADDR_W<R3STARTADDRrs> {
        REG3_START_ADDR_W::new(self, 0)
    }
}
/**OTFDEC region x start address register

You can [`read`](crate::Reg::read) this register and get [`r3startaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r3startaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#OTFDEC1:R3STARTADDR)*/
pub struct R3STARTADDRrs;
impl crate::RegisterSpec for R3STARTADDRrs {
    type Ux = u32;
}
///`read()` method returns [`r3startaddr::R`](R) reader structure
impl crate::Readable for R3STARTADDRrs {}
///`write(|w| ..)` method takes [`r3startaddr::W`](W) writer structure
impl crate::Writable for R3STARTADDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets R3STARTADDR to value 0
impl crate::Resettable for R3STARTADDRrs {
    const RESET_VALUE: u32 = 0;
}