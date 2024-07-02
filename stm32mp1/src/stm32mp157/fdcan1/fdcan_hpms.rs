///Register `FDCAN_HPMS` reader
pub type R = crate::R<FDCAN_HPMSrs>;
///Field `BIDX` reader - BIDX
pub type BIDX_R = crate::FieldReader;
///Field `MSI` reader - MSI
pub type MSI_R = crate::FieldReader;
///Field `FIDX` reader - FIDX
pub type FIDX_R = crate::FieldReader;
///Field `FLST` reader - FLST
pub type FLST_R = crate::BitReader;
impl R {
    ///Bits 0:5 - BIDX
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7 - MSI
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:14 - FIDX
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 15 - FLST
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_HPMS")
            .field("bidx", &self.bidx())
            .field("msi", &self.msi())
            .field("fidx", &self.fidx())
            .field("flst", &self.flst())
            .finish()
    }
}
/**This register is updated every time a message ID filter element configured to generate a priority event match. This can be used to monitor the status of incoming high priority messages and to enable fast access to these messages.

You can [`read`](crate::Reg::read) this register and get [`fdcan_hpms::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:FDCAN_HPMS)*/
pub struct FDCAN_HPMSrs;
impl crate::RegisterSpec for FDCAN_HPMSrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_hpms::R`](R) reader structure
impl crate::Readable for FDCAN_HPMSrs {}
///`reset()` method sets FDCAN_HPMS to value 0
impl crate::Resettable for FDCAN_HPMSrs {
    const RESET_VALUE: u32 = 0;
}
