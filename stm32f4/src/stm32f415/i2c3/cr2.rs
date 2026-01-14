///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `FREQ` reader - Peripheral clock frequency
pub type FREQ_R = crate::FieldReader;
///Field `FREQ` writer - Peripheral clock frequency
pub type FREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `ITERREN` reader - Error interrupt enable
pub type ITERREN_R = crate::BitReader;
///Field `ITERREN` writer - Error interrupt enable
pub type ITERREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITEVTEN` reader - Event interrupt enable
pub type ITEVTEN_R = crate::BitReader;
///Field `ITEVTEN` writer - Event interrupt enable
pub type ITEVTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITBUFEN` reader - Buffer interrupt enable
pub type ITBUFEN_R = crate::BitReader;
///Field `ITBUFEN` writer - Buffer interrupt enable
pub type ITBUFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAEN` reader - DMA requests enable
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - DMA requests enable
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LAST` reader - DMA last transfer
pub type LAST_R = crate::BitReader;
///Field `LAST` writer - DMA last transfer
pub type LAST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - Peripheral clock frequency
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 8 - Error interrupt enable
    #[inline(always)]
    pub fn iterren(&self) -> ITERREN_R {
        ITERREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Event interrupt enable
    #[inline(always)]
    pub fn itevten(&self) -> ITEVTEN_R {
        ITEVTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Buffer interrupt enable
    #[inline(always)]
    pub fn itbufen(&self) -> ITBUFEN_R {
        ITBUFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DMA requests enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DMA last transfer
    #[inline(always)]
    pub fn last(&self) -> LAST_R {
        LAST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("last", &self.last())
            .field("dmaen", &self.dmaen())
            .field("itbufen", &self.itbufen())
            .field("itevten", &self.itevten())
            .field("iterren", &self.iterren())
            .field("freq", &self.freq())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Peripheral clock frequency
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W<'_, CR2rs> {
        FREQ_W::new(self, 0)
    }
    ///Bit 8 - Error interrupt enable
    #[inline(always)]
    pub fn iterren(&mut self) -> ITERREN_W<'_, CR2rs> {
        ITERREN_W::new(self, 8)
    }
    ///Bit 9 - Event interrupt enable
    #[inline(always)]
    pub fn itevten(&mut self) -> ITEVTEN_W<'_, CR2rs> {
        ITEVTEN_W::new(self, 9)
    }
    ///Bit 10 - Buffer interrupt enable
    #[inline(always)]
    pub fn itbufen(&mut self) -> ITBUFEN_W<'_, CR2rs> {
        ITBUFEN_W::new(self, 10)
    }
    ///Bit 11 - DMA requests enable
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, CR2rs> {
        DMAEN_W::new(self, 11)
    }
    ///Bit 12 - DMA last transfer
    #[inline(always)]
    pub fn last(&mut self) -> LAST_W<'_, CR2rs> {
        LAST_W::new(self, 12)
    }
}
/**Control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#I2C3:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
