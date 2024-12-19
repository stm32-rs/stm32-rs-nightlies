///Register `HWCFGR1` reader
pub type R = crate::R<HWCFGR1rs>;
///Field `BACKUP_REGS` reader - BACKUP_REGS
pub type BACKUP_REGS_R = crate::FieldReader;
///Field `TAMPER` reader - TAMPER
pub type TAMPER_R = crate::FieldReader;
///Field `ACTIVE_TAMPER` reader - ACTIVE_TAMPER
pub type ACTIVE_TAMPER_R = crate::FieldReader;
///Field `INT_TAMPER` reader - INT_TAMPER
pub type INT_TAMPER_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:7 - BACKUP_REGS
    #[inline(always)]
    pub fn backup_regs(&self) -> BACKUP_REGS_R {
        BACKUP_REGS_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - TAMPER
    #[inline(always)]
    pub fn tamper(&self) -> TAMPER_R {
        TAMPER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - ACTIVE_TAMPER
    #[inline(always)]
    pub fn active_tamper(&self) -> ACTIVE_TAMPER_R {
        ACTIVE_TAMPER_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:31 - INT_TAMPER
    #[inline(always)]
    pub fn int_tamper(&self) -> INT_TAMPER_R {
        INT_TAMPER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR1")
            .field("backup_regs", &self.backup_regs())
            .field("tamper", &self.tamper())
            .field("active_tamper", &self.active_tamper())
            .field("int_tamper", &self.int_tamper())
            .finish()
    }
}
/**TAMP hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TAMP:HWCFGR1)*/
pub struct HWCFGR1rs;
impl crate::RegisterSpec for HWCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr1::R`](R) reader structure
impl crate::Readable for HWCFGR1rs {}
///`reset()` method sets HWCFGR1 to value 0x009d_1320
impl crate::Resettable for HWCFGR1rs {
    const RESET_VALUE: u32 = 0x009d_1320;
}
