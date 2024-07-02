///Register `HSPI_WPIR` reader
pub type R = crate::R<HSPI_WPIRrs>;
///Register `HSPI_WPIR` writer
pub type W = crate::W<HSPI_WPIRrs>;
///Field `INSTRUCTION` reader - 31: 0\]: Instruction Instruction to be sent to the external SPI device
pub type INSTRUCTION_R = crate::FieldReader<u32>;
///Field `INSTRUCTION` writer - 31: 0\]: Instruction Instruction to be sent to the external SPI device
pub type INSTRUCTION_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - 31: 0\]: Instruction Instruction to be sent to the external SPI device
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSPI_WPIR")
            .field("instruction", &self.instruction())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - 31: 0\]: Instruction Instruction to be sent to the external SPI device
    #[inline(always)]
    #[must_use]
    pub fn instruction(&mut self) -> INSTRUCTION_W<HSPI_WPIRrs> {
        INSTRUCTION_W::new(self, 0)
    }
}
/**HSPI wrap instruction register

You can [`read`](crate::Reg::read) this register and get [`hspi_wpir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspi_wpir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:HSPI_WPIR)*/
pub struct HSPI_WPIRrs;
impl crate::RegisterSpec for HSPI_WPIRrs {
    type Ux = u32;
}
///`read()` method returns [`hspi_wpir::R`](R) reader structure
impl crate::Readable for HSPI_WPIRrs {}
///`write(|w| ..)` method takes [`hspi_wpir::W`](W) writer structure
impl crate::Writable for HSPI_WPIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HSPI_WPIR to value 0
impl crate::Resettable for HSPI_WPIRrs {
    const RESET_VALUE: u32 = 0;
}
