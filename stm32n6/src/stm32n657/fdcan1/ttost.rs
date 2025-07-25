///Register `TTOST` reader
pub type R = crate::R<TTOSTrs>;
///Field `EL` reader - Error level
pub type EL_R = crate::FieldReader;
///Field `MS` reader - Master state
pub type MS_R = crate::FieldReader;
///Field `SYS` reader - Synchronization state
pub type SYS_R = crate::FieldReader;
///Field `QGTP` reader - Quality of global time phase
pub type QGTP_R = crate::BitReader;
///Field `QCS` reader - Quality of clock speed
pub type QCS_R = crate::BitReader;
///Field `RTO` reader - Reference trigger offset
pub type RTO_R = crate::FieldReader;
///Field `WGTD` reader - Wait for global time discontinuity
pub type WGTD_R = crate::BitReader;
///Field `GFI` reader - Gap finished indicator
pub type GFI_R = crate::BitReader;
///Field `TMP` reader - Time master priority
pub type TMP_R = crate::FieldReader;
///Field `GSI` reader - Gap started indicator
pub type GSI_R = crate::BitReader;
///Field `WFE` reader - Wait for event
pub type WFE_R = crate::BitReader;
///Field `AWE` reader - Application watchdog event
pub type AWE_R = crate::BitReader;
///Field `WECS` reader - Wait for external clock synchronization.
pub type WECS_R = crate::BitReader;
///Field `SPL` reader - Schedule phase lock
pub type SPL_R = crate::BitReader;
impl R {
    ///Bits 0:1 - Error level
    #[inline(always)]
    pub fn el(&self) -> EL_R {
        EL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Master state
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Synchronization state
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Quality of global time phase
    #[inline(always)]
    pub fn qgtp(&self) -> QGTP_R {
        QGTP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Quality of clock speed
    #[inline(always)]
    pub fn qcs(&self) -> QCS_R {
        QCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Reference trigger offset
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 22 - Wait for global time discontinuity
    #[inline(always)]
    pub fn wgtd(&self) -> WGTD_R {
        WGTD_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Gap finished indicator
    #[inline(always)]
    pub fn gfi(&self) -> GFI_R {
        GFI_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:26 - Time master priority
    #[inline(always)]
    pub fn tmp(&self) -> TMP_R {
        TMP_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 27 - Gap started indicator
    #[inline(always)]
    pub fn gsi(&self) -> GSI_R {
        GSI_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Wait for event
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Application watchdog event
    #[inline(always)]
    pub fn awe(&self) -> AWE_R {
        AWE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Wait for external clock synchronization.
    #[inline(always)]
    pub fn wecs(&self) -> WECS_R {
        WECS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Schedule phase lock
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTOST")
            .field("el", &self.el())
            .field("ms", &self.ms())
            .field("sys", &self.sys())
            .field("qgtp", &self.qgtp())
            .field("qcs", &self.qcs())
            .field("rto", &self.rto())
            .field("wgtd", &self.wgtd())
            .field("gfi", &self.gfi())
            .field("tmp", &self.tmp())
            .field("gsi", &self.gsi())
            .field("wfe", &self.wfe())
            .field("awe", &self.awe())
            .field("wecs", &self.wecs())
            .field("spl", &self.spl())
            .finish()
    }
}
/**FDCAN TT operation status register

You can [`read`](crate::Reg::read) this register and get [`ttost::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FDCAN1:TTOST)*/
pub struct TTOSTrs;
impl crate::RegisterSpec for TTOSTrs {
    type Ux = u32;
}
///`read()` method returns [`ttost::R`](R) reader structure
impl crate::Readable for TTOSTrs {}
///`reset()` method sets TTOST to value 0x80
impl crate::Resettable for TTOSTrs {
    const RESET_VALUE: u32 = 0x80;
}
