///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `RELOAD` reader - Counter reload value
pub type RELOAD_R = crate::FieldReader<u16>;
///Field `RELOAD` writer - Counter reload value
pub type RELOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `FELIM` reader - Frequency error limit
pub type FELIM_R = crate::FieldReader;
///Field `FELIM` writer - Frequency error limit
pub type FELIM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SYNCDIV` reader - SYNC divider
pub type SYNCDIV_R = crate::FieldReader;
///Field `SYNCDIV` writer - SYNC divider
pub type SYNCDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SYNCSRC` reader - SYNC signal source selection
pub type SYNCSRC_R = crate::FieldReader;
///Field `SYNCSRC` writer - SYNC signal source selection
pub type SYNCSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SYNCPOL` reader - SYNC polarity selection
pub type SYNCPOL_R = crate::BitReader;
///Field `SYNCPOL` writer - SYNC polarity selection
pub type SYNCPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Counter reload value
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - Frequency error limit
    #[inline(always)]
    pub fn felim(&self) -> FELIM_R {
        FELIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:26 - SYNC divider
    #[inline(always)]
    pub fn syncdiv(&self) -> SYNCDIV_R {
        SYNCDIV_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:29 - SYNC signal source selection
    #[inline(always)]
    pub fn syncsrc(&self) -> SYNCSRC_R {
        SYNCSRC_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 31 - SYNC polarity selection
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("syncpol", &self.syncpol())
            .field("syncsrc", &self.syncsrc())
            .field("syncdiv", &self.syncdiv())
            .field("felim", &self.felim())
            .field("reload", &self.reload())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Counter reload value
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W<CFGRrs> {
        RELOAD_W::new(self, 0)
    }
    ///Bits 16:23 - Frequency error limit
    #[inline(always)]
    pub fn felim(&mut self) -> FELIM_W<CFGRrs> {
        FELIM_W::new(self, 16)
    }
    ///Bits 24:26 - SYNC divider
    #[inline(always)]
    pub fn syncdiv(&mut self) -> SYNCDIV_W<CFGRrs> {
        SYNCDIV_W::new(self, 24)
    }
    ///Bits 28:29 - SYNC signal source selection
    #[inline(always)]
    pub fn syncsrc(&mut self) -> SYNCSRC_W<CFGRrs> {
        SYNCSRC_W::new(self, 28)
    }
    ///Bit 31 - SYNC polarity selection
    #[inline(always)]
    pub fn syncpol(&mut self) -> SYNCPOL_W<CFGRrs> {
        SYNCPOL_W::new(self, 31)
    }
}
/**configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#CRS:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0x2022_bb7f
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0x2022_bb7f;
}
