///Register `SWIER1` reader
pub type R = crate::R<SWIER1rs>;
///Register `SWIER1` writer
pub type W = crate::W<SWIER1rs>;
///Field `SWI0` reader - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI0_R = crate::BitReader;
///Field `SWI0` writer - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI1` reader - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI1_R = crate::BitReader;
///Field `SWI1` writer - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI2` reader - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI2_R = crate::BitReader;
///Field `SWI2` writer - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI3` reader - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI3_R = crate::BitReader;
///Field `SWI3` writer - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI4` reader - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI4_R = crate::BitReader;
///Field `SWI4` writer - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI5` reader - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI5_R = crate::BitReader;
///Field `SWI5` writer - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI6` reader - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI6_R = crate::BitReader;
///Field `SWI6` writer - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI7` reader - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI7_R = crate::BitReader;
///Field `SWI7` writer - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI8` reader - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI8_R = crate::BitReader;
///Field `SWI8` writer - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI9` reader - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI9_R = crate::BitReader;
///Field `SWI9` writer - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI10` reader - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI10_R = crate::BitReader;
///Field `SWI10` writer - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI11` reader - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI11_R = crate::BitReader;
///Field `SWI11` writer - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI12` reader - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI12_R = crate::BitReader;
///Field `SWI12` writer - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI13` reader - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI13_R = crate::BitReader;
///Field `SWI13` writer - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI14` reader - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI14_R = crate::BitReader;
///Field `SWI14` writer - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI15` reader - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI15_R = crate::BitReader;
///Field `SWI15` writer - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI16` reader - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI16_R = crate::BitReader;
///Field `SWI16` writer - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
pub type SWI16_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi8(&self) -> SWI8_R {
        SWI8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi9(&self) -> SWI9_R {
        SWI9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi10(&self) -> SWI10_R {
        SWI10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi11(&self) -> SWI11_R {
        SWI11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi12(&self) -> SWI12_R {
        SWI12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi13(&self) -> SWI13_R {
        SWI13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi14(&self) -> SWI14_R {
        SWI14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi15(&self) -> SWI15_R {
        SWI15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi16(&self) -> SWI16_R {
        SWI16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER1")
            .field("swi0", &self.swi0())
            .field("swi1", &self.swi1())
            .field("swi2", &self.swi2())
            .field("swi3", &self.swi3())
            .field("swi4", &self.swi4())
            .field("swi5", &self.swi5())
            .field("swi6", &self.swi6())
            .field("swi7", &self.swi7())
            .field("swi8", &self.swi8())
            .field("swi9", &self.swi9())
            .field("swi10", &self.swi10())
            .field("swi11", &self.swi11())
            .field("swi12", &self.swi12())
            .field("swi13", &self.swi13())
            .field("swi14", &self.swi14())
            .field("swi15", &self.swi15())
            .field("swi16", &self.swi16())
            .finish()
    }
}
impl W {
    ///Bit 0 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi0(&mut self) -> SWI0_W<'_, SWIER1rs> {
        SWI0_W::new(self, 0)
    }
    ///Bit 1 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi1(&mut self) -> SWI1_W<'_, SWIER1rs> {
        SWI1_W::new(self, 1)
    }
    ///Bit 2 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi2(&mut self) -> SWI2_W<'_, SWIER1rs> {
        SWI2_W::new(self, 2)
    }
    ///Bit 3 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi3(&mut self) -> SWI3_W<'_, SWIER1rs> {
        SWI3_W::new(self, 3)
    }
    ///Bit 4 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi4(&mut self) -> SWI4_W<'_, SWIER1rs> {
        SWI4_W::new(self, 4)
    }
    ///Bit 5 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi5(&mut self) -> SWI5_W<'_, SWIER1rs> {
        SWI5_W::new(self, 5)
    }
    ///Bit 6 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi6(&mut self) -> SWI6_W<'_, SWIER1rs> {
        SWI6_W::new(self, 6)
    }
    ///Bit 7 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi7(&mut self) -> SWI7_W<'_, SWIER1rs> {
        SWI7_W::new(self, 7)
    }
    ///Bit 8 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi8(&mut self) -> SWI8_W<'_, SWIER1rs> {
        SWI8_W::new(self, 8)
    }
    ///Bit 9 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi9(&mut self) -> SWI9_W<'_, SWIER1rs> {
        SWI9_W::new(self, 9)
    }
    ///Bit 10 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi10(&mut self) -> SWI10_W<'_, SWIER1rs> {
        SWI10_W::new(self, 10)
    }
    ///Bit 11 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi11(&mut self) -> SWI11_W<'_, SWIER1rs> {
        SWI11_W::new(self, 11)
    }
    ///Bit 12 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi12(&mut self) -> SWI12_W<'_, SWIER1rs> {
        SWI12_W::new(self, 12)
    }
    ///Bit 13 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi13(&mut self) -> SWI13_W<'_, SWIER1rs> {
        SWI13_W::new(self, 13)
    }
    ///Bit 14 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi14(&mut self) -> SWI14_W<'_, SWIER1rs> {
        SWI14_W::new(self, 14)
    }
    ///Bit 15 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi15(&mut self) -> SWI15_W<'_, SWIER1rs> {
        SWI15_W::new(self, 15)
    }
    ///Bit 16 - Software interrupt on event x When EXTI_SECFGR.SECx is disabled, SWIx can be accessed with non-secure and secure access. When EXTI_SECCFGR.SECx is enabled, SWIx can only be accessed with secure access. Non-secure write to this SWI x is discarded, non-secure read returns 0. When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independently from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.
    #[inline(always)]
    pub fn swi16(&mut self) -> SWI16_W<'_, SWIER1rs> {
        SWI16_W::new(self, 16)
    }
}
/**EXTI software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#EXTI:SWIER1)*/
pub struct SWIER1rs;
impl crate::RegisterSpec for SWIER1rs {
    type Ux = u32;
}
///`read()` method returns [`swier1::R`](R) reader structure
impl crate::Readable for SWIER1rs {}
///`write(|w| ..)` method takes [`swier1::W`](W) writer structure
impl crate::Writable for SWIER1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWIER1 to value 0
impl crate::Resettable for SWIER1rs {}
