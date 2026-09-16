///Register `UR0` reader
pub type R = crate::R<UR0rs>;
///Field `BKS` reader - Bank Swap
pub type BKS_R = crate::BitReader;
///Field `RDP` reader - Readout protection
pub type RDP_R = crate::FieldReader;
impl R {
    ///Bit 0 - Bank Swap
    #[inline(always)]
    pub fn bks(&self) -> BKS_R {
        BKS_R::new((self.bits & 1) != 0)
    }
    ///Bits 16:23 - Readout protection
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR0")
            .field("bks", &self.bks())
            .field("rdp", &self.rdp())
            .finish()
    }
}
/**SYSCFG user register 0

You can [`read`](crate::Reg::read) this register and get [`ur0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#SYSCFG:UR0)*/
pub struct UR0rs;
impl crate::RegisterSpec for UR0rs {
    type Ux = u32;
}
///`read()` method returns [`ur0::R`](R) reader structure
impl crate::Readable for UR0rs {}
///`reset()` method sets UR0 to value 0
impl crate::Resettable for UR0rs {}
