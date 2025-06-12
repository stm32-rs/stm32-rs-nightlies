///Register `RMR` reader
pub type R = crate::R<RMRrs>;
///Field `IBIRDCNT` reader - IBI received payload data count (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the number of data bytes effectively received in the I3C_IBIDR register.
pub type IBIRDCNT_R = crate::FieldReader;
///Field `RCODE` reader - received CCC code (when the I3C is configured as target) When the I3C is configured as target, this field logs the received CCC code.
pub type RCODE_R = crate::FieldReader;
///Field `RADD` reader - received target address (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the received dynamic address from the target during acknowledged IBI or controller-role request.
pub type RADD_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - IBI received payload data count (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the number of data bytes effectively received in the I3C_IBIDR register.
    #[inline(always)]
    pub fn ibirdcnt(&self) -> IBIRDCNT_R {
        IBIRDCNT_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:15 - received CCC code (when the I3C is configured as target) When the I3C is configured as target, this field logs the received CCC code.
    #[inline(always)]
    pub fn rcode(&self) -> RCODE_R {
        RCODE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 17:23 - received target address (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the received dynamic address from the target during acknowledged IBI or controller-role request.
    #[inline(always)]
    pub fn radd(&self) -> RADD_R {
        RADD_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMR")
            .field("ibirdcnt", &self.ibirdcnt())
            .field("rcode", &self.rcode())
            .field("radd", &self.radd())
            .finish()
    }
}
/**I3C received message register

You can [`read`](crate::Reg::read) this register and get [`rmr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#I3C:RMR)*/
pub struct RMRrs;
impl crate::RegisterSpec for RMRrs {
    type Ux = u32;
}
///`read()` method returns [`rmr::R`](R) reader structure
impl crate::Readable for RMRrs {}
///`reset()` method sets RMR to value 0
impl crate::Resettable for RMRrs {}
