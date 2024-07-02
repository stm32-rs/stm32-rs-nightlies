///Register `OTG_DSTS` reader
pub type R = crate::R<OTG_DSTSrs>;
///Field `SUSPSTS` reader - SUSPSTS
pub type SUSPSTS_R = crate::BitReader;
///Field `ENUMSPD` reader - ENUMSPD
pub type ENUMSPD_R = crate::FieldReader;
///Field `EERR` reader - EERR
pub type EERR_R = crate::BitReader;
///Field `FNSOF` reader - FNSOF
pub type FNSOF_R = crate::FieldReader<u16>;
///Field `DEVLNSTS` reader - DEVLNSTS
pub type DEVLNSTS_R = crate::FieldReader;
impl R {
    ///Bit 0 - SUSPSTS
    #[inline(always)]
    pub fn suspsts(&self) -> SUSPSTS_R {
        SUSPSTS_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - ENUMSPD
    #[inline(always)]
    pub fn enumspd(&self) -> ENUMSPD_R {
        ENUMSPD_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - EERR
    #[inline(always)]
    pub fn eerr(&self) -> EERR_R {
        EERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 8:21 - FNSOF
    #[inline(always)]
    pub fn fnsof(&self) -> FNSOF_R {
        FNSOF_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
    ///Bits 22:23 - DEVLNSTS
    #[inline(always)]
    pub fn devlnsts(&self) -> DEVLNSTS_R {
        DEVLNSTS_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_DSTS")
            .field("suspsts", &self.suspsts())
            .field("enumspd", &self.enumspd())
            .field("eerr", &self.eerr())
            .field("fnsof", &self.fnsof())
            .field("devlnsts", &self.devlnsts())
            .finish()
    }
}
/**This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (OTG_DAINT) register.

You can [`read`](crate::Reg::read) this register and get [`otg_dsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#OTG:OTG_DSTS)*/
pub struct OTG_DSTSrs;
impl crate::RegisterSpec for OTG_DSTSrs {
    type Ux = u32;
}
///`read()` method returns [`otg_dsts::R`](R) reader structure
impl crate::Readable for OTG_DSTSrs {}
///`reset()` method sets OTG_DSTS to value 0x10
impl crate::Resettable for OTG_DSTSrs {
    const RESET_VALUE: u32 = 0x10;
}
