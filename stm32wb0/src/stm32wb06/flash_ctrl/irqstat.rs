///Register `IRQSTAT` reader
pub type R = crate::R<IRQSTATrs>;
///Register `IRQSTAT` writer
pub type W = crate::W<IRQSTATrs>;
///Field `CMDDONE_MIS` reader - Command done masked interrupt status.
pub type CMDDONE_MIS_R = crate::BitReader;
///Field `CMDDONE_MIS` writer - Command done masked interrupt status.
pub type CMDDONE_MIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDSTART_MIS` reader - Command started masked interrupt status.
pub type CMDSTART_MIS_R = crate::BitReader;
///Field `CMDSTART_MIS` writer - Command started masked interrupt status.
pub type CMDSTART_MIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDERR_MIS` reader - Command error masked interrupt status.
pub type CMDERR_MIS_R = crate::BitReader;
///Field `CMDERR_MIS` writer - Command error masked interrupt status.
pub type CMDERR_MIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ILLCMD_MIS` reader - Illegal command masked interrupt status
pub type ILLCMD_MIS_R = crate::BitReader;
///Field `ILLCMD_MIS` writer - Illegal command masked interrupt status
pub type ILLCMD_MIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `READOK_MIS` reader - Mass read OK masked interrupt status.
pub type READOK_MIS_R = crate::BitReader;
///Field `READOK_MIS` writer - Mass read OK masked interrupt status.
pub type READOK_MIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Command done masked interrupt status.
    #[inline(always)]
    pub fn cmddone_mis(&self) -> CMDDONE_MIS_R {
        CMDDONE_MIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Command started masked interrupt status.
    #[inline(always)]
    pub fn cmdstart_mis(&self) -> CMDSTART_MIS_R {
        CMDSTART_MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Command error masked interrupt status.
    #[inline(always)]
    pub fn cmderr_mis(&self) -> CMDERR_MIS_R {
        CMDERR_MIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Illegal command masked interrupt status
    #[inline(always)]
    pub fn illcmd_mis(&self) -> ILLCMD_MIS_R {
        ILLCMD_MIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Mass read OK masked interrupt status.
    #[inline(always)]
    pub fn readok_mis(&self) -> READOK_MIS_R {
        READOK_MIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSTAT")
            .field("cmddone_mis", &self.cmddone_mis())
            .field("cmdstart_mis", &self.cmdstart_mis())
            .field("cmderr_mis", &self.cmderr_mis())
            .field("illcmd_mis", &self.illcmd_mis())
            .field("readok_mis", &self.readok_mis())
            .finish()
    }
}
impl W {
    ///Bit 0 - Command done masked interrupt status.
    #[inline(always)]
    pub fn cmddone_mis(&mut self) -> CMDDONE_MIS_W<'_, IRQSTATrs> {
        CMDDONE_MIS_W::new(self, 0)
    }
    ///Bit 1 - Command started masked interrupt status.
    #[inline(always)]
    pub fn cmdstart_mis(&mut self) -> CMDSTART_MIS_W<'_, IRQSTATrs> {
        CMDSTART_MIS_W::new(self, 1)
    }
    ///Bit 2 - Command error masked interrupt status.
    #[inline(always)]
    pub fn cmderr_mis(&mut self) -> CMDERR_MIS_W<'_, IRQSTATrs> {
        CMDERR_MIS_W::new(self, 2)
    }
    ///Bit 3 - Illegal command masked interrupt status
    #[inline(always)]
    pub fn illcmd_mis(&mut self) -> ILLCMD_MIS_W<'_, IRQSTATrs> {
        ILLCMD_MIS_W::new(self, 3)
    }
    ///Bit 4 - Mass read OK masked interrupt status.
    #[inline(always)]
    pub fn readok_mis(&mut self) -> READOK_MIS_W<'_, IRQSTATrs> {
        READOK_MIS_W::new(self, 4)
    }
}
/**IRQSTAT register

You can [`read`](crate::Reg::read) this register and get [`irqstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#FLASH_CTRL:IRQSTAT)*/
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
