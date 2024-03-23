#[doc = "Register `HSPI_IR` reader"]
pub type R = crate::R<HSPI_IRrs>;
#[doc = "Register `HSPI_IR` writer"]
pub type W = crate::W<HSPI_IRrs>;
#[doc = "Field `INSTRUCTION` reader - Instruction Instruction to be sent to the external SPI device"]
pub type INSTRUCTION_R = crate::FieldReader<u32>;
#[doc = "Field `INSTRUCTION` writer - Instruction Instruction to be sent to the external SPI device"]
pub type INSTRUCTION_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Instruction Instruction to be sent to the external SPI device"]
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Instruction Instruction to be sent to the external SPI device"]
    #[inline(always)]
    #[must_use]
    pub fn instruction(&mut self) -> INSTRUCTION_W<HSPI_IRrs> {
        INSTRUCTION_W::new(self, 0)
    }
}
#[doc = "HSPI instruction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_ir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_ir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSPI_IRrs;
impl crate::RegisterSpec for HSPI_IRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hspi_ir::R`](R) reader structure"]
impl crate::Readable for HSPI_IRrs {}
#[doc = "`write(|w| ..)` method takes [`hspi_ir::W`](W) writer structure"]
impl crate::Writable for HSPI_IRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSPI_IR to value 0"]
impl crate::Resettable for HSPI_IRrs {
    const RESET_VALUE: u32 = 0;
}
