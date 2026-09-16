///Register `IPGR3` reader
pub type R = crate::R<IPGR3rs>;
///Field `IDLE` reader - Status of IP-Plug
pub type IDLE_R = crate::BitReader;
impl R {
    ///Bit 0 - Status of IP-Plug
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPGR3").field("idle", &self.idle()).finish()
    }
}
/**DCMIPP IPPLUG global register 3

You can [`read`](crate::Reg::read) this register and get [`ipgr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:IPGR3)*/
pub struct IPGR3rs;
impl crate::RegisterSpec for IPGR3rs {
    type Ux = u32;
}
///`read()` method returns [`ipgr3::R`](R) reader structure
impl crate::Readable for IPGR3rs {}
///`reset()` method sets IPGR3 to value 0x01
impl crate::Resettable for IPGR3rs {
    const RESET_VALUE: u32 = 0x01;
}
