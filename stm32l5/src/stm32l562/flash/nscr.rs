///Register `NSCR` reader
pub type R = crate::R<NSCRrs>;
///Register `NSCR` writer
pub type W = crate::W<NSCRrs>;
///Field `NSPG` reader - NSPG
pub type NSPG_R = crate::BitReader;
///Field `NSPG` writer - NSPG
pub type NSPG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSPER` reader - NSPER
pub type NSPER_R = crate::BitReader;
///Field `NSPER` writer - NSPER
pub type NSPER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSMER1` reader - NSMER1
pub type NSMER1_R = crate::BitReader;
///Field `NSMER1` writer - NSMER1
pub type NSMER1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSPNB` reader - NSPNB
pub type NSPNB_R = crate::FieldReader;
///Field `NSPNB` writer - NSPNB
pub type NSPNB_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `NSBKER` reader - NSBKER
pub type NSBKER_R = crate::BitReader;
///Field `NSBKER` writer - NSBKER
pub type NSBKER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSMER2` reader - NSMER2
pub type NSMER2_R = crate::BitReader;
///Field `NSMER2` writer - NSMER2
pub type NSMER2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSSTRT` reader - Options modification start
pub type NSSTRT_R = crate::BitReader;
///Field `NSSTRT` writer - Options modification start
pub type NSSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTSTRT` reader - Options modification start
pub type OPTSTRT_R = crate::BitReader;
///Field `OPTSTRT` writer - Options modification start
pub type OPTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSEOPIE` reader - NSEOPIE
pub type NSEOPIE_R = crate::BitReader;
///Field `NSEOPIE` writer - NSEOPIE
pub type NSEOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSERRIE` reader - NSERRIE
pub type NSERRIE_R = crate::BitReader;
///Field `NSERRIE` writer - NSERRIE
pub type NSERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OBL_LAUNCH` reader - Force the option byte loading
pub type OBL_LAUNCH_R = crate::BitReader;
///Field `OBL_LAUNCH` writer - Force the option byte loading
pub type OBL_LAUNCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTLOCK` reader - Options Lock
pub type OPTLOCK_R = crate::BitReader;
///Field `OPTLOCK` writer - Options Lock
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSLOCK` reader - NSLOCK
pub type NSLOCK_R = crate::BitReader;
///Field `NSLOCK` writer - NSLOCK
pub type NSLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - NSPG
    #[inline(always)]
    pub fn nspg(&self) -> NSPG_R {
        NSPG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - NSPER
    #[inline(always)]
    pub fn nsper(&self) -> NSPER_R {
        NSPER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NSMER1
    #[inline(always)]
    pub fn nsmer1(&self) -> NSMER1_R {
        NSMER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:9 - NSPNB
    #[inline(always)]
    pub fn nspnb(&self) -> NSPNB_R {
        NSPNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    ///Bit 11 - NSBKER
    #[inline(always)]
    pub fn nsbker(&self) -> NSBKER_R {
        NSBKER_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - NSMER2
    #[inline(always)]
    pub fn nsmer2(&self) -> NSMER2_R {
        NSMER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Options modification start
    #[inline(always)]
    pub fn nsstrt(&self) -> NSSTRT_R {
        NSSTRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Options modification start
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 24 - NSEOPIE
    #[inline(always)]
    pub fn nseopie(&self) -> NSEOPIE_R {
        NSEOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - NSERRIE
    #[inline(always)]
    pub fn nserrie(&self) -> NSERRIE_R {
        NSERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 27 - Force the option byte loading
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 30 - Options Lock
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - NSLOCK
    #[inline(always)]
    pub fn nslock(&self) -> NSLOCK_R {
        NSLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NSCR")
            .field("nspg", &self.nspg())
            .field("nsper", &self.nsper())
            .field("nsmer1", &self.nsmer1())
            .field("nspnb", &self.nspnb())
            .field("nsbker", &self.nsbker())
            .field("nsmer2", &self.nsmer2())
            .field("nsstrt", &self.nsstrt())
            .field("optstrt", &self.optstrt())
            .field("nseopie", &self.nseopie())
            .field("nserrie", &self.nserrie())
            .field("obl_launch", &self.obl_launch())
            .field("optlock", &self.optlock())
            .field("nslock", &self.nslock())
            .finish()
    }
}
impl W {
    ///Bit 0 - NSPG
    #[inline(always)]
    pub fn nspg(&mut self) -> NSPG_W<'_, NSCRrs> {
        NSPG_W::new(self, 0)
    }
    ///Bit 1 - NSPER
    #[inline(always)]
    pub fn nsper(&mut self) -> NSPER_W<'_, NSCRrs> {
        NSPER_W::new(self, 1)
    }
    ///Bit 2 - NSMER1
    #[inline(always)]
    pub fn nsmer1(&mut self) -> NSMER1_W<'_, NSCRrs> {
        NSMER1_W::new(self, 2)
    }
    ///Bits 3:9 - NSPNB
    #[inline(always)]
    pub fn nspnb(&mut self) -> NSPNB_W<'_, NSCRrs> {
        NSPNB_W::new(self, 3)
    }
    ///Bit 11 - NSBKER
    #[inline(always)]
    pub fn nsbker(&mut self) -> NSBKER_W<'_, NSCRrs> {
        NSBKER_W::new(self, 11)
    }
    ///Bit 15 - NSMER2
    #[inline(always)]
    pub fn nsmer2(&mut self) -> NSMER2_W<'_, NSCRrs> {
        NSMER2_W::new(self, 15)
    }
    ///Bit 16 - Options modification start
    #[inline(always)]
    pub fn nsstrt(&mut self) -> NSSTRT_W<'_, NSCRrs> {
        NSSTRT_W::new(self, 16)
    }
    ///Bit 17 - Options modification start
    #[inline(always)]
    pub fn optstrt(&mut self) -> OPTSTRT_W<'_, NSCRrs> {
        OPTSTRT_W::new(self, 17)
    }
    ///Bit 24 - NSEOPIE
    #[inline(always)]
    pub fn nseopie(&mut self) -> NSEOPIE_W<'_, NSCRrs> {
        NSEOPIE_W::new(self, 24)
    }
    ///Bit 25 - NSERRIE
    #[inline(always)]
    pub fn nserrie(&mut self) -> NSERRIE_W<'_, NSCRrs> {
        NSERRIE_W::new(self, 25)
    }
    ///Bit 27 - Force the option byte loading
    #[inline(always)]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<'_, NSCRrs> {
        OBL_LAUNCH_W::new(self, 27)
    }
    ///Bit 30 - Options Lock
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W<'_, NSCRrs> {
        OPTLOCK_W::new(self, 30)
    }
    ///Bit 31 - NSLOCK
    #[inline(always)]
    pub fn nslock(&mut self) -> NSLOCK_W<'_, NSCRrs> {
        NSLOCK_W::new(self, 31)
    }
}
/**Flash non-secure control register

You can [`read`](crate::Reg::read) this register and get [`nscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#FLASH:NSCR)*/
pub struct NSCRrs;
impl crate::RegisterSpec for NSCRrs {
    type Ux = u32;
}
///`read()` method returns [`nscr::R`](R) reader structure
impl crate::Readable for NSCRrs {}
///`write(|w| ..)` method takes [`nscr::W`](W) writer structure
impl crate::Writable for NSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NSCR to value 0xc000_0000
impl crate::Resettable for NSCRrs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
