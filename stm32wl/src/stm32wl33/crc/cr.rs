///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `RESET` reader - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware
pub type RESET_R = crate::BitReader;
///Field `RESET` writer - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POLYSIZE` reader - Polynomial size These bits control the size of the polynomial. -00: 32 bit polynomial -01: 16 bit polynomial -10: 8 bit polynomial -11: 7 bit polynomial
pub type POLYSIZE_R = crate::FieldReader;
///Field `POLYSIZE` writer - Polynomial size These bits control the size of the polynomial. -00: 32 bit polynomial -01: 16 bit polynomial -10: 8 bit polynomial -11: 7 bit polynomial
pub type POLYSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `REV_IN` reader - Reverse input data These bits control the reversal of the bit order of the input data -00: Bit order not affected -01: Bit reversal done by byte -10: Bit reversal done by half-word -11: Bit reversal done by word
pub type REV_IN_R = crate::FieldReader;
///Field `REV_IN` writer - Reverse input data These bits control the reversal of the bit order of the input data -00: Bit order not affected -01: Bit reversal done by byte -10: Bit reversal done by half-word -11: Bit reversal done by word
pub type REV_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `REV_OUT` reader - Reverse output data This bit controls the reversal of the bit order of the output data. -0: Bit order not affected -1: Bit-reversed output format
pub type REV_OUT_R = crate::BitReader;
///Field `REV_OUT` writer - Reverse output data This bit controls the reversal of the bit order of the output data. -0: Bit order not affected -1: Bit-reversed output format
pub type REV_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 1) != 0)
    }
    ///Bits 3:4 - Polynomial size These bits control the size of the polynomial. -00: 32 bit polynomial -01: 16 bit polynomial -10: 8 bit polynomial -11: 7 bit polynomial
    #[inline(always)]
    pub fn polysize(&self) -> POLYSIZE_R {
        POLYSIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - Reverse input data These bits control the reversal of the bit order of the input data -00: Bit order not affected -01: Bit reversal done by byte -10: Bit reversal done by half-word -11: Bit reversal done by word
    #[inline(always)]
    pub fn rev_in(&self) -> REV_IN_R {
        REV_IN_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Reverse output data This bit controls the reversal of the bit order of the output data. -0: Bit order not affected -1: Bit-reversed output format
    #[inline(always)]
    pub fn rev_out(&self) -> REV_OUT_R {
        REV_OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("reset", &self.reset())
            .field("polysize", &self.polysize())
            .field("rev_in", &self.rev_in())
            .field("rev_out", &self.rev_out())
            .finish()
    }
}
impl W {
    ///Bit 0 - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<'_, CRrs> {
        RESET_W::new(self, 0)
    }
    ///Bits 3:4 - Polynomial size These bits control the size of the polynomial. -00: 32 bit polynomial -01: 16 bit polynomial -10: 8 bit polynomial -11: 7 bit polynomial
    #[inline(always)]
    pub fn polysize(&mut self) -> POLYSIZE_W<'_, CRrs> {
        POLYSIZE_W::new(self, 3)
    }
    ///Bits 5:6 - Reverse input data These bits control the reversal of the bit order of the input data -00: Bit order not affected -01: Bit reversal done by byte -10: Bit reversal done by half-word -11: Bit reversal done by word
    #[inline(always)]
    pub fn rev_in(&mut self) -> REV_IN_W<'_, CRrs> {
        REV_IN_W::new(self, 5)
    }
    ///Bit 7 - Reverse output data This bit controls the reversal of the bit order of the output data. -0: Bit order not affected -1: Bit-reversed output format
    #[inline(always)]
    pub fn rev_out(&mut self) -> REV_OUT_W<'_, CRrs> {
        REV_OUT_W::new(self, 7)
    }
}
/**CRC_CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#CRC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
