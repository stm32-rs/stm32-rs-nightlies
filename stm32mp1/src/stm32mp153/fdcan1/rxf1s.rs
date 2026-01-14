///Register `RXF1S` reader
pub type R = crate::R<RXF1Srs>;
///Field `F1FL` reader - F1FL
pub type F1FL_R = crate::FieldReader;
///Field `F1GI` reader - F1GI
pub type F1GI_R = crate::FieldReader;
///Field `F1PI` reader - F1PI
pub type F1PI_R = crate::FieldReader;
///Field `F1F` reader - F1F
pub type F1F_R = crate::BitReader;
///Field `RF1L` reader - RF1L
pub type RF1L_R = crate::BitReader;
///Field `DMS` reader - DMS
pub type DMS_R = crate::FieldReader;
impl R {
    ///Bits 0:6 - F1FL
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:13 - F1GI
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:21 - F1PI
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bit 24 - F1F
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - RF1L
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 30:31 - DMS
    #[inline(always)]
    pub fn dms(&self) -> DMS_R {
        DMS_R::new(((self.bits >> 30) & 3) as u8)
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
            .field("dms", &self.dms())
            .finish()
    }
}
/**FDCAN Rx FIFO 1 status register

You can [`read`](crate::Reg::read) this register and get [`rxf1s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:RXF1S)*/
pub struct RXF1Srs;
impl crate::RegisterSpec for RXF1Srs {
    type Ux = u32;
}
///`read()` method returns [`rxf1s::R`](R) reader structure
impl crate::Readable for RXF1Srs {}
///`reset()` method sets RXF1S to value 0
impl crate::Resettable for RXF1Srs {}
