///Register `SR3` reader
pub type R = crate::R<SR3rs>;
///Register `SR3` writer
pub type W = crate::W<SR3rs>;
///Field `IRS` reader - IRS
pub type IRS_R = crate::BitReader;
///Field `IRS` writer - IRS
pub type IRS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ILS` reader - ILS
pub type ILS_R = crate::BitReader;
///Field `ILS` writer - ILS
pub type ILS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IFS` reader - IFS
pub type IFS_R = crate::BitReader;
///Field `IFS` writer - IFS
pub type IFS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IREN` reader - IREN
pub type IREN_R = crate::BitReader;
///Field `IREN` writer - IREN
pub type IREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ILEN` reader - ILEN
pub type ILEN_R = crate::BitReader;
///Field `ILEN` writer - ILEN
pub type ILEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IFEN` reader - IFEN
pub type IFEN_R = crate::BitReader;
///Field `IFEN` writer - IFEN
pub type IFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FEMPT` reader - FEMPT
pub type FEMPT_R = crate::BitReader;
impl R {
    ///Bit 0 - IRS
    #[inline(always)]
    pub fn irs(&self) -> IRS_R {
        IRS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ILS
    #[inline(always)]
    pub fn ils(&self) -> ILS_R {
        ILS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IFS
    #[inline(always)]
    pub fn ifs(&self) -> IFS_R {
        IFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IREN
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ILEN
    #[inline(always)]
    pub fn ilen(&self) -> ILEN_R {
        ILEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IFEN
    #[inline(always)]
    pub fn ifen(&self) -> IFEN_R {
        IFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - FEMPT
    #[inline(always)]
    pub fn fempt(&self) -> FEMPT_R {
        FEMPT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR3")
            .field("fempt", &self.fempt())
            .field("ifen", &self.ifen())
            .field("ilen", &self.ilen())
            .field("iren", &self.iren())
            .field("ifs", &self.ifs())
            .field("ils", &self.ils())
            .field("irs", &self.irs())
            .finish()
    }
}
impl W {
    ///Bit 0 - IRS
    #[inline(always)]
    pub fn irs(&mut self) -> IRS_W<SR3rs> {
        IRS_W::new(self, 0)
    }
    ///Bit 1 - ILS
    #[inline(always)]
    pub fn ils(&mut self) -> ILS_W<SR3rs> {
        ILS_W::new(self, 1)
    }
    ///Bit 2 - IFS
    #[inline(always)]
    pub fn ifs(&mut self) -> IFS_W<SR3rs> {
        IFS_W::new(self, 2)
    }
    ///Bit 3 - IREN
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W<SR3rs> {
        IREN_W::new(self, 3)
    }
    ///Bit 4 - ILEN
    #[inline(always)]
    pub fn ilen(&mut self) -> ILEN_W<SR3rs> {
        ILEN_W::new(self, 4)
    }
    ///Bit 5 - IFEN
    #[inline(always)]
    pub fn ifen(&mut self) -> IFEN_W<SR3rs> {
        IFEN_W::new(self, 5)
    }
}
/**FIFO status and interrupt register 3

You can [`read`](crate::Reg::read) this register and get [`sr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#FMC:SR3)*/
pub struct SR3rs;
impl crate::RegisterSpec for SR3rs {
    type Ux = u32;
}
///`read()` method returns [`sr3::R`](R) reader structure
impl crate::Readable for SR3rs {}
///`write(|w| ..)` method takes [`sr3::W`](W) writer structure
impl crate::Writable for SR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR3 to value 0x40
impl crate::Resettable for SR3rs {
    const RESET_VALUE: u32 = 0x40;
}
