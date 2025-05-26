///Register `ROTSR` reader
pub type R = crate::R<ROTSRrs>;
///Field `OEM_PROVD` reader - OEM provisioned device Any other value: device is not provisioned by the OEM.
pub type OEM_PROVD_R = crate::FieldReader;
///Field `DBG_AUTH` reader - Debug authentication method Any other value: no authentication method selected (NotSet).
pub type DBG_AUTH_R = crate::FieldReader;
///Field `IROT_SELECT` reader - iRoT selection This option is ignored for STM32H7R devices (OEM iRoT is always selected). Any other value: OEM iRoT is selected at boot.
pub type IROT_SELECT_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - OEM provisioned device Any other value: device is not provisioned by the OEM.
    #[inline(always)]
    pub fn oem_provd(&self) -> OEM_PROVD_R {
        OEM_PROVD_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Debug authentication method Any other value: no authentication method selected (NotSet).
    #[inline(always)]
    pub fn dbg_auth(&self) -> DBG_AUTH_R {
        DBG_AUTH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 24:31 - iRoT selection This option is ignored for STM32H7R devices (OEM iRoT is always selected). Any other value: OEM iRoT is selected at boot.
    #[inline(always)]
    pub fn irot_select(&self) -> IROT_SELECT_R {
        IROT_SELECT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROTSR")
            .field("oem_provd", &self.oem_provd())
            .field("dbg_auth", &self.dbg_auth())
            .field("irot_select", &self.irot_select())
            .finish()
    }
}
/**FLASH RoT status register

You can [`read`](crate::Reg::read) this register and get [`rotsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:ROTSR)*/
pub struct ROTSRrs;
impl crate::RegisterSpec for ROTSRrs {
    type Ux = u32;
}
///`read()` method returns [`rotsr::R`](R) reader structure
impl crate::Readable for ROTSRrs {}
///`reset()` method sets ROTSR to value 0
impl crate::Resettable for ROTSRrs {}
