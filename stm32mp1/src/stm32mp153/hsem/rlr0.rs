///Register `RLR0` reader
pub type R = crate::R<RLR0rs>;
///Field `PROCID` reader - PROCID
pub type PROCID_R = crate::FieldReader;
///Field `COREID` reader - COREID
pub type COREID_R = crate::FieldReader;
///Field `LOCK` reader - LOCK
pub type LOCK_R = crate::BitReader;
impl R {
    ///Bits 0:7 - PROCID
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - COREID
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RLR0")
            .field("procid", &self.procid())
            .field("coreid", &self.coreid())
            .field("lock", &self.lock())
            .finish()
    }
}
/**Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR0)*/
pub struct RLR0rs;
impl crate::RegisterSpec for RLR0rs {
    type Ux = u32;
}
///`read()` method returns [`rlr0::R`](R) reader structure
impl crate::Readable for RLR0rs {}
///`reset()` method sets RLR0 to value 0
impl crate::Resettable for RLR0rs {}
