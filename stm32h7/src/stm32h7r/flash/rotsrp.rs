///Register `ROTSRP` reader
pub type R = crate::R<ROTSRPrs>;
///Register `ROTSRP` writer
pub type W = crate::W<ROTSRPrs>;
///Field `OEM_PROVD` reader - OEM provisioned device Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 1.
pub type OEM_PROVD_R = crate::FieldReader;
///Field `OEM_PROVD` writer - OEM provisioned device Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 1.
pub type OEM_PROVD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DBG_AUTH` reader - Debug authentication method programming Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 0.
pub type DBG_AUTH_R = crate::FieldReader;
///Field `DBG_AUTH` writer - Debug authentication method programming Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 0.
pub type DBG_AUTH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IROT_SELECT` reader - iRoT selection This option is ignored for STM32H7R devices. Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 1 and if NVSTATE is not 0xB4 (OPEN).
pub type IROT_SELECT_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - OEM provisioned device Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 1.
    #[inline(always)]
    pub fn oem_provd(&self) -> OEM_PROVD_R {
        OEM_PROVD_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Debug authentication method programming Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 0.
    #[inline(always)]
    pub fn dbg_auth(&self) -> DBG_AUTH_R {
        DBG_AUTH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 24:31 - iRoT selection This option is ignored for STM32H7R devices. Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 1 and if NVSTATE is not 0xB4 (OPEN).
    #[inline(always)]
    pub fn irot_select(&self) -> IROT_SELECT_R {
        IROT_SELECT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROTSRP")
            .field("oem_provd", &self.oem_provd())
            .field("dbg_auth", &self.dbg_auth())
            .field("irot_select", &self.irot_select())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - OEM provisioned device Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 1.
    #[inline(always)]
    pub fn oem_provd(&mut self) -> OEM_PROVD_W<'_, ROTSRPrs> {
        OEM_PROVD_W::new(self, 0)
    }
    ///Bits 8:15 - Debug authentication method programming Write to change corresponding bits in FLASH_ROTSR register. Write are ignored if HDPL is greater than 0.
    #[inline(always)]
    pub fn dbg_auth(&mut self) -> DBG_AUTH_W<'_, ROTSRPrs> {
        DBG_AUTH_W::new(self, 8)
    }
}
/**FLASH RoT status register programming

You can [`read`](crate::Reg::read) this register and get [`rotsrp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rotsrp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:ROTSRP)*/
pub struct ROTSRPrs;
impl crate::RegisterSpec for ROTSRPrs {
    type Ux = u32;
}
///`read()` method returns [`rotsrp::R`](R) reader structure
impl crate::Readable for ROTSRPrs {}
///`write(|w| ..)` method takes [`rotsrp::W`](W) writer structure
impl crate::Writable for ROTSRPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ROTSRP to value 0
impl crate::Resettable for ROTSRPrs {}
