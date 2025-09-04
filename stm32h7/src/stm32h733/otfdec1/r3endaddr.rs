///Register `R3ENDADDR` reader
pub type R = crate::R<R3ENDADDRrs>;
///Register `R3ENDADDR` writer
pub type W = crate::W<R3ENDADDRrs>;
///Field `REGx_END_ADDR` reader - Region AXI end address This register must be written before the region corresponding REG_EN bit in the RxCFGR register is set, and RxENDADDR must be strictly greater than RxSTARTADDR to be valid. Writing this register while the region CONFIGLOCK bit in the RxCFGR register is set will be discarded. Note: When determining the region the first 12 bits (LSB) and the last 4 bits (MSB) are ignored. When this register is accessed in read the 4 MSB bits returns zeros and the 12 LSB bits return ones.
pub type REGX_END_ADDR_R = crate::FieldReader<u32>;
///Field `REGx_END_ADDR` writer - Region AXI end address This register must be written before the region corresponding REG_EN bit in the RxCFGR register is set, and RxENDADDR must be strictly greater than RxSTARTADDR to be valid. Writing this register while the region CONFIGLOCK bit in the RxCFGR register is set will be discarded. Note: When determining the region the first 12 bits (LSB) and the last 4 bits (MSB) are ignored. When this register is accessed in read the 4 MSB bits returns zeros and the 12 LSB bits return ones.
pub type REGX_END_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Region AXI end address This register must be written before the region corresponding REG_EN bit in the RxCFGR register is set, and RxENDADDR must be strictly greater than RxSTARTADDR to be valid. Writing this register while the region CONFIGLOCK bit in the RxCFGR register is set will be discarded. Note: When determining the region the first 12 bits (LSB) and the last 4 bits (MSB) are ignored. When this register is accessed in read the 4 MSB bits returns zeros and the 12 LSB bits return ones.
    #[inline(always)]
    pub fn regx_end_addr(&self) -> REGX_END_ADDR_R {
        REGX_END_ADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R3ENDADDR")
            .field("regx_end_addr", &self.regx_end_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Region AXI end address This register must be written before the region corresponding REG_EN bit in the RxCFGR register is set, and RxENDADDR must be strictly greater than RxSTARTADDR to be valid. Writing this register while the region CONFIGLOCK bit in the RxCFGR register is set will be discarded. Note: When determining the region the first 12 bits (LSB) and the last 4 bits (MSB) are ignored. When this register is accessed in read the 4 MSB bits returns zeros and the 12 LSB bits return ones.
    #[inline(always)]
    pub fn regx_end_addr(&mut self) -> REGX_END_ADDR_W<R3ENDADDRrs> {
        REGX_END_ADDR_W::new(self, 0)
    }
}
/**OTFDEC region 3 end address register

You can [`read`](crate::Reg::read) this register and get [`r3endaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r3endaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:R3ENDADDR)*/
pub struct R3ENDADDRrs;
impl crate::RegisterSpec for R3ENDADDRrs {
    type Ux = u32;
}
///`read()` method returns [`r3endaddr::R`](R) reader structure
impl crate::Readable for R3ENDADDRrs {}
///`write(|w| ..)` method takes [`r3endaddr::W`](W) writer structure
impl crate::Writable for R3ENDADDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R3ENDADDR to value 0x0fff
impl crate::Resettable for R3ENDADDRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
