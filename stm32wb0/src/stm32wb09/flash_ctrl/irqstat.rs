///Register `IRQSTAT` reader
pub type R = crate::R<IRQSTATrs>;
///Register `IRQSTAT` writer
pub type W = crate::W<IRQSTATrs>;
///Field `CMDDONE_MIS` reader - (1: clear, 0: inactive) CMDDONE_MIS flag
pub type CMDDONE_MIS_R = crate::BitReader;
///Field `CMDDONE_MIS` writer - (1: clear, 0: inactive) CMDDONE_MIS flag
pub type CMDDONE_MIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDSTART_MIS` reader - (1: clear, 0: inactive) CMDSTART_MIS flag
pub type CMDSTART_MIS_R = crate::BitReader;
///Field `CMDSTART_MIS` writer - (1: clear, 0: inactive) CMDSTART_MIS flag
pub type CMDSTART_MIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDBUSYERR_MIS` reader - (1: clear, 0: inactive) CMDBUSYERR_MIS flag
pub type CMDBUSYERR_MIS_R = crate::BitReader;
///Field `CMDBUSYERR_MIS` writer - (1: clear, 0: inactive) CMDBUSYERR_MIS flag
pub type CMDBUSYERR_MIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ILLCMD_MIS` reader - (1: clear, 0: inactive) ILLCMD_MIS flag
pub type ILLCMD_MIS_R = crate::BitReader;
///Field `ILLCMD_MIS` writer - (1: clear, 0: inactive) ILLCMD_MIS flag
pub type ILLCMD_MIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `READOK_MIS` reader - (1: clear, 0: inactive) READOK_MIS flag
pub type READOK_MIS_R = crate::BitReader;
///Field `READOK_MIS` writer - (1: clear, 0: inactive) READOK_MIS flag
pub type READOK_MIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FNREADY_MIS` reader - (1: clear, 0: inactive) FNREADY_MIS flag
pub type FNREADY_MIS_R = crate::BitReader;
///Field `FNREADY_MIS` writer - (1: clear, 0: inactive) FNREADY_MIS flag
pub type FNREADY_MIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - (1: clear, 0: inactive) CMDDONE_MIS flag
    #[inline(always)]
    pub fn cmddone_mis(&self) -> CMDDONE_MIS_R {
        CMDDONE_MIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - (1: clear, 0: inactive) CMDSTART_MIS flag
    #[inline(always)]
    pub fn cmdstart_mis(&self) -> CMDSTART_MIS_R {
        CMDSTART_MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - (1: clear, 0: inactive) CMDBUSYERR_MIS flag
    #[inline(always)]
    pub fn cmdbusyerr_mis(&self) -> CMDBUSYERR_MIS_R {
        CMDBUSYERR_MIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - (1: clear, 0: inactive) ILLCMD_MIS flag
    #[inline(always)]
    pub fn illcmd_mis(&self) -> ILLCMD_MIS_R {
        ILLCMD_MIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - (1: clear, 0: inactive) READOK_MIS flag
    #[inline(always)]
    pub fn readok_mis(&self) -> READOK_MIS_R {
        READOK_MIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - (1: clear, 0: inactive) FNREADY_MIS flag
    #[inline(always)]
    pub fn fnready_mis(&self) -> FNREADY_MIS_R {
        FNREADY_MIS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSTAT")
            .field("cmddone_mis", &self.cmddone_mis())
            .field("cmdstart_mis", &self.cmdstart_mis())
            .field("cmdbusyerr_mis", &self.cmdbusyerr_mis())
            .field("illcmd_mis", &self.illcmd_mis())
            .field("readok_mis", &self.readok_mis())
            .field("fnready_mis", &self.fnready_mis())
            .finish()
    }
}
impl W {
    ///Bit 0 - (1: clear, 0: inactive) CMDDONE_MIS flag
    #[inline(always)]
    pub fn cmddone_mis(&mut self) -> CMDDONE_MIS_W<'_, IRQSTATrs> {
        CMDDONE_MIS_W::new(self, 0)
    }
    ///Bit 1 - (1: clear, 0: inactive) CMDSTART_MIS flag
    #[inline(always)]
    pub fn cmdstart_mis(&mut self) -> CMDSTART_MIS_W<'_, IRQSTATrs> {
        CMDSTART_MIS_W::new(self, 1)
    }
    ///Bit 2 - (1: clear, 0: inactive) CMDBUSYERR_MIS flag
    #[inline(always)]
    pub fn cmdbusyerr_mis(&mut self) -> CMDBUSYERR_MIS_W<'_, IRQSTATrs> {
        CMDBUSYERR_MIS_W::new(self, 2)
    }
    ///Bit 3 - (1: clear, 0: inactive) ILLCMD_MIS flag
    #[inline(always)]
    pub fn illcmd_mis(&mut self) -> ILLCMD_MIS_W<'_, IRQSTATrs> {
        ILLCMD_MIS_W::new(self, 3)
    }
    ///Bit 4 - (1: clear, 0: inactive) READOK_MIS flag
    #[inline(always)]
    pub fn readok_mis(&mut self) -> READOK_MIS_W<'_, IRQSTATrs> {
        READOK_MIS_W::new(self, 4)
    }
    ///Bit 5 - (1: clear, 0: inactive) FNREADY_MIS flag
    #[inline(always)]
    pub fn fnready_mis(&mut self) -> FNREADY_MIS_W<'_, IRQSTATrs> {
        FNREADY_MIS_W::new(self, 5)
    }
}
/**IRQSTAT register

You can [`read`](crate::Reg::read) this register and get [`irqstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#FLASH_CTRL:IRQSTAT)*/
pub struct IRQSTATrs;
impl crate::RegisterSpec for IRQSTATrs {
    type Ux = u32;
}
///`read()` method returns [`irqstat::R`](R) reader structure
impl crate::Readable for IRQSTATrs {}
///`write(|w| ..)` method takes [`irqstat::W`](W) writer structure
impl crate::Writable for IRQSTATrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IRQSTAT to value 0
impl crate::Resettable for IRQSTATrs {}
