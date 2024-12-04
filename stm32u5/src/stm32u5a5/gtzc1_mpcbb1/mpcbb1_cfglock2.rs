///Register `MPCBB1_CFGLOCK2` reader
pub type R = crate::R<MPCBB1_CFGLOCK2rs>;
///Register `MPCBB1_CFGLOCK2` writer
pub type W = crate::W<MPCBB1_CFGLOCK2rs>;
///Field `SPLCK32` reader - SPLCK32
pub type SPLCK32_R = crate::BitReader;
///Field `SPLCK32` writer - SPLCK32
pub type SPLCK32_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK33` reader - SPLCK33
pub type SPLCK33_R = crate::BitReader;
///Field `SPLCK33` writer - SPLCK33
pub type SPLCK33_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK34` reader - SPLCK34
pub type SPLCK34_R = crate::BitReader;
///Field `SPLCK34` writer - SPLCK34
pub type SPLCK34_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK35` reader - SPLCK35
pub type SPLCK35_R = crate::BitReader;
///Field `SPLCK35` writer - SPLCK35
pub type SPLCK35_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK36` reader - SPLCK36
pub type SPLCK36_R = crate::BitReader;
///Field `SPLCK36` writer - SPLCK36
pub type SPLCK36_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK37` reader - SPLCK37
pub type SPLCK37_R = crate::BitReader;
///Field `SPLCK37` writer - SPLCK37
pub type SPLCK37_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK38` reader - SPLCK38
pub type SPLCK38_R = crate::BitReader;
///Field `SPLCK38` writer - SPLCK38
pub type SPLCK38_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK39` reader - SPLCK39
pub type SPLCK39_R = crate::BitReader;
///Field `SPLCK39` writer - SPLCK39
pub type SPLCK39_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK40` reader - SPLCK40
pub type SPLCK40_R = crate::BitReader;
///Field `SPLCK40` writer - SPLCK40
pub type SPLCK40_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK41` reader - SPLCK41
pub type SPLCK41_R = crate::BitReader;
///Field `SPLCK41` writer - SPLCK41
pub type SPLCK41_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK42` reader - SPLCK42
pub type SPLCK42_R = crate::BitReader;
///Field `SPLCK42` writer - SPLCK42
pub type SPLCK42_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK43` reader - SPLCK43
pub type SPLCK43_R = crate::BitReader;
///Field `SPLCK43` writer - SPLCK43
pub type SPLCK43_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK44` reader - SPLCK44
pub type SPLCK44_R = crate::BitReader;
///Field `SPLCK44` writer - SPLCK44
pub type SPLCK44_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK45` reader - SPLCK45
pub type SPLCK45_R = crate::BitReader;
///Field `SPLCK45` writer - SPLCK45
pub type SPLCK45_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK46` reader - SPLCK46
pub type SPLCK46_R = crate::BitReader;
///Field `SPLCK46` writer - SPLCK46
pub type SPLCK46_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK47` reader - SPLCK47
pub type SPLCK47_R = crate::BitReader;
///Field `SPLCK47` writer - SPLCK47
pub type SPLCK47_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK48` reader - SPLCK48
pub type SPLCK48_R = crate::BitReader;
///Field `SPLCK48` writer - SPLCK48
pub type SPLCK48_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK49` reader - SPLCK49
pub type SPLCK49_R = crate::BitReader;
///Field `SPLCK49` writer - SPLCK49
pub type SPLCK49_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK50` reader - SPLCK50
pub type SPLCK50_R = crate::BitReader;
///Field `SPLCK50` writer - SPLCK50
pub type SPLCK50_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLCK51` reader - SPLCK51
pub type SPLCK51_R = crate::BitReader;
///Field `SPLCK51` writer - SPLCK51
pub type SPLCK51_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SPLCK32
    #[inline(always)]
    pub fn splck32(&self) -> SPLCK32_R {
        SPLCK32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SPLCK33
    #[inline(always)]
    pub fn splck33(&self) -> SPLCK33_R {
        SPLCK33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SPLCK34
    #[inline(always)]
    pub fn splck34(&self) -> SPLCK34_R {
        SPLCK34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SPLCK35
    #[inline(always)]
    pub fn splck35(&self) -> SPLCK35_R {
        SPLCK35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SPLCK36
    #[inline(always)]
    pub fn splck36(&self) -> SPLCK36_R {
        SPLCK36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SPLCK37
    #[inline(always)]
    pub fn splck37(&self) -> SPLCK37_R {
        SPLCK37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SPLCK38
    #[inline(always)]
    pub fn splck38(&self) -> SPLCK38_R {
        SPLCK38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SPLCK39
    #[inline(always)]
    pub fn splck39(&self) -> SPLCK39_R {
        SPLCK39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SPLCK40
    #[inline(always)]
    pub fn splck40(&self) -> SPLCK40_R {
        SPLCK40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SPLCK41
    #[inline(always)]
    pub fn splck41(&self) -> SPLCK41_R {
        SPLCK41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SPLCK42
    #[inline(always)]
    pub fn splck42(&self) -> SPLCK42_R {
        SPLCK42_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SPLCK43
    #[inline(always)]
    pub fn splck43(&self) -> SPLCK43_R {
        SPLCK43_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPLCK44
    #[inline(always)]
    pub fn splck44(&self) -> SPLCK44_R {
        SPLCK44_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPLCK45
    #[inline(always)]
    pub fn splck45(&self) -> SPLCK45_R {
        SPLCK45_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SPLCK46
    #[inline(always)]
    pub fn splck46(&self) -> SPLCK46_R {
        SPLCK46_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPLCK47
    #[inline(always)]
    pub fn splck47(&self) -> SPLCK47_R {
        SPLCK47_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SPLCK48
    #[inline(always)]
    pub fn splck48(&self) -> SPLCK48_R {
        SPLCK48_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SPLCK49
    #[inline(always)]
    pub fn splck49(&self) -> SPLCK49_R {
        SPLCK49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SPLCK50
    #[inline(always)]
    pub fn splck50(&self) -> SPLCK50_R {
        SPLCK50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SPLCK51
    #[inline(always)]
    pub fn splck51(&self) -> SPLCK51_R {
        SPLCK51_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPCBB1_CFGLOCK2")
            .field("splck32", &self.splck32())
            .field("splck33", &self.splck33())
            .field("splck34", &self.splck34())
            .field("splck35", &self.splck35())
            .field("splck36", &self.splck36())
            .field("splck37", &self.splck37())
            .field("splck38", &self.splck38())
            .field("splck39", &self.splck39())
            .field("splck40", &self.splck40())
            .field("splck41", &self.splck41())
            .field("splck42", &self.splck42())
            .field("splck43", &self.splck43())
            .field("splck44", &self.splck44())
            .field("splck45", &self.splck45())
            .field("splck46", &self.splck46())
            .field("splck47", &self.splck47())
            .field("splck48", &self.splck48())
            .field("splck49", &self.splck49())
            .field("splck50", &self.splck50())
            .field("splck51", &self.splck51())
            .finish()
    }
}
impl W {
    ///Bit 0 - SPLCK32
    #[inline(always)]
    pub fn splck32(&mut self) -> SPLCK32_W<MPCBB1_CFGLOCK2rs> {
        SPLCK32_W::new(self, 0)
    }
    ///Bit 1 - SPLCK33
    #[inline(always)]
    pub fn splck33(&mut self) -> SPLCK33_W<MPCBB1_CFGLOCK2rs> {
        SPLCK33_W::new(self, 1)
    }
    ///Bit 2 - SPLCK34
    #[inline(always)]
    pub fn splck34(&mut self) -> SPLCK34_W<MPCBB1_CFGLOCK2rs> {
        SPLCK34_W::new(self, 2)
    }
    ///Bit 3 - SPLCK35
    #[inline(always)]
    pub fn splck35(&mut self) -> SPLCK35_W<MPCBB1_CFGLOCK2rs> {
        SPLCK35_W::new(self, 3)
    }
    ///Bit 4 - SPLCK36
    #[inline(always)]
    pub fn splck36(&mut self) -> SPLCK36_W<MPCBB1_CFGLOCK2rs> {
        SPLCK36_W::new(self, 4)
    }
    ///Bit 5 - SPLCK37
    #[inline(always)]
    pub fn splck37(&mut self) -> SPLCK37_W<MPCBB1_CFGLOCK2rs> {
        SPLCK37_W::new(self, 5)
    }
    ///Bit 6 - SPLCK38
    #[inline(always)]
    pub fn splck38(&mut self) -> SPLCK38_W<MPCBB1_CFGLOCK2rs> {
        SPLCK38_W::new(self, 6)
    }
    ///Bit 7 - SPLCK39
    #[inline(always)]
    pub fn splck39(&mut self) -> SPLCK39_W<MPCBB1_CFGLOCK2rs> {
        SPLCK39_W::new(self, 7)
    }
    ///Bit 8 - SPLCK40
    #[inline(always)]
    pub fn splck40(&mut self) -> SPLCK40_W<MPCBB1_CFGLOCK2rs> {
        SPLCK40_W::new(self, 8)
    }
    ///Bit 9 - SPLCK41
    #[inline(always)]
    pub fn splck41(&mut self) -> SPLCK41_W<MPCBB1_CFGLOCK2rs> {
        SPLCK41_W::new(self, 9)
    }
    ///Bit 10 - SPLCK42
    #[inline(always)]
    pub fn splck42(&mut self) -> SPLCK42_W<MPCBB1_CFGLOCK2rs> {
        SPLCK42_W::new(self, 10)
    }
    ///Bit 11 - SPLCK43
    #[inline(always)]
    pub fn splck43(&mut self) -> SPLCK43_W<MPCBB1_CFGLOCK2rs> {
        SPLCK43_W::new(self, 11)
    }
    ///Bit 12 - SPLCK44
    #[inline(always)]
    pub fn splck44(&mut self) -> SPLCK44_W<MPCBB1_CFGLOCK2rs> {
        SPLCK44_W::new(self, 12)
    }
    ///Bit 13 - SPLCK45
    #[inline(always)]
    pub fn splck45(&mut self) -> SPLCK45_W<MPCBB1_CFGLOCK2rs> {
        SPLCK45_W::new(self, 13)
    }
    ///Bit 14 - SPLCK46
    #[inline(always)]
    pub fn splck46(&mut self) -> SPLCK46_W<MPCBB1_CFGLOCK2rs> {
        SPLCK46_W::new(self, 14)
    }
    ///Bit 15 - SPLCK47
    #[inline(always)]
    pub fn splck47(&mut self) -> SPLCK47_W<MPCBB1_CFGLOCK2rs> {
        SPLCK47_W::new(self, 15)
    }
    ///Bit 16 - SPLCK48
    #[inline(always)]
    pub fn splck48(&mut self) -> SPLCK48_W<MPCBB1_CFGLOCK2rs> {
        SPLCK48_W::new(self, 16)
    }
    ///Bit 17 - SPLCK49
    #[inline(always)]
    pub fn splck49(&mut self) -> SPLCK49_W<MPCBB1_CFGLOCK2rs> {
        SPLCK49_W::new(self, 17)
    }
    ///Bit 18 - SPLCK50
    #[inline(always)]
    pub fn splck50(&mut self) -> SPLCK50_W<MPCBB1_CFGLOCK2rs> {
        SPLCK50_W::new(self, 18)
    }
    ///Bit 19 - SPLCK51
    #[inline(always)]
    pub fn splck51(&mut self) -> SPLCK51_W<MPCBB1_CFGLOCK2rs> {
        SPLCK51_W::new(self, 19)
    }
}
/**GTZC1 SRAMz MPCBB configuration lock register 2

You can [`read`](crate::Reg::read) this register and get [`mpcbb1_cfglock2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_cfglock2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#GTZC1_MPCBB1:MPCBB1_CFGLOCK2)*/
pub struct MPCBB1_CFGLOCK2rs;
impl crate::RegisterSpec for MPCBB1_CFGLOCK2rs {
    type Ux = u32;
}
///`read()` method returns [`mpcbb1_cfglock2::R`](R) reader structure
impl crate::Readable for MPCBB1_CFGLOCK2rs {}
///`write(|w| ..)` method takes [`mpcbb1_cfglock2::W`](W) writer structure
impl crate::Writable for MPCBB1_CFGLOCK2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MPCBB1_CFGLOCK2 to value 0
impl crate::Resettable for MPCBB1_CFGLOCK2rs {
    const RESET_VALUE: u32 = 0;
}
