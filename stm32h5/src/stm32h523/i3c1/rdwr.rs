///Register `RDWR` reader
pub type R = crate::R<RDWRrs>;
///Field `RDB0` reader - 8-bit received data (earliest byte on I3C bus).
pub type RDB0_R = crate::FieldReader;
///Field `RDB1` reader - 8-bit received data (next byte after RDB0 on I3C bus).
pub type RDB1_R = crate::FieldReader;
///Field `RDB2` reader - 8-bit received data (next byte after RDB1 on I3C bus).
pub type RDB2_R = crate::FieldReader;
///Field `RDB3` reader - 8-bit received data (latest byte on I3C bus).
pub type RDB3_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - 8-bit received data (earliest byte on I3C bus).
    #[inline(always)]
    pub fn rdb0(&self) -> RDB0_R {
        RDB0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - 8-bit received data (next byte after RDB0 on I3C bus).
    #[inline(always)]
    pub fn rdb1(&self) -> RDB1_R {
        RDB1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - 8-bit received data (next byte after RDB1 on I3C bus).
    #[inline(always)]
    pub fn rdb2(&self) -> RDB2_R {
        RDB2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - 8-bit received data (latest byte on I3C bus).
    #[inline(always)]
    pub fn rdb3(&self) -> RDB3_R {
        RDB3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDWR")
            .field("rdb0", &self.rdb0())
            .field("rdb1", &self.rdb1())
            .field("rdb2", &self.rdb2())
            .field("rdb3", &self.rdb3())
            .finish()
    }
}
/**I3C receive data word register

You can [`read`](crate::Reg::read) this register and get [`rdwr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#I3C1:RDWR)*/
pub struct RDWRrs;
impl crate::RegisterSpec for RDWRrs {
    type Ux = u32;
}
///`read()` method returns [`rdwr::R`](R) reader structure
impl crate::Readable for RDWRrs {}
///`reset()` method sets RDWR to value 0
impl crate::Resettable for RDWRrs {}
