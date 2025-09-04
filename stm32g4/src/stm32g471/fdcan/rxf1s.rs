///Register `RXF1S` reader
pub type R = crate::R<RXF1Srs>;
///Field `F1FL` reader - Rx FIFO 1 fill level Number of elements stored in Rx FIFO 1, range 0 to 3.
pub type F1FL_R = crate::FieldReader;
///Field `F1GI` reader - Rx FIFO 1 get index Rx FIFO 1 read index pointer, range 0 to 2.
pub type F1GI_R = crate::FieldReader;
///Field `F1PI` reader - Rx FIFO 1 put index Rx FIFO 1 write index pointer, range 0 to 2.
pub type F1PI_R = crate::FieldReader;
///Field `F1F` reader - Rx FIFO 1 full
pub type F1F_R = crate::BitReader;
///Field `RF1L` reader - Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\[RF1L\]. When IR\[RF1L\] is reset, this bit is also reset.
pub type RF1L_R = crate::BitReader;
impl R {
    ///Bits 0:3 - Rx FIFO 1 fill level Number of elements stored in Rx FIFO 1, range 0 to 3.
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:9 - Rx FIFO 1 get index Rx FIFO 1 read index pointer, range 0 to 2.
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - Rx FIFO 1 put index Rx FIFO 1 write index pointer, range 0 to 2.
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 24 - Rx FIFO 1 full
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\[RF1L\]. When IR\[RF1L\] is reset, this bit is also reset.
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXF1S")
            .field("f1fl", &self.f1fl())
            .field("f1gi", &self.f1gi())
            .field("f1pi", &self.f1pi())
            .field("f1f", &self.f1f())
            .field("rf1l", &self.rf1l())
            .finish()
    }
}
/**FDCAN Rx FIFO 1 status register

You can [`read`](crate::Reg::read) this register and get [`rxf1s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G471.html#FDCAN:RXF1S)*/
pub struct RXF1Srs;
impl crate::RegisterSpec for RXF1Srs {
    type Ux = u32;
}
///`read()` method returns [`rxf1s::R`](R) reader structure
impl crate::Readable for RXF1Srs {}
///`reset()` method sets RXF1S to value 0
impl crate::Resettable for RXF1Srs {}
