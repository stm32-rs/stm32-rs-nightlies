///Register `RXESC` reader
pub type R = crate::R<RXESCrs>;
///Field `F0DS` reader - Rx FIFO 1 data field size
pub type F0DS_R = crate::FieldReader;
///Field `F1DS` reader - Rx FIFO 0 data field size
pub type F1DS_R = crate::FieldReader;
///Field `RBDS` reader - Rx buffer data field size
pub type RBDS_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - Rx FIFO 1 data field size
    #[inline(always)]
    pub fn f0ds(&self) -> F0DS_R {
        F0DS_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Rx FIFO 0 data field size
    #[inline(always)]
    pub fn f1ds(&self) -> F1DS_R {
        F1DS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - Rx buffer data field size
    #[inline(always)]
    pub fn rbds(&self) -> RBDS_R {
        RBDS_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXESC")
            .field("f0ds", &self.f0ds())
            .field("f1ds", &self.f1ds())
            .field("rbds", &self.rbds())
            .finish()
    }
}
/**FDCAN Rx buffer element size configuration register

You can [`read`](crate::Reg::read) this register and get [`rxesc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FDCAN1:RXESC)*/
pub struct RXESCrs;
impl crate::RegisterSpec for RXESCrs {
    type Ux = u32;
}
///`read()` method returns [`rxesc::R`](R) reader structure
impl crate::Readable for RXESCrs {}
///`reset()` method sets RXESC to value 0
impl crate::Resettable for RXESCrs {}
