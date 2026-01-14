///Register `RDL1R` reader
pub type R = crate::R<RDL1Rrs>;
///Field `DATA0` reader - DATA0
pub type DATA0_R = crate::FieldReader;
///Field `DATA1` reader - DATA1
pub type DATA1_R = crate::FieldReader;
///Field `DATA2` reader - DATA2
pub type DATA2_R = crate::FieldReader;
///Field `DATA3` reader - DATA3
pub type DATA3_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - DATA0
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - DATA1
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - DATA2
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - DATA3
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDL1R")
            .field("data3", &self.data3())
            .field("data2", &self.data2())
            .field("data1", &self.data1())
            .field("data0", &self.data0())
            .finish()
    }
}
/**mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`rdl1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#CAN1:RDL1R)*/
pub struct RDL1Rrs;
impl crate::RegisterSpec for RDL1Rrs {
    type Ux = u32;
}
///`read()` method returns [`rdl1r::R`](R) reader structure
impl crate::Readable for RDL1Rrs {}
///`reset()` method sets RDL1R to value 0
impl crate::Resettable for RDL1Rrs {}
