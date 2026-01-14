///Register `RDH1R` reader
pub type R = crate::R<RDH1Rrs>;
///Field `DATA4` reader - DATA4
pub type DATA4_R = crate::FieldReader;
///Field `DATA5` reader - DATA5
pub type DATA5_R = crate::FieldReader;
///Field `DATA6` reader - DATA6
pub type DATA6_R = crate::FieldReader;
///Field `DATA7` reader - DATA7
pub type DATA7_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - DATA4
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - DATA5
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - DATA6
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - DATA7
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDH1R")
            .field("data7", &self.data7())
            .field("data6", &self.data6())
            .field("data5", &self.data5())
            .field("data4", &self.data4())
            .finish()
    }
}
/**mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`rdh1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#CAN1:RDH1R)*/
pub struct RDH1Rrs;
impl crate::RegisterSpec for RDH1Rrs {
    type Ux = u32;
}
///`read()` method returns [`rdh1r::R`](R) reader structure
impl crate::Readable for RDH1Rrs {}
///`reset()` method sets RDH1R to value 0
impl crate::Resettable for RDH1Rrs {}
