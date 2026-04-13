///Register `C14BR1` reader
pub type R = crate::R<C14BR1rs>;
///Register `C14BR1` writer
pub type W = crate::W<C14BR1rs>;
///Field `BNDT` reader - block number of data bytes to transfer from the source
pub type BNDT_R = crate::FieldReader<u16>;
///Field `BNDT` writer - block number of data bytes to transfer from the source
pub type BNDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `BRC` reader - Block repeat counter
pub type BRC_R = crate::FieldReader<u16>;
///Field `BRC` writer - Block repeat counter
pub type BRC_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `SDEC` reader - source address decrement
pub type SDEC_R = crate::BitReader;
///Field `SDEC` writer - source address decrement
pub type SDEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDEC` reader - destination address decrement
pub type DDEC_R = crate::BitReader;
///Field `DDEC` writer - destination address decrement
pub type DDEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRSDEC` reader - Block repeat source address decrement
pub type BRSDEC_R = crate::BitReader;
///Field `BRSDEC` writer - Block repeat source address decrement
pub type BRSDEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRDDEC` reader - Block repeat destination address decrement
pub type BRDDEC_R = crate::BitReader;
///Field `BRDDEC` writer - Block repeat destination address decrement
pub type BRDDEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - block number of data bytes to transfer from the source
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:26 - Block repeat counter
    #[inline(always)]
    pub fn brc(&self) -> BRC_R {
        BRC_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    ///Bit 28 - source address decrement
    #[inline(always)]
    pub fn sdec(&self) -> SDEC_R {
        SDEC_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - destination address decrement
    #[inline(always)]
    pub fn ddec(&self) -> DDEC_R {
        DDEC_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Block repeat source address decrement
    #[inline(always)]
    pub fn brsdec(&self) -> BRSDEC_R {
        BRSDEC_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Block repeat destination address decrement
    #[inline(always)]
    pub fn brddec(&self) -> BRDDEC_R {
        BRDDEC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C14BR1")
            .field("bndt", &self.bndt())
            .field("brc", &self.brc())
            .field("sdec", &self.sdec())
            .field("ddec", &self.ddec())
            .field("brsdec", &self.brsdec())
            .field("brddec", &self.brddec())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - block number of data bytes to transfer from the source
    #[inline(always)]
    pub fn bndt(&mut self) -> BNDT_W<'_, C14BR1rs> {
        BNDT_W::new(self, 0)
    }
    ///Bits 16:26 - Block repeat counter
    #[inline(always)]
    pub fn brc(&mut self) -> BRC_W<'_, C14BR1rs> {
        BRC_W::new(self, 16)
    }
    ///Bit 28 - source address decrement
    #[inline(always)]
    pub fn sdec(&mut self) -> SDEC_W<'_, C14BR1rs> {
        SDEC_W::new(self, 28)
    }
    ///Bit 29 - destination address decrement
    #[inline(always)]
    pub fn ddec(&mut self) -> DDEC_W<'_, C14BR1rs> {
        DDEC_W::new(self, 29)
    }
    ///Bit 30 - Block repeat source address decrement
    #[inline(always)]
    pub fn brsdec(&mut self) -> BRSDEC_W<'_, C14BR1rs> {
        BRSDEC_W::new(self, 30)
    }
    ///Bit 31 - Block repeat destination address decrement
    #[inline(always)]
    pub fn brddec(&mut self) -> BRDDEC_W<'_, C14BR1rs> {
        BRDDEC_W::new(self, 31)
    }
}
/**GPDMA channel 14 alternate block register 1

You can [`read`](crate::Reg::read) this register and get [`c14br1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14br1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPDMA:C14BR1)*/
pub struct C14BR1rs;
impl crate::RegisterSpec for C14BR1rs {
    type Ux = u32;
}
///`read()` method returns [`c14br1::R`](R) reader structure
impl crate::Readable for C14BR1rs {}
///`write(|w| ..)` method takes [`c14br1::W`](W) writer structure
impl crate::Writable for C14BR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C14BR1 to value 0
impl crate::Resettable for C14BR1rs {}
