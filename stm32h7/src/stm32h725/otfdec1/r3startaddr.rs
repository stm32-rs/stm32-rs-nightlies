///Register `R3STARTADDR` reader
pub type R = crate::R<R3STARTADDRrs>;
///Register `R3STARTADDR` writer
pub type W = crate::W<R3STARTADDRrs>;
///Field `REGx_START_ADDR` reader - Region AXI start address This register must be written before the region corresponding REG_EN bit in the RxCFGR register is set. Writing this register while the region CONFIGLOCK bit in the RxCFGR register is set will be discarded. Note: When determining the region the first 12 bits (LSB) and the last 4 bits (MSB) are ignored. When this register is accessed in read the 4 MSB bits and the 12 LSB bits return zero.
pub type REGX_START_ADDR_R = crate::FieldReader<u32>;
///Field `REGx_START_ADDR` writer - Region AXI start address This register must be written before the region corresponding REG_EN bit in the RxCFGR register is set. Writing this register while the region CONFIGLOCK bit in the RxCFGR register is set will be discarded. Note: When determining the region the first 12 bits (LSB) and the last 4 bits (MSB) are ignored. When this register is accessed in read the 4 MSB bits and the 12 LSB bits return zero.
pub type REGX_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Region AXI start address This register must be written before the region corresponding REG_EN bit in the RxCFGR register is set. Writing this register while the region CONFIGLOCK bit in the RxCFGR register is set will be discarded. Note: When determining the region the first 12 bits (LSB) and the last 4 bits (MSB) are ignored. When this register is accessed in read the 4 MSB bits and the 12 LSB bits return zero.
    #[inline(always)]
    pub fn regx_start_addr(&self) -> REGX_START_ADDR_R {
        REGX_START_ADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R3STARTADDR")
            .field("regx_start_addr", &self.regx_start_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Region AXI start address This register must be written before the region corresponding REG_EN bit in the RxCFGR register is set. Writing this register while the region CONFIGLOCK bit in the RxCFGR register is set will be discarded. Note: When determining the region the first 12 bits (LSB) and the last 4 bits (MSB) are ignored. When this register is accessed in read the 4 MSB bits and the 12 LSB bits return zero.
    #[inline(always)]
    pub fn regx_start_addr(&mut self) -> REGX_START_ADDR_W<'_, R3STARTADDRrs> {
        REGX_START_ADDR_W::new(self, 0)
    }
}
/**OTFDEC region 3 start address register

You can [`read`](crate::Reg::read) this register and get [`r3startaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r3startaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#OTFDEC1:R3STARTADDR)*/
pub struct R3STARTADDRrs;
impl crate::RegisterSpec for R3STARTADDRrs {
    type Ux = u32;
}
///`read()` method returns [`r3startaddr::R`](R) reader structure
impl crate::Readable for R3STARTADDRrs {}
///`write(|w| ..)` method takes [`r3startaddr::W`](W) writer structure
impl crate::Writable for R3STARTADDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R3STARTADDR to value 0
impl crate::Resettable for R3STARTADDRrs {}
