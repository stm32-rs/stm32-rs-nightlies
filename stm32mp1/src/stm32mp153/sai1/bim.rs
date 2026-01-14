///Register `BIM` reader
pub type R = crate::R<BIMrs>;
///Register `BIM` writer
pub type W = crate::W<BIMrs>;
///Field `OVRUDRIE` reader - OVRUDRIE
pub type OVRUDRIE_R = crate::BitReader;
///Field `OVRUDRIE` writer - OVRUDRIE
pub type OVRUDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUTEDETIE` reader - MUTEDETIE
pub type MUTEDETIE_R = crate::BitReader;
///Field `MUTEDETIE` writer - MUTEDETIE
pub type MUTEDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WCKCFGIE` reader - WCKCFGIE
pub type WCKCFGIE_R = crate::BitReader;
///Field `WCKCFGIE` writer - WCKCFGIE
pub type WCKCFGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FREQIE` reader - FREQIE
pub type FREQIE_R = crate::BitReader;
///Field `FREQIE` writer - FREQIE
pub type FREQIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNRDYIE` reader - CNRDYIE
pub type CNRDYIE_R = crate::BitReader;
///Field `CNRDYIE` writer - CNRDYIE
pub type CNRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AFSDETIE` reader - AFSDETIE
pub type AFSDETIE_R = crate::BitReader;
///Field `AFSDETIE` writer - AFSDETIE
pub type AFSDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFSDETIE` reader - LFSDETIE
pub type LFSDETIE_R = crate::BitReader;
///Field `LFSDETIE` writer - LFSDETIE
pub type LFSDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - OVRUDRIE
    #[inline(always)]
    pub fn ovrudrie(&self) -> OVRUDRIE_R {
        OVRUDRIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MUTEDETIE
    #[inline(always)]
    pub fn mutedetie(&self) -> MUTEDETIE_R {
        MUTEDETIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WCKCFGIE
    #[inline(always)]
    pub fn wckcfgie(&self) -> WCKCFGIE_R {
        WCKCFGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FREQIE
    #[inline(always)]
    pub fn freqie(&self) -> FREQIE_R {
        FREQIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CNRDYIE
    #[inline(always)]
    pub fn cnrdyie(&self) -> CNRDYIE_R {
        CNRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AFSDETIE
    #[inline(always)]
    pub fn afsdetie(&self) -> AFSDETIE_R {
        AFSDETIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LFSDETIE
    #[inline(always)]
    pub fn lfsdetie(&self) -> LFSDETIE_R {
        LFSDETIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BIM")
            .field("ovrudrie", &self.ovrudrie())
            .field("mutedetie", &self.mutedetie())
            .field("wckcfgie", &self.wckcfgie())
            .field("freqie", &self.freqie())
            .field("cnrdyie", &self.cnrdyie())
            .field("afsdetie", &self.afsdetie())
            .field("lfsdetie", &self.lfsdetie())
            .finish()
    }
}
impl W {
    ///Bit 0 - OVRUDRIE
    #[inline(always)]
    pub fn ovrudrie(&mut self) -> OVRUDRIE_W<'_, BIMrs> {
        OVRUDRIE_W::new(self, 0)
    }
    ///Bit 1 - MUTEDETIE
    #[inline(always)]
    pub fn mutedetie(&mut self) -> MUTEDETIE_W<'_, BIMrs> {
        MUTEDETIE_W::new(self, 1)
    }
    ///Bit 2 - WCKCFGIE
    #[inline(always)]
    pub fn wckcfgie(&mut self) -> WCKCFGIE_W<'_, BIMrs> {
        WCKCFGIE_W::new(self, 2)
    }
    ///Bit 3 - FREQIE
    #[inline(always)]
    pub fn freqie(&mut self) -> FREQIE_W<'_, BIMrs> {
        FREQIE_W::new(self, 3)
    }
    ///Bit 4 - CNRDYIE
    #[inline(always)]
    pub fn cnrdyie(&mut self) -> CNRDYIE_W<'_, BIMrs> {
        CNRDYIE_W::new(self, 4)
    }
    ///Bit 5 - AFSDETIE
    #[inline(always)]
    pub fn afsdetie(&mut self) -> AFSDETIE_W<'_, BIMrs> {
        AFSDETIE_W::new(self, 5)
    }
    ///Bit 6 - LFSDETIE
    #[inline(always)]
    pub fn lfsdetie(&mut self) -> LFSDETIE_W<'_, BIMrs> {
        LFSDETIE_W::new(self, 6)
    }
}
/**Interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`bim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SAI1:BIM)*/
pub struct BIMrs;
impl crate::RegisterSpec for BIMrs {
    type Ux = u32;
}
///`read()` method returns [`bim::R`](R) reader structure
impl crate::Readable for BIMrs {}
///`write(|w| ..)` method takes [`bim::W`](W) writer structure
impl crate::Writable for BIMrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BIM to value 0
impl crate::Resettable for BIMrs {}
