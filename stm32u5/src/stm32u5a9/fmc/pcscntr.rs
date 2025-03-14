///Register `PCSCNTR` reader
pub type R = crate::R<PCSCNTRrs>;
///Register `PCSCNTR` writer
pub type W = crate::W<PCSCNTRrs>;
///Field `CSCOUNT` reader - Chip select counter
pub type CSCOUNT_R = crate::FieldReader<u16>;
///Field `CSCOUNT` writer - Chip select counter
pub type CSCOUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `CNTB1EN` reader - Counter Bank 1 enable
pub type CNTB1EN_R = crate::BitReader;
///Field `CNTB1EN` writer - Counter Bank 1 enable
pub type CNTB1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTB2EN` reader - Counter Bank 2 enable
pub type CNTB2EN_R = crate::BitReader;
///Field `CNTB2EN` writer - Counter Bank 2 enable
pub type CNTB2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTB3EN` reader - Counter Bank 3 enable
pub type CNTB3EN_R = crate::BitReader;
///Field `CNTB3EN` writer - Counter Bank 3 enable
pub type CNTB3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTB4EN` reader - Counter Bank 4 enable
pub type CNTB4EN_R = crate::BitReader;
///Field `CNTB4EN` writer - Counter Bank 4 enable
pub type CNTB4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Chip select counter
    #[inline(always)]
    pub fn cscount(&self) -> CSCOUNT_R {
        CSCOUNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - Counter Bank 1 enable
    #[inline(always)]
    pub fn cntb1en(&self) -> CNTB1EN_R {
        CNTB1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Counter Bank 2 enable
    #[inline(always)]
    pub fn cntb2en(&self) -> CNTB2EN_R {
        CNTB2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Counter Bank 3 enable
    #[inline(always)]
    pub fn cntb3en(&self) -> CNTB3EN_R {
        CNTB3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Counter Bank 4 enable
    #[inline(always)]
    pub fn cntb4en(&self) -> CNTB4EN_R {
        CNTB4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCSCNTR")
            .field("cscount", &self.cscount())
            .field("cntb1en", &self.cntb1en())
            .field("cntb2en", &self.cntb2en())
            .field("cntb3en", &self.cntb3en())
            .field("cntb4en", &self.cntb4en())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Chip select counter
    #[inline(always)]
    pub fn cscount(&mut self) -> CSCOUNT_W<PCSCNTRrs> {
        CSCOUNT_W::new(self, 0)
    }
    ///Bit 16 - Counter Bank 1 enable
    #[inline(always)]
    pub fn cntb1en(&mut self) -> CNTB1EN_W<PCSCNTRrs> {
        CNTB1EN_W::new(self, 16)
    }
    ///Bit 17 - Counter Bank 2 enable
    #[inline(always)]
    pub fn cntb2en(&mut self) -> CNTB2EN_W<PCSCNTRrs> {
        CNTB2EN_W::new(self, 17)
    }
    ///Bit 18 - Counter Bank 3 enable
    #[inline(always)]
    pub fn cntb3en(&mut self) -> CNTB3EN_W<PCSCNTRrs> {
        CNTB3EN_W::new(self, 18)
    }
    ///Bit 19 - Counter Bank 4 enable
    #[inline(always)]
    pub fn cntb4en(&mut self) -> CNTB4EN_W<PCSCNTRrs> {
        CNTB4EN_W::new(self, 19)
    }
}
/**PSRAM chip select counter register

You can [`read`](crate::Reg::read) this register and get [`pcscntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcscntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#FMC:PCSCNTR)*/
pub struct PCSCNTRrs;
impl crate::RegisterSpec for PCSCNTRrs {
    type Ux = u32;
}
///`read()` method returns [`pcscntr::R`](R) reader structure
impl crate::Readable for PCSCNTRrs {}
///`write(|w| ..)` method takes [`pcscntr::W`](W) writer structure
impl crate::Writable for PCSCNTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCSCNTR to value 0
impl crate::Resettable for PCSCNTRrs {}
