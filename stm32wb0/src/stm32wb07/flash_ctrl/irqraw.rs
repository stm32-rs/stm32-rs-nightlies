///Register `IRQRAW` reader
pub type R = crate::R<IRQRAWrs>;
///Register `IRQRAW` writer
pub type W = crate::W<IRQRAWrs>;
///Field `CMDDONE_RIS` reader - Command done raw/unmasked interrupt status. This it is set once the requested command execution is completed. Cleared by writing 1.
pub type CMDDONE_RIS_R = crate::BitReader;
///Field `CMDDONE_RIS` writer - Command done raw/unmasked interrupt status. This it is set once the requested command execution is completed. Cleared by writing 1.
pub type CMDDONE_RIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDSTART_RIS` reader - Command started raw/unmasked interrupt status. This bit is set once the requested command execution has started.
pub type CMDSTART_RIS_R = crate::BitReader;
///Field `CMDSTART_RIS` writer - Command started raw/unmasked interrupt status. This bit is set once the requested command execution has started.
pub type CMDSTART_RIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDERR_RIS` reader - Command error raw/unmasked interrupt status
pub type CMDERR_RIS_R = crate::BitReader;
///Field `CMDERR_RIS` writer - Command error raw/unmasked interrupt status
pub type CMDERR_RIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ILLCMD_RIS` reader - Illegal command raw/unmasked interrupt status.
pub type ILLCMD_RIS_R = crate::BitReader;
///Field `ILLCMD_RIS` writer - Illegal command raw/unmasked interrupt status.
pub type ILLCMD_RIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `READOK_RIS` reader - Mass read OK raw/unmasked interrupt status
pub type READOK_RIS_R = crate::BitReader;
///Field `READOK_RIS` writer - Mass read OK raw/unmasked interrupt status
pub type READOK_RIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Command done raw/unmasked interrupt status. This it is set once the requested command execution is completed. Cleared by writing 1.
    #[inline(always)]
    pub fn cmddone_ris(&self) -> CMDDONE_RIS_R {
        CMDDONE_RIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Command started raw/unmasked interrupt status. This bit is set once the requested command execution has started.
    #[inline(always)]
    pub fn cmdstart_ris(&self) -> CMDSTART_RIS_R {
        CMDSTART_RIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Command error raw/unmasked interrupt status
    #[inline(always)]
    pub fn cmderr_ris(&self) -> CMDERR_RIS_R {
        CMDERR_RIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Illegal command raw/unmasked interrupt status.
    #[inline(always)]
    pub fn illcmd_ris(&self) -> ILLCMD_RIS_R {
        ILLCMD_RIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Mass read OK raw/unmasked interrupt status
    #[inline(always)]
    pub fn readok_ris(&self) -> READOK_RIS_R {
        READOK_RIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQRAW")
            .field("cmddone_ris", &self.cmddone_ris())
            .field("cmdstart_ris", &self.cmdstart_ris())
            .field("cmderr_ris", &self.cmderr_ris())
            .field("illcmd_ris", &self.illcmd_ris())
            .field("readok_ris", &self.readok_ris())
            .finish()
    }
}
impl W {
    ///Bit 0 - Command done raw/unmasked interrupt status. This it is set once the requested command execution is completed. Cleared by writing 1.
    #[inline(always)]
    pub fn cmddone_ris(&mut self) -> CMDDONE_RIS_W<'_, IRQRAWrs> {
        CMDDONE_RIS_W::new(self, 0)
    }
    ///Bit 1 - Command started raw/unmasked interrupt status. This bit is set once the requested command execution has started.
    #[inline(always)]
    pub fn cmdstart_ris(&mut self) -> CMDSTART_RIS_W<'_, IRQRAWrs> {
        CMDSTART_RIS_W::new(self, 1)
    }
    ///Bit 2 - Command error raw/unmasked interrupt status
    #[inline(always)]
    pub fn cmderr_ris(&mut self) -> CMDERR_RIS_W<'_, IRQRAWrs> {
        CMDERR_RIS_W::new(self, 2)
    }
    ///Bit 3 - Illegal command raw/unmasked interrupt status.
    #[inline(always)]
    pub fn illcmd_ris(&mut self) -> ILLCMD_RIS_W<'_, IRQRAWrs> {
        ILLCMD_RIS_W::new(self, 3)
    }
    ///Bit 4 - Mass read OK raw/unmasked interrupt status
    #[inline(always)]
    pub fn readok_ris(&mut self) -> READOK_RIS_W<'_, IRQRAWrs> {
        READOK_RIS_W::new(self, 4)
    }
}
/**IRQRAW register

You can [`read`](crate::Reg::read) this register and get [`irqraw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqraw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#FLASH_CTRL:IRQRAW)*/
pub struct IRQRAWrs;
impl crate::RegisterSpec for IRQRAWrs {
    type Ux = u32;
}
///`read()` method returns [`irqraw::R`](R) reader structure
impl crate::Readable for IRQRAWrs {}
///`write(|w| ..)` method takes [`irqraw::W`](W) writer structure
impl crate::Writable for IRQRAWrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IRQRAW to value 0x01
impl crate::Resettable for IRQRAWrs {
    const RESET_VALUE: u32 = 0x01;
}
