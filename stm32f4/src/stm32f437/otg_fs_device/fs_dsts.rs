///Register `FS_DSTS` reader
pub type R = crate::R<FS_DSTSrs>;
///Field `SUSPSTS` reader - Suspend status
pub type SUSPSTS_R = crate::BitReader;
///Field `ENUMSPD` reader - Enumerated speed
pub type ENUMSPD_R = crate::FieldReader;
///Field `EERR` reader - Erratic error
pub type EERR_R = crate::BitReader;
///Field `FNSOF` reader - Frame number of the received SOF
pub type FNSOF_R = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - Suspend status
    #[inline(always)]
    pub fn suspsts(&self) -> SUSPSTS_R {
        SUSPSTS_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Enumerated speed
    #[inline(always)]
    pub fn enumspd(&self) -> ENUMSPD_R {
        ENUMSPD_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - Erratic error
    #[inline(always)]
    pub fn eerr(&self) -> EERR_R {
        EERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 8:21 - Frame number of the received SOF
    #[inline(always)]
    pub fn fnsof(&self) -> FNSOF_R {
        FNSOF_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_DSTS")
            .field("suspsts", &self.suspsts())
            .field("enumspd", &self.enumspd())
            .field("eerr", &self.eerr())
            .field("fnsof", &self.fnsof())
            .finish()
    }
}
/**OTG_FS device status register (OTG_FS_DSTS)

You can [`read`](crate::Reg::read) this register and get [`fs_dsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_FS_DEVICE:FS_DSTS)*/
pub struct FS_DSTSrs;
impl crate::RegisterSpec for FS_DSTSrs {
    type Ux = u32;
}
///`read()` method returns [`fs_dsts::R`](R) reader structure
impl crate::Readable for FS_DSTSrs {}
///`reset()` method sets FS_DSTS to value 0x10
impl crate::Resettable for FS_DSTSrs {
    const RESET_VALUE: u32 = 0x10;
}
