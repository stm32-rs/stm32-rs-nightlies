///Register `BIM` reader
pub type R = crate::R<BIMrs>;
///Register `BIM` writer
pub type W = crate::W<BIMrs>;
///Field `OVRUDRIE` reader - Overrun/underrun interrupt enable.
pub type OVRUDRIE_R = crate::BitReader;
///Field `OVRUDRIE` writer - Overrun/underrun interrupt enable.
pub type OVRUDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUTEDETIE` reader - Mute detection interrupt enable.
pub type MUTEDETIE_R = crate::BitReader;
///Field `MUTEDETIE` writer - Mute detection interrupt enable.
pub type MUTEDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WCKCFGIE` reader - Wrong clock configuration interrupt enable.
pub type WCKCFGIE_R = crate::BitReader;
///Field `WCKCFGIE` writer - Wrong clock configuration interrupt enable.
pub type WCKCFGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FREQIE` reader - FIFO request interrupt enable.
pub type FREQIE_R = crate::BitReader;
///Field `FREQIE` writer - FIFO request interrupt enable.
pub type FREQIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNRDYIE` reader - Codec not ready interrupt enable (AC'97).
pub type CNRDYIE_R = crate::BitReader;
///Field `CNRDYIE` writer - Codec not ready interrupt enable (AC'97).
pub type CNRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AFSDETIE` reader - Anticipated frame synchronization detection interrupt enable.
pub type AFSDETIE_R = crate::BitReader;
///Field `AFSDETIE` writer - Anticipated frame synchronization detection interrupt enable.
pub type AFSDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFSDETIE` reader - Late frame synchronization detection interrupt enable.
pub type LFSDETIE_R = crate::BitReader;
///Field `LFSDETIE` writer - Late frame synchronization detection interrupt enable.
pub type LFSDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Overrun/underrun interrupt enable.
    #[inline(always)]
    pub fn ovrudrie(&self) -> OVRUDRIE_R {
        OVRUDRIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Mute detection interrupt enable.
    #[inline(always)]
    pub fn mutedetie(&self) -> MUTEDETIE_R {
        MUTEDETIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wrong clock configuration interrupt enable.
    #[inline(always)]
    pub fn wckcfgie(&self) -> WCKCFGIE_R {
        WCKCFGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FIFO request interrupt enable.
    #[inline(always)]
    pub fn freqie(&self) -> FREQIE_R {
        FREQIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Codec not ready interrupt enable (AC'97).
    #[inline(always)]
    pub fn cnrdyie(&self) -> CNRDYIE_R {
        CNRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Anticipated frame synchronization detection interrupt enable.
    #[inline(always)]
    pub fn afsdetie(&self) -> AFSDETIE_R {
        AFSDETIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Late frame synchronization detection interrupt enable.
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
    ///Bit 0 - Overrun/underrun interrupt enable.
    #[inline(always)]
    pub fn ovrudrie(&mut self) -> OVRUDRIE_W<BIMrs> {
        OVRUDRIE_W::new(self, 0)
    }
    ///Bit 1 - Mute detection interrupt enable.
    #[inline(always)]
    pub fn mutedetie(&mut self) -> MUTEDETIE_W<BIMrs> {
        MUTEDETIE_W::new(self, 1)
    }
    ///Bit 2 - Wrong clock configuration interrupt enable.
    #[inline(always)]
    pub fn wckcfgie(&mut self) -> WCKCFGIE_W<BIMrs> {
        WCKCFGIE_W::new(self, 2)
    }
    ///Bit 3 - FIFO request interrupt enable.
    #[inline(always)]
    pub fn freqie(&mut self) -> FREQIE_W<BIMrs> {
        FREQIE_W::new(self, 3)
    }
    ///Bit 4 - Codec not ready interrupt enable (AC'97).
    #[inline(always)]
    pub fn cnrdyie(&mut self) -> CNRDYIE_W<BIMrs> {
        CNRDYIE_W::new(self, 4)
    }
    ///Bit 5 - Anticipated frame synchronization detection interrupt enable.
    #[inline(always)]
    pub fn afsdetie(&mut self) -> AFSDETIE_W<BIMrs> {
        AFSDETIE_W::new(self, 5)
    }
    ///Bit 6 - Late frame synchronization detection interrupt enable.
    #[inline(always)]
    pub fn lfsdetie(&mut self) -> LFSDETIE_W<BIMrs> {
        LFSDETIE_W::new(self, 6)
    }
}
/**SAI interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`bim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#SAI1:BIM)*/
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
