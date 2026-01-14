///Register `DR` reader
pub type R = crate::R<DRrs>;
///Field `Byte0` reader - Data byte 0
pub type BYTE0_R = crate::FieldReader;
///Field `Byte1` reader - Data byte 1
pub type BYTE1_R = crate::FieldReader;
///Field `Byte2` reader - Data byte 2
pub type BYTE2_R = crate::FieldReader;
///Field `Byte3` reader - Data byte 3
pub type BYTE3_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Data byte 0
    #[inline(always)]
    pub fn byte0(&self) -> BYTE0_R {
        BYTE0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data byte 1
    #[inline(always)]
    pub fn byte1(&self) -> BYTE1_R {
        BYTE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data byte 2
    #[inline(always)]
    pub fn byte2(&self) -> BYTE2_R {
        BYTE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data byte 3
    #[inline(always)]
    pub fn byte3(&self) -> BYTE3_R {
        BYTE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR")
            .field("byte3", &self.byte3())
            .field("byte2", &self.byte2())
            .field("byte1", &self.byte1())
            .field("byte0", &self.byte0())
            .finish()
    }
}
/**data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#DCMI:DR)*/
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
///`read()` method returns [`dr::R`](R) reader structure
impl crate::Readable for DRrs {}
///`reset()` method sets DR to value 0
impl crate::Resettable for DRrs {}
