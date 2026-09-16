///Register `FLT0CR2` reader
pub type R = crate::R<FLT0CR2rs>;
///Register `FLT0CR2` writer
pub type W = crate::W<FLT0CR2rs>;
///Field `JEOCIE` reader - JEOCIE
pub type JEOCIE_R = crate::BitReader;
///Field `JEOCIE` writer - JEOCIE
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REOCIE` reader - REOCIE
pub type REOCIE_R = crate::BitReader;
///Field `REOCIE` writer - REOCIE
pub type REOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JOVRIE` reader - JOVRIE
pub type JOVRIE_R = crate::BitReader;
///Field `JOVRIE` writer - JOVRIE
pub type JOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROVRIE` reader - ROVRIE
pub type ROVRIE_R = crate::BitReader;
///Field `ROVRIE` writer - ROVRIE
pub type ROVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWDIE` reader - AWDIE
pub type AWDIE_R = crate::BitReader;
///Field `AWDIE` writer - AWDIE
pub type AWDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCDIE` reader - SCDIE
pub type SCDIE_R = crate::BitReader;
///Field `SCDIE` writer - SCDIE
pub type SCDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKABIE` reader - CKABIE
pub type CKABIE_R = crate::BitReader;
///Field `CKABIE` writer - CKABIE
pub type CKABIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXCH` reader - EXCH
pub type EXCH_R = crate::FieldReader;
///Field `EXCH` writer - EXCH
pub type EXCH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `AWDCH` reader - AWDCH
pub type AWDCH_R = crate::FieldReader;
///Field `AWDCH` writer - AWDCH
pub type AWDCH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - JEOCIE
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - REOCIE
    #[inline(always)]
    pub fn reocie(&self) -> REOCIE_R {
        REOCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - JOVRIE
    #[inline(always)]
    pub fn jovrie(&self) -> JOVRIE_R {
        JOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ROVRIE
    #[inline(always)]
    pub fn rovrie(&self) -> ROVRIE_R {
        ROVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AWDIE
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SCDIE
    #[inline(always)]
    pub fn scdie(&self) -> SCDIE_R {
        SCDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CKABIE
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:15 - EXCH
    #[inline(always)]
    pub fn exch(&self) -> EXCH_R {
        EXCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - AWDCH
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT0CR2")
            .field("jeocie", &self.jeocie())
            .field("reocie", &self.reocie())
            .field("jovrie", &self.jovrie())
            .field("rovrie", &self.rovrie())
            .field("awdie", &self.awdie())
            .field("scdie", &self.scdie())
            .field("ckabie", &self.ckabie())
            .field("exch", &self.exch())
            .field("awdch", &self.awdch())
            .finish()
    }
}
impl W {
    ///Bit 0 - JEOCIE
    #[inline(always)]
    pub fn jeocie(&mut self) -> JEOCIE_W<'_, FLT0CR2rs> {
        JEOCIE_W::new(self, 0)
    }
    ///Bit 1 - REOCIE
    #[inline(always)]
    pub fn reocie(&mut self) -> REOCIE_W<'_, FLT0CR2rs> {
        REOCIE_W::new(self, 1)
    }
    ///Bit 2 - JOVRIE
    #[inline(always)]
    pub fn jovrie(&mut self) -> JOVRIE_W<'_, FLT0CR2rs> {
        JOVRIE_W::new(self, 2)
    }
    ///Bit 3 - ROVRIE
    #[inline(always)]
    pub fn rovrie(&mut self) -> ROVRIE_W<'_, FLT0CR2rs> {
        ROVRIE_W::new(self, 3)
    }
    ///Bit 4 - AWDIE
    #[inline(always)]
    pub fn awdie(&mut self) -> AWDIE_W<'_, FLT0CR2rs> {
        AWDIE_W::new(self, 4)
    }
    ///Bit 5 - SCDIE
    #[inline(always)]
    pub fn scdie(&mut self) -> SCDIE_W<'_, FLT0CR2rs> {
        SCDIE_W::new(self, 5)
    }
    ///Bit 6 - CKABIE
    #[inline(always)]
    pub fn ckabie(&mut self) -> CKABIE_W<'_, FLT0CR2rs> {
        CKABIE_W::new(self, 6)
    }
    ///Bits 8:15 - EXCH
    #[inline(always)]
    pub fn exch(&mut self) -> EXCH_W<'_, FLT0CR2rs> {
        EXCH_W::new(self, 8)
    }
    ///Bits 16:23 - AWDCH
    #[inline(always)]
    pub fn awdch(&mut self) -> AWDCH_W<'_, FLT0CR2rs> {
        AWDCH_W::new(self, 16)
    }
}
/**DFSDM filter 0 control register 2

You can [`read`](crate::Reg::read) this register and get [`flt0cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt0cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:FLT0CR2)*/
pub struct FLT0CR2rs;
impl crate::RegisterSpec for FLT0CR2rs {
    type Ux = u32;
}
///`read()` method returns [`flt0cr2::R`](R) reader structure
impl crate::Readable for FLT0CR2rs {}
///`write(|w| ..)` method takes [`flt0cr2::W`](W) writer structure
impl crate::Writable for FLT0CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLT0CR2 to value 0
impl crate::Resettable for FLT0CR2rs {}
