///Register `HWCFGR10` reader
pub type R = crate::R<HWCFGR10rs>;
///Field `AHB_IOP` reader - AHB_IOP
pub type AHB_IOP_R = crate::FieldReader;
///Field `AF_SIZE` reader - AF_SIZE
pub type AF_SIZE_R = crate::FieldReader;
///Field `SPEED_CFG` reader - SPEED_CFG
pub type SPEED_CFG_R = crate::FieldReader;
///Field `LOCK_CFG` reader - LOCK_CFG
pub type LOCK_CFG_R = crate::FieldReader;
///Field `SEC_CFG` reader - SEC_CFG
pub type SEC_CFG_R = crate::FieldReader;
///Field `OR_CFG` reader - OR_CFG
pub type OR_CFG_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - AHB_IOP
    #[inline(always)]
    pub fn ahb_iop(&self) -> AHB_IOP_R {
        AHB_IOP_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - AF_SIZE
    #[inline(always)]
    pub fn af_size(&self) -> AF_SIZE_R {
        AF_SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - SPEED_CFG
    #[inline(always)]
    pub fn speed_cfg(&self) -> SPEED_CFG_R {
        SPEED_CFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - LOCK_CFG
    #[inline(always)]
    pub fn lock_cfg(&self) -> LOCK_CFG_R {
        LOCK_CFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - SEC_CFG
    #[inline(always)]
    pub fn sec_cfg(&self) -> SEC_CFG_R {
        SEC_CFG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - OR_CFG
    #[inline(always)]
    pub fn or_cfg(&self) -> OR_CFG_R {
        OR_CFG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR10")
            .field("ahb_iop", &self.ahb_iop())
            .field("af_size", &self.af_size())
            .field("speed_cfg", &self.speed_cfg())
            .field("lock_cfg", &self.lock_cfg())
            .field("sec_cfg", &self.sec_cfg())
            .field("or_cfg", &self.or_cfg())
            .finish()
    }
}
/**For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`hwcfgr10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOJ:HWCFGR10)*/
pub struct HWCFGR10rs;
impl crate::RegisterSpec for HWCFGR10rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr10::R`](R) reader structure
impl crate::Readable for HWCFGR10rs {}
///`reset()` method sets HWCFGR10 to value 0x0001_1240
impl crate::Resettable for HWCFGR10rs {
    const RESET_VALUE: u32 = 0x0001_1240;
}
