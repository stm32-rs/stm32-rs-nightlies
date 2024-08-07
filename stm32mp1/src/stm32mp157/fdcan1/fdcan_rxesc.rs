///Register `FDCAN_RXESC` reader
pub type R = crate::R<FDCAN_RXESCrs>;
///Field `F0DS` reader - F0DS
pub type F0DS_R = crate::FieldReader;
///Field `F1DS` reader - F1DS
pub type F1DS_R = crate::FieldReader;
///Field `RBDS` reader - RBDS
pub type RBDS_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - F0DS
    #[inline(always)]
    pub fn f0ds(&self) -> F0DS_R {
        F0DS_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - F1DS
    #[inline(always)]
    pub fn f1ds(&self) -> F1DS_R {
        F1DS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - RBDS
    #[inline(always)]
    pub fn rbds(&self) -> RBDS_R {
        RBDS_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXESC")
            .field("f0ds", &self.f0ds())
            .field("f1ds", &self.f1ds())
            .field("rbds", &self.rbds())
            .finish()
    }
}
/**Configures the number of data bytes belonging to an Rx buffer / Rx FIFO element. Data field sizes higher than 8 bytes are intended for CAN FD operation only.

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxesc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:FDCAN_RXESC)*/
pub struct FDCAN_RXESCrs;
impl crate::RegisterSpec for FDCAN_RXESCrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_rxesc::R`](R) reader structure
impl crate::Readable for FDCAN_RXESCrs {}
///`reset()` method sets FDCAN_RXESC to value 0
impl crate::Resettable for FDCAN_RXESCrs {
    const RESET_VALUE: u32 = 0;
}
