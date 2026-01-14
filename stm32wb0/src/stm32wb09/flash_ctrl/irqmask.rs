///Register `IRQMASK` reader
pub type R = crate::R<IRQMASKrs>;
///Register `IRQMASK` writer
pub type W = crate::W<IRQMASKrs>;
///Field `CMDDONEM` reader - (1: mask, 0: inactive) CMDDONE_MIS mask
pub type CMDDONEM_R = crate::BitReader;
///Field `CMDDONEM` writer - (1: mask, 0: inactive) CMDDONE_MIS mask
pub type CMDDONEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDSTARTM` reader - (1: mask, 0: inactive) CMDSTART_MIS mask
pub type CMDSTARTM_R = crate::BitReader;
///Field `CMDSTARTM` writer - (1: mask, 0: inactive) CMDSTART_MIS mask
pub type CMDSTARTM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDBUSYERRM` reader - (1: mask, 0: inactive) CMDBUSYERR_MIS mask
pub type CMDBUSYERRM_R = crate::BitReader;
///Field `CMDBUSYERRM` writer - (1: mask, 0: inactive) CMDBUSYERR_MIS mask
pub type CMDBUSYERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ILLCMDM` reader - (1: mask, 0: inactive) ILLCMD_MIS mask
pub type ILLCMDM_R = crate::BitReader;
///Field `ILLCMDM` writer - (1: mask, 0: inactive) ILLCMD_MIS mask
pub type ILLCMDM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `READOKM` reader - (1: mask, 0: inactive) READOK_MIS mask
pub type READOKM_R = crate::BitReader;
///Field `READOKM` writer - (1: mask, 0: inactive) READOK_MIS mask
pub type READOKM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FNREADYM` reader - (1: mask, 0: inactive) FNREADY_MIS mask
pub type FNREADYM_R = crate::BitReader;
///Field `FNREADYM` writer - (1: mask, 0: inactive) FNREADY_MIS mask
pub type FNREADYM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - (1: mask, 0: inactive) CMDDONE_MIS mask
    #[inline(always)]
    pub fn cmddonem(&self) -> CMDDONEM_R {
        CMDDONEM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - (1: mask, 0: inactive) CMDSTART_MIS mask
    #[inline(always)]
    pub fn cmdstartm(&self) -> CMDSTARTM_R {
        CMDSTARTM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - (1: mask, 0: inactive) CMDBUSYERR_MIS mask
    #[inline(always)]
    pub fn cmdbusyerrm(&self) -> CMDBUSYERRM_R {
        CMDBUSYERRM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - (1: mask, 0: inactive) ILLCMD_MIS mask
    #[inline(always)]
    pub fn illcmdm(&self) -> ILLCMDM_R {
        ILLCMDM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - (1: mask, 0: inactive) READOK_MIS mask
    #[inline(always)]
    pub fn readokm(&self) -> READOKM_R {
        READOKM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - (1: mask, 0: inactive) FNREADY_MIS mask
    #[inline(always)]
    pub fn fnreadym(&self) -> FNREADYM_R {
        FNREADYM_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQMASK")
            .field("cmddonem", &self.cmddonem())
            .field("cmdstartm", &self.cmdstartm())
            .field("cmdbusyerrm", &self.cmdbusyerrm())
            .field("illcmdm", &self.illcmdm())
            .field("readokm", &self.readokm())
            .field("fnreadym", &self.fnreadym())
            .finish()
    }
}
impl W {
    ///Bit 0 - (1: mask, 0: inactive) CMDDONE_MIS mask
    #[inline(always)]
    pub fn cmddonem(&mut self) -> CMDDONEM_W<'_, IRQMASKrs> {
        CMDDONEM_W::new(self, 0)
    }
    ///Bit 1 - (1: mask, 0: inactive) CMDSTART_MIS mask
    #[inline(always)]
    pub fn cmdstartm(&mut self) -> CMDSTARTM_W<'_, IRQMASKrs> {
        CMDSTARTM_W::new(self, 1)
    }
    ///Bit 2 - (1: mask, 0: inactive) CMDBUSYERR_MIS mask
    #[inline(always)]
    pub fn cmdbusyerrm(&mut self) -> CMDBUSYERRM_W<'_, IRQMASKrs> {
        CMDBUSYERRM_W::new(self, 2)
    }
    ///Bit 3 - (1: mask, 0: inactive) ILLCMD_MIS mask
    #[inline(always)]
    pub fn illcmdm(&mut self) -> ILLCMDM_W<'_, IRQMASKrs> {
        ILLCMDM_W::new(self, 3)
    }
    ///Bit 4 - (1: mask, 0: inactive) READOK_MIS mask
    #[inline(always)]
    pub fn readokm(&mut self) -> READOKM_W<'_, IRQMASKrs> {
        READOKM_W::new(self, 4)
    }
    ///Bit 5 - (1: mask, 0: inactive) FNREADY_MIS mask
    #[inline(always)]
    pub fn fnreadym(&mut self) -> FNREADYM_W<'_, IRQMASKrs> {
        FNREADYM_W::new(self, 5)
    }
}
/**IRQMASK register

You can [`read`](crate::Reg::read) this register and get [`irqmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#FLASH_CTRL:IRQMASK)*/
pub struct IRQMASKrs;
impl crate::RegisterSpec for IRQMASKrs {
    type Ux = u32;
}
///`read()` method returns [`irqmask::R`](R) reader structure
impl crate::Readable for IRQMASKrs {}
///`write(|w| ..)` method takes [`irqmask::W`](W) writer structure
impl crate::Writable for IRQMASKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IRQMASK to value 0x3f
impl crate::Resettable for IRQMASKrs {
    const RESET_VALUE: u32 = 0x3f;
}
