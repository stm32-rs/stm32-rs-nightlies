///Register `TXEFS` reader
pub type R = crate::R<TXEFSrs>;
///Register `TXEFS` writer
pub type W = crate::W<TXEFSrs>;
///Field `EFFL` reader - Event FIFO Fill Level
pub type EFFL_R = crate::FieldReader;
///Field `EFFL` writer - Event FIFO Fill Level
pub type EFFL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `EFGI` reader - Event FIFO Get Index.
pub type EFGI_R = crate::FieldReader;
///Field `EFGI` writer - Event FIFO Get Index.
pub type EFGI_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `EFPI` reader - Event FIFO put index.
pub type EFPI_R = crate::FieldReader;
///Field `EFPI` writer - Event FIFO put index.
pub type EFPI_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `EFF` reader - Event FIFO Full.
pub type EFF_R = crate::BitReader;
///Field `EFF` writer - Event FIFO Full.
pub type EFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFL` reader - Tx Event FIFO Element Lost.
pub type TEFL_R = crate::BitReader;
///Field `TEFL` writer - Tx Event FIFO Element Lost.
pub type TEFL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - Event FIFO Fill Level
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:12 - Event FIFO Get Index.
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - Event FIFO put index.
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 24 - Event FIFO Full.
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Tx Event FIFO Element Lost.
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXEFS")
            .field("effl", &self.effl())
            .field("efgi", &self.efgi())
            .field("efpi", &self.efpi())
            .field("eff", &self.eff())
            .field("tefl", &self.tefl())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Event FIFO Fill Level
    #[inline(always)]
    pub fn effl(&mut self) -> EFFL_W<'_, TXEFSrs> {
        EFFL_W::new(self, 0)
    }
    ///Bits 8:12 - Event FIFO Get Index.
    #[inline(always)]
    pub fn efgi(&mut self) -> EFGI_W<'_, TXEFSrs> {
        EFGI_W::new(self, 8)
    }
    ///Bits 16:20 - Event FIFO put index.
    #[inline(always)]
    pub fn efpi(&mut self) -> EFPI_W<'_, TXEFSrs> {
        EFPI_W::new(self, 16)
    }
    ///Bit 24 - Event FIFO Full.
    #[inline(always)]
    pub fn eff(&mut self) -> EFF_W<'_, TXEFSrs> {
        EFF_W::new(self, 24)
    }
    ///Bit 25 - Tx Event FIFO Element Lost.
    #[inline(always)]
    pub fn tefl(&mut self) -> TEFL_W<'_, TXEFSrs> {
        TEFL_W::new(self, 25)
    }
}
/**FDCAN Tx Event FIFO Status Register

You can [`read`](crate::Reg::read) this register and get [`txefs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#FDCAN1:TXEFS)*/
pub struct TXEFSrs;
impl crate::RegisterSpec for TXEFSrs {
    type Ux = u32;
}
///`read()` method returns [`txefs::R`](R) reader structure
impl crate::Readable for TXEFSrs {}
///`write(|w| ..)` method takes [`txefs::W`](W) writer structure
impl crate::Writable for TXEFSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXEFS to value 0
impl crate::Resettable for TXEFSrs {}
