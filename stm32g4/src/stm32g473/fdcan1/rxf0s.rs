///Register `RXF0S` reader
pub type R = crate::R<RXF0Srs>;
///Field `F0FL` reader - F0FL
pub type F0FL_R = crate::FieldReader;
///Field `F0GI` reader - F0GI
pub type F0GI_R = crate::FieldReader;
///Field `F0PI` reader - F0PI
pub type F0PI_R = crate::FieldReader;
///Field `F0F` reader - F0F
pub type F0F_R = crate::BitReader;
///Field `RF0L` reader - RF0L
pub type RF0L_R = crate::BitReader;
impl R {
    ///Bits 0:3 - F0FL
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:9 - F0GI
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - F0PI
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 24 - F0F
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - RF0L
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXF0S")
            .field("f0fl", &self.f0fl())
            .field("f0gi", &self.f0gi())
            .field("f0pi", &self.f0pi())
            .field("f0f", &self.f0f())
            .field("rf0l", &self.rf0l())
            .finish()
    }
}
/**FDCAN Rx FIFO 0 Status Register

You can [`read`](crate::Reg::read) this register and get [`rxf0s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#FDCAN1:RXF0S)*/
pub struct RXF0Srs;
impl crate::RegisterSpec for RXF0Srs {
    type Ux = u32;
}
///`read()` method returns [`rxf0s::R`](R) reader structure
impl crate::Readable for RXF0Srs {}
///`reset()` method sets RXF0S to value 0
impl crate::Resettable for RXF0Srs {
    const RESET_VALUE: u32 = 0;
}