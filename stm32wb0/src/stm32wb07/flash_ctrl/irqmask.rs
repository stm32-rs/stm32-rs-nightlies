///Register `IRQMASK` reader
pub type R = crate::R<IRQMASKrs>;
///Register `IRQMASK` writer
pub type W = crate::W<IRQMASKrs>;
///Field `CMDDONEM` reader - Command done mask
pub type CMDDONEM_R = crate::BitReader;
///Field `CMDDONEM` writer - Command done mask
pub type CMDDONEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDSTARTM` reader - Command started mask.
pub type CMDSTARTM_R = crate::BitReader;
///Field `CMDSTARTM` writer - Command started mask.
pub type CMDSTARTM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDERRM` reader - Command error mask.
pub type CMDERRM_R = crate::BitReader;
///Field `CMDERRM` writer - Command error mask.
pub type CMDERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ILLCMDM` reader - Illegal command mask.
pub type ILLCMDM_R = crate::BitReader;
///Field `ILLCMDM` writer - Illegal command mask.
pub type ILLCMDM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `READOKM` reader - Mass read OK mask.
pub type READOKM_R = crate::BitReader;
///Field `READOKM` writer - Mass read OK mask.
pub type READOKM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Command done mask
    #[inline(always)]
    pub fn cmddonem(&self) -> CMDDONEM_R {
        CMDDONEM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Command started mask.
    #[inline(always)]
    pub fn cmdstartm(&self) -> CMDSTARTM_R {
        CMDSTARTM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Command error mask.
    #[inline(always)]
    pub fn cmderrm(&self) -> CMDERRM_R {
        CMDERRM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Illegal command mask.
    #[inline(always)]
    pub fn illcmdm(&self) -> ILLCMDM_R {
        ILLCMDM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Mass read OK mask.
    #[inline(always)]
    pub fn readokm(&self) -> READOKM_R {
        READOKM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQMASK")
            .field("cmddonem", &self.cmddonem())
            .field("cmdstartm", &self.cmdstartm())
            .field("cmderrm", &self.cmderrm())
            .field("illcmdm", &self.illcmdm())
            .field("readokm", &self.readokm())
            .finish()
    }
}
impl W {
    ///Bit 0 - Command done mask
    #[inline(always)]
    pub fn cmddonem(&mut self) -> CMDDONEM_W<'_, IRQMASKrs> {
        CMDDONEM_W::new(self, 0)
    }
    ///Bit 1 - Command started mask.
    #[inline(always)]
    pub fn cmdstartm(&mut self) -> CMDSTARTM_W<'_, IRQMASKrs> {
        CMDSTARTM_W::new(self, 1)
    }
    ///Bit 2 - Command error mask.
    #[inline(always)]
    pub fn cmderrm(&mut self) -> CMDERRM_W<'_, IRQMASKrs> {
        CMDERRM_W::new(self, 2)
    }
    ///Bit 3 - Illegal command mask.
    #[inline(always)]
    pub fn illcmdm(&mut self) -> ILLCMDM_W<'_, IRQMASKrs> {
        ILLCMDM_W::new(self, 3)
    }
    ///Bit 4 - Mass read OK mask.
    #[inline(always)]
    pub fn readokm(&mut self) -> READOKM_W<'_, IRQMASKrs> {
        READOKM_W::new(self, 4)
    }
}
/**IRQMASK register

You can [`read`](crate::Reg::read) this register and get [`irqmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#FLASH_CTRL:IRQMASK)*/
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
