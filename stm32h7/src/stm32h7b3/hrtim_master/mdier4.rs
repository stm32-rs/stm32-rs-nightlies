///Register `MDIER4` reader
pub type R = crate::R<MDIER4rs>;
///Register `MDIER4` writer
pub type W = crate::W<MDIER4rs>;
///Field `MCMP1IE` reader - MCMP1IE
pub type MCMP1IE_R = crate::BitReader;
///Field `MCMP1IE` writer - MCMP1IE
pub type MCMP1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCMP2IE` reader - MCMP2IE
pub type MCMP2IE_R = crate::BitReader;
///Field `MCMP2IE` writer - MCMP2IE
pub type MCMP2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCMP3IE` reader - MCMP3IE
pub type MCMP3IE_R = crate::BitReader;
///Field `MCMP3IE` writer - MCMP3IE
pub type MCMP3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCMP4IE` reader - MCMP4IE
pub type MCMP4IE_R = crate::BitReader;
///Field `MCMP4IE` writer - MCMP4IE
pub type MCMP4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MREPIE` reader - MREPIE
pub type MREPIE_R = crate::BitReader;
///Field `MREPIE` writer - MREPIE
pub type MREPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCIE` reader - SYNCIE
pub type SYNCIE_R = crate::BitReader;
///Field `SYNCIE` writer - SYNCIE
pub type SYNCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUPDIE` reader - MUPDIE
pub type MUPDIE_R = crate::BitReader;
///Field `MUPDIE` writer - MUPDIE
pub type MUPDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCMP1DE` reader - MCMP1DE
pub type MCMP1DE_R = crate::BitReader;
///Field `MCMP1DE` writer - MCMP1DE
pub type MCMP1DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCMP2DE` reader - MCMP2DE
pub type MCMP2DE_R = crate::BitReader;
///Field `MCMP2DE` writer - MCMP2DE
pub type MCMP2DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCMP3DE` reader - MCMP3DE
pub type MCMP3DE_R = crate::BitReader;
///Field `MCMP3DE` writer - MCMP3DE
pub type MCMP3DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCMP4DE` reader - MCMP4DE
pub type MCMP4DE_R = crate::BitReader;
///Field `MCMP4DE` writer - MCMP4DE
pub type MCMP4DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MREPDE` reader - MREPDE
pub type MREPDE_R = crate::BitReader;
///Field `MREPDE` writer - MREPDE
pub type MREPDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCDE` reader - SYNCDE
pub type SYNCDE_R = crate::BitReader;
///Field `SYNCDE` writer - SYNCDE
pub type SYNCDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUPDDE` reader - MUPDDE
pub type MUPDDE_R = crate::BitReader;
///Field `MUPDDE` writer - MUPDDE
pub type MUPDDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MCMP1IE
    #[inline(always)]
    pub fn mcmp1ie(&self) -> MCMP1IE_R {
        MCMP1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MCMP2IE
    #[inline(always)]
    pub fn mcmp2ie(&self) -> MCMP2IE_R {
        MCMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MCMP3IE
    #[inline(always)]
    pub fn mcmp3ie(&self) -> MCMP3IE_R {
        MCMP3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MCMP4IE
    #[inline(always)]
    pub fn mcmp4ie(&self) -> MCMP4IE_R {
        MCMP4IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MREPIE
    #[inline(always)]
    pub fn mrepie(&self) -> MREPIE_R {
        MREPIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SYNCIE
    #[inline(always)]
    pub fn syncie(&self) -> SYNCIE_R {
        SYNCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MUPDIE
    #[inline(always)]
    pub fn mupdie(&self) -> MUPDIE_R {
        MUPDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - MCMP1DE
    #[inline(always)]
    pub fn mcmp1de(&self) -> MCMP1DE_R {
        MCMP1DE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - MCMP2DE
    #[inline(always)]
    pub fn mcmp2de(&self) -> MCMP2DE_R {
        MCMP2DE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - MCMP3DE
    #[inline(always)]
    pub fn mcmp3de(&self) -> MCMP3DE_R {
        MCMP3DE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - MCMP4DE
    #[inline(always)]
    pub fn mcmp4de(&self) -> MCMP4DE_R {
        MCMP4DE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - MREPDE
    #[inline(always)]
    pub fn mrepde(&self) -> MREPDE_R {
        MREPDE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SYNCDE
    #[inline(always)]
    pub fn syncde(&self) -> SYNCDE_R {
        SYNCDE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - MUPDDE
    #[inline(always)]
    pub fn mupdde(&self) -> MUPDDE_R {
        MUPDDE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDIER4")
            .field("mupdde", &self.mupdde())
            .field("syncde", &self.syncde())
            .field("mrepde", &self.mrepde())
            .field("mcmp4de", &self.mcmp4de())
            .field("mcmp3de", &self.mcmp3de())
            .field("mcmp2de", &self.mcmp2de())
            .field("mcmp1de", &self.mcmp1de())
            .field("mupdie", &self.mupdie())
            .field("syncie", &self.syncie())
            .field("mrepie", &self.mrepie())
            .field("mcmp4ie", &self.mcmp4ie())
            .field("mcmp3ie", &self.mcmp3ie())
            .field("mcmp2ie", &self.mcmp2ie())
            .field("mcmp1ie", &self.mcmp1ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - MCMP1IE
    #[inline(always)]
    #[must_use]
    pub fn mcmp1ie(&mut self) -> MCMP1IE_W<MDIER4rs> {
        MCMP1IE_W::new(self, 0)
    }
    ///Bit 1 - MCMP2IE
    #[inline(always)]
    #[must_use]
    pub fn mcmp2ie(&mut self) -> MCMP2IE_W<MDIER4rs> {
        MCMP2IE_W::new(self, 1)
    }
    ///Bit 2 - MCMP3IE
    #[inline(always)]
    #[must_use]
    pub fn mcmp3ie(&mut self) -> MCMP3IE_W<MDIER4rs> {
        MCMP3IE_W::new(self, 2)
    }
    ///Bit 3 - MCMP4IE
    #[inline(always)]
    #[must_use]
    pub fn mcmp4ie(&mut self) -> MCMP4IE_W<MDIER4rs> {
        MCMP4IE_W::new(self, 3)
    }
    ///Bit 4 - MREPIE
    #[inline(always)]
    #[must_use]
    pub fn mrepie(&mut self) -> MREPIE_W<MDIER4rs> {
        MREPIE_W::new(self, 4)
    }
    ///Bit 5 - SYNCIE
    #[inline(always)]
    #[must_use]
    pub fn syncie(&mut self) -> SYNCIE_W<MDIER4rs> {
        SYNCIE_W::new(self, 5)
    }
    ///Bit 6 - MUPDIE
    #[inline(always)]
    #[must_use]
    pub fn mupdie(&mut self) -> MUPDIE_W<MDIER4rs> {
        MUPDIE_W::new(self, 6)
    }
    ///Bit 16 - MCMP1DE
    #[inline(always)]
    #[must_use]
    pub fn mcmp1de(&mut self) -> MCMP1DE_W<MDIER4rs> {
        MCMP1DE_W::new(self, 16)
    }
    ///Bit 17 - MCMP2DE
    #[inline(always)]
    #[must_use]
    pub fn mcmp2de(&mut self) -> MCMP2DE_W<MDIER4rs> {
        MCMP2DE_W::new(self, 17)
    }
    ///Bit 18 - MCMP3DE
    #[inline(always)]
    #[must_use]
    pub fn mcmp3de(&mut self) -> MCMP3DE_W<MDIER4rs> {
        MCMP3DE_W::new(self, 18)
    }
    ///Bit 19 - MCMP4DE
    #[inline(always)]
    #[must_use]
    pub fn mcmp4de(&mut self) -> MCMP4DE_W<MDIER4rs> {
        MCMP4DE_W::new(self, 19)
    }
    ///Bit 20 - MREPDE
    #[inline(always)]
    #[must_use]
    pub fn mrepde(&mut self) -> MREPDE_W<MDIER4rs> {
        MREPDE_W::new(self, 20)
    }
    ///Bit 21 - SYNCDE
    #[inline(always)]
    #[must_use]
    pub fn syncde(&mut self) -> SYNCDE_W<MDIER4rs> {
        SYNCDE_W::new(self, 21)
    }
    ///Bit 22 - MUPDDE
    #[inline(always)]
    #[must_use]
    pub fn mupdde(&mut self) -> MUPDDE_W<MDIER4rs> {
        MUPDDE_W::new(self, 22)
    }
}
/**MDIER4

You can [`read`](crate::Reg::read) this register and get [`mdier4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdier4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_Master:MDIER4)*/
pub struct MDIER4rs;
impl crate::RegisterSpec for MDIER4rs {
    type Ux = u32;
}
///`read()` method returns [`mdier4::R`](R) reader structure
impl crate::Readable for MDIER4rs {}
///`write(|w| ..)` method takes [`mdier4::W`](W) writer structure
impl crate::Writable for MDIER4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MDIER4 to value 0
impl crate::Resettable for MDIER4rs {
    const RESET_VALUE: u32 = 0;
}
